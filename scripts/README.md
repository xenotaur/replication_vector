# Validation Scripts

Repository-owned validation scripts are the canonical command contract for local development, CI, and agent workflows.

Run scripts from the repository root.

## Quick Reference

| Command | Purpose |
| --- | --- |
| `scripts/develop` | Setup/bootstrap local dependencies, Velumin checkout, and CI tools. |
| `scripts/version tools` | Print validation tool versions and Velumin checkout commit. |
| `scripts/format` | Format Rust code with rustfmt. |
| `scripts/format --check --diff` | Check Rust formatting without rewriting files. |
| `scripts/lint` | Run Clippy for the WASM target with warnings denied. |
| `scripts/test` | Run Rust tests. |
| `scripts/baseline` | Rebuild the Rust/WASM/Vite browser baseline. |
| `scripts/render-smoke` | Build the browser baseline, capture the Velumin-rendered scene, and save local inspection artifacts. |
| `scripts/render-replay-smoke` | Build the browser baseline, capture the deterministic parent-probe replay, and save local inspection artifacts. |
| `scripts/render-sandbox-smoke` | Build the browser baseline, drive the controllable parent-probe sandbox, and save local inspection artifacts. |
| `scripts/validate` | Run the full canonical validation sequence. |

## Recommended Workflow

For ordinary code review and edit cycles:

```sh
scripts/version tools
scripts/format --check --diff
scripts/lint
scripts/test
```

For full local validation:

```sh
scripts/validate
```

For setup/bootstrap:

```sh
scripts/develop
```

Do not run `scripts/develop` routinely during ordinary validation. It is for setup, CI bootstrap, or setup debugging.

## Script Details

### `scripts/develop`

Ensures the development environment has:

- Rust `rustfmt` and Clippy components;
- the `wasm32-unknown-unknown` target;
- Velumin checked out at `.deps/velumin`;
- npm dependencies from `replication_vector/web/package.json`;
- pinned `wasm-pack` version `0.13.1`.

### `scripts/version`

Prints versions for Rust, Cargo, rustfmt, Clippy, wasm-pack, Node, npm, Vite, and the Velumin checkout.

### `scripts/baseline`

Rebuilds the browser baseline:

```sh
cargo check --manifest-path replication_vector/Cargo.toml --target wasm32-unknown-unknown
wasm-pack build replication_vector --target web
npm run build --prefix replication_vector/web
```

Run this when Rust/WASM/Vite build behavior, browser harness behavior, or Velumin integration may be affected.

### `scripts/render-smoke`

Builds the browser baseline, starts the Vite harness, waits for the Velumin-rendered first scene status, and saves:

```text
replication_vector/web/smoke-out/first-scene.png
replication_vector/web/smoke-out/first-scene.json
```

This command is opt-in and is not part of `scripts/validate`. It requires a WebGPU-capable Chromium through Playwright; if Chromium is missing, run `npx playwright install chromium` from `replication_vector/web`. If the browser environment lacks WebGPU support, the command prints a clear `SKIP` message rather than writing a misleading capture. Generated `smoke-out/` artifacts are ignored by git.

### `scripts/render-replay-smoke`

Builds the browser baseline, starts the Vite harness with `?scene=replay`, waits for the Velumin-rendered deterministic parent-probe replay status, and saves:

```text
replication_vector/web/smoke-out/replay.png
replication_vector/web/smoke-out/replay.json
```

The replay uses the Rust `step_parent_probe_motion(...)` model through a fixed scripted sequence and writes JSON metadata next to the PNG, including the captured replay step, timestep, final parent-probe state, command count, viewport, Velumin checkout, and artifact path. Like `scripts/render-smoke`, this command is opt-in, writes only ignored local artifacts, and reports a clear `SKIP` when Playwright Chromium or WebGPU support is unavailable.

### Parent-probe tuning sandbox

Run the browser harness and open the opt-in sandbox route:

```sh
scripts/baseline
npm run dev --prefix replication_vector/web -- --host 127.0.0.1
```

Then visit:

```text
http://127.0.0.1:5173/?scene=sandbox
```

The sandbox keeps parent-probe motion authoritative in Rust. The browser sends normalized keyboard thrust and turn input plus slider values into the exported Rust stepping helper, then renders the returned parent pose through Velumin `renderFrame`.

- `W` or `ArrowUp`: forward thrust.
- `A` or `ArrowLeft`: turn left.
- `D` or `ArrowRight`: turn right.
- `Weight`: maps to Rust thrust acceleration and max speed; higher values feel heavier and slower.
- `Inertia`: maps to Rust linear and angular drag; higher values drift longer.
- `Response`: maps to Rust turn acceleration and max angular speed; higher values turn more readily.

### `scripts/render-sandbox-smoke`

Builds the browser baseline, starts the Vite harness with `?scene=sandbox`, adjusts the tuning sliders, drives keyboard thrust/turn input, waits for the Rust-stepped parent probe to move and turn, and saves:

```text
replication_vector/web/smoke-out/sandbox.png
replication_vector/web/smoke-out/sandbox.json
```

The JSON metadata includes the latest sandbox state, keyboard input, slider values, command count, viewport, Velumin checkout, and artifact path. Like the other render smoke commands, this is opt-in, writes only ignored local artifacts, and reports a clear `SKIP` when Playwright Chromium or WebGPU support is unavailable.
