---
id: ROADMAP-INITIAL
title: Initial Project Roadmap
status: draft
owner: project maintainers
---

# Roadmap

## Phase 0: Project Bootstrap
- Add LRH `project/` scaffold.
- Preserve repository intent and record unknowns.
- Establish initial goal, design, focus, guardrails, evidence, status, and work item artifacts.

## Phase 1: Rendering Spike
- Add a minimal project/package skeleton.
- Establish Velumin as an external dependency.
- Render parent probe, asteroid outlines, shield arcs, bullets, and simple enemy silhouettes.
- Add render smoke tests.

## Phase 2: Core Simulation
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
- Current repository evidence does not yet show package layout, runtime platform, CI, or Velumin dependency configuration.
