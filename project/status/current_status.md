---
id: STATUS-CURRENT
title: Current Project Status
scope: project
status: active
health: yellow
---

# Current Status

## Summary
- Replication Vector is entering its first rendering-spike stage.
- The repository now has an LRH control-plane scaffold, Velumin-style guidance, canonical validation scripts, CI, and a minimal Rust/WASM/Vite skeleton.
- The first project-owned scene is now expressed as Velumin `VectorCommand` data and validated by focused Rust tests.
- The game concept is well described by the supplied design summary and lightly corroborated by `README.md`.

## Evidence Basis
- `README.md` identifies Replication Vector as a retro space combat game with the phrase "Mine. Build. Replicate."
- `EV-0001` records repository path, file inventory, absence of pre-existing `project/`, and supplied design background.
- `EV-0002` records the Velumin-style repository infrastructure adaptation.
- `EV-0003` records the focus realignment from bootstrap to rendering spike.
- `EV-0004` records the first project-owned Velumin command scene and the downstream browser-rendering API gap.
- `DP-0001` records the adopted decision to use a scripted Velumin checkout and minimal Rust/WASM/Vite skeleton.

## Current Health
- Yellow: strong concept/design direction and repository infrastructure exist, but gameplay and rendering behavior are not yet implemented.

## Active Priorities
- Keep validation aligned with the Velumin-compatible script contract.
- Resolve the Velumin downstream-rendering API gap before expanding rendering or gameplay scope.

## Risks
- Scope creep into campaign, tech tree, and content systems before the core loop is proven.
- Passive player feel if the heavy probe lacks frequent tactical actions.
- Visual clutter from vector shields, asteroids, beams, particles, projectiles, enemies, and UI.
- Velumin checkout and npm setup can fail in fresh environments until `scripts/develop` succeeds.
- The current Replication Vector scene proves Velumin command-data construction, but direct browser rendering of downstream-owned scene commands is not exposed by the current Velumin API.

## Recommended Next Actions
1. Add or request an upstream Velumin API that lets downstream consumers submit `VectorCommand` scenes to the browser renderer.
2. Once that API exists, wire the existing Replication Vector scene into the Velumin browser renderer without introducing an alternate rendering stack.
3. Keep broader gameplay systems blocked until the rendering path can show project-owned scene data honestly.
