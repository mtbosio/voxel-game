# Phase 3: Biomes

**Status:** 🔄 Not Started

**Objective:** Introduce multiple biomes (e.g. forest, plains, desert) with distinct terrain height, surface blocks, and optional vegetation so the world has recognizable regions.

**Completion Tracking:**
- [ ] Phase 3 Complete

## Task Checklist

### Biome Definition
- [ ] Section Complete
- [ ] T026 Define a biome type (e.g. enum or ID) and biome data: base height range, surface block, subsurface block, and optional filler (e.g. sand, grass, stone).
- [ ] T027 Implement biome sampling from world position (e.g. 2D x/z) using the `noise` crate so biomes form contiguous regions (e.g. use noise for biome blend or discrete zones).
- [ ] T028 Ensure biome choice is deterministic from world coordinates so the same seed/position always yields the same biome.

### Biome Terrain
- [ ] Section Complete
- [ ] T029 For each biome, apply height variation (e.g. different noise scale/amplitude for hills vs flat); plains flatter, forest/desert as defined.
- [ ] T030 Set surface and subsurface blocks per biome (e.g. grass/dirt for plains and forest, sand/sandstone for desert).
- [ ] T031 Add at least three biomes (e.g. plains, forest, desert) with visibly different terrain and block appearance.

### Vegetation & Detail (Optional for MVP)
- [ ] Section Complete
- [ ] T032 Optionally place simple “vegetation” (e.g. extra blocks like tall grass or cactus) on the surface based on biome; skip if deferring to a later phase.
- [ ] T033 Ensure vegetation does not break chunk generation or cause duplicate blocks at same position.

### Verification
- [ ] Section Complete
- [ ] T034 In-game verification: walk between biomes and confirm distinct terrain and blocks; no crashes or missing chunks.
- [ ] T035 Update architecture or setup docs if new config or data files are added; mark Phase 3 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
