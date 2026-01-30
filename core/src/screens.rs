//! Screen modules for Colang Desktop

pub mod chat;
pub mod dictionary;
pub mod favorites;
pub mod feedback;
pub mod home;
pub mod login;
pub mod profile;
pub mod reading;
pub mod review;
pub mod scenes;
pub mod settings;
pub mod subscription;

// pub use dialog::{DialogScreen, DialogScreenWidgetRefExt};
// pub use home::{HomeScreen, HomeScreenWidgetRefExt};
// pub use settings::{SettingsScreen, SettingsScreenWidgetRefExt};

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    println!("==============screens live design");
    home::live_design(cx);
    chat::live_design(cx);
    dictionary::live_design(cx);
    review::live_design(cx);
    scenes::live_design(cx);
    settings::live_design(cx);
    reading::live_design(cx);
    login::live_design(cx);
    favorites::live_design(cx);
    profile::live_design(cx);
    subscription::live_design(cx);
    feedback::live_design(cx);
}
