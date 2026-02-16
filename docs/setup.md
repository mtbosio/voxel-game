# Setup

Chronological setup instructions for building and running the voxel game locally.

## Build and run (exact commands)

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

## Verification

- **Build success:** `cargo build` completes with exit code 0 and no errors.
- **Run success:** `cargo run` launches a window; closing the window exits the process cleanly.
