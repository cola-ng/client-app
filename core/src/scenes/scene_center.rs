pub mod scene_center_scene;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============scene center scene live design");
    scene_center_scene::live_design(cx);
}
