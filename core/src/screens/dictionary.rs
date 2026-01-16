pub mod dictionary_screen;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============dictionary screen live design");
    dictionary_screen::live_design(cx);
}
