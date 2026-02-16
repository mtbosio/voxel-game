//! Player and first-person systems (Phase 4+).
//! Controller, movement, gravity, and collision will be implemented here.

use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

use crate::world::VoxelWorld;

/// Marker for the player entity (first-person controller).
#[derive(Component)]
pub struct Player;

/// Stores yaw (horizontal) and pitch (vertical) in radians for first-person look (T037).
#[derive(Component)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
}

/// Mouse look sensitivity (radians per pixel of motion).
pub const MOUSE_SENSITIVITY: f32 = 0.001;

/// Pitch clamp in radians (avoid gimbal flip); ±89° ≈ ±1.553.
const PITCH_MIN: f32 = -1.553_343_f32;
const PITCH_MAX: f32 = 1.553_343_f32;

/// Eye height offset from player feet (camera attached at this height).
/// Minecraft-like default ~1.62m; use 1.6 for a round value.
pub const PLAYER_EYE_HEIGHT: f32 = 1.6;

/// Spawns the player entity with a first-person camera at eye height.
/// Camera is a child of the player so it follows the player transform.
pub fn setup_player(mut commands: Commands) {
    let spawn_position = Vec3::new(0.0, 2.0, 0.0);
    commands.spawn((
        Player,
        PlayerLook { yaw: 0.0, pitch: 0.0 },
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

/// Applies mouse motion to player yaw (horizontal) and pitch (vertical) with sensitivity and pitch clamping (T037).
pub fn mouse_look(
    motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(Entity, &mut Transform, &mut PlayerLook), With<Player>>,
    children_query: Query<&Children>,
    mut camera_query: Query<&mut Transform, Without<Player>>,
) {
    let delta = motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    for (player_entity, mut player_transform, mut look) in query.iter_mut() {
        look.yaw -= delta.x * MOUSE_SENSITIVITY;
        look.pitch += delta.y * MOUSE_SENSITIVITY;
        look.pitch = look.pitch.clamp(PITCH_MIN, PITCH_MAX);

        player_transform.rotation = Quat::from_rotation_y(look.yaw);

        if let Ok(children) = children_query.get(player_entity) {
            if let Some(&camera_entity) = children.first() {
                if let Ok(mut cam_transform) = camera_query.get_mut(camera_entity) {
                    cam_transform.rotation = Quat::from_rotation_x(-look.pitch);
                }
            }
        }
    }
}
