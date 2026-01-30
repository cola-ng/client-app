//! # Profile Module
//!
//! User profile management screen.

pub mod profile_screen;

pub use profile_screen::*;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    profile_screen::live_design(cx);
}
