//! Scene modules for Colang Desktop

pub mod dialog;
pub mod home;
pub mod settings;

pub use dialog::{DialogScene, DialogSceneWidgetRefExt};
pub use home::{HomeScene, HomeSceneWidgetRefExt};
pub use settings::{SettingsScene, SettingsSceneWidgetRefExt};
