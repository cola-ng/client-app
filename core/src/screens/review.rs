mod components;
mod due_screen;
mod mastered_screen;
mod mistakes_screen;
pub mod review_screen;
mod stats_screen;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    components::live_design(cx);
    due_screen::live_design(cx);
    mastered_screen::live_design(cx);
    mistakes_screen::live_design(cx);
    stats_screen::live_design(cx);
    review_screen::live_design(cx);
}
