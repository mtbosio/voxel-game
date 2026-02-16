# Phase 1: Project Foundation

**Status:** 🔄 Not Started

**Objective:** Establish the Rust + Bevy project: Cargo workspace, runnable window, project structure, dependencies per architecture, and developer setup documentation so the game builds and runs locally.

**Completion Tracking:**
- [ ] Phase 1 Complete

## Task Checklist

### Project Setup
- [x] Section Complete
- [x] T001 Create Cargo workspace (e.g. `voxel-game`) with binary target; pin Bevy version in `Cargo.toml` per `@docs/architecture.md`.
- [x] T002 Add dependencies: `bevy`, `serde`, `bincode`, `glam`, `rand`, `noise`, `tracing` (or use Bevy’s re-exports where applicable); ensure no extra voxel/networking crates yet.
- [x] T003 Create minimal `main.rs` that runs a Bevy app with default plugin and a window; verify window opens and closes.
- [x] T004 Define project layout: e.g. `src/` modules for future systems (world, player, ui, net) without implementing them; keep `main.rs` as entry point.

### Configuration & Tooling
- [x] Section Complete
- [x] T005 Configure `Cargo.toml` (edition, release profile, optional lints) and add `.cargo/config.toml` if needed (e.g. target, build flags).
- [x] T006 Add `.gitignore` for `target/`, `Cargo.lock` (if not desired in repo), and any local env or IDE files.
- [x] T007 Run `cargo build` and `cargo run` successfully; document exact commands.

### Setup Documentation
- [ ] Section Complete
- [x] T008 Create `@docs/setup.md` with: clone repo, install Rust toolchain, `cargo build` and `cargo run`, and how to verify the window launches.
- [ ] T009 Add any environment or config steps (e.g. Rust version) to `docs/setup.md` in chronological order.

### Verification
- [ ] Section Complete
- [ ] T010 Run `cargo clippy` (or equivalent) and fix any warnings; ensure no build errors.
- [ ] T011 Confirm runnable window and clean exit; mark Phase 1 complete when all tasks are done.

## Lessons Learned

_(To be filled in as phase progresses)_
