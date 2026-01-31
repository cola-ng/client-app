//! Dictionary API client
//!
//! This module provides a client for querying the dictionary from the backend server.
//! The dictionary provides English-Chinese word lookups with phonetics, examples, and more.

use reqwest::Client;
use serde::{Deserialize, Serialize};

// ============================================================================
// API Response Types (matching colang-website models)
// ============================================================================

/// Word record from the dictionary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub id: i64,
    pub word: String,
    pub word_lower: String,
    pub word_type: Option<String>,
    pub language: Option<String>,
    pub frequency: Option<i16>,
    pub difficulty: Option<i16>,
    pub syllable_count: Option<i16>,
    pub is_lemma: Option<bool>,
    pub word_count: Option<i32>,
    pub is_active: Option<bool>,
    pub memory: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

/// Word definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub id: i64,
    pub word_id: i64,
    pub language: String,
    pub definition: String,
    pub part_of_speech: Option<String>,
    pub definition_order: Option<i16>,
    pub register: Option<String>,
    pub region: Option<String>,
    pub context: Option<String>,
    pub usage_notes: Option<String>,
    pub is_primary: Option<bool>,
    pub created_at: String,
}

/// Sentence (example sentence)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentence {
    pub id: i64,
    pub language: String,
    pub sentence: String,
    pub source: Option<String>,
    pub author: Option<String>,
    pub priority_order: Option<i16>,
    pub difficulty: Option<i16>,
    pub is_common: Option<bool>,
    pub created_at: String,
}

/// Pronunciation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pronunciation {
    pub id: i64,
    pub word_id: i64,
    pub definition_id: Option<i64>,
    pub ipa: String,
    pub audio_url: Option<String>,
    pub audio_path: Option<String>,
    pub dialect: Option<String>,
    pub gender: Option<String>,
    pub is_primary: Option<bool>,
    pub created_at: String,
}

/// Word relation (synonym, antonym, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: i64,
    pub word_id: i64,
    pub relation_type: Option<String>,
    pub related_word_id: i64,
    pub related_word: String,
    pub semantic_field: Option<String>,
    pub relation_strength: Option<i16>,
}

/// Etymology entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Etymology {
    pub id: i64,
    pub origin_language: Option<String>,
    pub origin_word: Option<String>,
    pub origin_meaning: Option<String>,
    pub language: String,
    pub etymology: String,
    pub first_attested_year: Option<i32>,
    pub historical_forms: Option<serde_json::Value>,
    pub cognate_words: Option<serde_json::Value>,
    pub created_at: String,
}

/// Word form (conjugation, plural, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub id: i64,
    pub word_id: i64,
    pub form_type: Option<String>,
    pub form: String,
    pub is_irregular: Option<bool>,
    pub notes: Option<String>,
    pub created_at: String,
}

/// Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub created_at: String,
}

/// Word image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: i64,
    pub word_id: i64,
    pub image_url: Option<String>,
    pub image_path: Option<String>,
    pub image_type: Option<String>,
    pub alt_text_en: Option<String>,
    pub alt_text_zh: Option<String>,
    pub is_primary: Option<bool>,
    pub created_by: Option<i64>,
    pub created_at: String,
}

/// Dictionary source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dictionary {
    pub id: i64,
    pub name: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub version: Option<String>,
    pub publisher: Option<String>,
    pub license_type: Option<String>,
    pub license_url: Option<String>,
    pub source_url: Option<String>,
    pub total_entries: Option<i32>,
    pub is_active: Option<bool>,
    pub is_official: Option<bool>,
    pub priority_order: Option<i16>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub updated_at: String,
    pub created_at: String,
}

/// Word-Dictionary association
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDictionary {
    pub id: i64,
    pub word_id: i64,
    pub dictionary_id: i64,
    pub definition_id: Option<i64>,
    pub priority_order: Option<i16>,
    pub name: String,
    pub created_at: String,
    pub dictionary: Dictionary,
}

/// Full word lookup response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordQueryResponse {
    pub word: Word,
    pub definitions: Vec<Definition>,
    pub sentences: Vec<Sentence>,
    pub pronunciations: Vec<Pronunciation>,
    pub relations: Vec<Relation>,
    pub etymologies: Vec<Etymology>,
    pub forms: Vec<Form>,
    pub categories: Vec<Category>,
    pub images: Vec<Image>,
    pub dictionaries: Vec<WordDictionary>,
}

/// Brief dictionary entry for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictEntryBrief {
    pub id: i64,
    pub word: String,
    pub phonetic: Option<String>,
    pub part_of_speech: Option<String>,
    pub definition_zh: String,
}

