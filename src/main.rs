//! Entry point for voxel-game. Minimal Bevy app with default plugins and a window (T003).
//! Project layout: world, player, ui, net modules (T004).

mod net;
mod player;
mod ui;
mod world;

use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use world::VoxelWorld;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelWorldPlugin::with_config(VoxelWorld))
        .add_systems(Startup, (world::setup_voxel_camera, player::setup_player))
        .run();
}
