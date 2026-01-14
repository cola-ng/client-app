pub mod classic_dialogues_screen;

use makepad_widgets::Cx;

pub(super) fn live_design(cx: &mut Cx) {
    println!("==============classic_dialogues screen live design");
    classic_dialogues_screen::live_design(cx);
}
