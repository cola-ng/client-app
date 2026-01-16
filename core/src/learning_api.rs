//! Learning content API client
//!
//! This module provides a client for fetching learning content from the backend server.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct LearningApiClient {
    client: Client,
    base_url: String,
    auth_token: Option<String>,
}

// ============================================================================
// API Response Types (Shared Content)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenes {
    pub id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub icon_emoji: Option<String>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDialogue {
    pub id: i64,
    pub sceneid: i64,
    pub title_en: String,
    pub title_zh: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub total_turns: Option<i32>,
    pub estimated_duration_seconds: Option<i32>,
    pub difficulty_level: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTurn {
    pub id: i64,
    pub scene_dialogue_id: i64,
    pub turn_number: i32,
    pub speaker_role: String,
    pub speaker_name: Option<String>,
    pub content_en: String,
    pub content_zh: String,
    pub audio_path: Option<String>,
    pub phonetic_transcription: Option<String>,
    pub key_phrases: Option<Value>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicDialogueSource {
    pub id: i64,
    pub source_type: String,
    pub title: String,
    pub year: Option<i32>,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub thumbnail_url: Option<String>,
    pub imdb_id: Option<String>,
    pub difficulty_level: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicDialogueClip {
    pub id: i64,
    pub source_id: i64,
    pub clip_title_en: String,
    pub clip_title_zh: String,
    pub start_time_seconds: Option<i32>,
    pub end_time_seconds: Option<i32>,
    pub video_url: Option<String>,
    pub transcript_en: String,
    pub transcript_zh: String,
    pub key_vocabulary: Option<Value>,
    pub cultural_notes: Option<String>,
    pub grammar_points: Option<Value>,
    pub difficulty_vocab: Option<i32>,
    pub difficulty_speed: Option<i32>,
    pub difficulty_slang: Option<i32>,
    pub popularity_score: Option<i32>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingExercise {
    pub id: i64,
    pub title_en: String,
    pub title_zh: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub difficulty_level: Option<String>,
    pub exercise_type: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingSentence {
    pub id: i64,
    pub exercise_id: i64,
    pub sentence_order: i32,
    pub content_en: String,
    pub content_zh: String,
    pub phonetic_transcription: Option<String>,
    pub native_audio_path: Option<String>,
    pub focus_sounds: Option<Value>,
    pub common_mistakes: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPhrase {
    pub id: i64,
    pub phrase_en: String,
    pub phrase_zh: String,
    pub phonetic_transcription: Option<String>,
    pub usage_context: Option<String>,
    pub example_sentence_en: Option<String>,
    pub example_sentence_zh: Option<String>,
    pub category: Option<String>,
    pub formality_level: Option<String>,
    pub frequency_score: Option<i32>,
    pub created_at: String,
}

// ============================================================================
// API Response Types (User-specific)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueWord {
    pub id: i64,
    pub user_id: i64,
    pub word: String,
    pub issue_type: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub last_picked_at: Option<String>,
    pub pick_count: i32,
    pub next_review_at: Option<String>,
    pub review_interval_days: Option<i32>,
    pub difficulty_level: Option<i32>,
    pub context: Option<String>,
    pub audio_timestamp: Option<i32>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSession {
    pub id: i64,
    pub session_id: String,
    pub user_id: i64,
    pub session_type: Option<String>,
    pub sceneid: Option<i64>,
    pub scene_dialogue_id: Option<i64>,
    pub classic_clip_id: Option<i64>,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_seconds: Option<i32>,
    pub total_words_spoken: Option<i32>,
    pub average_wpm: Option<f32>,
    pub error_count: Option<i32>,
    pub correction_count: Option<i32>,
    pub notes: Option<String>,
    pub ai_summary_en: Option<String>,
    pub ai_summary_zh: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVocabulary {
    pub id: i64,
    pub user_id: i64,
    pub word: String,
    pub word_zh: Option<String>,
    pub mastery_level: Option<i32>,
    pub first_seen_at: String,
    pub last_practiced_at: Option<String>,
    pub practice_count: Option<i32>,
    pub correct_count: Option<i32>,
    pub next_review_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStat {
    pub id: i64,
    pub user_id: i64,
    pub stat_date: String,
    pub minutes_studied: Option<i32>,
    pub words_practiced: Option<i32>,
    pub sessions_completed: Option<i32>,
    pub errors_corrected: Option<i32>,
    pub new_words_learned: Option<i32>,
    pub review_words_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAchievement {
    pub id: i64,
    pub user_id: i64,
    pub achievement_type: String,
    pub achievement_name: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub metadata: Option<Value>,
    pub earned_at: String,
}

// ============================================================================
// Request Types
// ============================================================================

#[derive(Debug, Serialize)]
pub struct CreateIssueWordRequest {
    pub word: String,
    pub issue_type: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionRequest {
    pub session_id: String,
    pub session_type: Option<String>,
    pub sceneid: Option<i64>,
    pub scene_dialogue_id: Option<i64>,
    pub classic_clip_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CreateVocabularyRequest {
    pub word: String,
    pub word_zh: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpsertDailyStatRequest {
    pub stat_date: String,
    pub minutes_studied: Option<i32>,
    pub words_practiced: Option<i32>,
    pub sessions_completed: Option<i32>,
    pub errors_corrected: Option<i32>,
    pub new_words_learned: Option<i32>,
    pub review_words_count: Option<i32>,
}

// ============================================================================
// Client Implementation
// ============================================================================

impl LearningApiClient {
    pub fn new(base_url: &str, auth_token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            auth_token,
        }
    }

    pub fn set_auth_token(&mut self, token: Option<String>) {
        self.auth_token = token;
    }

    fn auth_header(&self) -> Option<String> {
        self.auth_token.as_ref().map(|t| format!("Bearer {}", t))
    }

    // ========================================================================
    // Shared Content APIs (no auth required)
    // ========================================================================

    pub async fn list_scenes(
        &self,
        category: Option<&str>,
        difficulty: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Scenes>, String> {
        let mut url = format!("{}/scenes", self.base_url);
        let mut params = vec![];

        if let Some(cat) = category {
            params.push(format!("category={}", cat));
        }
        if let Some(diff) = difficulty {
            params.push(format!("difficulty={}", diff));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn get_scenario(&self, id: i64) -> Result<Scenes, String> {
        let url = format!("{}/scenes/{}", self.base_url, id);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn get_scenedialogues(&self, sceneid: i64) -> Result<Vec<SceneDialogue>, String> {
        let url = format!("{}/scenes/{}/dialogues", self.base_url, sceneid);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn get_dialogue_turns(&self, dialogue_id: i64) -> Result<Vec<DialogueTurn>, String> {
        let url = format!("{}/dialogues/{}/turns", self.base_url, dialogue_id);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn list_classic_sources(
        &self,
        source_type: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ClassicDialogueSource>, String> {
        let mut url = format!("{}/classic-sources", self.base_url);
        let mut params = vec![];

        if let Some(st) = source_type {
            params.push(format!("type={}", st));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn list_classic_clips(
        &self,
        source_id: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<ClassicDialogueClip>, String> {
        let mut url = format!("{}/classic-clips", self.base_url);
        let mut params = vec![];

        if let Some(sid) = source_id {
            params.push(format!("source_id={}", sid));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn list_reading_exercises(
        &self,
        difficulty: Option<&str>,
        exercise_type: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ReadingExercise>, String> {
        let mut url = format!("{}/reading-exercises", self.base_url);
        let mut params = vec![];

        if let Some(diff) = difficulty {
            params.push(format!("difficulty={}", diff));
        }
        if let Some(et) = exercise_type {
            params.push(format!("type={}", et));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn get_reading_sentences(&self, exercise_id: i64) -> Result<Vec<ReadingSentence>, String> {
        let url = format!("{}/reading-exercises/{}/sentences", self.base_url, exercise_id);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn list_key_phrases(
        &self,
        category: Option<&str>,
        formality: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<KeyPhrase>, String> {
        let mut url = format!("{}/key-phrases", self.base_url);
        let mut params = vec![];

        if let Some(cat) = category {
            params.push(format!("category={}", cat));
        }
        if let Some(form) = formality {
            params.push(format!("formality={}", form));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    // ========================================================================
    // User-specific APIs (auth required)
    // ========================================================================

    pub async fn list_issue_words(
        &self,
        due_only: bool,
        limit: Option<i64>,
    ) -> Result<Vec<IssueWord>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learning/issue-words", self.base_url);
        let mut params = vec![];

        if due_only {
            params.push("due_only=true".to_string());
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn create_issue_word(&self, req: CreateIssueWordRequest) -> Result<IssueWord, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learning/issue-words", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn list_sessions(
        &self,
        session_type: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<LearningSession>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learning/sessions", self.base_url);
        let mut params = vec![];

        if let Some(st) = session_type {
            params.push(format!("type={}", st));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn create_session(&self, req: CreateSessionRequest) -> Result<LearningSession, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learning/sessions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn list_vocabulary(
        &self,
        due_only: bool,
        limit: Option<i64>,
    ) -> Result<Vec<UserVocabulary>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learning/vocabulary", self.base_url);
        let mut params = vec![];

        if due_only {
            params.push("due_only=true".to_string());
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn create_vocabulary(&self, req: CreateVocabularyRequest) -> Result<UserVocabulary, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learning/vocabulary", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn list_daily_stats(&self, limit: Option<i64>) -> Result<Vec<DailyStat>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learning/daily-stats", self.base_url);
        if let Some(lim) = limit {
            url = format!("{}?limit={}", url, lim);
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn upsert_daily_stat(&self, req: UpsertDailyStatRequest) -> Result<DailyStat, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learning/daily-stats", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    pub async fn list_achievements(&self) -> Result<Vec<UserAchievement>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learning/achievements", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }
}

// ============================================================================
// Global client instance
// ============================================================================

use std::sync::{OnceLock, RwLock};

static LEARNING_API: OnceLock<RwLock<LearningApiClient>> = OnceLock::new();

/// Initialize the global learning API client.
/// This should be called once at app startup.
pub fn init_learning_api(base_url: &str, auth_token: Option<String>) {
    let _ = LEARNING_API.set(RwLock::new(LearningApiClient::new(base_url, auth_token)));
}

/// Get the global learning API client.
pub fn get_learning_api() -> Option<&'static RwLock<LearningApiClient>> {
    LEARNING_API.get()
}

/// Update the auth token for the global learning API client.
/// This is synchronous and can be called from non-async contexts.
pub fn set_learning_api_token(token: Option<String>) {
    if let Some(api) = LEARNING_API.get() {
        if let Ok(mut client) = api.write() {
            client.set_auth_token(token);
        }
    }
}
