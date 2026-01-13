pub mod home_scene;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============home scene live design");
    home_scene::live_design(cx);
}
