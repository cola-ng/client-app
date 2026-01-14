pub mod scene_center_screen;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============screen center screen live design");
    scene_center_screen::live_design(cx);
}
