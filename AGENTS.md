# AGENTS.md

Guidance for AI coding agents working on Replication Vector.

## Mission
- Help develop Replication Vector as a Rust/WASM/Vite retro vector arcade game in the Velumin ecosystem.
- Preserve the evidence-backed project direction captured in the LRH-style `project/` control plane.
- Keep changes narrow, reviewable, and grounded in project evidence.

## Source of Truth
- Authoritative project intent and status live under `project/`.
- Read these first when orienting:
  1. `project/principles/principles.md`
  2. `project/goal/project_goal.md`
  3. `project/roadmap/roadmap.md`
  4. `project/focus/current_focus.md`
  5. `project/guardrails/`
  6. `project/evidence/`
  7. `project/status/current_status.md`
  8. `project/memory/decision_log.md`
- Treat `project/context/` as derived summary, not as independent authority.
- Keep adopted design proposals under `project/design/proposals/adopted/` aligned with the current implementation direction.

## Validation
- Canonical local validation from the repository root:

```sh
scripts/validate
```

- For ordinary task-phase validation, prefer:

```sh
scripts/version tools
scripts/format --check --diff
scripts/lint
scripts/test
```

- Run `scripts/baseline` when Rust/WASM/Vite browser build behavior may be affected.
- Do not routinely run `scripts/develop` during ordinary validation. Use it for setup/bootstrap or when explicitly debugging setup.
- If validation fails because required tools or the Velumin checkout are missing, report a setup/bootstrap mismatch rather than treating it as a code regression.

## Development Rules
- Follow `STYLE.md` for style guidance.
- Follow `REVIEWS.md` when addressing PR review comments.
- Do not invent roadmap commitments, gameplay systems, browser support, or release maturity.
- Preserve uncertainty markers until maintainers resolve them.
- Keep scripts thin; prefer standard Rust, wasm-pack, npm, Vite, and Velumin behavior over custom logic.
- Do not add Dependabot, cargo-deny, dependency review, release automation, or visual browser smoke gates unless there is an explicit work item.

## Current Technical Shape
- Rust/WASM crate: `replication_vector/`
- Browser harness: `replication_vector/web/`
- Scripted Velumin checkout: `.deps/velumin/`
- Canonical validation scripts: `scripts/`
- Current baseline: compile-time Velumin consumer skeleton, not a gameplay or rendering spike.
