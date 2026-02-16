//! Entry point for voxel-game. Minimal Bevy app with default plugins and a window (T003).
//! Project layout: world, player, ui, net modules (T004).

mod net;
mod player;
mod ui;
mod world;

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, WindowPlugin};
use bevy_voxel_world::prelude::*;
use world::VoxelWorld;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_cursor_options: Some(CursorOptions {
                    grab_mode: CursorGrabMode::Locked,
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(VoxelWorldPlugin::with_config(VoxelWorld))
        .add_systems(Startup, (world::setup_voxel_camera, player::setup_player))
        .add_systems(
            Update,
            (
                player::mouse_look,
                player::movement_input.after(player::mouse_look),
                player::apply_movement.after(player::movement_input),
                player::query_voxel_colliders.after(player::apply_movement),
                player::resolve_voxel_collision.after(player::query_voxel_colliders),
            ),
        )
        .run();
}
