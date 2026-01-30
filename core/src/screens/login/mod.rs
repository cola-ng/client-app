//! # Login Module
//!
//! User authentication screen with login/register functionality.

mod components;
pub mod login_screen;

pub use login_screen::*;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    components::live_design(cx);
    login_screen::live_design(cx);
}
