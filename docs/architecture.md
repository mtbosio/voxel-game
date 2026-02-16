# Technical Architecture

This document is the **single source of truth for the tech stack and core technical decisions**. Agents must not introduce new frameworks, languages, or major libraries to solve one-off problems—propose changes here first and update this doc (and Change Log) when the architecture changes.

**Product context:** See `@docs/REQUIREMENTS.md` for what we are building (Minecraft-like voxel survival, first-person, multiplayer).

---

## 1. Core Stack

| Layer | Choice | Notes |
|-------|--------|--------|
| **Language** | **Rust** | No other languages for game logic or engine code. |
| **Game engine** | **Bevy** | ECS, rendering, input, app lifecycle. Pin to a specific Bevy version in `Cargo.toml`; upgrades are architectural changes. |

All game systems (world, entities, UI, networking) run on this stack. Do not introduce another engine or language for “just one feature.”

---

## 2. Voxel World & Rendering

- **Voxel/chunk representation and meshing:** **bevy_voxel_world**. It provides procedural + persistent voxels, chunk spawning/despawning, multithreaded meshing, and texture mapping (uses `block-mesh` under the hood), which fits infinite procedural + editable worlds. Do not add another voxel/mesh crate without documenting it here and updating the Change Log.
- **Rendering** is Bevy’s default 3D pipeline (no replacement renderer unless we explicitly change this section).

---

## 3. Pathfinding (A* for Enemies & NPCs)

- **pathfinding**. Use the **pathfinding** crate for grid/voxel-world A* (and related algorithms); it works with 3D grids, has minimal dependencies, and no Bevy coupling. Map chunk/collision data into its graph. Do not introduce another pathfinding library without updating this section and the Change Log.

---

## 4. Multiplayer: Abstraction & Approved Options

Multiplayer must be implemented behind an **abstraction** so we can swap transports or backends without rewriting game logic.

### 4.1 Abstraction

- **Network / multiplayer layer** is abstracted behind a small set of concepts, for example:
  - **Connection lifecycle:** connect, disconnect, listen (server), get connection events.
  - **Channels:** send/receive serialized messages (reliable and/or unreliable) per channel or message type.
  - **Identity:** stable peer/session IDs for players; no transport-specific types in core game code.
- Game systems (movement, block edits, combat, inventory) depend only on this abstraction (e.g. a trait or interface), not on a specific crate or protocol.
- **Serialization** of game messages uses the stack’s chosen format (see §5); the transport layer only deals in bytes or a single serialization type.

### 4.2 Implementation

- **Lightyear** (`bevy_lightyear` / `lightyear_*`). Use Lightyear to implement the multiplayer abstraction: UDP and WebTransport (QUIC) for low-latency server–client, with optional netcode.io and Steam. Game code talks only to the abstract multiplayer layer, not directly to Lightyear. Do not add a different networking library without an architecture update (and Change Log).

### 4.3 Server Model

- **Dedicated server** is the target: one authoritative server, multiple clients. No reliance on “host = server” in the architecture unless we explicitly document it.
- Matchmaking, auth, and persistence are out of scope for this section; when we add them, they will sit behind the same abstraction or a separate, documented interface.

---

## 5. Serialization & Data

- **serde** with **bincode** for all game and network data: multiplayer messages, chunk/save format, and any cross-cutting serialization. Do not introduce another format or framework (e.g. Protobuf, JSON for persistence) for this scope without updating this section.
- **Chunk / world save format** is defined in-repo using the same serialization (bincode/serde). JSON may be used only for dev tooling or debug dumps if documented in setup; it is not the primary format.

---

## 6. Allowed Supporting Crates (Non-Exhaustive)

- **Math:** `glam` (often pulled in by Bevy).
- **Random:** `rand` for procedural generation; use one RNG approach per subsystem to avoid fragmentation.
- **Noise:** **noise** (the `noise` crate, e.g. noise-rs) for terrain and biome generation. Do not add another noise crate without updating this section.
- **Logging / diagnostics:** Bevy’s `tracing`; no second logging framework.

When in doubt, prefer a crate already in the dependency tree or listed in this document. New “supporting” crates that become part of the long-term stack should be added to this section and the Change Log.

---

## 7. What Agents Must Not Do

- Introduce a **new language** or **game engine** for any part of the game.
- Add a **new networking or transport library** without updating §4 and the Change Log.
- Add a **new voxel/mesh or pathfinding library** without updating §2 / §3 and the Change Log.
- Introduce a **second serialization format or framework** for game/network data without updating §5.
- Bypass the **multiplayer abstraction** (e.g. calling a networking crate directly from game logic).

One-off scripts or dev-only tooling (e.g. asset pipelines, build scripts) may use other tools if they are not part of the runtime game stack and are documented in setup or CI.

---

## 8. Change Log

| Date | Change |
|------|--------|
| (initial) | Rust + Bevy; voxel: bevy_voxel_world; pathfinding: pathfinding; multiplayer: Lightyear behind abstraction; serde + bincode; noise: noise crate; no ad-hoc stack additions. |

---

*When you change the stack or approved options, update the relevant section and add an entry to §8 Change Log.*
