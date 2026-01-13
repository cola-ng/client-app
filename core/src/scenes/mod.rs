//! Scene modules for Colang Desktop

pub mod dialog;
pub mod home;
pub mod settings;

// Re-exports
pub use dialog::{ColangScreen, ColangScreenWidgetRefExt, DialogScene};
pub use home::{HomeScreen, HomeScreenWidgetRefExt, SceneApp as HomeApp};
pub use settings::SettingsScene;
