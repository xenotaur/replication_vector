---
id: STATUS-CURRENT
title: Current Project Status
scope: project
status: active
health: yellow
---

# Current Status

## Summary
- Replication Vector is moving from deterministic motion replay evidence into a narrow controllable parent-probe tuning sandbox.
- The repository now has an LRH control-plane scaffold, Velumin-style guidance, canonical validation scripts, CI, and a minimal Rust/WASM/Vite skeleton.
- The first project-owned scene is now expressed as Velumin `VectorCommand` data and validated by focused Rust tests.
- The updated Velumin API appears to provide a downstream browser frame path via `VectorFrame` and `WebGPU.renderFrame(frame)`.
- Replication Vector now renders the existing project-owned scene through Velumin's downstream `renderFrame` browser harness.
- A local opt-in render smoke command now saves inspectable PNG and JSON artifacts for the Velumin-rendered first scene.
- The first deterministic Rust parent-probe motion model now exists with focused tests.
- `FOCUS-REPLAY-0001` authorized deterministic parent-probe motion replay through Velumin with inspectable PNG/JSON capture artifacts and is now complete.
- `WI-REPLAY-0001` is resolved: the deterministic motion model now renders through the Velumin browser harness and saves local replay artifacts.
- `FOCUS-INTERACT-0001` now authorizes a best-effort controllable parent-probe tuning sandbox for keyboard thrust/turn input and weight/inertia/responsiveness sliders.
- The game concept is well described by the supplied design summary and lightly corroborated by `README.md`.

## Evidence Basis
- `README.md` identifies Replication Vector as a retro space combat game with the phrase "Mine. Build. Replicate."
- `EV-0001` records repository path, file inventory, absence of pre-existing `project/`, and supplied design background.
- `EV-0002` records the Velumin-style repository infrastructure adaptation.
- `EV-0003` records the focus realignment from bootstrap to rendering spike.
- `EV-0004` records the first project-owned Velumin command scene and the downstream browser-rendering API gap.
- `EV-0005` records the Replication Vector integration with Velumin's downstream frame API.
- `EV-0006` records the opt-in render smoke artifact command and successful local capture.
- `EV-0007` records the deterministic parent-probe motion model and focused test coverage.
- `EV-0008` records the deterministic parent-probe replay artifact path.
- `DP-0001` records the adopted decision to use a scripted Velumin checkout and minimal Rust/WASM/Vite skeleton.
- `DP-0003` records the adopted deterministic parent-probe motion model.

## Current Health
- Yellow: strong concept/design direction, repository infrastructure, the first Velumin-rendered scene, an opt-in local render artifact path, the first deterministic motion model, and a replay artifact path exist, but no playable input, mining, shield, enemy, child-probe, or launch loop exists yet.

## Active Priorities
- Keep validation aligned with the Velumin-compatible script contract.
- Execute `WI-INTERACT-0001` before expanding into mining, shields, enemies, or child-probe behavior.
- Keep the tuning sandbox narrow, developer-facing, and sourced from the Rust simulation model where practical.

## Risks
- Scope creep into campaign, tech tree, and content systems before the core loop is proven.
- Passive player feel if the heavy probe lacks frequent tactical actions.
- Visual clutter from vector shields, asteroids, beams, particles, projectiles, enemies, and UI.
- Velumin checkout and npm setup can fail in fresh environments until `scripts/develop` succeeds.
- The tuning sandbox could drift into production UI or gameplay systems if not kept explicitly scoped.

## Recommended Next Actions
1. Execute `WI-INTERACT-0001` as the next narrow best-effort slice.
2. Keep mining, asteroid collision, shields, enemies, child-probe behavior, scoring, and progression blocked until future explicit work items authorize them.
3. After motion feel is inspectable under live control, decide whether to proceed breadth-first into mining design or depth-first into capture/evidence for the interactive sandbox.
