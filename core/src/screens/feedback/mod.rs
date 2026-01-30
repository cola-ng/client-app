//! # Feedback Module
//!
//! User feedback submission screen.

pub mod feedback_screen;

pub use feedback_screen::*;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    feedback_screen::live_design(cx);
}
