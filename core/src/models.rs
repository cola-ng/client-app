// Database models and operations for English Learning Companion

mod preferences;
mod providers;

use std::time::{SystemTime, UNIX_EPOCH};

pub use preferences::*;
pub use providers::*;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IssueWord {
    pub id: Option<i64>,
    pub word: String,
    pub issue_type: IssueType,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub last_picked_at: Option<i64>,
    pub created_at: i64,
    pub pick_count: i64,
    pub next_review_at: Option<i64>,
    pub review_interval_days: i64,
    pub difficulty_level: i64,
    pub context: Option<String>,
    pub audio_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IssueType {
    Pronunciation,
    Usage,
    Unfamiliar,
    Grammar,
}

impl ToString for IssueType {
    fn to_string(&self) -> String {
        match self {
            IssueType::Pronunciation => "pronunciation".to_string(),
            IssueType::Usage => "usage".to_string(),
            IssueType::Unfamiliar => "unfamiliar".to_string(),
            IssueType::Grammar => "grammar".to_string(),
        }
    }
}

impl std::str::FromStr for IssueType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pronunciation" => Ok(IssueType::Pronunciation),
            "usage" => Ok(IssueType::Usage),
            "unfamiliar" => Ok(IssueType::Unfamiliar),
            "grammar" => Ok(IssueType::Grammar),
            _ => Err(format!("Invalid issue type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: Option<i64>,
    pub session_id: String,
    pub speaker: Speaker,
    pub use_lang: UseLang,
    pub content_en: String,
    pub content_zh: String,
    pub audio_path: Option<String>,
    pub created_at: i64,
    pub duration_ms: Option<i64>,
    pub words_per_minute: Option<f64>,
    pub pause_count: Option<i64>,
    pub hesitation_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Speaker {
    User,
    Teacher,
}

impl ToString for Speaker {
    fn to_string(&self) -> String {
        match self {
            Speaker::User => "user".to_string(),
            Speaker::Teacher => "teacher".to_string(),
        }
    }
}

impl std::str::FromStr for Speaker {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Speaker::User),
            "teacher" => Ok(Speaker::Teacher),
            _ => Err(format!("Invalid speaker: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UseLang {
    En,
    Zh,
    Mix,
}

impl ToString for UseLang {
    fn to_string(&self) -> String {
        match self {
            UseLang::En => "en".to_string(),
            UseLang::Zh => "zh".to_string(),
            UseLang::Mix => "mix".to_string(),
        }
    }
}

impl std::str::FromStr for UseLang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(UseLang::En),
            "zh" => Ok(UseLang::Zh),
            "mix" => Ok(UseLang::Mix),
            _ => Err(format!("Invalid use_lang: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversationAnnotation {
    pub id: Option<i64>,
    pub conversation_id: i64,
    pub annotation_type: AnnotationType,
    pub start_position: Option<i64>,
    pub end_position: Option<i64>,
    pub original_text: Option<String>,
    pub suggested_text: Option<String>,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub severity: Severity,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationType {
    PronunciationError,
    GrammarError,
    WordChoice,
    FluencyIssue,
    Suggestion,
    Correction,
}

impl ToString for AnnotationType {
    fn to_string(&self) -> String {
        match self {
            AnnotationType::PronunciationError => "pronunciation_error".to_string(),
            AnnotationType::GrammarError => "grammar_error".to_string(),
            AnnotationType::WordChoice => "word_choice".to_string(),
            AnnotationType::FluencyIssue => "fluency_issue".to_string(),
            AnnotationType::Suggestion => "suggestion".to_string(),
            AnnotationType::Correction => "correction".to_string(),
        }
    }
}

impl std::str::FromStr for AnnotationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pronunciation_error" => Ok(AnnotationType::PronunciationError),
            "grammar_error" => Ok(AnnotationType::GrammarError),
            "word_choice" => Ok(AnnotationType::WordChoice),
            "fluency_issue" => Ok(AnnotationType::FluencyIssue),
            "suggestion" => Ok(AnnotationType::Suggestion),
            "correction" => Ok(AnnotationType::Correction),
            _ => Err(format!("Invalid annotation type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
}

impl ToString for Severity {
    fn to_string(&self) -> String {
        match self {
            Severity::Low => "low".to_string(),
            Severity::Medium => "medium".to_string(),
            Severity::High => "high".to_string(),
        }
    }
}

impl std::str::FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Severity::Low),
            "medium" => Ok(Severity::Medium),
            "high" => Ok(Severity::High),
            _ => Err(format!("Invalid severity: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LearningSession {
    pub id: Option<i64>,
    pub session_id: String,
    pub topic: String,
    pub target_words: Option<String>, // JSON array
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub total_exchanges: i64,
    pub user_satisfaction: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WordPracticeLog {
    pub id: Option<i64>,
    pub word_id: i64,
    pub session_id: String,
    pub practiced_at: i64,
    pub success_level: Option<i64>,
    pub notes: Option<String>,
}
