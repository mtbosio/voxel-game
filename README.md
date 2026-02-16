# Voxel Game

A first-person voxel survival game built in **Rust** with **Bevy**. The design follows a Minecraft-like baseline: procedural world, biomes, break/place blocks, day/night cycle, crafting, mobs with A* pathfinding, health and death, and multiplayer.

## Prerequisites

- **Rust** (latest stable). Install from [rustup.rs](https://rustup.rs).

## Quick start

```bash
# Clone the repository
git clone https://github.com/mtbosio/voxel-game.git
cd voxel-game

# Build
cargo build

# Run
cargo run
```

A window opens with the game; close it to exit. For full setup and verification steps, see [docs/setup.md](docs/setup.md).

## Tech stack

| Area        | Choice              |
|------------|---------------------|
| Language   | Rust                |
| Game engine| Bevy 0.18           |
| Voxel world| bevy_voxel_world   |
| Pathfinding| pathfinding crate   |
| Multiplayer| Lightyear (abstracted) |
| Serialization | serde + bincode  |

See [docs/architecture.md](docs/architecture.md) for the full technical architecture and constraints.

## Documentation

- **[docs/requirements.md](docs/requirements.md)** — Product intent and feature scope
- **[docs/architecture.md](docs/architecture.md)** — Tech stack and technical decisions
- **[docs/setup.md](docs/setup.md)** — Build, run, and verification
- **[docs/phases/phase-plan.md](docs/phases/phase-plan.md)** — Development phase plan

## License

See repository settings or a `LICENSE` file in the repo for license terms.
