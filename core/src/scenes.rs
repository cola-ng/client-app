//! Scene modules for Colang Desktop

pub mod dialog;
pub mod home;
pub mod review;
pub mod scene_center;
pub mod settings;

// pub use dialog::{DialogScene, DialogSceneWidgetRefExt};
// pub use home::{HomeScene, HomeSceneWidgetRefExt};
// pub use settings::{SettingsScene, SettingsSceneWidgetRefExt};

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    println!("==============scenes live design");
    home::live_design(cx);
    dialog::live_design(cx);
    review::live_design(cx);
    scene_center::live_design(cx);
    settings::live_design(cx);
}
