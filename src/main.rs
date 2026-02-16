//! Entry point for voxel-game. Minimal Bevy app with default plugins and a window (T003).

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
