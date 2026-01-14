pub mod assistant_screen;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============assistant screen live design");
    assistant_screen::live_design(cx);
}
