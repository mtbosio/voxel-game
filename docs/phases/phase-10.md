# Phase 10: Multiplayer

**Status:** 🔄 Not Started

**Objective:** Implement the multiplayer abstraction (connection lifecycle, channels, identity), integrate Lightyear as the transport, run a dedicated server and client, and sync world state and player positions/actions so multiple players can play together in the same world.

**Completion Tracking:**
- [ ] Phase 10 Complete

## Task Checklist

### Multiplayer Abstraction
- [ ] Section Complete
- [ ] T115 Define the multiplayer abstraction (e.g. Rust traits): connect, disconnect, listen (server), connection events, send/receive messages by channel or type, and stable peer/session IDs. See `@docs/architecture.md` §4.
- [ ] T116 Implement a minimal in-memory or loopback implementation for single-player so game code only talks to the abstraction; no direct Lightyear calls in game logic.
- [ ] T117 Define message types for world and player sync (e.g. block change, player position, player action, join/leave); serialize with serde + bincode.

### Lightyear Integration
- [ ] Section Complete
- [ ] T118 Add Lightyear (bevy_lightyear / lightyear_* crates) to the project; implement the multiplayer abstraction using Lightyear (UDP or WebTransport) for transport.
- [ ] T119 Server: run a Bevy app that starts a listen server, accepts client connections, and receives/sends messages via the abstraction.
- [ ] T120 Client: run a Bevy app that connects to a server address (e.g. config or CLI), sends/receives messages via the abstraction, and runs the same game systems as single-player where authoritative.
- [ ] T121 Ensure server and client can be built as separate binaries or modes (e.g. `--server` flag); document in `@docs/setup.md`.

### Authority & Sync Model
- [ ] Section Complete
- [ ] T122 Server is authoritative for world state: block breaks/places are validated and broadcast; clients apply updates from server.
- [ ] T123 Server is authoritative for player positions: accept client input (movement, look), run server-side movement/collision, and broadcast positions to all clients; clients interpolate or predict as needed.
- [ ] T124 Clients send input (WASD, jump, look, break/place) to server; server ticks at fixed rate and broadcasts state updates (e.g. block changes, entity positions).
- [ ] T125 Handle join: new client receives current world delta (modified chunks) and all connected players’ positions; existing clients receive new player join event and spawn remote player entity.

### World Sync
- [ ] Section Complete
- [ ] T126 When a block is placed or broken, server updates its world and broadcasts a block-change message; clients apply the change locally so all see the same world.
- [ ] T127 Chunk requests: if a client needs a chunk it does not have (e.g. far from spawn), server sends chunk data (or delta) on demand; ensure no duplicate or conflicting chunk ownership.
- [ ] T128 Persistence: server writes world saves (reuse Phase 5 format); on server start, load saved world so multiplayer sessions persist.

### Player Sync
- [ ] Section Complete
- [ ] T129 Spawn a remote player entity for each connected client (except local); update position and rotation from server messages; optional interpolation for smooth movement.
- [ ] T130 Local player: send position/input to server; render local player as in single-player; do not apply server position for local player (or use for reconciliation).
- [ ] T131 Handle disconnect: remove remote player entity, free slot; optional rejoin with same or new identity per design.

### Mobs & Entities in Multiplayer
- [ ] Section Complete
- [ ] T132 Mobs are server-authoritative: server runs mob AI and pathfinding, broadcasts mob positions and state; clients render and optionally predict nothing.
- [ ] T133 Combat: server validates mob attacks and player damage; broadcast health changes and death so all clients stay in sync.

### Verification & Documentation
- [ ] Section Complete
- [ ] T134 In-game verification: start server, start two clients, join same world, move and break/place blocks; confirm both see the same world and each other; no desync or crashes.
- [ ] T135 Update `@docs/setup.md` with how to run server and client, and any port or config; update `@docs/architecture.md` if the abstraction or Lightyear usage is refined; mark Phase 10 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
