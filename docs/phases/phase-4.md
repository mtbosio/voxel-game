# Phase 4: Player & First-Person

**Status:** 🔄 Not Started

**Objective:** Add a first-person player: controllable camera, movement (WASD), jump, gravity, and collision with the voxel world so the player can walk and look around the terrain.

**Completion Tracking:**
- [ ] Phase 4 Complete

## Task Checklist

### Player Entity & Camera
- [ ] Section Complete
- [x] T036 Spawn a player entity with a camera (first-person); position camera at eye height and attach to player transform.
- [x] T037 Implement mouse look: yaw (horizontal) and pitch (vertical) with sensitivity and pitch clamping to avoid flip.
- [ ] T038 Use Bevy’s input system for mouse motion and cursor capture (cursor locked/hidden during play).

### Movement
- [ ] Section Complete
- [ ] T039 Implement movement input (WASD) relative to camera yaw so forward/back/strafe match view direction.
- [ ] T040 Apply horizontal movement with configurable speed; apply gravity (negative Y) when not grounded.
- [ ] T041 Implement jump: single jump when grounded (e.g. space); optional sprint (e.g. shift) for faster movement.

### Collision With Voxel World
- [ ] Section Complete
- [ ] T042 Query the voxel world (e.g. bevy_voxel_world API) for solid blocks at player AABB or capsule; treat solid blocks as colliders.
- [ ] T043 Resolve collision: prevent penetration (e.g. push player out of blocks) and set “grounded” when standing on a solid surface so gravity and jump behave correctly.
- [ ] T044 Handle vertical collision (ceiling) so the player does not get stuck in blocks above.

### Physics & Tuning
- [ ] Section Complete
- [ ] T045 Tune movement speed, gravity strength, and jump height to feel responsive; document default values.
- [ ] T046 Ensure player spawns at a valid position (e.g. above terrain, not inside a block) on load or when world is ready.

### Verification
- [ ] Section Complete
- [ ] T047 In-game verification: move, look, jump, and walk on/around terrain with no falling through or getting stuck; mark Phase 4 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
