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

/// Desired horizontal movement direction in world XZ (T039). Y is 0; length 0 or 1.
#[derive(Component)]
pub struct PlayerMovementInput {
    pub direction: Vec3,
}

/// Current velocity in world space (m/s). T043 will zero vertical when grounded.
#[derive(Component)]
pub struct PlayerVelocity {
    pub value: Vec3,
}

/// Whether the player is standing on solid ground (T043 sets from collision; T040 uses for gravity).
#[derive(Component)]
pub struct Grounded(pub bool);

/// Horizontal walk speed in m/s (T040).
pub const WALK_SPEED: f32 = 4.5;

/// Gravity magnitude (positive; applied as negative Y) in m/s² (T040).
pub const GRAVITY: f32 = 20.0;

/// Initial upward velocity for jump in m/s (T041). Height ≈ JUMP_VELOCITY² / (2 * GRAVITY).
pub const JUMP_VELOCITY: f32 = 7.0;

/// Horizontal sprint speed in m/s when Shift held (T041).
pub const SPRINT_SPEED: f32 = 6.0;

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
        PlayerMovementInput {
            direction: Vec3::ZERO,
        },
        PlayerVelocity {
            value: Vec3::ZERO,
        },
        Grounded(false),
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

/// Updates desired movement direction from WASD relative to where the player is looking (T039).
/// Uses the player Transform rotation (same as camera) so forward/back/strafe match view.
fn horizontal_xz(v: Vec3) -> Vec3 {
    let flat = Vec3::new(v.x, 0.0, v.z);
    let len_sq = flat.length_squared();
    if len_sq > 0.0 {
        flat / len_sq.sqrt()
    } else {
        Vec3::ZERO
    }
}

/// Updates desired movement direction from WASD relative to camera yaw (T039).
/// Forward/back/strafe match view direction in the horizontal plane.
pub fn movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut PlayerMovementInput), With<Player>>,
) {
    for (transform, mut input) in query.iter_mut() {
        let forward = horizontal_xz(transform.rotation * Vec3::NEG_Z);
        let right = horizontal_xz(transform.rotation * Vec3::X);

        let mut dir = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            dir += forward;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            dir -= forward;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            dir += right;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            dir -= right;
        }

        input.direction = if dir.length_squared() > 0.0 {
            dir.normalize_or_zero()
        } else {
            Vec3::ZERO
        };
    }
}

/// Applies horizontal movement from input (configurable speed), gravity when not grounded, jump when grounded, and sprint (T040, T041).
pub fn apply_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &PlayerMovementInput,
            &Grounded,
            &mut PlayerVelocity,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    let dt = time.delta_secs();
    let sprinting = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let speed = if sprinting { SPRINT_SPEED } else { WALK_SPEED };

    for (input, grounded, mut velocity, mut transform) in query.iter_mut() {
        velocity.value.x = input.direction.x * speed;
        velocity.value.z = input.direction.z * speed;

        if grounded.0 {
            if keyboard.just_pressed(KeyCode::Space) {
                velocity.value.y = JUMP_VELOCITY;
            } else {
                velocity.value.y = 0.0;
            }
        } else {
            velocity.value.y -= GRAVITY * dt;
        }

        transform.translation += velocity.value * dt;
    }
}

/// Temporary ground plane at Y=0 until voxel collision is implemented (T042/T043).
/// Prevents the player from falling through the world so the view stays above terrain.
pub fn temporary_ground_plane(
    mut query: Query<(&mut Transform, &mut Grounded), With<Player>>,
) {
    for (mut transform, mut grounded) in query.iter_mut() {
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            grounded.0 = true;
        } else {
            grounded.0 = false;
        }
    }
}
