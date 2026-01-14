//! Screen modules for Colang Desktop

pub mod assistant;
pub mod classic_dialogues;
pub mod dialog;
pub mod home;
pub mod reading;
pub mod review;
pub mod scene_center;
pub mod settings;

// pub use dialog::{DialogScreen, DialogScreenWidgetRefExt};
// pub use home::{HomeScreen, HomeScreenWidgetRefExt};
// pub use settings::{SettingsScreen, SettingsScreenWidgetRefExt};

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    println!("==============screens live design");
    home::live_design(cx);
    dialog::live_design(cx);
    review::live_design(cx);
    scene_center::live_design(cx);
    settings::live_design(cx);
    assistant::live_design(cx);
    reading::live_design(cx);
    classic_dialogues::live_design(cx);
}
