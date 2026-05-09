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
- Current focus is the Velumin-compatible infrastructure baseline and the next rendering spike.
- The repository has root guidance files, canonical scripts, CI, and a minimal Rust/WASM/Vite skeleton.
- Velumin is consumed through a scripted checkout at `.deps/velumin`; run `scripts/develop` when setup is missing.
- The intended implementation should consume Velumin as the rendering package/library; do not introduce an alternate rendering stack without an explicit design decision.

## Execution Constraints
- Keep work narrow and evidence-backed.
- Preserve the mine, defend, replicate, and launch loop.
- Defer broad campaign, tech tree, multiplayer, large content systems, and complex economies until the core loop is proven.
- Do not treat this derived file as authoritative; it is a summary of `context/humans.md`, which is itself non-authoritative.

## Confidence and Uncertainty
- High confidence: project identity is Replication Vector / `replication_vector`; the game direction comes from the supplied design summary and README.
- High confidence: `DP-0001` adopts the Velumin-style infrastructure and scripted checkout dependency shape.
- Medium confidence: initial roadmap phases are useful for sequencing but remain draft until gameplay implementation begins.
- Low confidence: owner assignments and precise gameplay/rendering API usage.
