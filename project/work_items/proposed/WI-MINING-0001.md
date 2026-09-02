---
id: WI-MINING-0001
title: Add first mining beam and matter resource model
type: deliverable
status: proposed
priority: high
owner: project maintainers
created: 2026-09-02
blocked: false
blocked_reason: null
resolution: null
related_focus:
  - FOCUS-MINING-0001
related_roadmap:
  - ROADMAP-INITIAL
related_workstreams: []
related_design:
  - project/design/proposals/adopted/DP-0000-replication-vector-game-design.md
  - project/design/proposals/adopted/DP-0003-parent-probe-motion-model.md
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
  - implement_collision
  - implement_shields
  - implement_enemies
  - implement_child_probe
  - implement_scoring
  - implement_progression
  - implement_game_over
  - modify_ci_pipeline
acceptance:
  - A Rust simulation boundary represents one primary matter resource for the parent probe.
  - A deterministic mining helper transfers matter from one resource-bearing asteroid/source to the parent probe under explicit input and range conditions.
  - Focused Rust tests cover successful mining, inactive mining, out-of-range mining, depletion or capacity behavior, and deterministic repeatability.
  - Existing static scene, replay, and parent-probe tuning sandbox paths remain available.
  - Documentation and evidence identify how the mining/resource slice was validated.
  - Required validation passes.
required_evidence:
  - test_output
  - validation_output
  - lrh_validate
artifacts_expected:
  - replication_vector/src/simulation.rs
  - replication_vector/src/lib.rs
  - replication_vector/web/index.html
  - replication_vector/web/render-smoke.mjs
  - scripts/README.md
  - project/evidence/EV-XXXX.md
---

# WI-MINING-0001: Add First Mining Beam and Matter Resource Model

## Summary
Add the first deterministic mining/resource slice: one primary matter resource and a narrow Rust helper that transfers matter from a resource-bearing asteroid/source to the parent probe.

## Problem / Context
Replication Vector now has project-owned Velumin rendering, deterministic parent-probe motion, replay artifacts, and a controllable parent-probe tuning sandbox. The next roadmap Phase 2 capability is mining with one primary resource, but the project does not yet have any authoritative resource state or mining rule. This item should create the smallest simulation-owned mining boundary that later shield, child-probe, enemy, and launch work can build on without adding those systems now.

### Duplication search
- In-repo: No existing mining or matter-resource implementation found. Related but not duplicate: `WI-SIM-0001` and `WI-INTERACT-0001` provide parent-probe motion/control but explicitly exclude mining and matter resources.
- Sibling repos: None identified.
- External libraries: None identified; use the existing Rust/WASM/Vite/Velumin path and a small deterministic Rust model.
- Recommendation: Proceed under `FOCUS-MINING-0001`.

### Demand search
- Work items: None found.
- Proposals: Related: adopted `DP-0000` calls for a short-range mining beam and one primary matter resource; adopted `DP-0003` notes mining as future tactical agency but does not implement it.
- Backlog: Roadmap Phase 2 lists "Implement mining beam and one primary matter resource."
- Recommendation: No close/link action.

## Scope
- Add one primary resource, matter, to the Rust simulation boundary.
- Add a deterministic mining helper driven by explicit input, source state, parent-probe state, configuration, and `delta_seconds`.
- Model one resource-bearing asteroid/source well enough to test extraction and depletion/cap behavior.
- Preserve existing render, replay, and tuning sandbox paths.

## Required Changes
1. Extend `replication_vector/src/simulation.rs` or a small sibling module with resource/mining data types, such as parent matter storage, mining source state, mining input, and mining configuration.
2. Add a deterministic mining step/helper that transfers matter only when mining is active and the parent probe satisfies the chosen first-slice range or contact condition.
3. Clamp extraction so source matter cannot go negative and parent storage cannot exceed its configured capacity, if capacity is included in the first slice.
4. Add focused Rust tests for successful mining, inactive mining, out-of-range mining, depletion or capacity limits, and deterministic repeatability.
5. Expose a narrow WASM/browser or replay artifact path only if needed to make the mining slice inspectable through existing Velumin tooling.
6. Update `scripts/README.md` with any new validation, smoke, or manual inspection command added for this slice.
7. Create a new evidence record under `project/evidence/` recording what was proven, validation output, and any browser/WebGPU setup skip or blocker.

## Non-Goals
- Do not implement asteroid collision, damage, parent integrity, shield construction, shield repair, enemies, child-probe construction, launch sequence, scoring, progression, or game-over states.
- Do not add multiple resources, resource routing, inventory UI, economy balancing, costs, upgrades, or production game UI.
- Do not implement procedural asteroid fields beyond the minimum source state required for deterministic mining tests.
- Do not replace Velumin or introduce an alternate renderer.
- Do not add mandatory browser visual-regression gates, committed golden screenshots, or broad CI infrastructure.

## Acceptance Criteria
- A Rust simulation boundary represents one primary matter resource for the parent probe.
- A deterministic mining helper transfers matter from one resource-bearing asteroid/source to the parent probe under explicit input and range conditions.
- Mining is inactive when the input is off and fails cleanly when the parent probe is outside the first-slice mining condition.
- Extraction is bounded by source depletion and any parent capacity rule introduced by the slice.
- Focused Rust tests cover successful mining, inactive mining, out-of-range mining, depletion or capacity behavior, and deterministic repeatability.
- Existing static scene, replay, and parent-probe tuning sandbox paths remain available.
- Documentation and evidence identify how the mining/resource slice was validated.
- No out-of-scope gameplay systems are added.

## Validation
- `scripts/version tools`
- `scripts/format --check --diff`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- `lrh validate`

## Risk Notes
- The mining slice could sprawl into production economy or UI; keep it to a deterministic simulation boundary and minimal evidence path.
- Mining range/contact behavior affects future feel, so encode the first choice clearly in tests and evidence rather than burying it in browser code.
- Browser/WebGPU setup can fail in fresh environments until `scripts/develop` succeeds; report setup skips separately from code regressions.
- Visual mining affordances can become cluttered later, so defer particles, beams, and resource-flow polish unless needed for this slice's evidence.
