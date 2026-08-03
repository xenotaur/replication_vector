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

## Session Start

When beginning a session without `lrh snapshot`, orient using these steps in order:

1. Read `project/focus/current_focus.md` — confirm active scope and exit criteria.
2. Read `project/work_items/` — identify items that are in-scope, unblocked, and not done.
3. Read `project/status/current_status.md` — note any known risks or blockers.
4. Select the highest-priority unblocked item and proceed.

Do not invent scope based on general knowledge of the project. If the focus and work
items do not authorize a change, surface the question rather than proceeding.

## Work Item Lifecycle

Work items transition through the following states. Update the `status` frontmatter
field at each transition.

| Transition | Trigger | Agent action |
|---|---|---|
| `proposed` → `in_progress` | Item selected, consistent with current focus | Update `status`; note in commit or session |
| `in_progress` → `done` | Acceptance criteria met, evidence exists | Update `status`; create evidence record |
| `in_progress` → `blocked` | Dependency or external blocker encountered | Update `status`; note blocking cause |
| `blocked` → `in_progress` | Blocker resolved | Update `status`; note resolution |

Keep changes scoped to the work item's acceptance criteria. If useful work is
discovered outside the current item's scope, propose a new work item rather than
incorporating it silently.

## Control Plane Updates

Update the LRH control plane when the following triggers are met. Do not update
it speculatively or to record routine implementation details.

- **Work item done:** update `status` to `done`; create `project/evidence/EV-XXXX.md`
  referencing the commit SHA or PR; update `project/status/current_status.md` if
  overall project health has changed.
- **Non-obvious design or implementation decision:** append to
  `project/memory/decision_log.md` or create an adopted proposal under
  `project/design/proposals/adopted/` when the decision changes project direction.
  Capture what was decided, why, and what alternatives were considered.
- **Surprising observation about behavior, tooling, or dependencies:** create a
  lightweight evidence record even if no work item directly requires it.
- **Focus complete:** archive `current_focus.md` to `project/focus/archive/`; update
  roadmap phase status; confirm or select the next focus.
- **Principle or scope conflict:** do not self-resolve. Surface the conflict in
  `project/status/current_status.md` under Risks or leave a clearly marked TODO.

## Git Hygiene

### Commit messages

Use Conventional Commits format:

```
<type>(<scope>): <short description>

[optional body]

[optional footer: WI reference, co-author]
```

Types: `feat`, `fix`, `chore`, `docs`, `test`, `refactor`, `ci`

Include the work item ID in the footer when the commit corresponds to a specific
LRH work item:

```
feat(renderer): render parent probe triangle via Velumin

Establishes the basic VectorCommand emission pattern for entity rendering.

WI: WI-RENDER-0001
```

### Branch naming

Use `wi/<WI-ID>-short-description` for agent-driven branches:

```
wi/WI-RENDER-0001-parent-probe-triangle
wi/WI-SIM-0002-mining-collision-rules
```

### Atomicity

One work item, one PR. Do not accumulate unrelated fixes within a branch. If
incidental issues are found, note them as new proposed work items.

### What not to commit

- Build artifacts: `target/`, `dist/`, `node_modules/`, wasm-pack outputs unless
  intentionally tracked
- Generated test outputs or validation logs
- Dependency caches or local checkout artifacts (`.deps/velumin/` is managed by
  `scripts/develop` and must not be committed)
- Session scratch files or temporary work

### Validate before commit

Run the task-phase validation sequence before committing:

```sh
scripts/version tools
scripts/format --check --diff
scripts/lint
scripts/test
```

Do not push commits that break validation. If validation fails due to a
setup/bootstrap mismatch, report the mismatch rather than modifying code to work
around it.
