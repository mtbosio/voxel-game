# Phase 2: Voxel World & Procedural Generation

**Status:** 🔄 Not Started

**Objective:** Integrate bevy_voxel_world, implement procedural terrain using the noise crate, and establish basic block types (e.g. dirt, stone, grass) so the game has an infinite editable voxel world.

**Completion Tracking:**
- [ ] Phase 2 Complete

## Task Checklist

### Voxel World Integration
- [x] Section Complete
- [x] T012 Add `bevy_voxel_world` (and its dependencies, e.g. `block-mesh`) to `Cargo.toml`; ensure version is compatible with pinned Bevy.
- [x] T013 Register the bevy_voxel_world plugin in the Bevy app and configure chunk size / spawn range so the world loads around the origin.
- [x] T014 Implement a terrain lookup function that returns block type (e.g. voxel ID or material index) from world position (e.g. `IVec3`); start with a simple heightmap (e.g. flat or single noise layer).
- [x] T015 Wire the terrain function into bevy_voxel_world so chunks are generated with the correct blocks; verify terrain appears in-game.

### Noise-Based Terrain
- [ ] Section Complete
- [x] T016 Add the `noise` crate per `@docs/architecture.md`; use it in the terrain function to generate height (e.g. Perlin/simplex) for terrain.
- [x] T017 Define surface layer: above a height threshold use grass (or top block), one layer below use dirt, below that use stone; ensure caves or overhangs are optional for this phase.
- [ ] T018 Tune terrain scale and height so the world looks varied but navigable; document any magic numbers or config used.

### Block Types & Materials
- [ ] Section Complete
- [ ] T019 Define a small set of block types (e.g. air, grass, dirt, stone) and map them to bevy_voxel_world material indices or equivalent.
- [ ] T020 Configure texture or color for each block type so they are visually distinct in the default pipeline.
- [ ] T021 Ensure block types are used consistently in the terrain lookup and in any future persistence (e.g. same IDs for save format).

### Chunk Loading & Performance
- [ ] Section Complete
- [ ] T022 Verify chunks load and unload as the camera moves; no hard crashes or infinite load.
- [ ] T023 Optionally add a simple debug display (e.g. chunk count or FPS) to confirm performance is acceptable; remove or gate behind a flag if desired.

### Verification
- [ ] Section Complete
- [ ] T024 Run `cargo build` and in-game verification: terrain generates, blocks render, moving around shows new chunks; fix any regressions.
- [ ] T025 Update `@docs/setup.md` if new run steps or dependencies are required; mark Phase 2 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
