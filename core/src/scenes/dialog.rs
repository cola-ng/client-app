//! MoFA FM App - AI-powered audio streaming and voice interface

pub mod mofa_hero;
pub mod screen;

use makepad_widgets::Cx;
pub use mofa_hero::{ConnectionStatus, MofaHero, MofaHeroAction};
pub use screen::DialogSceneWidgetRefExt; // Export WidgetRefExt for timer control
use widgets::{AppInfo, AppScene};

pub use crate::audio::AudioManager;
pub use crate::db::Database;

/// MoFA FM app descriptor
pub struct DialogScene;

impl AppScene for DialogScene {
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
    DialogScene::live_design(cx);
}
