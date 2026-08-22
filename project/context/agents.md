# Project Context (Agent-Oriented)

## Mission Summary
- Help build Replication Vector: a retro vector arcade survival game where a self-replicating probe mines matter, builds shields, survives anti-replicator attacks, constructs a child probe, and chooses when to launch.
- Treat the game as both a standalone project and downstream Velumin dogfood.

## Read Order
1. `project/principles/principles.md`
2. `project/goal/project_goal.md`
3. `project/roadmap/roadmap.md`
4. `project/design/design.md`
5. `project/focus/current_focus.md`
6. `project/work_items/`
7. `project/guardrails/`
8. `project/evidence/`
9. `project/status/current_status.md`
10. `project/memory/decision_log.md`

## Current Operating Context
- Current focus is `FOCUS-INTERACT-0001`: add a controllable parent-probe tuning sandbox for keyboard thrust/turn input and weight/inertia/responsiveness sliders.
- The repository has root guidance files, canonical scripts, CI, and a minimal Rust/WASM/Vite skeleton.
- Velumin is consumed through a scripted checkout at `.deps/velumin`; run `scripts/develop` when setup is missing.
- The intended implementation should consume Velumin as the rendering package/library; do not introduce an alternate rendering stack without an explicit design decision.
- The rendering spike is complete: `WI-RENDER-0001`, `WI-RENDER-0002`, and `WI-SMOKE-0001` are resolved.
- `EV-0004` proves the first project-owned scene exists as Velumin `VectorCommand` data; `EV-0005` proves downstream Velumin browser rendering through `VectorFrame` and `WebGPU.renderFrame(frame)`; `EV-0006` proves the opt-in `scripts/render-smoke` artifact path.
- `WI-INTERACT-0001` is the proposed next implementation item; move it to `active` only when beginning the controllable tuning sandbox.

## Execution Constraints
- Keep work narrow and evidence-backed.
- Preserve the mine, defend, replicate, and launch loop.
- Defer broad campaign, tech tree, multiplayer, large content systems, and complex economies until the core loop is proven.
- Do not treat this derived file or `project/context/repository_map.md` as authoritative; they are navigation summaries, not sources of project commitments.

## Confidence and Uncertainty
- High confidence: project identity is Replication Vector / `replication_vector`; the game direction comes from the supplied design summary and README.
- High confidence: `DP-0001` adopts the Velumin-style infrastructure and scripted checkout dependency shape.
- High confidence: the first project-owned scene, downstream Velumin browser rendering, and opt-in render smoke artifact path already exist.
- Medium confidence: the next authorized slice is the live parent-probe tuning sandbox, but exact keyboard-control feel and slider mapping still need implementation evidence.
- Low confidence: owner assignments and precise future gameplay behavior beyond the current focus.
