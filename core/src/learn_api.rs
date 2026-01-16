//! Learn content API client
//!
//! This module provides a client for fetching user-specific learning data from the backend server.
//! Learn content includes issue words, sessions, vocabulary, daily stats, and achievements.
//! These are user-specific data that require authentication.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub scene_id: Option<i64>,
    pub dialogue_id: Option<i64>,
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
pub struct Conversation {
    pub id: i64,
    pub user_id: i64,
    pub session_id: String,
    pub speaker: String,
    pub use_lang: String,
    pub content_en: String,
    pub content_zh: String,
    pub audio_path: Option<String>,
    pub duration_ms: Option<i32>,
    pub words_per_minute: Option<f32>,
    pub pause_count: Option<i32>,
    pub hesitation_count: Option<i32>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationAnnotation {
    pub id: i64,
    pub user_id: i64,
    pub conversation_id: i64,
    pub annotation_type: String,
    pub start_position: Option<i32>,
    pub end_position: Option<i32>,
    pub original_text: Option<String>,
    pub suggested_text: Option<String>,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub severity: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordPractice {
    pub id: i64,
    pub user_id: i64,
    pub word_id: i64,
    pub session_id: String,
    pub success_level: Option<i32>,
    pub notes: Option<String>,
    pub practiced_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadPractice {
    pub id: i64,
    pub user_id: i64,
    pub sentence_id: i64,
    pub session_id: String,
    pub user_audio_path: Option<String>,
    pub pronunciation_score: Option<i32>,
    pub fluency_score: Option<i32>,
    pub intonation_score: Option<i32>,
    pub overall_score: Option<i32>,
    pub detected_errors: Option<Value>,
    pub ai_feedback_en: Option<String>,
    pub ai_feedback_zh: Option<String>,
    pub waveform_data: Option<Value>,
    pub attempted_at: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub id: i64,
    pub user_id: i64,
    pub suggestion_type: Option<String>,
    pub suggested_text: String,
    pub was_accepted: Option<bool>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetResponse {
    pub deleted_count: usize,
    pub table: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetAllResponse {
    pub tables_reset: Vec<ResetResponse>,
    pub total_deleted: usize,
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
    pub scene_id: Option<i64>,
    pub dialogue_id: Option<i64>,
    pub classic_clip_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CreateConversationRequest {
    pub session_id: String,
    pub speaker: String,
    pub use_lang: String,
    pub content_en: String,
    pub content_zh: String,
    pub audio_path: Option<String>,
    pub duration_ms: Option<i32>,
    pub words_per_minute: Option<f32>,
    pub pause_count: Option<i32>,
    pub hesitation_count: Option<i32>,
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

#[derive(Debug, Serialize)]
pub struct CreateSuggestionRequest {
    pub suggestion_type: Option<String>,
    pub suggested_text: String,
    pub was_accepted: Option<bool>,
}

// ============================================================================
// Learn API Client
// ============================================================================

#[derive(Debug, Clone)]
pub struct LearnApiClient {
    client: Client,
    base_url: String,
    auth_token: Option<String>,
}

impl LearnApiClient {
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

    pub fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }

    fn auth_header(&self) -> Option<String> {
        self.auth_token.as_ref().map(|t| format!("Bearer {}", t))
    }

    // ========================================================================
    // Issue Words API
    // ========================================================================

    pub async fn list_issue_words(
        &self,
        due_only: bool,
        limit: Option<i64>,
    ) -> Result<Vec<IssueWord>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learn/issue-words", self.base_url);
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

    pub async fn create_issue_word(
        &self,
        req: CreateIssueWordRequest,
    ) -> Result<IssueWord, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/issue-words", self.base_url);

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

    pub async fn reset_issue_words(&self) -> Result<ResetResponse, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/issue-words", self.base_url);

        let response = self
            .client
            .delete(&url)
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

    // ========================================================================
    // Sessions API
    // ========================================================================

    pub async fn list_sessions(
        &self,
        session_type: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<LearningSession>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learn/sessions", self.base_url);
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

    pub async fn create_session(
        &self,
        req: CreateSessionRequest,
    ) -> Result<LearningSession, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/sessions", self.base_url);

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

    pub async fn reset_sessions(&self) -> Result<ResetResponse, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/sessions", self.base_url);

        let response = self
            .client
            .delete(&url)
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

    // ========================================================================
    // Conversations API
    // ========================================================================

    pub async fn list_conversations(
        &self,
        session_id: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Conversation>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learn/conversations", self.base_url);
        let mut params = vec![];

        if let Some(sid) = session_id {
            params.push(format!("session_id={}", sid));
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

    pub async fn create_conversation(
        &self,
        req: CreateConversationRequest,
    ) -> Result<Conversation, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/conversations", self.base_url);

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

    pub async fn reset_conversations(&self) -> Result<ResetResponse, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/conversations", self.base_url);

        let response = self
            .client
            .delete(&url)
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

    // ========================================================================
    // Vocabulary API
    // ========================================================================

    pub async fn list_vocabulary(
        &self,
        due_only: bool,
        limit: Option<i64>,
    ) -> Result<Vec<UserVocabulary>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learn/vocabulary", self.base_url);
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

    pub async fn create_vocabulary(
        &self,
        req: CreateVocabularyRequest,
    ) -> Result<UserVocabulary, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/vocabulary", self.base_url);

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

    pub async fn reset_vocabulary(&self) -> Result<ResetResponse, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/vocabulary", self.base_url);

        let response = self
            .client
            .delete(&url)
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

    // ========================================================================
    // Daily Stats API
    // ========================================================================

    pub async fn list_daily_stats(&self, limit: Option<i64>) -> Result<Vec<DailyStat>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learn/daily-stats", self.base_url);
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

    pub async fn upsert_daily_stat(
        &self,
        req: UpsertDailyStatRequest,
    ) -> Result<DailyStat, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/daily-stats", self.base_url);

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

    pub async fn reset_daily_stats(&self) -> Result<ResetResponse, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/daily-stats", self.base_url);

        let response = self
            .client
            .delete(&url)
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

    // ========================================================================
    // Achievements API
    // ========================================================================

    pub async fn list_achievements(&self) -> Result<Vec<UserAchievement>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/achievements", self.base_url);

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

    pub async fn reset_achievements(&self) -> Result<ResetResponse, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/achievements", self.base_url);

        let response = self
            .client
            .delete(&url)
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

    // ========================================================================
    // Suggestions API
    // ========================================================================

    pub async fn list_suggestions(
        &self,
        suggestion_type: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Suggestion>, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;

        let mut url = format!("{}/learn/suggestions", self.base_url);
        let mut params = vec![];

        if let Some(st) = suggestion_type {
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

    pub async fn create_suggestion(
        &self,
        req: CreateSuggestionRequest,
    ) -> Result<Suggestion, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/suggestions", self.base_url);

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

    pub async fn reset_suggestions(&self) -> Result<ResetResponse, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/suggestions", self.base_url);

        let response = self
            .client
            .delete(&url)
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

    // ========================================================================
    // Reset All API
    // ========================================================================

    pub async fn reset_all(&self) -> Result<ResetAllResponse, String> {
        let auth = self.auth_header().ok_or("Not authenticated")?;
        let url = format!("{}/learn/reset-all", self.base_url);

        let response = self
            .client
            .delete(&url)
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

static LEARN_API: OnceLock<RwLock<LearnApiClient>> = OnceLock::new();

/// Initialize the global learn API client.
/// This should be called once at app startup.
pub fn init_learn_api(base_url: &str, auth_token: Option<String>) {
    let _ = LEARN_API.set(RwLock::new(LearnApiClient::new(base_url, auth_token)));
}

/// Get the global learn API client.
pub fn get_learn_api() -> Option<&'static RwLock<LearnApiClient>> {
    LEARN_API.get()
}

/// Update the auth token for the global learn API client.
/// This is synchronous and can be called from non-async contexts.
pub fn set_learn_api_token(token: Option<String>) {
    if let Some(api) = LEARN_API.get() {
        if let Ok(mut client) = api.write() {
            client.set_auth_token(token);
        }
    }
}
