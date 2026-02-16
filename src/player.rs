//! Player and first-person systems (Phase 4+).
//! Controller, movement, gravity, and collision will be implemented here.

use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

use crate::world::VoxelWorld;

/// Marker for the player entity (first-person controller).
#[derive(Component)]
pub struct Player;

/// Eye height offset from player feet (camera attached at this height).
/// Minecraft-like default ~1.62m; use 1.6 for a round value.
pub const PLAYER_EYE_HEIGHT: f32 = 1.6;

/// Spawns the player entity with a first-person camera at eye height.
/// Camera is a child of the player so it follows the player transform.
pub fn setup_player(mut commands: Commands) {
    let spawn_position = Vec3::new(0.0, 2.0, 0.0);
    commands.spawn((
        Player,
        Transform::from_translation(spawn_position),
        GlobalTransform::default(),
    )).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, PLAYER_EYE_HEIGHT, 0.0),
            VoxelWorldCamera::<VoxelWorld>::default(),
        ));
    });
}
