//! Player and first-person systems (Phase 4+).
//! Controller, movement, gravity, and collision will be implemented here.

use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy_voxel_world::prelude::{VoxelWorld as VoxelWorldParam, VoxelWorldCamera, WorldVoxel};

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

/// Total player height for collision AABB (T042). Minecraft-like ~1.8m.
pub const PLAYER_HEIGHT: f32 = 1.8;

/// Player width (XZ) for collision AABB (T042). Minecraft-like ~0.6m.
pub const PLAYER_WIDTH: f32 = 0.6;

/// Solid voxel positions that overlap the player AABB (T042). Updated each frame; T043 uses for collision resolution.
#[derive(Component, Default)]
pub struct CollidingVoxels(pub Vec<IVec3>);

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
        CollidingVoxels::default(),
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

/// Resolves collision with solid voxels: pushes player out of blocks and sets grounded when standing on a surface (T043).
/// Vertical collision: floor (push up, set grounded) and ceiling (push down, zero upward velocity) so the player does not get stuck (T044).
/// Runs after query_voxel_colliders; uses CollidingVoxels to resolve penetration and set Grounded.
pub fn resolve_voxel_collision(
    mut query: Query<
        (
            &CollidingVoxels,
            &mut Transform,
            &mut Grounded,
            &mut PlayerVelocity,
        ),
        With<Player>,
    >,
) {
    let half_w = PLAYER_WIDTH * 0.5;
    let half_h = PLAYER_HEIGHT * 0.5;
    for (colliders, mut transform, mut grounded, mut velocity) in query.iter_mut() {
        let pos = &mut transform.translation;
        let mut push_up = 0.0_f32;
        let mut push_down = 0.0_f32;
        let mut push_x_neg = 0.0_f32;
        let mut push_x_pos = 0.0_f32;
        let mut push_z_neg = 0.0_f32;
        let mut push_z_pos = 0.0_f32;

        let player_min = Vec3::new(pos.x - half_w, pos.y, pos.z - half_w);
        let player_max = Vec3::new(pos.x + half_w, pos.y + PLAYER_HEIGHT, pos.z + half_w);
        let player_center_y = pos.y + half_h;

        for &v in &colliders.0 {
            let vx = v.x as f32;
            let vy = v.y as f32;
            let vz = v.z as f32;
            let voxel_min = Vec3::new(vx, vy, vz);
            let voxel_max = Vec3::new(vx + 1.0, vy + 1.0, vz + 1.0);
            let voxel_center_y = vy + 0.5;

            let overlap_x = player_max.x.min(voxel_max.x) - player_min.x.max(voxel_min.x);
            let overlap_y = player_max.y.min(voxel_max.y) - player_min.y.max(voxel_min.y);
            let overlap_z = player_max.z.min(voxel_max.z) - player_min.z.max(voxel_min.z);

            if overlap_x <= 0.0 || overlap_y <= 0.0 || overlap_z <= 0.0 {
                continue;
            }

            if overlap_y > 0.0 {
                if player_center_y < voxel_center_y {
                    push_up = push_up.max(overlap_y);
                } else {
                    push_down = push_down.max(overlap_y);
                }
            }
            if overlap_x > 0.0 {
                let player_center_x = pos.x;
                let voxel_center_x = vx + 0.5;
                if player_center_x < voxel_center_x {
                    push_x_neg = push_x_neg.max(overlap_x);
                } else {
                    push_x_pos = push_x_pos.max(overlap_x);
                }
            }
            if overlap_z > 0.0 {
                let player_center_z = pos.z;
                let voxel_center_z = vz + 0.5;
                if player_center_z < voxel_center_z {
                    push_z_neg = push_z_neg.max(overlap_z);
                } else {
                    push_z_pos = push_z_pos.max(overlap_z);
                }
            }
        }

        if push_up > 0.0 && (push_down <= 0.0 || push_up <= push_down) {
            pos.y += push_up;
            grounded.0 = true;
            velocity.value.y = velocity.value.y.max(0.0);
        } else if push_down > 0.0 {
            // T044: ceiling — push player down out of block and cancel upward velocity so player does not get stuck
            pos.y -= push_down;
            grounded.0 = false;
            velocity.value.y = velocity.value.y.min(0.0);
        } else {
            grounded.0 = false;
        }

        if push_x_neg > 0.0 && (push_x_pos <= 0.0 || push_x_neg <= push_x_pos) {
            pos.x -= push_x_neg;
        } else if push_x_pos > 0.0 {
            pos.x += push_x_pos;
        }
        if push_z_neg > 0.0 && (push_z_pos <= 0.0 || push_z_neg <= push_z_pos) {
            pos.z -= push_z_neg;
        } else if push_z_pos > 0.0 {
            pos.z += push_z_pos;
        }
    }
}

/// Queries the voxel world for solid blocks at the player AABB; treats solid blocks as colliders (T042).
/// Fills CollidingVoxels with voxel positions that overlap the player and are solid.
pub fn query_voxel_colliders(
    voxel_world: VoxelWorldParam<VoxelWorld>,
    mut query: Query<(&Transform, &mut CollidingVoxels), With<Player>>,
) {
    let half_w = PLAYER_WIDTH * 0.5;
    let half_h = PLAYER_HEIGHT * 0.5;
    for (transform, mut colliders) in query.iter_mut() {
        colliders.0.clear();
        let pos = transform.translation;
        let center = pos + Vec3::new(0.0, half_h, 0.0);
        let min_x = (center.x - half_w).floor() as i32;
        let max_x = (center.x + half_w).floor() as i32;
        let min_y = (center.y - half_h).floor() as i32;
        let max_y = (center.y + half_h).floor() as i32;
        let min_z = (center.z - half_w).floor() as i32;
        let max_z = (center.z + half_w).floor() as i32;
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let v = voxel_world.get_voxel(IVec3::new(x, y, z));
                    if let WorldVoxel::Solid(_) = v {
                        colliders.0.push(IVec3::new(x, y, z));
                    }
                }
            }
        }
    }
}

