pub mod about_panel;
pub mod add_provider_modal;
pub mod audio_panel;
pub mod general_panel;
pub mod provider_view;
pub mod providers_panel;
pub mod settings_scene;

pub use settings_scene::{SettingsScene, SettingsSceneWidgetRefExt};

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    about_panel::live_design(cx);
    add_provider_modal::live_design(cx);
    audio_panel::live_design(cx);
    general_panel::live_design(cx);
    provider_view::live_design(cx);
    providers_panel::live_design(cx);
}
