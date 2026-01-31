pub mod chat_screen;
pub mod chat_sidebar;
pub mod context_dialog;
pub mod message_bubble;
pub mod mofa_hero;

pub use chat_screen::{ChatScreen, ChatScreenWidgetRefExt};
pub use chat_sidebar::{ChatSidebar, ChatSidebarAction, ChatSidebarWidgetRefExt};
pub use context_dialog::{ContextCard, ContextCardWidgetRefExt, ContextDialog, ContextDialogAction, ContextDialogWidgetRefExt};
pub use message_bubble::{AiBubble, AiBubbleWidgetRefExt, MessageList, UserBubble, UserBubbleWidgetRefExt};
use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    message_bubble::live_design(cx);
    chat_sidebar::live_design(cx);
    context_dialog::live_design(cx);
    mofa_hero::live_design(cx);
    chat_screen::live_design(cx);
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
