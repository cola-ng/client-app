//! Asset content API client
//!
//! This module provides a client for fetching shared asset content from the backend server.
//! Asset content includes stages, scripts, reading exercises, and key phrases.
//! These are shared content that don't require authentication.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// API Response Types (Shared Content)
// ============================================================================

/// Stage matches backend asset_stages table (renamed from Scene)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    pub id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub icon_emoji: Option<String>,
    pub display_order: Option<i32>,
    pub difficulty: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: String,
}

/// Script within a stage (dialogues)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: i64,
    pub stage_id: i64,
    pub title_en: String,
    pub title_zh: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub total_turns: Option<i32>,
    pub estimated_duration_seconds: Option<i32>,
    pub difficulty_level: Option<String>,
    pub created_at: String,
}

/// Script turn (dialogue line)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTurn {
    pub id: i64,
    pub script_id: i64,
    pub turn_number: i32,
    pub speaker_role: String,
    pub speaker_name: Option<String>,
    pub content_en: String,
    pub content_zh: String,
    pub audio_path: Option<String>,
    pub phonetic_transcription: Option<String>,
    pub asset_phrases: Option<Value>,
    pub notes: Option<String>,
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

/// Chat context for context-based conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContext {
    pub id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub icon_emoji: Option<String>,
    pub display_order: Option<i32>,
    pub difficulty: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: String,
}

// ============================================================================
// Asset API Client
// ============================================================================

#[derive(Debug, Clone)]
pub struct AssetApiClient {
    client: Client,
    base_url: String,
}

impl AssetApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    // ========================================================================
    // Stages API (matches website /api/asset/stages)
    // ========================================================================

    /// List all stages
    pub async fn list_stages(&self) -> Result<Vec<Stage>, String> {
        let url = format!("{}/asset/stages", self.base_url);
        log::debug!("[AssetAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[AssetAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[AssetAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[AssetAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[AssetAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[AssetAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[AssetAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    /// Get a specific stage by ID
    pub async fn get_stage(&self, id: i64) -> Result<Stage, String> {
        let url = format!("{}/asset/stages/{}", self.base_url, id);
        log::debug!("[AssetAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[AssetAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[AssetAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[AssetAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[AssetAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[AssetAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[AssetAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    /// Get scripts (dialogues) for a stage
    pub async fn get_stage_scripts(&self, stage_id: i64) -> Result<Vec<Script>, String> {
        let url = format!("{}/asset/stages/{}/scripts", self.base_url, stage_id);
        log::debug!("[AssetAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[AssetAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[AssetAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[AssetAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[AssetAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[AssetAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[AssetAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    /// Get turns for a script
    pub async fn get_script_turns(&self, script_id: i64) -> Result<Vec<ScriptTurn>, String> {
        let url = format!("{}/asset/scripts/{}/turns", self.base_url, script_id);
        log::debug!("[AssetAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[AssetAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[AssetAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[AssetAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[AssetAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[AssetAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[AssetAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    // ========================================================================
    // Reading Exercises API
    // ========================================================================

    pub async fn list_reading_exercises(
        &self,
        difficulty: Option<&str>,
        exercise_type: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ReadingExercise>, String> {
        let mut url = format!("{}/asset/reading-exercises", self.base_url);
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

        log::debug!("[AssetAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[AssetAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[AssetAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[AssetAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[AssetAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[AssetAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[AssetAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    pub async fn get_reading_sentences(
        &self,
        exercise_id: i64,
    ) -> Result<Vec<ReadingSentence>, String> {
        let url = format!(
            "{}/asset/reading-exercises/{}/sentences",
            self.base_url, exercise_id
        );
        log::debug!("[AssetAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[AssetAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[AssetAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[AssetAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[AssetAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[AssetAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[AssetAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    // ========================================================================
    // Key Phrases API
    // ========================================================================

    pub async fn list_key_phrases(
        &self,
        category: Option<&str>,
        formality: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<KeyPhrase>, String> {
        let mut url = format!("{}/asset/key-phrases", self.base_url);
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

        log::debug!("[AssetAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[AssetAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[AssetAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[AssetAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[AssetAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[AssetAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[AssetAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    // ========================================================================
    // Chat Contexts API
    // ========================================================================

    /// List all available chat contexts for context-based conversations
    pub async fn list_contexts(&self) -> Result<Vec<ChatContext>, String> {
        let url = format!("{}/asset/contexts", self.base_url);
        log::debug!("[AssetAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[AssetAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[AssetAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[AssetAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[AssetAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[AssetAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[AssetAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }
}

// ============================================================================
// Global client instance
// ============================================================================

use std::sync::{OnceLock, RwLock};

static ASSET_API: OnceLock<RwLock<AssetApiClient>> = OnceLock::new();

/// Initialize the global asset API client.
/// This should be called once at app startup.
pub fn init_asset_api(base_url: &str) {
    let _ = ASSET_API.set(RwLock::new(AssetApiClient::new(base_url)));
}

/// Get the global asset API client.
pub fn get_asset_api() -> Option<&'static RwLock<AssetApiClient>> {
    ASSET_API.get()
}
