//! Entry point for voxel-game. Minimal Bevy app with default plugins and a window (T003).
//! Project layout: world, player, ui, net modules (T004).

mod net;
mod player;
mod ui;
mod world;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
