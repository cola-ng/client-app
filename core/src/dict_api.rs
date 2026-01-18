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
    pub created_at: String,
    pub updated_at: String,
}

/// Word definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDefinition {
    pub id: i64,
    pub word_id: i64,
    pub part_of_speech: Option<String>,
    pub definition_en: Option<String>,
    pub definition_zh: String,
    pub definition_order: Option<i16>,
    pub register: Option<String>,
    pub domain: Option<String>,
    pub region: Option<String>,
    pub source: Option<String>,
}

/// Word example sentence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordExample {
    pub id: i64,
    pub word_id: i64,
    pub example_en: String,
    pub example_zh: Option<String>,
    pub example_order: Option<i16>,
    pub source: Option<String>,
    pub audio_path: Option<String>,
}

/// Word form (conjugation, plural, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordForm {
    pub id: i64,
    pub word_id: i64,
    pub form_type: String,
    pub form_value: String,
    pub phonetic_us: Option<String>,
    pub phonetic_uk: Option<String>,
    pub audio_us_path: Option<String>,
    pub audio_uk_path: Option<String>,
}

/// Reference to a related word
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordRef {
    pub id: i64,
    pub word: String,
}

/// Synonym link with reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordSynonymView {
    pub link: WordSynonymLink,
    pub synonym: WordRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordSynonymLink {
    pub id: i64,
    pub word_id: i64,
    pub synonym_word_id: i64,
    pub similarity_score: Option<f32>,
    pub context: Option<String>,
}

/// Antonym link with reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordAntonymView {
    pub link: WordAntonymLink,
    pub antonym: WordRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordAntonymLink {
    pub id: i64,
    pub word_id: i64,
    pub antonym_word_id: i64,
    pub antonym_type: Option<String>,
}

/// Word family link with reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordFamilyView {
    pub link: WordFamilyLink,
    pub related: WordRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordFamilyLink {
    pub id: i64,
    pub root_word_id: i64,
    pub related_word_id: i64,
    pub relationship_type: Option<String>,
    pub morpheme: Option<String>,
}

/// Word collocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordCollocation {
    pub id: i64,
    pub word_id: i64,
    pub collocation: String,
    pub collocation_type: Option<String>,
    pub frequency: Option<i16>,
    pub example_en: Option<String>,
    pub example_zh: Option<String>,
}

/// Phrase (idiom or common phrase)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    pub id: i64,
    pub phrase: String,
    pub phrase_type: Option<String>,
    pub definition_en: Option<String>,
    pub definition_zh: Option<String>,
    pub example_en: Option<String>,
    pub example_zh: Option<String>,
    pub origin: Option<String>,
    pub usage_notes: Option<String>,
}

/// Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
}

/// Etymology entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordEtymology {
    pub id: i64,
    pub word_id: i64,
    pub origin_language: Option<String>,
    pub origin_word: Option<String>,
    pub etymology_description: Option<String>,
    pub first_known_use: Option<String>,
}

/// Usage note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordUsageNote {
    pub id: i64,
    pub word_id: i64,
    pub note_type: Option<String>,
    pub note_content: String,
}

/// Word image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordImage {
    pub id: i64,
    pub word_id: i64,
    pub image_url: String,
    pub image_type: Option<String>,
    pub caption: Option<String>,
}

/// Full word lookup response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordQueryResponse {
    pub word: Word,
    pub definitions: Vec<WordDefinition>,
    pub examples: Vec<WordExample>,
    pub synonyms: Vec<WordSynonymView>,
    pub antonyms: Vec<WordAntonymView>,
    pub forms: Vec<WordForm>,
    pub collocations: Vec<WordCollocation>,
    pub word_family: Vec<WordFamilyView>,
    pub phrases: Vec<Phrase>,
    pub idioms: Vec<Phrase>,
    pub categories: Vec<Category>,
    pub etymology: Vec<WordEtymology>,
    pub usage_notes: Vec<WordUsageNote>,
    pub images: Vec<WordImage>,
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

        log::info!("Dictionary search: {}", url);

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

        log::info!("Dictionary lookup: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {} - word not found", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
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

    /// Save search history
    ///
    /// # Arguments
    /// * `word` - The word that was searched
    pub async fn save_search_history(&self, word: &str) -> Result<SearchHistoryEntry, String> {
        let url = format!("{}/dict/history", self.base_url);

        let payload = serde_json::json!({ "word": word });

        log::info!("Saving search history: {}", url);

        let response = self
            .client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
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

    /// Get search history
    ///
    /// # Arguments
    /// * `limit` - Maximum number of history items
    pub async fn get_search_history(&self, limit: i64) -> Result<Vec<SearchHistoryEntry>, String> {
        let url = format!("{}/dict/history?limit={}", self.base_url, limit);

        log::info!("Fetching search history: {}", url);

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

    /// Clear search history
    pub async fn clear_search_history(&self) -> Result<(), String> {
        let url = format!("{}/dict/history", self.base_url);

        log::info!("Clearing search history: {}", url);

        let response = self
            .client
            .delete(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

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
