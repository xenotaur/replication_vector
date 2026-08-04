---
id: WI-SMOKE-0001
title: Add opt-in render smoke artifact capture
type: deliverable
status: resolved
priority: medium
owner: project maintainers
created: 2026-08-04
blocked: false
blocked_reason: null
resolution: Added an opt-in `scripts/render-smoke` command that saves ignored PNG and JSON artifacts for the Velumin-rendered first scene; verified locally in EV-0006.
---

# WI-SMOKE-0001: Add Opt-In Render Smoke Artifact Capture

## Objective
Create a local, opt-in render smoke command that saves an inspectable screenshot of the Velumin-rendered Replication Vector scene.

## Context
- `DP-0002` proposes an opt-in screenshot artifact workflow for the browser harness.
- The current canonical validation path builds Rust/WASM/Vite artifacts but does not save a browser-rendered image for human inspection.
- The artifact should help maintainers inspect the rendered parent probe, asteroid, shield arc, and projectile without making visual browser smoke a mandatory CI gate.

## Scope
- Add a thin repository command, such as `scripts/render-smoke`, for local render artifact capture.
- Capture the Velumin-rendered browser harness, not a canvas/SVG/coordinate-only alternate renderer.
- Save the screenshot to a stable ignored path such as `replication_vector/web/smoke-out/first-scene.png`.
- Save lightweight metadata such as viewport, command count, Velumin commit, timestamp, and artifact path.
- Print the saved artifact path for the user.
- Document how to run the command and where to inspect the output.
- Ignore generated smoke artifacts in git.

## Non-Goals
- No mandatory CI visual regression gate.
- No committed golden screenshots or binary reference images.
- No alternate rendering stack.
- No gameplay, physics, input, mining, enemies, scoring, or progression.
- No broad visual-diff infrastructure.

## Acceptance Criteria
- A local command saves `replication_vector/web/smoke-out/first-scene.png`.
- A metadata file is saved next to the screenshot.
- The command captures the Velumin browser harness and waits for the rendered scene status before saving.
- Missing WebGPU/browser support reports a clear skip/setup message rather than a misleading success.
- Generated `smoke-out` artifacts are ignored by git.
- Documentation points maintainers to the command and output path.
- Canonical validation remains green.

## Evidence
- `EV-0006` records the implemented command, artifact paths, metadata, and local successful capture.
