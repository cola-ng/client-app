//! Scene modules for Colang Desktop

pub mod dialog;
pub mod home;
pub mod settings;

// Re-exports
pub use dialog::ColangApp;
pub use dialog::ColangScreen;
pub use dialog::ColangScreenWidgetRefExt;
pub use home::SceneApp as HomeApp;
pub use home::HomeScreen;
pub use home::HomeScreenWidgetRefExt;
pub use settings::SettingsScene;
