//! MoFA FM App - AI-powered audio streaming and voice interface

pub mod audio;
pub mod audio_player;
pub mod database;
pub mod dora_integration;
pub mod doubao_api;
pub mod log_bridge;
pub mod mofa_hero;
pub mod screen;

pub use audio::AudioManager;
pub use database::Database;
pub use dora_integration::{DoraCommand, DoraEvent, DoraIntegration, DoraState};
pub use doubao_api::DoubaoClient;
pub use mofa_hero::{ConnectionStatus, MofaHero, MofaHeroAction};
pub use screen::ColangScreen;
pub use screen::ColangScreenWidgetRefExt; // Export WidgetRefExt for timer control

use makepad_widgets::Cx;
use widgets::{AppInfo, AppScene};

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
