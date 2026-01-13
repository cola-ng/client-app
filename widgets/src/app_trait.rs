//! # AppScene Trait - Plugin App Interface
//!
//! This module defines the standard interface for apps that integrate with the MoFA Studio shell.
//!
//! ## Architecture
//!
//! Due to Makepad's compile-time `live_design!` macro requirements, widget types must
//! still be imported directly in the shell. This trait provides:
//!
//! - **Standardized metadata** - App name, ID, description via [`AppInfo`]
//! - **Consistent registration** - Widget registration via [`AppScene::live_design`]
//! - **Timer lifecycle** - Resource management via [`TimerControl`]

use makepad_widgets::Cx;
/// Trait for apps with timer-based animations that need lifecycle control
///
/// Apps implementing this trait should stop their timers when hidden
/// and restart them when shown, to prevent resource waste.
pub trait TimerControl {
    /// Stop all timers (call when app becomes hidden)
    fn stop_timers(&self, cx: &mut Cx);

    /// Start/restart timers (call when app becomes visible)
    fn start_timers(&self, cx: &mut Cx);
}

/// Trait for widgets that respond to global state changes
///
/// Apps implement this trait to receive notifications when global state
/// changes (e.g., dark mode toggle, provider configuration updates).
///
/// # Example
/// ```ignore
/// impl StateChangeListener for MyScreenRef {
///     fn on_dark_mode_change(&self, cx: &mut Cx, dark_mode: f64) {
///         if let Some(mut inner) = self.borrow_mut() {
///             inner.view.apply_over(cx, live!{
///                 draw_bg: { dark_mode: (dark_mode) }
///             });
///         }
///     }
/// }
/// ```
pub trait StateChangeListener {
    /// Called when dark mode setting changes
    ///
    /// # Arguments
    /// * `cx` - Makepad context for applying UI updates
    /// * `dark_mode` - Dark mode value (0.0 = light, 1.0 = dark)
    fn on_dark_mode_change(&self, cx: &mut Cx, dark_mode: f64);
}
