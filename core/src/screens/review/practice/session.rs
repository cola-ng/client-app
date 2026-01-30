//! # Review Session Manager
//!
//! Manages review practice sessions including loading items, tracking progress,
//! and submitting results.

use std::sync::mpsc;
use std::thread;

use makepad_widgets::{ActionDefaultRef, DefaultNone};

use super::types::{ReviewItem, ReviewResult, ReviewSession, ReviewType, SessionSummary};

/// Messages sent from background threads to the UI
#[derive(Clone, Debug)]
pub enum SessionMessage {
    /// Items loaded successfully
    ItemsLoaded(Vec<ReviewItem>),
    /// Error loading items
    LoadError(String),
    /// Result submitted successfully
    ResultSubmitted,
    /// Error submitting result
    SubmitError(String),
    /// Pronunciation score received
    PronunciationScore { item_id: String, score: u8 },
}

/// Session manager action for UI coordination
#[derive(Clone, Debug, DefaultNone)]
pub enum SessionAction {
    None,
    /// Session started, items loaded
    SessionStarted,
    /// Current item changed
    ItemChanged,
    /// Session completed
    SessionCompleted(SessionSummary),
    /// Error occurred
    Error(String),
}

/// Manages review practice sessions
pub struct ReviewSessionManager {
    /// Current active session
    session: Option<ReviewSession>,
    /// Channel receiver for background messages
    rx: Option<mpsc::Receiver<SessionMessage>>,
    /// Whether items are currently loading
    loading: bool,
    /// Current item start time for timing
    item_start_time_ms: u64,
}

