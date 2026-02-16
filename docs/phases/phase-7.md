# Phase 7: Crafting System

**Status:** 🔄 Not Started

**Objective:** Implement a crafting system: inventory UI, crafting grid (e.g. 2x2 or 3x3), recipe definitions (serde/bincode), and production of tools, weapons, and blocks from raw materials so the player can progress via crafting.

**Completion Tracking:**
- [ ] Phase 7 Complete

## Task Checklist

### Recipe Definition
- [ ] Section Complete
- [ ] T074 Define a recipe data structure: input pattern (grid of item/block IDs) and output (item ID and count); use serde for serialization per architecture.
- [ ] T075 Load recipes from data (e.g. embedded or file) in bincode or a format agreed in architecture; support at least 2x2 and 3x3 patterns.
- [ ] T076 Implement recipe matching: given current crafting grid contents, find a recipe that matches (exact or with wildcards) and return output.
- [ ] T077 Add initial recipes for Minecraft-like basics: e.g. planks from wood, sticks from planks, crafting table, wooden pickaxe, stone tools (cobblestone), furnace if in scope.

### Inventory System
- [ ] Section Complete
- [ ] T078 Extend inventory to support both blocks and items (e.g. tools, materials); each slot has item type and count; define item type enum or ID table.
- [ ] T079 Implement stack splitting and merging (e.g. shift-click, half stack); ensure max stack size per item type.
- [ ] T080 Add a dedicated “crafting grid” state (e.g. 3x3) and “crafting result” slot; moving items from result slot consumes materials in the grid per recipe.

### Crafting UI
- [ ] Section Complete
- [ ] T081 Build an inventory screen UI: grid of inventory slots and crafting grid; show item icons or placeholders and counts.
- [ ] T082 When the crafting grid changes, run recipe matching and show result in the result slot; on take from result slot, deduct ingredients and give output.
- [ ] T083 Open/close inventory with a key (e.g. E); lock movement or camera when inventory is open if desired; ensure hotbar remains usable when inventory is closed.

### Tools & Durability (Optional)
- [ ] Section Complete
- [ ] T084 Define tool types (e.g. pickaxe, axe, shovel) and material tiers (wood, stone, iron); tools can have durability that decreases on use (break block, attack).
- [ ] T085 When breaking blocks, check for correct tool and apply speed modifier or requirement per block type (e.g. stone requires pickaxe); optional for MVP.

### Verification
- [ ] Section Complete
- [ ] T086 In-game verification: gather materials, open inventory, craft items, and use crafted tools/blocks; mark Phase 7 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
