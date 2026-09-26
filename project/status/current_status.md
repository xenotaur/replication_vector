---
id: STATUS-CURRENT
title: Current Project Status
scope: project
status: active
health: yellow
---

# Current Status

## Summary
- Replication Vector now has a narrow controllable parent-probe tuning sandbox landed and closed out.
- The repository now has an LRH control-plane scaffold, Velumin-style guidance, canonical validation scripts, CI, and a minimal Rust/WASM/Vite skeleton.
- The first project-owned scene is now expressed as Velumin `VectorCommand` data and validated by focused Rust tests.
- The updated Velumin API appears to provide a downstream browser frame path via `VectorFrame` and `WebGPU.renderFrame(frame)`.
- Replication Vector now renders the existing project-owned scene through Velumin's downstream `renderFrame` browser harness.
- A local opt-in render smoke command now saves inspectable PNG and JSON artifacts for the Velumin-rendered first scene.
- The first deterministic Rust parent-probe motion model now exists with focused tests.
- `FOCUS-REPLAY-0001` authorized deterministic parent-probe motion replay through Velumin with inspectable PNG/JSON capture artifacts and is now complete.
- `WI-REPLAY-0001` is resolved: the deterministic motion model now renders through the Velumin browser harness and saves local replay artifacts.
- `FOCUS-INTERACT-0001` is complete: `WI-INTERACT-0001` resolved the sandbox route that drives Rust parent-probe motion with keyboard input and tuning sliders, renders through Velumin, and saves opt-in sandbox smoke artifacts.
- `WS-INTERACT-0001` is closed with `EV-0009` as its completion evidence.
- `FOCUS-MINING-0001` now authorizes the next narrow Phase 2 slice: one primary matter resource and a deterministic mining interaction.
- `WI-MINING-0001` is resolved and landed in PR #21 (merge commit `f43a55d`): its first Rust-owned matter resource and deterministic mining boundary are implemented and evidenced.
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
- `EV-0009` records the controllable parent-probe tuning sandbox and browser/WebGPU smoke evidence.
- `EV-0010` records the first deterministic matter-resource mining boundary and its validation/browser evidence.
- `DP-0001` records the adopted decision to use a scripted Velumin checkout and minimal Rust/WASM/Vite skeleton.
- `DP-0003` records the adopted deterministic parent-probe motion model.

## Current Health
- Yellow: the first Rust-owned matter-resource boundary is landed with focused tests, while no mining browser loop, shields, enemies, child-probe, or launch loop exists yet.

## Active Priorities
- Keep validation aligned with the Velumin-compatible script contract.
- Preserve the landed matter boundary and select the next explicit Phase 2 work item before expanding into shields, enemies, child-probe behavior, scoring, or progression.
- Keep the first mining/resource slice narrow, deterministic, and sourced from the Rust simulation model where practical.

## Risks
- Scope creep into campaign, tech tree, and content systems before the core loop is proven.
- Passive player feel if the heavy probe lacks frequent tactical actions.
- Visual clutter from vector shields, asteroids, beams, particles, projectiles, enemies, and UI.
- Velumin checkout and npm setup can fail in fresh environments until `scripts/develop` succeeds.
- The mining slice could drift into production economy, collision, or UI if not kept explicitly scoped.

## Recommended Next Actions
1. Select or refine the next explicit Phase 2 work item; no new focus is selected by this closeout.
2. Keep the matter boundary deterministic and simulation-owned if follow-up browser affordances are proposed.
3. Keep asteroid collision, shields, enemies, child-probe behavior, scoring, and progression blocked until future explicit work items authorize them.
