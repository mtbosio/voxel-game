//! World and voxel systems (Phase 2+).
//! Chunk loading, terrain generation, and block types will be implemented here.

use std::sync::Arc;

use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

/// Simple flat heightmap: surface at constant Y (T014). T016 will add noise.
const TERRAIN_SURFACE_Y: i32 = 0;

/// Terrain lookup: returns block type (voxel) at world position from a simple heightmap (T014).
/// Below surface = solid (material index 0); at or above surface = air.
#[must_use]
pub fn terrain_lookup(pos: IVec3) -> WorldVoxel {
    if pos.y < TERRAIN_SURFACE_Y {
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

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        Box::new(move |_chunk_pos, _lod, _previous| {
            Box::new(move |pos: IVec3, _prev| terrain_lookup(pos))
        })
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|_mat| [0, 0, 0])
    }
}

/// Spawns the camera (with VoxelWorldCamera so chunks load around it) and default lighting.
pub fn setup_voxel_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
        VoxelWorldCamera::<VoxelWorld>::default(),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
