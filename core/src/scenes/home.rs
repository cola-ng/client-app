//! Home App - Dashboard and learning hub
//!
//! Provides the main home screen with:
//! - Welcome section with personalized greeting
//! - Today's tasks with progress tracking
//! - Quick action shortcuts
//! - Learning statistics
//! - AI-powered insights and recommendations

pub mod screen;

pub use screen::HomeScreen;
pub use screen::HomeScreenWidgetRefExt;

use makepad_widgets::Cx;
use widgets::{AppInfo, AppScene};

/// Home app descriptor
pub struct SceneApp;

impl AppScene for SceneApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "Home",
            id: "home",
            description: "Dashboard and learning hub",
        }
    }

    fn live_design(cx: &mut Cx) {
        screen::live_design(cx);
    }
}

/// Register all Home widgets with Makepad
pub fn live_design(cx: &mut Cx) {
    SceneApp::live_design(cx);
}
