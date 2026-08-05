---
id: FOCUS-SIM-0001
title: Initial deterministic parent-probe simulation
status: active
priority: high
owner: project maintainers
started: 2026-08-05
---

# Current Focus

## Active Priority
- Prove Replication Vector has a deterministic Rust simulation model for slow parent-probe movement and rotation.

## Why This Appears Current
- `FOCUS-RENDER-0001` is complete: the first project-owned scene is expressed as Velumin command data, rendered through Velumin's downstream browser frame API, and capturable through an opt-in smoke artifact command.
- `ROADMAP-INITIAL` lists Phase 2 Core Simulation after the rendering spike, beginning with slow inertial movement and rotation.
- `DP-0003` proposes a narrow deterministic parent-probe motion model.
- `WI-SIM-0001` is proposed and prompt-ready for the first core simulation slice.

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
