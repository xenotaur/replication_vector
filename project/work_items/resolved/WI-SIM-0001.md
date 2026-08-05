---
id: WI-SIM-0001
title: Add deterministic parent-probe movement and rotation model
type: deliverable
status: resolved
priority: high
owner: project maintainers
created: 2026-08-05
blocked: false
blocked_reason: null
resolution: Added a deterministic Rust parent-probe motion model with focused tests for thrust, turn, drag, caps, heading normalization, and fixed-step repeatability; verified in EV-0007.
---

# WI-SIM-0001: Add Deterministic Parent-Probe Movement and Rotation Model

## Objective
Implement the first core simulation slice: a deterministic Rust model for parent-probe movement and rotation.

## Context
- `DP-0003` proposes the first post-rendering core simulation slice.
- `ROADMAP-INITIAL` lists slow inertial movement and rotation as the start of Phase 2 Core Simulation.
- The adopted game design says the parent probe should feel slow, heavy, industrial, powerful, and more like a mobile fortress than a nimble fighter.
- The current renderer scene is static; this item should add simulation-domain state and tests without wiring that state into browser rendering.

## Problem
Replication Vector has proven a static Velumin-rendered scene, but it does not yet have deterministic gameplay-domain state for the parent probe. The next slice should establish slow inertial movement and rotation in Rust so later input, rendering, mining, shield, and enemy work can build on a tested simulation boundary.

## Scope
- Add a small Rust simulation module for parent-probe motion.
- Define parent-probe state with position, velocity, heading, and angular velocity.
- Define motion input with normalized thrust and turn controls.
- Define motion configuration with thrust acceleration, turn acceleration, linear drag, angular drag, max speed, and max angular speed.
- Add a deterministic step function that advances state from explicit inputs, config, and `delta_seconds`.
- Choose and document the initial reverse-thrust behavior explicitly in code or tests.
- Add focused Rust tests for deterministic movement and rotation behavior.

## Required Changes
1. Add a small Rust simulation module inside `replication_vector/src/`, such as `simulation.rs` or `simulation/motion.rs`.
2. Define parent-probe motion data types:
   - `ParentProbeState`
   - `ParentProbeMotionInput`
   - `ParentProbeMotionConfig`
   - any small 2D vector helper needed by the model.
3. Add a deterministic step function, such as `step_parent_probe_motion(state, input, config, delta_seconds)`, that advances state without reading wall-clock time, random state, browser APIs, WebGPU state, or global mutable state.
4. Implement slow inertial behavior:
   - positive thrust accelerates along heading;
   - turn input changes angular velocity before heading changes;
   - zero input preserves drift and damps gradually;
   - linear and angular drag damp existing motion;
   - linear and angular velocity are clamped to configured maxima;
   - heading is normalized to a stable range.
5. Choose the first-slice reverse-thrust rule explicitly. Prefer one of:
   - clamp negative thrust to zero for no reverse thrust; or
   - allow weak reverse thrust with a documented config value.
6. Add focused Rust tests covering the acceptance criteria.
7. Keep existing WASM exports and browser rendering behavior unchanged unless a failing validation command reveals an unavoidable compile boundary adjustment.

## Likely Files
- `replication_vector/src/lib.rs`
- `replication_vector/src/simulation.rs` or `replication_vector/src/simulation/motion.rs`

## Out of Scope
- No browser keyboard, pointer, controller, or input loop.
- No gameplay loop.
- No rendering expansion or moving rendered scene primitives.
- No mining beam, matter resource, shield construction, collision, parent integrity, enemies, child-probe construction, launch sequence, scoring, progression, or UI.
- No physics engine dependency.
- No broad simulation architecture beyond the smallest parent-motion model.
- No changes to Velumin.

## Non-Goals
- Same as `Out of Scope`.

## Acceptance Criteria
- A Rust module exposes a deterministic parent-probe motion model consistent with `DP-0003`.
- The step function is pure Rust and does not read wall-clock time, random state, browser APIs, WebGPU state, or global mutable state.
- Positive thrust accelerates along heading.
- Turn input changes angular velocity and heading over time.
- Zero input with existing velocity continues drifting and damps gradually rather than stopping immediately.
- Linear and angular drag reduce existing motion.
- Max linear speed and max angular speed are enforced.
- Heading normalization is stable and covered by tests.
- Repeated fixed-step updates are deterministic for the same initial state, input sequence, config, and deltas.
- Canonical validation passes:
  - `scripts/version tools`
  - `scripts/format --check --diff`
  - `scripts/lint`
  - `scripts/test`
- `scripts/baseline` is not required unless the implementation touches WASM exports, browser harness behavior, or build output.

## Validation
Run, in order:

- `scripts/version tools`
- `scripts/format --check --diff`
- `scripts/lint`
- `scripts/test`

Also run:

- `lrh validate`

Run `scripts/baseline` only if the implementation touches WASM exports, browser harness behavior, or build output.

## Open Questions
- Should the first implementation clamp negative thrust to zero, or allow weak reverse thrust through configuration? Resolved for this slice: negative thrust is clamped to zero.
- Should the first motion model use a standalone vector helper local to Replication Vector, or should future work seek a shared type boundary with Velumin? Resolved for this slice: the model uses a local `SimVec2`.

## Evidence
- `EV-0007` records the implemented motion model and validation results.