impl Default for ReviewSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ReviewSessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            session: None,
            rx: None,
            loading: false,
            item_start_time_ms: 0,
        }
    }

    /// Start a new review session of the given type
    pub fn start_session(&mut self, review_type: ReviewType) {
        self.loading = true;

        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);

        // Spawn background task to load items
        thread::spawn(move || {
            // TODO: Replace with actual API call to load review items
            // For now, use mock data
            let items = Self::load_mock_items(review_type);
            let _ = tx.send(SessionMessage::ItemsLoaded(items));
        });
    }

    /// Load mock items for testing
    fn load_mock_items(review_type: ReviewType) -> Vec<ReviewItem> {
        // Generate mock review items based on type
        let base_words = vec![
            ("abandon", "放弃", "/əˈbændən/"),
            ("benefit", "好处，利益", "/ˈbenɪfɪt/"),
            ("challenge", "挑战", "/ˈtʃælɪndʒ/"),
            ("determine", "决定，确定", "/dɪˈtɜːrmɪn/"),
            ("environment", "环境", "/ɪnˈvaɪrənmənt/"),
            ("facilitate", "促进，使便利", "/fəˈsɪlɪteɪt/"),
            ("generate", "产生，生成", "/ˈdʒenəreɪt/"),
            ("hypothesis", "假设", "/haɪˈpɒθəsɪs/"),
            ("implement", "实施，执行", "/ˈɪmplɪment/"),
            ("justify", "证明…正当", "/ˈdʒʌstɪfaɪ/"),
        ];

        let wrong_meanings = vec![
            vec!["接受", "支持", "保留"],
            vec!["损失", "代价", "困难"],
            vec!["机会", "答案", "问题"],
            vec!["怀疑", "忽视", "改变"],
            vec!["发展", "经济", "社会"],
            vec!["阻止", "限制", "复杂"],
            vec!["消耗", "减少", "维持"],
            vec!["事实", "结论", "证据"],
            vec!["忽略", "取消", "推迟"],
            vec!["否认", "质疑", "解释"],
        ];

        let example_sentences = vec![
            ("Don't abandon your dreams.", "不要放弃你的梦想。", "abandon"),
            ("Education has many benefits.", "教育有很多好处。", "benefits"),
            ("This is a big challenge.", "这是一个大挑战。", "challenge"),
            ("We need to determine the cause.", "我们需要确定原因。", "determine"),
            ("Protect the environment.", "保护环境。", "environment"),
            ("Technology can facilitate learning.", "技术可以促进学习。", "facilitate"),
            ("Solar panels generate electricity.", "太阳能板产生电力。", "generate"),
            ("Test the hypothesis.", "检验这个假设。", "hypothesis"),
            ("We will implement the plan.", "我们将实施这个计划。", "implement"),
            ("Can you justify your decision?", "你能证明你的决定是正确的吗？", "justify"),
        ];

        base_words
            .into_iter()
            .enumerate()
            .map(|(i, (word, word_zh, phonetic))| {
                let (sent_en, sent_zh, blank) = example_sentences[i];
                ReviewItem {
                    id: format!("word_{}", i),
                    word: word.to_string(),
                    word_zh: word_zh.to_string(),
                    phonetic: phonetic.to_string(),
                    audio_url: Some(format!("https://api.example.com/audio/{}.mp3", word)),
                    mastery_level: (i * 10) as u8,
                    example_sentences: vec![super::types::ExampleSentence {
                        sentence: sent_en.to_string(),
                        sentence_zh: sent_zh.to_string(),
                        blank_word: blank.to_string(),
                    }],
                    wrong_meanings: wrong_meanings[i].iter().map(|s| s.to_string()).collect(),
                }
            })
            .collect()
    }

    /// Poll for messages from background threads
    pub fn poll(&mut self) -> SessionAction {
        let rx = match &self.rx {
            Some(rx) => rx,
            None => return SessionAction::None,
        };

        match rx.try_recv() {
            Ok(msg) => self.handle_message(msg),
            Err(mpsc::TryRecvError::Empty) => SessionAction::None,
            Err(mpsc::TryRecvError::Disconnected) => {
                self.rx = None;
                self.loading = false;
                SessionAction::None
            }
        }
    }

    /// Handle a message from background thread
    fn handle_message(&mut self, msg: SessionMessage) -> SessionAction {
        match msg {
            SessionMessage::ItemsLoaded(items) => {
                self.loading = false;
                if items.is_empty() {
                    return SessionAction::Error("No items to review".to_string());
                }

                let review_type = self.session.as_ref()
                    .map(|s| s.review_type)
                    .unwrap_or_default();

                self.session = Some(ReviewSession::new(review_type, items));
                self.item_start_time_ms = Self::current_time_ms();
                SessionAction::SessionStarted
            }
            SessionMessage::LoadError(e) => {
                self.loading = false;
                SessionAction::Error(e)
            }
            SessionMessage::ResultSubmitted => SessionAction::None,
            SessionMessage::SubmitError(e) => SessionAction::Error(e),
            SessionMessage::PronunciationScore { item_id, score } => {
                // Update the current result with the score
                if let Some(session) = &mut self.session {
                    if let Some(result) = session.results.iter_mut().rev().find(|r| r.item_id == item_id) {
                        result.score = Some(score);
                        result.correct = score >= 60; // Threshold for "correct"
                    }
                }
                SessionAction::None
            }
        }
    }

    /// Submit a result for the current item
    pub fn submit_result(&mut self, correct: bool, user_answer: Option<String>, self_rating: Option<u8>) -> SessionAction {
        let session = match &mut self.session {
            Some(s) => s,
            None => return SessionAction::Error("No active session".to_string()),
        };

        let item = match session.current_item() {
            Some(i) => i.clone(),
            None => return SessionAction::Error("No current item".to_string()),
        };

        let time_spent = Self::current_time_ms() - self.item_start_time_ms;

        let result = ReviewResult {
            item_id: item.id.clone(),
            correct,
            score: None, // Set by pronunciation scoring
            time_spent_ms: time_spent,
            user_answer,
            self_rating,
        };

        session.submit_result(result);

        if session.completed {
            let summary = SessionSummary::from(session as &ReviewSession);
            SessionAction::SessionCompleted(summary)
        } else {
            self.item_start_time_ms = Self::current_time_ms();
            SessionAction::ItemChanged
        }
    }

    /// Get current timestamp in milliseconds
    fn current_time_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Get the current session
    pub fn session(&self) -> Option<&ReviewSession> {
        self.session.as_ref()
    }

    /// Get the current item
    pub fn current_item(&self) -> Option<&ReviewItem> {
        self.session.as_ref()?.current_item()
    }

    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.loading
    }

    /// Get session progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        self.session.as_ref().map(|s| s.progress()).unwrap_or(0.0)
    }

    /// Get current item index (1-based for display)
    pub fn current_item_number(&self) -> usize {
        self.session.as_ref().map(|s| s.current_index + 1).unwrap_or(0)
    }

    /// Get total item count
    pub fn total_items(&self) -> usize {
        self.session.as_ref().map(|s| s.items.len()).unwrap_or(0)
    }

    /// Reset the session manager
    pub fn reset(&mut self) {
        self.session = None;
        self.rx = None;
        self.loading = false;
        self.item_start_time_ms = 0;
    }
}
