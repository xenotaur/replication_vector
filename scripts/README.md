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
