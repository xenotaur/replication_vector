---
id: FOCUS-SIM-0001
title: Initial deterministic parent-probe simulation
status: completed
priority: high
owner: project maintainers
started: 2026-08-05
completed: 2026-08-11
---

# FOCUS-SIM-0001: Initial Deterministic Parent-Probe Simulation

## Active Priority
- Prove Replication Vector has a deterministic Rust simulation model for slow parent-probe movement and rotation.

## Why This Was Current
- `FOCUS-RENDER-0001` was complete: the first project-owned scene was expressed as Velumin command data, rendered through Velumin's downstream browser frame API, and capturable through an opt-in smoke artifact command.
- `ROADMAP-INITIAL` listed Phase 2 Core Simulation after the rendering spike, beginning with slow inertial movement and rotation.
- `DP-0003` proposed a narrow deterministic parent-probe motion model.
- `WI-SIM-0001` was proposed and prompt-ready for the first core simulation slice.

## Priorities
1. Execute `WI-SIM-0001`.
2. Keep the motion model deterministic, Rust-only, and covered by focused unit tests.
3. Preserve the existing static Velumin render harness while simulation behavior is proven.

## Non-Goals
- Do not add browser keyboard, pointer, controller, or input loops in this focus.
- Do not connect simulation state to rendering yet.
- Do not implement mining, matter resources, shield construction, collision, parent integrity, enemies, child-probe construction, launch sequence, scoring, progression, or UI.
- Do not introduce a physics engine or broad simulation architecture.
- Do not change Velumin.

## Exit Criteria
- `WI-SIM-0001` is resolved.
- Evidence records whether the deterministic parent-probe movement and rotation model was implemented and validated.
- Canonical validation remains green.

## Completion Evidence
- `WI-SIM-0001` resolved the first deterministic parent-probe motion model.
- `EV-0007` records the deterministic parent-probe motion model and focused test coverage.
