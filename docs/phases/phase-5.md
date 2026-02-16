# Phase 5: Block Breaking & Placing

**Status:** 🔄 Not Started

**Objective:** Implement block targeting (raycast), breaking and placing blocks from player perspective, a simple hotbar/inventory, and persistence of block edits so the world can be modified and changes survive restarts.

**Completion Tracking:**
- [ ] Phase 5 Complete

## Task Checklist

### Block Targeting
- [ ] Section Complete
- [ ] T048 Implement a raycast from the camera: origin at eye, direction from screen center (forward vector), max range (e.g. 5 blocks).
- [ ] T049 Query the voxel world for the first solid block along the ray; return block position and face hit for placement.
- [ ] T050 Optionally render a highlight (e.g. outline or tint) on the targeted block; clear when no target.

### Breaking Blocks
- [ ] Section Complete
- [ ] T051 On input (e.g. left click), if a block is targeted, request removal of that block from the voxel world (use bevy_voxel_world API or equivalent).
- [ ] T052 When a block is broken, add the corresponding block item to player inventory (or drop as entity if inventory not yet implemented); handle block type → item type mapping.
- [ ] T053 Optional: break progress over time (e.g. hold to break) and tool-based speed later; for this phase instant break is acceptable.

### Placing Blocks
- [ ] Section Complete
- [ ] T054 On input (e.g. right click), if a block is targeted, compute placement position (adjacent face, inside player bounds check).
- [ ] T055 If the selected hotbar slot has a placeable block and the position is valid, place that block in the world and consume one from the slot.
- [ ] T056 Prevent placement inside the player AABB; prevent placement in invalid positions (e.g. outside world bounds).

### Hotbar & Inventory
- [ ] Section Complete
- [ ] T057 Define a simple inventory: hotbar slots (e.g. 9) and optional main inventory; store block type ID and count per slot.
- [ ] T058 Render hotbar UI (e.g. at bottom of screen): current slot highlight and block icon or name per slot; scroll or number keys to select slot.
- [ ] T059 When picking up a block item (from break or later from ground), add to current slot or first empty slot; stack up to a max stack size (e.g. 64).

### Persistence
- [ ] Section Complete
- [ ] T060 Define chunk or region save format using serde + bincode per `@docs/architecture.md`; store only modified blocks (delta from procedural) or full chunk data as chosen.
- [ ] T061 On world unload or chunk unload, write modified chunk data to disk (e.g. per-chunk files or a region file); ensure thread-safe or main-thread write as appropriate.
- [ ] T062 On chunk load, if persisted data exists for that chunk, apply it (override procedural terrain); otherwise generate procedurally.
- [ ] T063 Ensure a clean shutdown or save trigger (e.g. on quit) so no data loss; document save location and format in setup if needed.

### Verification
- [ ] Section Complete
- [ ] T064 In-game verification: break and place blocks, change hotbar selection, restart game and confirm edits persist; mark Phase 5 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
