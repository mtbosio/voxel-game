//! World and voxel systems (Phase 2+).
//! Chunk loading, terrain generation, and block types will be implemented here.

use std::sync::{Arc, OnceLock};

use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use noise::{NoiseFn, Perlin};

/// Perlin noise for terrain height (T016). Shared so the same (x, z) always yields the same height.
static TERRAIN_NOISE: OnceLock<Perlin> = OnceLock::new();

fn terrain_noise() -> &'static Perlin {
    TERRAIN_NOISE.get_or_init(|| Perlin::new(0))
}

/// Base surface height; noise is added to this (T016). T018 will tune scale/amplitude.
const TERRAIN_BASE_Y: i32 = 0;
/// Noise frequency (world units). Smaller = smoother, larger = more variation.
const TERRAIN_NOISE_FREQ: f64 = 0.02;
/// Noise amplitude in blocks; height varies by roughly ± this amount.
const TERRAIN_NOISE_AMP: f64 = 8.0;

/// Terrain lookup: returns block type (voxel) at world position (T014).
/// Height is from Perlin noise at (x, z) (T016). Below surface = solid (material 0); at or above = air.
#[must_use]
pub fn terrain_lookup(pos: IVec3) -> WorldVoxel {
    let perlin = terrain_noise();
    let sample = perlin.get([
        f64::from(pos.x) * TERRAIN_NOISE_FREQ,
        f64::from(pos.z) * TERRAIN_NOISE_FREQ,
    ]);
    let surface_y = TERRAIN_BASE_Y + (sample * TERRAIN_NOISE_AMP).round() as i32;
    if pos.y < surface_y {
        WorldVoxel::Solid(0)
    } else {
        WorldVoxel::Air
    }
}

/// Voxel world config: chunk spawn range and minimal terrain so the world loads around the origin (T013).
#[derive(Resource, Clone, Default)]
pub struct VoxelWorld;

impl VoxelWorldConfig for VoxelWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    /// Chunk spawn radius (chunks); world loads within this distance of the camera.
    fn spawning_distance(&self) -> u32 {
        16
    }

    /// Chunks within this radius stay loaded (margin before despawn).
    fn min_despawn_distance(&self) -> u32 {
        2
    }

    /// Terrain function wired for chunk generation (T015); chunks use terrain_lookup for block types.
    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        Box::new(move |_chunk_pos, _lod, _previous| {
            Box::new(move |pos: IVec3, _prev| terrain_lookup(pos))
        })
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|_mat| [0, 0, 0])
    }
}

/// Spawns default lighting. Player + first-person camera are spawned by player::setup_player.
pub fn setup_voxel_camera(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
