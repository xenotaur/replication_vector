---
id: WI-MINING-0002
title: Add browser mining interaction and resource feedback
type: deliverable
status: active
priority: high
owner: project maintainers
created: 2026-09-26
blocked: false
blocked_reason: null
resolution: null
related_focus:
  []
related_roadmap:
  - ROADMAP-INITIAL
related_workstreams: []
related_design:
  - project/design/proposals/adopted/DP-0000-replication-vector-game-design.md
  - project/design/proposals/adopted/DP-0003-parent-probe-motion-model.md
depends_on:
  - WI-MINING-0001
blocked_by: []
expected_actions:
  - edit_file
  - run_tests
  - write_docs
  - create_pr
forbidden_actions:
  - implement_collision
  - implement_shields
  - implement_enemies
  - implement_child_probe
  - implement_scoring
  - implement_progression
  - implement_game_over
  - add_resource_economy
  - modify_ci_pipeline
acceptance:
  - Browser keyboard input activates and deactivates mining through the existing Rust mining boundary.
  - A resource-bearing asteroid/source and mining beam are rendered through Velumin during active in-range mining.
  - Matter transfer and source depletion remain authoritative in Rust rather than duplicated in JavaScript.
  - Browser smoke or replay evidence records active in-range mining, inactive mining, out-of-range no-op behavior, and depleted-source behavior.
  - Existing static scene, replay, and parent-probe tuning sandbox paths remain available.
  - Focused Rust/browser validation and documentation pass.
required_evidence:
  - test_output
  - validation_output
  - browser_smoke
  - lrh_validate
artifacts_expected:
  - replication_vector/src/simulation.rs
  - replication_vector/src/lib.rs
  - replication_vector/web/index.html
  - replication_vector/web/render-sandbox-smoke.mjs
  - scripts/README.md
  - project/evidence/EV-XXXX.md
---

# WI-MINING-0002: Add Browser Mining Interaction and Resource Feedback

## Summary
Add the smallest browser-facing mining interaction on top of the Rust-owned matter/resource boundary from `WI-MINING-0001`: keyboard mining input, a Velumin-rendered mining beam and resource-bearing asteroid/source, and minimal inspectable resource feedback.

## Problem / Context
`WI-MINING-0001` established deterministic Rust matter transfer, but the current browser harness has no mining input or visible mining affordance. The next useful slice is to make that boundary inspectable through the existing Velumin sandbox without opening collision, shields, economy, enemies, child-probe, or progression work.

### Duplication search
- In-repo: No browser mining interaction, mining beam, or resource feedback path exists. `WI-MINING-0001` provides the Rust simulation boundary this item must reuse rather than duplicate.
- Sibling repos: None identified.
- External libraries: None needed; use the existing Rust/WASM/Vite/Velumin path.
- Recommendation: Proceed as a browser integration slice over the existing Rust helper.

### Demand search
- Work items: `WI-MINING-0001` explicitly establishes the Rust boundary and leaves browser mining controls deferred.
- Roadmap: Phase 2 lists mining beam and one primary matter resource before shields, enemies, child-probe behavior, scoring, and progression.
- Evidence: `EV-0010` records the Rust boundary and explicitly defers mining browser controls.
- Recommendation: Proceed and link this item to the prior resolved work item.

## Scope
- Add a keyboard mining action to the existing browser harness.
- Feed mining input and the relevant source/probe state into the Rust-owned mining boundary.
- Render one resource-bearing asteroid/source and a simple mining beam through Velumin while active and in range.
- Expose minimal matter/source feedback in the existing sandbox UI or smoke artifact metadata.
- Add focused tests for any new input mapper or browser-to-Rust simulation boundary helper.
- Add an opt-in mining smoke or replay artifact path and document it.

## Required Changes
1. Extend the existing Rust/WASM boundary with the smallest input/state adapter needed to invoke `step_mining(...)` from the browser harness.
2. Add keyboard mining input with clear active/inactive behavior and no JavaScript copy of transfer or depletion rules.
3. Add project-owned Velumin commands for the resource-bearing source and mining beam, preserving the existing parent probe and static scene paths.
4. Add focused tests for input mapping, active/inactive behavior, out-of-range no-op behavior, and any new boundary helper; retain the existing Rust mining tests.
5. Add an opt-in browser smoke or replay command that captures active in-range mining, inactive mining, out-of-range no-op behavior, and depleted-source behavior as PNG/JSON or equivalent inspectable artifacts.
6. Update `scripts/README.md` with the command and controls.
7. Create an evidence record under `project/evidence/` describing the browser/WebGPU behavior, validation, and any setup skip or blocker.

## Non-Goals
- Do not implement asteroid collision, docking, damage, parent integrity, shield construction, shield repair, enemies, child-probe construction, launch sequence, scoring, progression, or game-over states.
- Do not add multiple resources, resource routing, inventory UI, production UI, costs, upgrades, or an economy-balancing system.
- Do not duplicate authoritative mining, transfer, depletion, or capacity logic in JavaScript.
- Do not replace Velumin or introduce an alternate renderer.
- Do not add mandatory browser visual-regression gates, committed golden screenshots, or broad CI infrastructure.

## Acceptance Criteria
- Browser keyboard input activates and deactivates mining through the existing Rust mining boundary.
- A resource-bearing asteroid/source and mining beam render through Velumin during active in-range mining.
- Matter transfer and source depletion remain authoritative in Rust rather than duplicated in JavaScript.
- Browser smoke or replay evidence records active in-range mining, inactive mining, out-of-range no-op behavior, and depleted-source behavior.
- Existing static scene, replay, and parent-probe tuning sandbox paths remain available.
- Focused Rust/browser validation and documentation pass.
- No out-of-scope gameplay systems are added.

## Validation
- `scripts/version tools`
- `scripts/format --check --diff`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- `scripts/render-smoke`
- `scripts/render-replay-smoke`
- mining sandbox browser smoke command added by this item
- `lrh validate`

## Risk Notes
- Browser integration could accidentally make JavaScript authoritative; keep all transfer and depletion decisions in Rust and test the boundary.
- A visible beam can invite scope into collision or combat feedback; keep it as a simple input/state visualization.
- Existing WASM/WebGPU setup may require the known `scripts/develop` bootstrap path; report setup skips separately from code regressions.
- The existing sandbox and artifact paths should remain stable so this item adds inspectability without replacing prior evidence routes.
