---
id: DP-0003
title: Add deterministic parent-probe motion model
status: proposed
date: 2026-08-05
---

# DP-0003: Add Deterministic Parent-Probe Motion Model

## Decision
- Add the first core simulation slice as a small, deterministic Rust motion model for the parent probe.
- Model only parent-probe pose, angular velocity, linear velocity, slow thrust, and slow rotation.
- Keep the model independent from browser input, rendering, mining, enemies, child-probe construction, scoring, progression, and collision.
- Validate the model with focused Rust unit tests that step fixed time deltas and assert deterministic state changes.

## Rationale
- `FOCUS-RENDER-0001` has now proven the first Velumin-rendered scene and local render artifact path.
- `ROADMAP-INITIAL` lists Phase 2 Core Simulation after the rendering spike, beginning with slow inertial movement and rotation.
- The adopted game design says the parent probe should feel slow, heavy, industrial, powerful, and more like a mobile fortress than a nimble fighter.
- The same design recommends slow inertial movement and rotation while keeping most tactical agency in later systems such as mining, shield placement, aiming, and launch timing.
- A deterministic Rust model gives the project an early gameplay foundation without coupling simulation correctness to browser timing, keyboard input, WebGPU, or visual presentation.

## Proposed Model

### Types
- `ParentProbeState`
  - `position`: 2D simulation position.
  - `velocity`: 2D simulation velocity.
  - `heading_radians`: current facing direction.
  - `angular_velocity_radians_per_second`: current rotation velocity.
- `ParentProbeMotionInput`
  - `thrust`: normalized scalar in `[-1.0, 1.0]`.
  - `turn`: normalized scalar in `[-1.0, 1.0]`.
- `ParentProbeMotionConfig`
  - `thrust_acceleration`
  - `turn_acceleration`
  - `linear_drag`
  - `angular_drag`
  - `max_speed`
  - `max_angular_speed`
- `step_parent_probe_motion(state, input, config, delta_seconds)`
  - advances state by one fixed or caller-supplied time delta.

### Behavior
- Positive thrust accelerates along the current heading.
- Negative thrust may either be allowed as weak reverse thrust or omitted in the first implementation; the work item should choose one explicitly.
- Turn input changes angular velocity rather than snapping heading directly.
- Linear and angular drag damp velocities over time, preserving an inertial feel.
- Linear speed and angular speed are clamped to configured maxima.
- Heading is normalized to a stable range, such as `[-PI, PI)`.
- Zero input with nonzero velocity continues drifting and gradually damps, rather than stopping immediately.

## Determinism Policy
- The simulation step should be pure Rust with no wall-clock reads, random numbers, browser APIs, or rendering calls.
- Tests should use explicit `delta_seconds` values.
- Floating-point assertions should use small tolerances instead of exact equality where integration math is involved.
- The model should avoid hidden global mutable state.

## Module Boundary
- Prefer a small simulation module inside the existing Rust crate, for example `replication_vector/src/simulation.rs` or `replication_vector/src/simulation/motion.rs`.
- Keep exported WASM/browser functions unchanged unless a future work item explicitly connects simulation state to rendering.
- The existing first-scene rendering functions may remain static for this slice.

## Validation Expectations
- Add Rust tests for:
  - zero-input state remains stable when velocities are zero;
  - positive thrust increases velocity along heading;
  - turn input changes angular velocity and heading over time;
  - drag reduces existing linear and angular velocity;
  - max speed and max angular speed are enforced;
  - repeated fixed-step updates are deterministic for the same initial state, input sequence, config, and deltas.
- Canonical validation should remain green:
  - `scripts/version tools`
  - `scripts/format --check --diff`
  - `scripts/lint`
  - `scripts/test`
- `scripts/baseline` is not required unless the implementation touches WASM exports, browser harness behavior, or build output.

## Non-Goals
- No browser keyboard, pointer, controller, or input loop.
- No gameplay loop.
- No rendering expansion or moving rendered sprites.
- No mining beam, matter resource, shield construction, collision, parent integrity, enemies, child-probe construction, launch sequence, scoring, progression, or UI.
- No physics engine dependency.
- No broad simulation architecture beyond the smallest parent-motion model.

## Alternatives Considered

### Direct Position Control
- Pros: simplest to implement and test.
- Cons: does not express the intended heavy inertial feel.
- Reason not selected: the adopted game design specifically favors mass and inertia.

### Full Asteroids-Style Controller With Input
- Pros: closer to playable movement.
- Cons: couples model, input, timing, and browser behavior too early.
- Reason not selected: the current slice should prove deterministic simulation first.

### Mostly Stationary Parent Probe
- Pros: reinforces fortress-defense positioning.
- Cons: risks making the game feel passive or turret-like before the project can test active movement.
- Reason not selected: the roadmap calls for slow inertial movement and rotation as the first core simulation item.

### Physics Engine
- Pros: could provide rich future collision and dynamics.
- Cons: premature dependency and complexity for one parent-probe movement model.
- Reason not selected: the work needs a tiny, testable Rust model.

## Consequences
- Replication Vector gains its first deterministic gameplay-domain code without expanding rendering or browser interaction.
- Later work can connect this state to input and Velumin rendering after the model is tested.
- Tuning values may need iteration once input, mining distance, shield placement, and enemy pressure exist.
- This proposal leaves collision, world bounds, and camera behavior deliberately undecided.

## Acceptance Criteria for Implementation
- A focused work item exists for this model before implementation begins.
- The implementation adds a deterministic Rust parent-probe motion step and focused tests.
- The work remains independent from browser input and rendering.
- Canonical Rust validation remains green.

## Implementation Work Item
- Proposed next item: `WI-SIM-0001`: Add deterministic parent-probe movement and rotation model.
