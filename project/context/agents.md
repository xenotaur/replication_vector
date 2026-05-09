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
- Current focus is the bootstrap control plane and immediate follow-up preparation.
- The repository has minimal observed implementation evidence: `README.md`, `LICENSE`, and `.gitignore`.
- Do not assume package layout, runtime, CI, test framework, or Velumin dependency configuration already exist.

## Execution Constraints
- Keep work narrow and evidence-backed.
- Preserve the mine, defend, replicate, and launch loop.
- Defer broad campaign, tech tree, multiplayer, large content systems, and complex economies until the core loop is proven.
- Do not treat this derived file as authoritative; it is a summary of `context/humans.md`, which is itself non-authoritative.

## Confidence and Uncertainty
- High confidence: project identity is Replication Vector / `replication_vector`; the game direction comes from the supplied design summary and README.
- Medium confidence: initial roadmap phases are useful for sequencing but remain draft until implementation begins.
- Low confidence: runtime platform, package structure, owner assignments, and precise Velumin integration approach.
