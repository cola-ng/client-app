//! Dictionary API client
//!
//! This module provides a client for querying the dictionary from the backend server.
//! The dictionary provides English-Chinese word lookups with phonetics, examples, and more.

use reqwest::Client;
use serde::{Deserialize, Serialize};

// ============================================================================
// API Response Types
// ============================================================================

/// Full dictionary entry with all details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictEntry {
    pub id: i64,
    pub word: String,
    pub phonetic: Option<String>,
    pub phonetic_us: Option<String>,
    pub phonetic_uk: Option<String>,
    pub part_of_speech: Option<String>,
    pub definition_en: Option<String>,
    pub definition_zh: String,
    pub example_en: Option<String>,
    pub example_zh: Option<String>,
    pub synonyms: Option<Vec<String>>,
    pub antonyms: Option<Vec<String>>,
    pub related_words: Option<Vec<String>>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub audio_us_path: Option<String>,
    pub audio_uk_path: Option<String>,
    pub frequency_rank: Option<i32>,
    pub usage_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Brief dictionary entry for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictEntryBrief {
    pub id: i64,
    pub word: String,
    pub phonetic: Option<String>,
    pub part_of_speech: Option<String>,
    pub definition_zh: String,
    pub difficulty_level: Option<String>,
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
    /// * `query` - The search query (word prefix)
    /// * `exact` - If true, performs exact match; otherwise prefix match
    /// * `limit` - Maximum number of results
    pub async fn search(
        &self,
        query: &str,
        exact: bool,
        limit: Option<i64>,
    ) -> Result<Vec<DictEntryBrief>, String> {
        let mut url = format!(
            "{}/dict/search?q={}",
            self.base_url,
            urlencoding::encode(query)
        );

        if exact {
            url.push_str("&exact=true");
        }
        if let Some(lim) = limit {
            url.push_str(&format!("&limit={}", lim));
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

    /// Get full details of a dictionary entry
    pub async fn get_entry(&self, id: i64) -> Result<DictEntry, String> {
        let url = format!("{}/dict/entries/{}", self.base_url, id);

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

    /// List dictionary entries with optional filters
    pub async fn list_entries(
        &self,
        difficulty: Option<&str>,
        category: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<DictEntryBrief>, String> {
        let mut url = format!("{}/dict/entries", self.base_url);
        let mut params = vec![];

        if let Some(diff) = difficulty {
            params.push(format!("difficulty={}", diff));
        }
        if let Some(cat) = category {
            params.push(format!("category={}", cat));
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

    /// Get a random dictionary entry (for "word of the day" feature)
    pub async fn get_random(&self, difficulty: Option<&str>) -> Result<DictEntry, String> {
        let mut url = format!("{}/dict/random", self.base_url);

        if let Some(diff) = difficulty {
            url = format!("{}?difficulty={}", url, diff);
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

    /// Get list of available categories
    pub async fn list_categories(&self) -> Result<Vec<String>, String> {
        let url = format!("{}/dict/categories", self.base_url);

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
