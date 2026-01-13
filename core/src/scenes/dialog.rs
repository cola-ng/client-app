//! MoFA FM App - AI-powered audio streaming and voice interface

pub mod mofa_hero;
pub mod screen;

use makepad_widgets::Cx;
pub use mofa_hero::{ConnectionStatus, MofaHero, MofaHeroAction};
pub use screen::ColangScreen;
pub use screen::ColangScreenWidgetRefExt; // Export WidgetRefExt for timer control
use widgets::{AppInfo, AppScene};

pub use crate::audio::AudioManager;
pub use crate::db::Database;
use crate::dora_integration::{DoraCommand, DoraEvent, DoraIntegration, DoraState};
use crate::doubao_api::DoubaoClient;

/// MoFA FM app descriptor
pub struct ColangApp;

impl AppScene for ColangApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "MoFA FM",
            id: "mofa-fm",
            description: "AI-powered audio streaming and voice interface",
        }
    }

    fn live_design(cx: &mut Cx) {
        mofa_hero::live_design(cx);
        screen::live_design(cx);
    }
}

/// Register all MoFA FM widgets with Makepad
/// (Kept for backwards compatibility - calls DoraApp::live_design)
pub fn live_design(cx: &mut Cx) {
    ColangApp::live_design(cx);
}
