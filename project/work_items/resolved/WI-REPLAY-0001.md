---
id: WI-REPLAY-0001
title: Render deterministic parent-probe motion replay artifacts
type: deliverable
status: resolved
priority: high
owner: project maintainers
created: 2026-08-07
blocked: false
blocked_reason: null
resolution: Added a deterministic parent-probe motion replay through the Rust simulation model and Velumin browser harness, with opt-in PNG/JSON capture artifacts documented and verified in EV-0008.
related_focus:
  - FOCUS-REPLAY-0001
related_roadmap:
  - ROADMAP-INITIAL
related_workstreams: []
related_design:
  - project/design/proposals/proposed/DP-0003-parent-probe-motion-model.md
  - project/design/proposals/adopted/DP-0002-render-smoke-artifact.md
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
  - implement_player_input
  - implement_mining
  - implement_shields
  - implement_enemies
  - implement_child_probe
  - implement_scoring
  - implement_progression
  - modify_ci_pipeline
acceptance:
  - A fixed deterministic parent-probe motion replay is rendered through the Velumin browser harness.
  - The replay uses the existing Rust simulation model rather than duplicating motion logic in JavaScript.
  - Inspectable PNG and JSON artifacts are saved under an ignored smoke output path.
  - Metadata records enough replay state to understand what frame or sequence was captured.
  - No player input, gameplay loop, mining, shields, enemies, child-probe construction, scoring, progression, or UI is implemented.
  - Required validation passes.
required_evidence:
  - test_output
  - validation_output
  - lrh_validate
artifacts_expected:
  - replication_vector/src/lib.rs
  - replication_vector/src/simulation.rs
  - replication_vector/web/index.html
  - replication_vector/web/render-smoke.mjs
  - scripts/render-replay-smoke
  - scripts/README.md
  - project/evidence/EV-XXXX.md
---

# WI-REPLAY-0001: Render Deterministic Parent-Probe Motion Replay Artifacts

## Summary
Render a fixed deterministic parent-probe motion replay through the existing Rust/WASM/Velumin browser path and save inspectable PNG/JSON artifacts for maintainers.

## Problem / Context
`WI-SIM-0001` added a deterministic Rust parent-probe motion model, but its output is not yet discoverable in the browser or artifact capture path. The existing render smoke path captures the static first scene, so a narrow replay artifact is the smallest next step for showing current simulation state without opening mining or gameplay design. `FOCUS-REPLAY-0001` authorizes this motion replay scope.

### Duplication search
- In-repo: Related, not duplicate: `WI-SMOKE-0001` captures static Velumin screenshots; `WI-SIM-0001` implements motion without rendering.
- Sibling repos: None identified.
- External libraries: None identified; use the existing Rust/WASM/Vite/Velumin/Playwright path.
- Recommendation: Proceed under `FOCUS-REPLAY-0001`.

### Demand search
- Work items: None found.
- Proposals: Related: `DP-0002` and `DP-0003`, but neither requests this exact replay artifact.
- Backlog: Related design notes mention deterministic replay testing, but no proposed work item exists.
- Recommendation: No close/link action.

## Scope
- Render a fixed, scripted parent-probe motion replay using the existing deterministic Rust simulation model.
- Convert replayed parent-probe pose into Velumin-facing vector scene data.
- Capture the replay output through the existing browser harness and smoke artifact pattern.
- Save ignored PNG and JSON artifacts that make the simulated motion state inspectable.

## Required Changes
1. Add a small deterministic replay fixture or helper that steps `ParentProbeState` through a fixed `ParentProbeMotionInput` sequence.
2. Add or adapt scene construction so the parent-probe outline can be transformed by replayed position and heading.
3. Expose a WASM/browser path that renders the replay frame or a compact fixed replay state through Velumin's `VectorFrame` / `renderFrame` boundary.
4. Extend `scripts/render-smoke` or add a sibling smoke command to save replay PNG and JSON artifacts under `replication_vector/web/smoke-out/`.
5. Include replay metadata such as timestep, sample/frame index, final or captured `ParentProbeState`, command count, Velumin commit, viewport, timestamp, and artifact path.
6. Document the command and output paths in `scripts/README.md`.
7. Create evidence recording what was proven and any browser/WebGPU skip condition encountered.

## Non-Goals
- Do not add keyboard, pointer, controller, or live gameplay input.
- Do not implement a gameplay loop.
- Do not implement mining, matter resources, shields, enemies, child-probe construction, scoring, progression, or UI.
- Do not add broad visual regression infrastructure, committed golden screenshots, or mandatory CI visual gates.
- Do not replace Velumin with an alternate renderer.
- Do not change Velumin unless a real API blocker is discovered and documented.

## Acceptance Criteria
- A fixed deterministic replay advances parent-probe motion through the Rust simulation model.
- The browser harness renders replay-derived parent-probe pose through Velumin.
- A local opt-in command saves an inspectable PNG artifact for the replay.
- A JSON metadata artifact is saved next to the PNG and includes replay state/context.
- Generated replay artifacts are ignored by git.
- Documentation identifies how to run the command and where to inspect output.
- Existing static scene rendering remains available unless intentionally superseded by the replay command.
- No out-of-scope gameplay systems are added.

## Validation
- `scripts/version tools`
- `scripts/format --check --diff`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- `scripts/render-smoke` or the new replay smoke command
- `lrh validate`

## Risk Notes
- Browser/WebGPU support may skip capture in some local environments; the command should report a clear skip rather than false success.
- Video capture is intentionally deferred because PNG/JSON artifacts are smaller and match the existing smoke pattern.
- Rendering a replay frame can drift into gameplay input or UI if not kept to a fixed scripted sequence.
- `FOCUS-REPLAY-0001` authorizes this replay scope; keep implementation inside that focus.
