---
id: WI-RENDER-0002
title: Render the existing scene through Velumin's downstream frame API
type: deliverable
status: resolved
priority: high
owner: project maintainers
created: 2026-08-04
blocked: false
blocked_reason: null
resolution: Rendered the existing Replication Vector scene through Velumin's downstream frame API in the browser harness.
---

# WI-RENDER-0002: Render the Existing Scene Through Velumin's Downstream Frame API

## Objective
Wire Replication Vector's existing project-owned scene into Velumin's updated downstream browser rendering API.

## Context
- `WI-RENDER-0001` proved that Replication Vector can construct a project-owned scene as Velumin `VectorCommand` data.
- `EV-0004` recorded the prior blocker: Velumin's browser API did not expose a public method accepting downstream scene data.
- Updated Velumin now exposes a wasm-facing `VectorFrame` builder and `WebGPU.renderFrame(frame)` API, plus a Rust-side `render_commands(&[VectorCommand])` helper in the browser renderer boundary.

## Scope
- Update the local Velumin checkout/dependency path as needed so Replication Vector builds against the Velumin API that includes `VectorFrame` and `renderFrame`.
- Convert or expose the existing `first_replication_vector_scene()` primitives through the new Velumin downstream frame API.
- Update the browser harness to create a Velumin `WebGPU` renderer and render the existing Replication Vector scene to the canvas.
- Add focused validation that distinguishes scene-data construction from successful use of the Velumin downstream-rendering boundary.
- Record evidence for whether the project-owned scene renders through Velumin successfully.

## Non-Goals
- No gameplay loop.
- No physics, collision, mining, enemies, child-probe construction, progression, scoring, or input handling.
- No alternate rendering stack.
- No broad visual regression framework beyond the smallest smoke validation needed for this integration.
- No changes to Velumin unless a new blocker is discovered in the updated API.

## Acceptance Criteria
- Replication Vector's browser harness renders the existing parent probe outline, asteroid outline, shield arc, and projectile line through Velumin's downstream frame/render API.
- The implementation uses Velumin's renderer path rather than a canvas/SVG workaround.
- Canonical validation passes:
  - `scripts/version tools`
  - `scripts/format --check --diff`
  - `scripts/lint`
  - `scripts/test`
  - `scripts/baseline`
  - `lrh validate`
- Evidence records whether the updated Velumin API is sufficient or identifies any remaining blocker.
- The work remains consistent with `FOCUS-RENDER-0001`.

## Evidence
- `project/evidence/EV-0005.md`

## Status
- Done: the local Velumin checkout was updated to the API that exposes `VectorFrame` and `WebGPU.renderFrame(frame)`.
- Done: Replication Vector exports `replication_vector_first_scene_frame()` for the existing scene.
- Done: the browser harness creates a Velumin `WebGPU` renderer and submits the project-owned frame via `renderFrame`.
- Done: local browser verification showed the required scene primitives and `Velumin rendered 4 scene commands` status text.
