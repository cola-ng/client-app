pub mod home_scene;

pub use home_scene::{HomeScene, HomeSceneWidgetRefExt};

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    home_scene::live_design(cx);
}
