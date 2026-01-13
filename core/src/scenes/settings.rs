//! MoFA Settings App - Provider configuration and preferences

pub mod about_panel;
pub mod add_provider_modal;
pub mod audio_panel;
pub mod general_panel;
pub mod provider_view;
pub mod providers_panel;
pub mod screen;

pub use screen::SettingsScreenRef;

use makepad_widgets::Cx;
use widgets::{AppInfo, AppScene};

/// MoFA Settings app descriptor
pub struct SettingsScene;

impl AppScene for SettingsScene {
    fn info() -> AppInfo {
        AppInfo {
            name: "Settings",
            id: "settings",
            description: "Provider configuration and preferences",
        }
    }

    fn live_design(cx: &mut Cx) {
        general_panel::live_design(cx);
        audio_panel::live_design(cx);
        about_panel::live_design(cx);
        providers_panel::live_design(cx);
        provider_view::live_design(cx);
        add_provider_modal::live_design(cx);
        screen::live_design(cx);
    }
}

/// Register all Settings widgets with Makepad
/// (Kept for backwards compatibility - calls DoraApp::live_design)
pub fn live_design(cx: &mut Cx) {
    SettingsScene::live_design(cx);
}
