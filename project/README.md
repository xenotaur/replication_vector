# Replication Vector Project Control Plane

This `project/` directory is the planning, evidence, and memory layer for Replication Vector. It records the current authoritative project state separately from implementation code so agents and humans can make narrow, reviewable changes without inventing scope.

## Current Truth

- Project goal: build Replication Vector as a Rust/WASM/Vite retro vector arcade game in the Velumin ecosystem.
- Current focus: `FOCUS-INTERACT-0001`, a controllable parent-probe tuning sandbox.
- Current proposed implementation item: `WI-INTERACT-0001`.
- Completed foundations: project-owned scene rendering, downstream Velumin browser rendering, opt-in render smoke artifacts, deterministic parent-probe motion, and deterministic replay artifacts.
- Rendering and simulation evidence: `EV-0004`, `EV-0005`, `EV-0006`, `EV-0007`, and `EV-0008`.

## Directory Guide

- `principles/`: stable project principles.
- `goal/`: durable project goal.
- `roadmap/`: phase-level direction.
- `focus/`: current focus and archived completed focus records.
- `work_items/`: proposed, active, and resolved implementation units.
- `workstreams/`: larger streams that group related work items, design, evidence, and closeout.
- `design/`: project design notes, adopted/proposed design proposals, and design backlog.
- `evidence/`: proof records for completed changes and notable observations.
- `status/`: current project health, active priorities, risks, and next steps.
- `context/`: derived summaries for humans and agents; do not treat this as independent authority.
- `memory/`: decision log and durable observations.
- `executions/`: session execution records.
- `audits/`: structured control-plane or documentation audits.
- `guardrails/`: approval, cost, optics, and safety constraints.
- `config/`: local LRH/control-plane defaults.

## Operating Rules

- Treat `project/focus/current_focus.md`, `project/status/current_status.md`, adopted proposals, work items, and evidence as authoritative.
- Treat `project/context/` as a derived convenience layer that must catch up to authority, not override it.
- Resolve work items with evidence before expanding scope.
- Move adopted proposals into `project/design/proposals/adopted/` once implementation and evidence confirm the decision governs the project.
- Add workstreams when a stream is broader than one narrow implementation item or needs explicit coordination across design, evidence, and closeout.
