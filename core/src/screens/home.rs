pub mod home_screen;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============home screen live design");
    home_screen::live_design(cx);
}
