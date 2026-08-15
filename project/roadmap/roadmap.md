---
id: ROADMAP-INITIAL
title: Initial Project Roadmap
status: draft
owner: project maintainers
---

# Roadmap

## Phase 0: Project Bootstrap (Complete)
- Add LRH `project/` scaffold.
- Preserve repository intent and record unknowns.
- Establish initial goal, design, focus, guardrails, evidence, status, and work item artifacts.

## Phase 1: Rendering Spike (Initial Scene Complete)
- Add a minimal project/package skeleton.
- Establish Velumin as an external dependency.
- Render parent probe, asteroid outlines, shield arcs, and bullets for the first scene.
- Add render smoke tests.

## Phase 2: Core Simulation (Active)
- Implement slow inertial movement and rotation.
- Implement mining beam and one primary matter resource.
- Implement shield construction, damage, and repair.
- Implement basic collision and parent integrity.

## Phase 3: Child-Probe Objective
- Add visible child-probe construction.
- Define launch thresholds and launch readiness.
- Implement launch sequence and next-sector transition or scoring.

## Phase 4: Enemy Pressure
- Add MVP enemy types: skirmisher, cutter/breacher, and seeker/hunter.
- Add enemy spawning, escalation, and simple AI target priorities.
- Tune pressure around parent, shield, and child-probe threats.

## Phase 5: Playable MVP
- Integrate the mine, defend, replicate, and launch loop.
- Add basic UI, game-over state, and launch-success state.
- Add deterministic simulation tests where practical.
- Tune resource costs and attack pressure through playtesting.

## Phase 6: Post-MVP Expansion (Non-binding)
- Add inheritance/upgrades, sector variety, additional enemies, audio, visual effects, and packaging targets.

## Notes
- Phases are seeded from the supplied design summary and are not implementation commitments until converted into focused work items.
- `DP-0001` established the Rust/WASM/Vite package shape, CI, scripts, and scripted Velumin checkout.
- `FOCUS-RENDER-0001` completed the first Velumin-rendered scene and opt-in render smoke artifact path.
- `FOCUS-SIM-0001` and `WI-SIM-0001` completed the first deterministic parent-probe simulation slice.
- `FOCUS-REPLAY-0001` and `WI-REPLAY-0001` completed the narrow depth-first replay evidence slice before broader gameplay expansion.
- `FOCUS-INTERACT-0001` and proposed `WI-INTERACT-0001` bind the next narrow best-effort interactivity slice: controllable parent-probe motion tuning before mining, shields, enemies, or child-probe behavior.
- Simple enemy silhouettes remain deferred until enemy pressure or a future explicit rendering work item is selected.
