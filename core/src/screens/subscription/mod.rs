//! # Subscription Module
//!
//! Subscription management screen with plans and wallet.

pub mod subscription_screen;

pub use subscription_screen::*;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    subscription_screen::live_design(cx);
}
