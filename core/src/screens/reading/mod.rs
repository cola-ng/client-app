pub mod reading_screen;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============reading screen live design");
    reading_screen::live_design(cx);
}
