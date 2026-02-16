# Voxel Survival Game Phase Plan

**Last Updated:** 2026-02-16  
**Sources:** `@docs/REQUIREMENTS.md`, `@docs/architecture.md`

## Phase Overview

- [x] Phase 1: Project Foundation — Rust + Bevy project setup, runnable window, project structure, and developer setup. *(Complete; see `docs/phases/phase-1.md`.)*
- [ ] Phase 2: Voxel World & Procedural Generation — Integrate bevy_voxel_world, noise-based terrain, chunk loading, and basic block types.
- [ ] Phase 3: Biomes — Multiple biomes (e.g. forest, plains, desert) with distinct terrain and surface blocks.
- [ ] Phase 4: Player & First-Person — First-person controller, movement, gravity, and collision with the voxel world.
- [ ] Phase 5: Block Breaking & Placing — Raycast targeting, break/place blocks, hotbar/inventory, and persist edits.
- [ ] Phase 6: Day/Night Cycle — Game time, sky and lighting changes, and visibility.
- [ ] Phase 7: Crafting System — Inventory UI, crafting grid, recipes, and tool/block crafting.
- [ ] Phase 8: Mobs & Pathfinding — Passive and hostile entities, A* pathfinding (pathfinding crate), and basic AI.
- [ ] Phase 9: Health & Death — Player health, damage, death, respawn, and item drop.
- [ ] Phase 10: Multiplayer — Multiplayer abstraction, Lightyear integration, server and client, world and player sync.

## Guiding Principles

- Execute phases sequentially; do not pull tasks forward until the current phase is complete.
- Follow `@docs/architecture.md`: Rust, Bevy, bevy_voxel_world, pathfinding crate, Lightyear behind abstraction, serde + bincode, noise crate. Do not introduce new stack elements without an architecture update.
- Use Minecraft as the design baseline for behavior and feel (see `@docs/REQUIREMENTS.md`).
- Work on one task at a time; check off tasks in the phase file and update phase status.
- Update `@docs/architecture.md` when implementation introduces new dependencies or technical decisions. Update `@docs/setup.md` when new manual setup or run steps are required.

## Change Control

- Mark individual phase files (e.g. `docs/phases/phase-1.md`) as tasks progress; set "Phase N Complete" when all tasks are done.
- Update this plan when scope changes or additional phases are approved.
- Record architectural changes in `@docs/architecture.md` (relevant section + Change Log) before or immediately after implementation.
