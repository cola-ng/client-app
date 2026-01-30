//! # Favorites Module
//!
//! User favorites management for words and sentences.

pub mod favorites_screen;

pub use favorites_screen::*;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    favorites_screen::live_design(cx);
}
