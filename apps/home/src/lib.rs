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
use widgets::{AppInfo, MofaApp};

/// Home app descriptor
pub struct HomeApp;

impl MofaApp for HomeApp {
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
    HomeApp::live_design(cx);
}
