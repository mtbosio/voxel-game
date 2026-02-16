# Phase 8: Mobs & Pathfinding

**Status:** 🔄 Not Started

**Objective:** Add passive and hostile mobs that spawn in the world, use the pathfinding crate for A* movement on the voxel grid, and exhibit basic AI (wander, chase player) so the world feels alive and dangerous at night.

**Completion Tracking:**
- [ ] Phase 8 Complete

## Task Checklist

### Entity Foundation
- [ ] Section Complete
- [ ] T087 Define a mob component or entity type (e.g. passive vs hostile) and spawn positions; use Bevy ECS for mob entities.
- [ ] T088 Render mobs with a simple mesh (e.g. cube or capsule) and material; position and scale so they are visible and distinct from blocks.
- [ ] T089 Implement mob spawn rules: e.g. passive (cow, pig) in daylight in certain biomes; hostile (zombie, skeleton) at night or in dark; spawn on valid surface blocks within a range of the player.
- [ ] T090 Cap total mob count or per-chunk count to avoid performance issues; despawn or do not spawn when far from all players.

### Pathfinding Integration
- [ ] Section Complete
- [ ] T091 Build a voxel-to-graph adapter: from chunk/world data produce a graph (e.g. walkable neighbors per 3D position) that the pathfinding crate accepts. Use only walkable surfaces (e.g. top of blocks, one block step up/down).
- [ ] T092 Use the pathfinding crate’s A* to compute a path from mob position to target (e.g. player position or wander point); handle “no path” (e.g. wander in place or wait).
- [ ] T093 Cache or throttle pathfinding (e.g. recompute every N seconds or when target moves significantly) to avoid per-frame A* for many mobs.
- [ ] T094 Move mob along the path: advance position each frame, snap to voxel grid if desired; rotate mob to face movement direction.

### AI Behavior
- [ ] Section Complete
- [ ] T095 Passive mobs: wander (random direction for a few seconds) or idle; flee when hit (optional).
- [ ] T096 Hostile mobs: when player in range and line-of-sight (optional), set target to player and pathfind toward them; when in melee range, attack (trigger damage in Phase 9).
- [ ] T097 Optional: simple attack cooldown and range so mobs do not stack infinitely on the player.

### Collision & Physics
- [ ] Section Complete
- [ ] T098 Mobs collide with voxel world (do not fall through blocks); use same or similar collision as player (AABB/capsule vs blocks).
- [ ] T099 Mobs do not push the player (or use simple push-back); player can walk through passive mobs or block movement as designed.

### Verification
- [ ] Section Complete
- [ ] T100 In-game verification: mobs spawn, path toward player (hostile) or wander (passive), and navigate terrain; no falling through world or getting stuck; mark Phase 8 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
