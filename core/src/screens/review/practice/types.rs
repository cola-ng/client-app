//! # Review Practice Types
//!
//! Data models for the review practice system.

use serde::{Deserialize, Serialize};

/// Review exercise types
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReviewType {
    /// Pronunciation practice - record and compare with native audio
    #[default]
    Pronunciation,
    /// Meaning selection - choose correct meaning from 4 options
    Meaning,
    /// Spelling practice - listen and type the word
    Spelling,
    /// Recognition - flip card and self-evaluate
    Recognition,
    /// Phrase fill-in - complete the sentence with correct word
    Phrase,
}

impl ReviewType {
    /// Get display name for the review type (Chinese)
    pub fn display_name(&self) -> &'static str {
        match self {
            ReviewType::Pronunciation => "发音练习",
            ReviewType::Meaning => "词义选择",
            ReviewType::Spelling => "听写练习",
            ReviewType::Recognition => "翻卡识别",
            ReviewType::Phrase => "短语填空",
        }
    }

    /// Get icon for the review type
    pub fn icon(&self) -> &'static str {
        match self {
            ReviewType::Pronunciation => "🎤",
            ReviewType::Meaning => "📝",
            ReviewType::Spelling => "✍️",
            ReviewType::Recognition => "🔄",
            ReviewType::Phrase => "📜",
        }
    }

    /// Get description for the review type
    pub fn description(&self) -> &'static str {
        match self {
            ReviewType::Pronunciation => "录音与原声对比，AI评分",
            ReviewType::Meaning => "四选一，选择正确的中文释义",
            ReviewType::Spelling => "听音频，输入正确拼写",
            ReviewType::Recognition => "翻卡片，自我评估掌握程度",
            ReviewType::Phrase => "在句子中填入正确的单词",
        }
    }

    /// Get all review types
    pub fn all() -> &'static [ReviewType] {
        &[
            ReviewType::Pronunciation,
            ReviewType::Meaning,
            ReviewType::Spelling,
            ReviewType::Recognition,
            ReviewType::Phrase,
        ]
    }
}

/// A single review item (word to practice)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewItem {
    /// Unique identifier
    pub id: String,
    /// The word to review
    pub word: String,
    /// Chinese translation
    pub word_zh: String,
    /// Phonetic transcription (IPA)
    pub phonetic: String,
    /// Audio URL for native pronunciation
    pub audio_url: Option<String>,
    /// Current mastery level (0-100)
    pub mastery_level: u8,
    /// Example sentences
    pub example_sentences: Vec<ExampleSentence>,
    /// Wrong options for meaning exercise (generated)
    #[serde(default)]
    pub wrong_meanings: Vec<String>,
}

impl Default for ReviewItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            word: String::new(),
            word_zh: String::new(),
            phonetic: String::new(),
            audio_url: None,
            mastery_level: 0,
            example_sentences: Vec::new(),
            wrong_meanings: Vec::new(),
        }
    }
}

/// Example sentence for a word
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExampleSentence {
    /// English sentence
    pub sentence: String,
    /// Chinese translation
    pub sentence_zh: String,
    /// The word to fill in (for phrase exercise)
    pub blank_word: String,
}

/// Result of a single review attempt
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewResult {
    /// Item ID that was reviewed
    pub item_id: String,
    /// Whether the answer was correct
    pub correct: bool,
    /// Score (0-100) for graded exercises like pronunciation
    pub score: Option<u8>,
    /// Time spent on this item in milliseconds
    pub time_spent_ms: u64,
    /// User's answer (for spelling, etc.)
    pub user_answer: Option<String>,
    /// Self-evaluation rating for recognition (1-4)
    pub self_rating: Option<u8>,
}

impl Default for ReviewResult {
    fn default() -> Self {
        Self {
            item_id: String::new(),
            correct: false,
            score: None,
            time_spent_ms: 0,
            user_answer: None,
            self_rating: None,
        }
    }
}

/// A review session containing multiple items
#[derive(Clone, Debug, Default)]
pub struct ReviewSession {
    /// Session unique identifier
    pub id: String,
    /// Type of review exercise
    pub review_type: ReviewType,
    /// Items to review
    pub items: Vec<ReviewItem>,
    /// Current item index
    pub current_index: usize,
    /// Results for completed items
    pub results: Vec<ReviewResult>,
    /// Session start time (Unix timestamp ms)
    pub start_time_ms: u64,
    /// Whether the session is complete
    pub completed: bool,
}

impl ReviewSession {
    /// Create a new review session
    pub fn new(review_type: ReviewType, items: Vec<ReviewItem>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            review_type,
            items,
            current_index: 0,
            results: Vec::new(),
            start_time_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            completed: false,
        }
    }

    /// Get the current item being reviewed
    pub fn current_item(&self) -> Option<&ReviewItem> {
        self.items.get(self.current_index)
    }

    /// Record a result and move to next item
    pub fn submit_result(&mut self, result: ReviewResult) {
        self.results.push(result);
        self.current_index += 1;
        if self.current_index >= self.items.len() {
            self.completed = true;
        }
    }

    /// Get progress as a fraction (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.items.is_empty() {
            return 0.0;
        }
        self.current_index as f64 / self.items.len() as f64
    }

    /// Get the number of correct answers
    pub fn correct_count(&self) -> usize {
        self.results.iter().filter(|r| r.correct).count()
    }

    /// Get accuracy as a percentage
    pub fn accuracy(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        (self.correct_count() as f64 / self.results.len() as f64) * 100.0
    }

    /// Get total time spent in milliseconds
    pub fn total_time_ms(&self) -> u64 {
        self.results.iter().map(|r| r.time_spent_ms).sum()
    }

    /// Get average time per item in milliseconds
    pub fn avg_time_ms(&self) -> u64 {
        if self.results.is_empty() {
            return 0;
        }
        self.total_time_ms() / self.results.len() as u64
    }
}

/// Session summary statistics
#[derive(Clone, Debug, Default)]
pub struct SessionSummary {
    /// Total items reviewed
    pub total_items: usize,
    /// Correct answers
    pub correct_count: usize,
    /// Accuracy percentage
    pub accuracy: f64,
    /// Total time in milliseconds
    pub total_time_ms: u64,
    /// Average time per item
    pub avg_time_ms: u64,
    /// Average score (for scored exercises)
    pub avg_score: Option<f64>,
}

impl From<&ReviewSession> for SessionSummary {
    fn from(session: &ReviewSession) -> Self {
        let avg_score = if session.results.iter().any(|r| r.score.is_some()) {
            let scores: Vec<u8> = session.results.iter().filter_map(|r| r.score).collect();
            if scores.is_empty() {
                None
            } else {
                Some(scores.iter().map(|&s| s as f64).sum::<f64>() / scores.len() as f64)
            }
        } else {
            None
        };

        Self {
            total_items: session.items.len(),
            correct_count: session.correct_count(),
            accuracy: session.accuracy(),
            total_time_ms: session.total_time_ms(),
            avg_time_ms: session.avg_time_ms(),
            avg_score,
        }
    }
}
