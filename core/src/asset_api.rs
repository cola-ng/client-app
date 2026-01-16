//! Asset content API client
//!
//! This module provides a client for fetching shared asset content from the backend server.
//! Asset content includes scenes, dialogues, classic sources, reading exercises, and key phrases.
//! These are shared content that don't require authentication.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// API Response Types (Shared Content)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
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
    pub scene_id: i64,
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
    pub dialogue_id: i64,
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
    // Scenes API
    // ========================================================================

    pub async fn list_scenes(
        &self,
        category: Option<&str>,
        difficulty: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Scene>, String> {
        let mut url = format!("{}/asset/scenes", self.base_url);
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

    pub async fn get_scene(&self, id: i64) -> Result<Scene, String> {
        let url = format!("{}/asset/scenes/{}", self.base_url, id);

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

    pub async fn get_scene_dialogues(&self, scene_id: i64) -> Result<Vec<SceneDialogue>, String> {
        let url = format!("{}/asset/scenes/{}/dialogues", self.base_url, scene_id);

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
        let url = format!("{}/asset/dialogues/{}/turns", self.base_url, dialogue_id);

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
    // Classic Dialogues API
    // ========================================================================

    pub async fn list_classic_sources(
        &self,
        source_type: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ClassicDialogueSource>, String> {
        let mut url = format!("{}/asset/classic-sources", self.base_url);
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
        let mut url = format!("{}/asset/classic-clips", self.base_url);
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

    pub async fn get_reading_sentences(
        &self,
        exercise_id: i64,
    ) -> Result<Vec<ReadingSentence>, String> {
        let url = format!(
            "{}/asset/reading-exercises/{}/sentences",
            self.base_url, exercise_id
        );

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
