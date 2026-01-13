pub mod review_scene;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    review_scene::live_design(cx);
}
