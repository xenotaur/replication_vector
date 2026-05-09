---
id: DP-0001
title: Adopt Velumin-style repository infrastructure
status: adopted
date: 2026-05-09
---

# DP-0001: Adopt Velumin-Style Repository Infrastructure

## Decision
- Adopt Velumin-style repository guidance, canonical scripts, CI, and a minimal Rust/WASM/Vite skeleton for Replication Vector.
- Consume Velumin as the intended rendering package/library through a scripted checkout at `.deps/velumin`.
- Use a Cargo path dependency to `.deps/velumin/webgpu_vector_lib`.

## Rationale
- Replication Vector is intended to be a Velumin ecosystem game and downstream dogfood project.
- Velumin already has useful conventions for agent guidance, style, review repair, validation scripts, and CI.
- The current Replication Vector repository needs enough technical skeleton for those validation entrypoints to pass before gameplay work begins.

## Dependency Decision
- Use `scripts/develop` to clone or update `https://github.com/xenotaur/Velumin.git` into `.deps/velumin`.
- Use a path dependency from `replication_vector/Cargo.toml` to `.deps/velumin/webgpu_vector_lib`.
- This avoids a direct Cargo git dependency because Velumin currently does not expose a root Cargo manifest suitable for direct consumption.

## Non-Goals
- Do not implement the gameplay loop.
- Do not implement the first rendering spike.
- Do not add release automation, Dependabot, cargo-deny, dependency review, or visual browser smoke gates.

## Consequences
- Local setup and CI must run `scripts/develop` before validation when `.deps/velumin` is missing.
- Canonical validation is now script-based and aligned with the Velumin ecosystem.
- Future rendering and gameplay work can assume a Rust/WASM/Vite project shape and a Velumin consumer boundary.
