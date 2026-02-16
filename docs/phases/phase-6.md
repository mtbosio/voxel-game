# Phase 6: Day/Night Cycle

**Status:** 🔄 Not Started

**Objective:** Add a game time system that advances over real time, and change sky and ambient lighting (and optionally block light) so day and night are visually and gameplay-distinct.

**Completion Tracking:**
- [ ] Phase 6 Complete

## Task Checklist

### Game Time
- [ ] Section Complete
- [ ] T065 Define a game time resource (e.g. time of day 0–24 or 0–1) that increments each frame or at a fixed rate (e.g. 1 game minute per real second).
- [ ] T066 Persist or optionally persist game time with the world so it continues across sessions; if not persisted, start at noon each load.
- [ ] T067 Expose time for other systems (e.g. mob spawns in Phase 8); consider “day” vs “night” thresholds (e.g. night when time in [22, 6]).

### Sky & Atmosphere
- [ ] Section Complete
- [ ] T068 Configure Bevy’s 3D environment: directional light (sun) and ambient light; optionally skybox or clear color.
- [ ] T069 Drive sun direction and possibly color from game time (e.g. sun rises east, sets west; lower angle and warmer color at dawn/dusk).
- [ ] T070 Adjust ambient and directional intensity (and color if desired) so night is visibly darker; day is bright and readable.

### Visibility & Gameplay Hooks
- [ ] Section Complete
- [ ] T071 Ensure the player can see terrain and UI in both day and night; add optional simple player-held or block light in a later phase if needed.
- [ ] T072 Document the time scale (e.g. minutes per real second) and day/night thresholds for use by mobs and future features; mark Phase 6 complete when all tasks are done.

### Verification
- [ ] Section Complete
- [ ] T073 In-game verification: observe day/night transition and confirm lighting changes; no crashes or performance regressions.

## Lessons Learned

_(To be filled in as phase progresses)_
