---
id: DP-0002
title: Add opt-in render smoke artifact capture
status: proposed
date: 2026-08-04
---

# DP-0002: Add Opt-In Render Smoke Artifact Capture

## Decision
- Add an opt-in local render smoke command that captures the Replication Vector browser harness and writes an inspectable screenshot artifact.
- Store generated captures in an ignored output directory such as `replication_vector/web/smoke-out/`.
- Keep the command outside canonical `scripts/validate` until a future work item explicitly authorizes a recurring visual gate.
- Document the command and output path so maintainers can regenerate and inspect the latest rendered scene.

## Rationale
- `WI-RENDER-0002` is intended to render the existing project-owned scene through Velumin's downstream frame API.
- A browser screenshot is the most direct way for maintainers to inspect whether the Velumin-rendered output matches the intended parent probe, asteroid, shield arc, and projectile primitives.
- The current canonical validation path is build-oriented: `scripts/baseline` checks the wasm target, runs `wasm-pack build`, and runs the Vite production build, but it does not save a browser-rendered artifact.
- Project guidance says to keep scripts thin, prefer standard Rust/WASM/npm/Vite behavior, and avoid adding broad visual browser smoke gates unless explicitly authorized.
- Generated outputs should remain out of source control, so the screenshot should be a local artifact, not a committed reference image.

## Proposed Command Shape
- Add a thin repository entrypoint, for example:

```sh
scripts/render-smoke
```

- The command should:
  - ensure the WASM package and Vite harness are built or clearly require `scripts/baseline` first;
  - start or reuse a local Vite server for `replication_vector/web`;
  - drive a WebGPU-capable browser to `http://127.0.0.1:<port>/`;
  - wait for the status text `Velumin rendered 4 scene commands`;
  - capture the viewport screenshot;
  - write the screenshot to `replication_vector/web/smoke-out/first-scene.png`;
  - write lightweight metadata to `replication_vector/web/smoke-out/first-scene.json`, including viewport, command count, Velumin commit, and timestamp;
  - exit nonzero only for real harness/render failures when a suitable browser is available.

## Output Policy
- Add `**/smoke-out/` to `.gitignore`.
- Do not commit generated screenshots by default.
- The artifact path should be stable and documented so a maintainer can inspect the latest capture after running the command.

## Browser / WebGPU Policy
- Prefer Playwright or the minimal browser automation already used by Velumin's own smoke tooling.
- Treat missing WebGPU support as an explicit skip or setup mismatch unless a future work item makes visual smoke mandatory.
- Print a clear message when no suitable browser/GPU adapter is available.
- Do not replace Velumin rendering with a canvas, SVG, or coordinate-only fallback for this validation.

## Non-Goals
- Do not add a mandatory CI visual regression gate.
- Do not commit golden screenshots or binary reference images.
- Do not introduce an alternate renderer.
- Do not add gameplay, physics, input, mining, enemies, scoring, or progression.
- Do not make broad visual-diff infrastructure part of this design.

## Alternatives Considered

### Manual Preview Documentation Only
- Pros: lowest implementation cost and no new tooling.
- Cons: does not automatically save an inspectable artifact.
- Reason not selected: useful as supporting documentation, but it does not satisfy the artifact-capture goal.

### Mandatory `scripts/validate` Visual Gate
- Pros: strongest regression signal.
- Cons: likely brittle in CI or headless environments without WebGPU; conflicts with the current guidance against broad visual smoke gates without explicit scope.
- Reason not selected: premature for the current rendering spike.

### Unit-Generated SVG or PNG
- Pros: deterministic and fast.
- Cons: validates a separate drawing path rather than Velumin's browser renderer.
- Reason not selected: it would not prove the thing the project needs to dogfood.

### Committed Reference Screenshot
- Pros: easy human inspection and future comparison.
- Cons: binary artifact churn; premature before the visual style stabilizes.
- Reason not selected: save local artifacts first; consider references only after repeated rendering work proves the need.

## Consequences
- Maintainers gain a repeatable command that produces an inspectable local screenshot.
- Canonical validation remains lightweight and stable.
- A future work item can promote this into stronger visual regression infrastructure after the rendering path and browser environment are better understood.

## Acceptance Criteria for Implementation
- A command exists that saves `replication_vector/web/smoke-out/first-scene.png`.
- The command documents or prints the artifact path.
- The screenshot is captured from the Velumin-rendered browser harness.
- Missing WebGPU/browser support produces a clear skip/setup message rather than a misleading pass.
- Generated smoke artifacts are ignored by git.
- Documentation points maintainers to the command and output path.

## Implementation Work Item
- `WI-SMOKE-0001`: Add opt-in render smoke artifact capture.
