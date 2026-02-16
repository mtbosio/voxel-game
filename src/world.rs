//! World and voxel systems (Phase 2+).
//! Chunk loading, terrain generation, and block types will be implemented here.

use std::sync::{Arc, OnceLock};

use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use noise::{NoiseFn, Perlin};

/// Perlin noise for terrain height (T016). Shared so the same (x, z) always yields the same height.
static TERRAIN_NOISE: OnceLock<Perlin> = OnceLock::new();

fn terrain_noise() -> &'static Perlin {
    TERRAIN_NOISE.get_or_init(|| Perlin::new(TERRAIN_NOISE_SEED))
}

// -----------------------------------------------------------------------------
// Terrain configuration (T018): scale and height for varied, navigable terrain.
// All magic numbers for procedural height are below; change these to tune feel.
//
// - TERRAIN_NOISE_SEED: Perlin RNG seed; same seed = same world shape.
// - TERRAIN_BASE_Y: Average surface level (blocks). Noise is added on top.
// - TERRAIN_NOISE_FREQ: Input scale for noise (per block). Lower = smoother
//   hills (wider features); higher = more bumpy/jagged. ~0.01–0.03 is typical.
// - TERRAIN_NOISE_AMP: Height variation in blocks (±). Lower = flatter;
//   higher = steeper. Keep moderate (e.g. 4–10) for walkable terrain.
// -----------------------------------------------------------------------------
const TERRAIN_NOISE_SEED: u32 = 0;
const TERRAIN_BASE_Y: i32 = 0;
const TERRAIN_NOISE_FREQ: f64 = 0.012;
const TERRAIN_NOISE_AMP: f64 = 6.0;

/// Material indices for surface layers (T017). T019/T021 formalize block types and persistence.
const MATERIAL_STONE: u8 = 0;
const MATERIAL_DIRT: u8 = 1;
const MATERIAL_GRASS: u8 = 2;

/// Terrain lookup: returns block type (voxel) at world position (T014).
/// Height from Perlin at (x, z) (T016). Surface layer (T017): top block = grass, one below = dirt, below that = stone.
#[must_use]
pub fn terrain_lookup(pos: IVec3) -> WorldVoxel {
    let perlin = terrain_noise();
    let sample = perlin.get([
        f64::from(pos.x) * TERRAIN_NOISE_FREQ,
        f64::from(pos.z) * TERRAIN_NOISE_FREQ,
    ]);
    let surface_y = TERRAIN_BASE_Y + (sample * TERRAIN_NOISE_AMP).round() as i32;
    if pos.y >= surface_y {
        WorldVoxel::Air
    } else if pos.y == surface_y - 1 {
        WorldVoxel::Solid(MATERIAL_GRASS)
    } else if pos.y == surface_y - 2 {
        WorldVoxel::Solid(MATERIAL_DIRT)
    } else {
        WorldVoxel::Solid(MATERIAL_STONE)
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
