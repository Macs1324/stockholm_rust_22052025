pub mod presentation;

use bevy::{prelude::*, render::render_resource::Extent3d};
use presentation::PresentationPlugin;

const NEXT_SLIDE: KeyCode = KeyCode::Space;
const PREVIOUS_SLIDE: KeyCode = KeyCode::ArrowLeft;

const SLIDE_SIZE: Extent3d = Extent3d {
    width: 1920,
    height: 1080,
    depth_or_array_layers: 1,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PresentationPlugin {})
        .run();
}
