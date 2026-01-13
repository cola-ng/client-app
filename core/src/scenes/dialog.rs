
pub mod dialog_scene;
pub mod mofa_hero;

pub use dialog_scene::{DialogScene, DialogSceneWidgetRefExt};

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    mofa_hero::live_design(cx);
    dialog_scene::live_design(cx);
}

/// Chat message entry for display
#[derive(Clone, Debug)]
pub struct ChatMessageEntry {
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
    pub is_streaming: bool,
    pub session_id: Option<String>,
}

impl ChatMessageEntry {
    pub fn new(sender: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            sender: sender.into(),
            content: content.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            is_streaming: false,
            session_id: None,
        }
    }
}