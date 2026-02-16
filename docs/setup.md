# Setup

Chronological setup instructions for building and running the voxel game locally.

## 1. Install Rust toolchain

Install the Rust toolchain (rustup, rustc, cargo) if not already installed:

- **Download and install:** [https://rustup.rs](https://rustup.rs) (or run `winget install Rustlang.Rustup` on Windows).
- **Verify:** Open a new terminal and run:

  ```bash
  rustc --version
  cargo --version
  ```

Use the version recommended by the installer; the project builds with the current **stable** toolchain. The project uses Rust edition 2021 (Rust 1.70 or later).

**Optional config:** To use a specific build target (e.g. for CI or a platform), set `target` under `[build]` in `.cargo/config.toml`. This is not required for local development.

## 2. Clone the repository

Clone the repo and go to the project root:

```bash
git clone <repository-url> voxel-game
cd voxel-game
```

Replace `<repository-url>` with the actual clone URL (e.g. `https://github.com/your-org/voxel-game.git`).

## 3. Build and run (exact commands)

From the repository root:

**Build (debug):**

```bash
cargo build
```

**Run:**

```bash
cargo run
```

A window opens with the Bevy app; close the window to exit.

**Build for release (optional):**

```bash
cargo build --release
```

Run the release binary:

```bash
cargo run --release
```

## 4. Verification

- **Build success:** `cargo build` completes with exit code 0 and no errors.
- **Window launches:** Run `cargo run`; a Bevy window opens. Closing the window exits the process cleanly. If the window appears and closes without errors, setup is verified.
