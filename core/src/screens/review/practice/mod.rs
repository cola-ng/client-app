//! # Practice Module
//!
//! This module contains the review practice functionality with 5 exercise types:
//! - Pronunciation: Record and compare pronunciation with AI scoring
//! - Meaning: 4-choice meaning selection
//! - Spelling: Listen and type (dictation)
//! - Recognition: Flip card with self-evaluation
//! - Phrase: Fill-in-the-blank sentence completion

pub mod types;
pub mod session;
pub mod type_selector_screen;
pub mod practice_screen;
pub mod pronunciation_view;
pub mod meaning_view;
pub mod spelling_view;
pub mod recognition_view;
pub mod phrase_view;
pub mod score_display;

pub use types::*;
pub use session::*;
pub use type_selector_screen::*;
pub use practice_screen::*;
pub use pronunciation_view::*;
pub use meaning_view::*;
pub use spelling_view::*;
pub use recognition_view::*;
pub use phrase_view::*;
pub use score_display::*;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    type_selector_screen::live_design(cx);
    practice_screen::live_design(cx);
    pronunciation_view::live_design(cx);
    meaning_view::live_design(cx);
    spelling_view::live_design(cx);
    recognition_view::live_design(cx);
    phrase_view::live_design(cx);
    score_display::live_design(cx);
}
