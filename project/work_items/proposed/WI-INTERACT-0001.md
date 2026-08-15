---
id: WI-INTERACT-0001
title: Add controllable parent-probe tuning sandbox
type: deliverable
status: proposed
priority: high
owner: project maintainers
created: 2026-08-14
blocked: false
blocked_reason: null
resolution: null
related_focus:
  - FOCUS-INTERACT-0001
related_roadmap:
  - ROADMAP-INITIAL
related_workstreams: []
related_design:
  - project/design/proposals/adopted/DP-0000-replication-vector-game-design.md
  - project/design/proposals/proposed/DP-0003-parent-probe-motion-model.md
  - project/design/proposals/adopted/DP-0001-velumin-repo-infrastructure.md
depends_on: []
blocked_by: []
expected_actions:
  - edit_file
  - run_tests
  - write_docs
  - create_pr
forbidden_actions:
  - force_push
  - delete_branch
  - implement_mining
  - implement_collision
  - implement_shields
  - implement_enemies
  - implement_child_probe
  - implement_scoring
  - implement_progression
  - modify_ci_pipeline
acceptance:
  - A browser sandbox lets a user control parent-probe thrust and turn through the existing Velumin harness.
  - The sandbox uses the existing Rust parent-probe motion model instead of duplicating authoritative motion logic in JavaScript.
  - Developer-facing sliders tune weight, inertia, and responsiveness by mapping to existing motion configuration fields.
  - One or two static asteroid outlines provide spatial context without adding asteroid interaction.
  - Documentation identifies how to run the sandbox and what the controls/sliders affect.
  - Required validation passes.
required_evidence:
  - test_output
  - validation_output
  - lrh_validate
artifacts_expected:
  - replication_vector/src/simulation.rs
  - replication_vector/src/lib.rs
  - replication_vector/web/index.html
  - scripts/README.md
  - project/evidence/EV-XXXX.md
---

# WI-INTERACT-0001: Add Controllable Parent-Probe Tuning Sandbox

## Summary
Add a narrow browser tuning sandbox where maintainers can control the parent probe with keyboard thrust/turn input and adjust heavy-motion feel through designer-facing sliders.

## Problem / Context
`WI-SIM-0001` proved deterministic parent-probe motion in Rust and `WI-REPLAY-0001` made that motion inspectable through Velumin replay artifacts, but maintainers still cannot feel the parent probe under live control. The adopted game design calls for a slow, heavy parent probe that should not feel passive, so the next best-effort slice should expose motion feel before opening mining, collision, shield, enemy, or child-probe gameplay. `FOCUS-INTERACT-0001` authorizes a controllable sandbox for tuning motion feel while keeping broader gameplay systems deferred.

### Duplication search
- In-repo: No existing interactive harness or tuning sandbox implementation found. Related but not duplicate: `WI-SIM-0001` implements deterministic motion without browser input, and `WI-REPLAY-0001` renders a fixed replay without live control.
- Sibling repos: None identified.
- External libraries: None identified; use the existing Rust/WASM/Vite/Velumin browser path and standard browser input/range controls.
- Recommendation: Proceed under `FOCUS-INTERACT-0001`.

### Demand search
- Work items: None found.
- Proposals: Related: `DP-0003` excludes browser input and establishes the motion model that this item should reuse.
- Backlog: Related adopted design notes mention a minimal input loop, but no concrete proposed work item exists.
- Recommendation: No close/link action.

## Scope
- Add a controllable parent-probe sandbox to the existing browser harness.
- Use keyboard input for normalized thrust and turn controls.
- Provide developer-facing sliders for weight, inertia, and responsiveness.
- Render the controlled parent probe and one or two static asteroid outlines through Velumin.

## Required Changes
1. Add Rust/WASM-facing helpers as needed so the browser can step `ParentProbeState` with `ParentProbeMotionInput`, `ParentProbeMotionConfig`, and fixed or bounded `delta_seconds`.
2. Add a small tuning mapper that converts designer-facing `weight`, `inertia`, and `responsiveness` slider values into existing `ParentProbeMotionConfig` fields.
3. Add focused Rust tests for the tuning mapper and any new simulation boundary helpers.
4. Extend `replication_vector/web/index.html` with an opt-in sandbox route or mode that captures keyboard thrust/turn input, steps the Rust motion model, and submits frames through Velumin `renderFrame`.
5. Keep static scene and replay paths available unless the implementation explicitly documents a compatible replacement.
6. Add or document a browser-level sandbox verification path, such as an opt-in smoke command or manual evidence procedure, that can catch broken keyboard input, slider wiring, animation stepping, or Velumin frame submission.
7. Document the sandbox command/path, validation path, and meaning of its controls in `scripts/README.md`.
8. Create evidence recording what was proven and any browser/WebGPU blocker or skip condition encountered.

## Non-Goals
- Do not implement mining, matter resources, asteroid depletion, or resource counters.
- Do not implement asteroid collision, damage, parent integrity, or physics beyond the existing parent-probe motion model.
- Do not implement shield construction, shield repair, enemies, child-probe construction, launch sequence, scoring, progression, game-over, or production UI.
- Do not add committed golden screenshots, mandatory visual regression gates, or broad CI/browser infrastructure.
- Do not replace Velumin with an alternate renderer.
- Do not change Velumin unless a real API blocker is discovered and documented.

## Acceptance Criteria
- The browser exposes an opt-in sandbox where keyboard input controls parent-probe thrust and turn.
- The controlled parent probe is rendered through Velumin alongside one or two static asteroid outlines.
- Weight, inertia, and responsiveness sliders visibly affect motion feel by mapping to existing Rust motion configuration fields.
- New configuration-mapping or simulation-boundary behavior is covered by focused Rust tests.
- Existing static scene and replay behavior remains available unless intentionally superseded and documented.
- Browser-level evidence verifies the sandbox route and retained static/replay render routes, or records a clear WebGPU/browser skip condition.
- Documentation identifies how to run the sandbox and inspect the controls.
- No out-of-scope gameplay systems are added.

## Validation
- `scripts/version tools`
- `scripts/format --check --diff`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- `scripts/render-smoke`
- `scripts/render-replay-smoke`
- Sandbox browser smoke command if added; otherwise documented manual browser verification with PNG/JSON or equivalent evidence
- `lrh validate`

## Risk Notes
- The sandbox could drift into production gameplay UI; keep the controls developer-facing and limited to motion tuning.
- Too many raw physics sliders could make tuning harder; this item should expose designer-facing controls and keep the Rust config mapping explicit.
- Browser input timing is exploratory; deterministic validation should remain in Rust tests.
- Browser/WebGPU setup can fail in fresh environments until `scripts/develop` succeeds.
