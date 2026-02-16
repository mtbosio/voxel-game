# Phase 9: Health & Death

**Status:** 🔄 Not Started

**Objective:** Add player health, damage from mobs and environment (fall, drowning, etc.), death with respawn at world spawn or bed, and item drop on death so survival has stakes and recovery is possible.

**Completion Tracking:**
- [ ] Phase 9 Complete

## Task Checklist

### Health System
- [ ] Section Complete
- [ ] T101 Add a health resource or component for the player (e.g. current health, max health); display in UI (e.g. hearts or bar).
- [ ] T102 When health reaches zero, trigger death: disable movement, play death logic (e.g. respawn timer or immediate respawn), and optionally drop inventory.
- [ ] T103 Healing: eating food (Phase 7 inventory items) restores health; optional natural regen when full hunger or over time (match Minecraft-like behavior as desired).

### Damage Sources
- [ ] Section Complete
- [ ] T104 Mob melee: when hostile mob “attacks” (Phase 8), apply damage to player; use damage amount and optional armor reduction if armor exists.
- [ ] T105 Fall damage: when player lands after a fall exceeding a height threshold, apply damage proportional to fall distance.
- [ ] T106 Optional: drowning (when submerged in water for too long), fire/lava, suffocation (inside block); add at least one additional source or document as future work.
- [ ] T107 Invulnerability frames: brief period after taking damage where no additional damage is applied to avoid one-shot chains.

### Death & Respawn
- [ ] Section Complete
- [ ] T108 On death: record death position; drop inventory (or a subset) as pickable items at death location; clear or keep hotbar per design.
- [ ] T109 Define world spawn point (e.g. first spawn or set by bed); respawn player at spawn with full health and empty or default inventory.
- [ ] T110 Respawn flow: death screen or message, then respawn after delay or on key press; ensure player spawns in a safe position (not inside block or in void).
- [ ] T111 Bed as respawn point: when player uses a bed (place and interact), set respawn position to bed; if bed is missing or obstructed on death, use world spawn.

### Dropped Items
- [ ] Section Complete
- [ ] T112 Dropped items as entities: 3D position, item type and count, pickup radius; when player overlaps, add to inventory and remove entity.
- [ ] T113 Optional: despawn dropped items after a timeout (e.g. 5 minutes) to avoid clutter; mark Phase 9 complete when all tasks are done.

### Verification
- [ ] Section Complete
- [ ] T114 In-game verification: take damage from mob and fall, die, respawn, and recover dropped items; confirm health UI and death flow; mark Phase 9 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
