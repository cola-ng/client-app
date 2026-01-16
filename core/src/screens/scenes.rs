pub mod scenes_screen;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============screen center screen live design");
    scenes_screen::live_design(cx);
}
