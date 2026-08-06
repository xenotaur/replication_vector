---
id: STATUS-CURRENT
title: Current Project Status
scope: project
status: active
health: yellow
---

# Current Status

## Summary
- Replication Vector is moving from its first rendering-spike stage into the first core simulation slice.
- The repository now has an LRH control-plane scaffold, Velumin-style guidance, canonical validation scripts, CI, and a minimal Rust/WASM/Vite skeleton.
- The first project-owned scene is now expressed as Velumin `VectorCommand` data and validated by focused Rust tests.
- The updated Velumin API appears to provide a downstream browser frame path via `VectorFrame` and `WebGPU.renderFrame(frame)`.
- Replication Vector now renders the existing project-owned scene through Velumin's downstream `renderFrame` browser harness.
- A local opt-in render smoke command now saves inspectable PNG and JSON artifacts for the Velumin-rendered first scene.
- `FOCUS-SIM-0001` now authorizes the narrow deterministic parent-probe movement and rotation slice.
- The first deterministic Rust parent-probe motion model now exists with focused tests.
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
- `DP-0001` records the adopted decision to use a scripted Velumin checkout and minimal Rust/WASM/Vite skeleton.
- `DP-0003` proposes the first deterministic parent-probe motion model.

## Current Health
- Yellow: strong concept/design direction, repository infrastructure, the first Velumin-rendered scene, an opt-in local render artifact path, and the first deterministic motion model exist, but no playable input, mining, shield, enemy, child-probe, or launch loop exists yet.

## Active Priorities
- Keep validation aligned with the Velumin-compatible script contract.
- Keep the parent-probe motion model deterministic and covered by focused Rust tests.
- Select or propose the next narrow simulation work item before expanding into input or rendering integration.

## Risks
- Scope creep into campaign, tech tree, and content systems before the core loop is proven.
- Passive player feel if the heavy probe lacks frequent tactical actions.
- Visual clutter from vector shields, asteroids, beams, particles, projectiles, enemies, and UI.
- Velumin checkout and npm setup can fail in fresh environments until `scripts/develop` succeeds.
- Current browser verification is a focused opt-in artifact capture, not a recurring visual regression gate.

## Recommended Next Actions
1. Consider closing `FOCUS-SIM-0001` and selecting the next narrow simulation or integration focus.
2. Propose the next work item explicitly before adding input, rendering integration, mining, shields, enemies, or child-probe behavior.
3. Keep broader gameplay systems blocked until future explicit work items authorize them.