/// Search history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryEntry {
    pub id: i64,
    pub user_id: i64,
    pub word: String,
    pub searched_at: String,
}

/// New search history entry for insertion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSearchHistoryEntry {
    pub word: String,
}

// ============================================================================
// Dictionary API Client
// ============================================================================

#[derive(Debug, Clone)]
pub struct DictApiClient {
    client: Client,
    base_url: String,
}

impl DictApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Search dictionary entries by word prefix
    ///
    /// # Arguments
    /// * `query` - The search query (word or prefix)
    /// * `limit` - Maximum number of results
    pub async fn search(&self, query: &str, limit: Option<i64>) -> Result<Vec<Word>, String> {
        let limit = limit.unwrap_or(20);
        let url = format!(
            "{}/dict/words?search={}&limit={}",
            self.base_url,
            urlencoding::encode(query),
            limit
        );

        log::debug!("[DictAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[DictAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[DictAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[DictAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[DictAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[DictAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[DictAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    /// Lookup a word by exact match
    ///
    /// # Arguments
    /// * `word` - The word to lookup
    pub async fn lookup(&self, word: &str) -> Result<WordQueryResponse, String> {
        let url = format!(
            "{}/dict/lookup?word={}",
            self.base_url,
            urlencoding::encode(word)
        );

        log::debug!("[DictAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[DictAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[DictAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[DictAPI] Error response ({}): {}", status, body);
            return Err(format!("API error: {} - word not found", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[DictAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[DictAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[DictAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    /// List dictionary entries with optional filters
    pub async fn list_words(
        &self,
        min_difficulty: Option<i16>,
        max_difficulty: Option<i16>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Word>, String> {
        let mut url = format!("{}/dict/words", self.base_url);
        let mut params = vec![];

        if let Some(min_d) = min_difficulty {
            params.push(format!("min_difficulty={}", min_d));
        }
        if let Some(max_d) = max_difficulty {
            params.push(format!("max_difficulty={}", max_d));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        if let Some(off) = offset {
            params.push(format!("offset={}", off));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        log::debug!("[DictAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[DictAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[DictAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[DictAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[DictAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[DictAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[DictAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    /// Save search history
    ///
    /// # Arguments
    /// * `word` - The word that was searched
    pub async fn save_search_history(&self, word: &str) -> Result<SearchHistoryEntry, String> {
        let url = format!("{}/dict/history", self.base_url);

        let payload = serde_json::json!({ "word": word });

        log::debug!("[DictAPI] POST {}", url);
        log::debug!("[DictAPI] Request body: {}", payload);

        let response = self
            .client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                log::error!("[DictAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[DictAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[DictAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[DictAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[DictAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[DictAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    /// Get search history
    ///
    /// # Arguments
    /// * `limit` - Maximum number of history items
    pub async fn get_search_history(&self, limit: i64) -> Result<Vec<SearchHistoryEntry>, String> {
        let url = format!("{}/dict/history?limit={}", self.base_url, limit);

        log::debug!("[DictAPI] GET {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[DictAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[DictAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[DictAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        let body = response.text().await.map_err(|e| {
            log::error!("[DictAPI] Failed to read response body: {}", e);
            format!("Read error: {}", e)
        })?;
        log::debug!("[DictAPI] Response body ({}B): {}", body.len(), &body[..body.len().min(500)]);

        serde_json::from_str(&body).map_err(|e| {
            log::error!("[DictAPI] Parse error: {} - Body: {}", e, &body[..body.len().min(200)]);
            format!("Parse error: {}", e)
        })
    }

    /// Clear search history
    pub async fn clear_search_history(&self) -> Result<(), String> {
        let url = format!("{}/dict/history", self.base_url);

        log::debug!("[DictAPI] DELETE {}", url);

        let response = self
            .client
            .delete(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                log::error!("[DictAPI] Request failed: {}", e);
                format!("Request failed: {}", e)
            })?;

        let status = response.status();
        log::debug!("[DictAPI] Response status: {}", status);

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            log::error!("[DictAPI] Error response: {}", body);
            return Err(format!("API error: {}", status));
        }

        log::debug!("[DictAPI] Search history cleared successfully");
        Ok(())
    }
}

// ============================================================================
// Global client instance
// ============================================================================

use std::sync::{OnceLock, RwLock};

static DICT_API: OnceLock<RwLock<DictApiClient>> = OnceLock::new();

/// Initialize the global dictionary API client.
/// This should be called once at app startup.
pub fn init_dict_api(base_url: &str) {
    let _ = DICT_API.set(RwLock::new(DictApiClient::new(base_url)));
}

/// Get the global dictionary API client.
pub fn get_dict_api() -> Option<&'static RwLock<DictApiClient>> {
    DICT_API.get()
}
