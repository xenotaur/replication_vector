---
id: FOCUS-RENDER-0001
title: Initial Velumin rendering spike
status: active
priority: high
owner: project maintainers
started: 2026-08-03
---

# Current Focus

## Active Priority
- Prove the first Replication Vector scene can be expressed through Velumin-facing vector scene data.

## Why This Appears Current
- `STATUS-CURRENT` identifies the next priority as rendering a parent probe, asteroid outline, shield arcs, and simple projectile lines through Velumin.
- `DP-0001` adopted the Velumin-style Rust/WASM/Vite skeleton and scripted Velumin checkout.
- `project/context/repository_map.md` records that Replication Vector can build Velumin `VectorCommand` scenes in Rust, but the browser renderer is not yet wired to render project-owned scenes.

## Priorities
1. Define the smallest Velumin-facing scene data for the rendering spike.
2. Render or otherwise validate parent probe, asteroid, shield arc, and projectile line primitives through the current Velumin boundary.
3. Add focused render/build smoke validation without adding gameplay simulation.

## Non-Goals
- Do not implement mining, enemies, child-probe construction, progression, scoring, or input handling in this focus.
- Do not introduce an alternate rendering stack.
- Do not add release automation, dependency policy gates, or broad visual smoke infrastructure beyond what this spike needs.

## Exit Criteria
- A focused work item for the first rendering spike is active or resolved.
- The project has evidence showing whether Velumin can render project-owned scene data directly or which Velumin API gap blocks it.
- Canonical validation remains green.
