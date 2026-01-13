// Database models and operations for English Learning Companion

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Row};

use crate::models::{
    Conversation, ConversationAnnotation, ConversationAnnotationType, LearningSession, SpeakerType,
    UseLanguage, WordPracticeLog,
};

/// Database manager for English Learning Companion
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    /// Run migrations
    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    /// Get current Unix timestamp
    fn now() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    // ============ IssueWord Operations ============

    /// Insert a new issue word
    pub async fn insert_issue_word(&self, word: &IssueWord) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO issue_words (
                word, issue_type, description_en, description_zh, created_at, pick_count,
                review_interval_days, difficulty_level, context, audio_timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(word, issue_type) DO UPDATE SET
                description_en = excluded.description_en,
                description_zh = excluded.description_zh,
                context = excluded.context,
                audio_timestamp = excluded.audio_timestamp
            "#,
        )
        .bind(&word.word)
        .bind(word.issue_type.to_string())
        .bind(&word.description_en)
        .bind(&word.description_zh)
        .bind(word.created_at)
        .bind(word.pick_count)
        .bind(word.review_interval_days)
        .bind(word.difficulty_level)
        .bind(&word.context)
        .bind(word.audio_timestamp)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get words due for review (max 30, respecting daily frequency limit)
    pub async fn get_words_for_review(&self, limit: i64) -> Result<Vec<IssueWord>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                w.id, w.word, w.issue_type, w.description_en, w.description_zh, w.last_picked_at,
                w.created_at, w.pick_count, w.next_review_at, w.review_interval_days,
                w.difficulty_level, w.context, w.audio_timestamp,
                COALESCE(
                    (SELECT COUNT(*) FROM word_practice_log 
                     WHERE word_id = w.id 
                     AND practiced_at >= ?),
                    0
                ) as today_count
            FROM issue_words w
            WHERE 
                (w.next_review_at IS NULL OR w.next_review_at <= ?)
            HAVING today_count < 5
            ORDER BY 
                CASE WHEN w.next_review_at IS NULL THEN 0 ELSE 1 END,
                w.next_review_at ASC,
                w.difficulty_level DESC,
                w.created_at ASC
            LIMIT ?
            "#,
        )
        .bind(Self::now() - 86400) // 24 hours ago
        .bind(Self::now())
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut words = Vec::new();
        for row in rows {
            words.push(IssueWord {
                id: row.get("id"),
                word: row.get("word"),
                issue_type: row.get::<String, _>("issue_type").parse().unwrap(),
                description_en: row.get("description_en"),
                description_zh: row.get("description_zh"),
                last_picked_at: row.get("last_picked_at"),
                created_at: row.get("created_at"),
                pick_count: row.get("pick_count"),
                next_review_at: row.get("next_review_at"),
                review_interval_days: row.get("review_interval_days"),
                difficulty_level: row.get("difficulty_level"),
                context: row.get("context"),
                audio_timestamp: row.get("audio_timestamp"),
            });
        }

        Ok(words)
    }

    /// Update word after practice (implements spaced repetition)
    pub async fn update_word_after_practice(
        &self,
        word_id: i64,
        success: bool,
    ) -> Result<(), sqlx::Error> {
        let (new_interval, new_difficulty) = if success {
            // Increase interval on success (1 -> 2 -> 4 -> 7 -> 14 -> 30 days)
            let row = sqlx::query(
                "SELECT review_interval_days, difficulty_level FROM issue_words WHERE id = ?",
            )
            .bind(word_id)
            .fetch_one(&self.pool)
            .await?;

            let interval: i64 = row.get("review_interval_days");
            let diff: i64 = row.get("difficulty_level");

            let new_interval = match interval {
                1 => 2,
                2 => 4,
                4 => 7,
                7 => 14,
                14 => 30,
                _ => 30,
            };
            let new_diff = (diff - 1).max(1);
            (new_interval, new_diff)
        } else {
            // Reset interval on failure, increase difficulty
            let diff = sqlx::query_scalar::<_, i64>(
                "SELECT difficulty_level FROM issue_words WHERE id = ?",
            )
            .bind(word_id)
            .fetch_one(&self.pool)
            .await?;

            (1, (diff + 1).min(5))
        };

        let next_review = Self::now() + (new_interval * 86400);

        sqlx::query(
            r#"
            UPDATE issue_words 
            SET last_picked_at = ?,
                pick_count = pick_count + 1,
                next_review_at = ?,
                review_interval_days = ?,
                difficulty_level = ?
            WHERE id = ?
            "#,
        )
        .bind(Self::now())
        .bind(next_review)
        .bind(new_interval)
        .bind(new_difficulty)
        .bind(word_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ============ Conversation Operations ============

    /// Insert a new conversation entry
    pub async fn insert_conversation(&self, conv: &Conversation) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO conversations (
                session_id, speaker, use_lang, content_en, content_zh, audio_path, created_at,
                duration_ms, words_per_minute, pause_count, hesitation_count
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&conv.session_id)
        .bind(conv.speaker.to_string())
        .bind(conv.use_lang.to_string())
        .bind(&conv.content_en)
        .bind(&conv.content_zh)
        .bind(&conv.audio_path)
        .bind(conv.created_at)
        .bind(conv.duration_ms)
        .bind(conv.words_per_minute)
        .bind(conv.pause_count)
        .bind(conv.hesitation_count)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get conversation history for a session
    pub async fn get_conversation_history(
        &self,
        session_id: &str,
        limit: i64,
    ) -> Result<Vec<Conversation>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM conversations 
            WHERE session_id = ?
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(session_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut conversations = Vec::new();
        for row in rows {
            conversations.push(Conversation {
                id: row.get("id"),
                session_id: row.get("session_id"),
                speaker: row.get::<String, _>("speaker").parse().unwrap(),
                use_lang: row.get::<String, _>("use_lang").parse().unwrap(),
                content_en: row.get("content_en"),
                content_zh: row.get("content_zh"),
                audio_path: row.get("audio_path"),
                created_at: row.get("created_at"),
                duration_ms: row.get("duration_ms"),
                words_per_minute: row.get("words_per_minute"),
                pause_count: row.get("pause_count"),
                hesitation_count: row.get("hesitation_count"),
            });
        }

        Ok(conversations)
    }

    // ============ Annotation Operations ============

    /// Insert a conversation annotation
    pub async fn insert_annotation(
        &self,
        annotation: &ConversationAnnotation,
    ) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO conversation_annotations (
                conversation_id, annotation_type, start_position, end_position,
                original_text, suggested_text, description_en, description_zh, severity, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(annotation.conversation_id)
        .bind(annotation.annotation_type.to_string())
        .bind(annotation.start_position)
        .bind(annotation.end_position)
        .bind(&annotation.original_text)
        .bind(&annotation.suggested_text)
        .bind(&annotation.description_en)
        .bind(&annotation.description_zh)
        .bind(annotation.severity.to_string())
        .bind(annotation.created_at)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get annotations for a conversation
    pub async fn get_annotations(
        &self,
        conversation_id: i64,
    ) -> Result<Vec<ConversationAnnotation>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM conversation_annotations WHERE conversation_id = ?")
            .bind(conversation_id)
            .fetch_all(&self.pool)
            .await?;

        let mut annotations = Vec::new();
        for row in rows {
            annotations.push(ConversationAnnotation {
                id: row.get("id"),
                conversation_id: row.get("conversation_id"),
                annotation_type: row.get::<String, _>("annotation_type").parse().unwrap(),
                start_position: row.get("start_position"),
                end_position: row.get("end_position"),
                original_text: row.get("original_text"),
                suggested_text: row.get("suggested_text"),
                description_en: row.get("description_en"),
                description_zh: row.get("description_zh"),
                severity: row.get::<String, _>("severity").parse().unwrap(),
                created_at: row.get("created_at"),
            });
        }

        Ok(annotations)
    }

    // ============ Learning Session Operations ============

    /// Create a new learning session
    pub async fn create_session(&self, session: &LearningSession) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO learning_sessions (
                session_id, topic, target_words, started_at, total_exchanges
            ) VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&session.session_id)
        .bind(&session.topic)
        .bind(&session.target_words)
        .bind(session.started_at)
        .bind(session.total_exchanges)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Update session on completion
    pub async fn end_session(
        &self,
        session_id: &str,
        satisfaction: Option<i64>,
        notes: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE learning_sessions 
            SET ended_at = ?, user_satisfaction = ?, notes = ?
            WHERE session_id = ?
            "#,
        )
        .bind(Self::now())
        .bind(satisfaction)
        .bind(notes)
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ============ Word Practice Log Operations ============

    /// Log a word practice
    pub async fn log_word_practice(&self, log: &WordPracticeLog) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO word_practice_log (
                word_id, session_id, practiced_at, success_level, notes
            ) VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(log.word_id)
        .bind(&log.session_id)
        .bind(log.practiced_at)
        .bind(log.success_level)
        .bind(&log.notes)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }
}
