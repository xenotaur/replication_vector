---
id: FOCUS-MINING-0001
title: Initial mining and matter resource slice
status: completed
priority: high
owner: project maintainers
started: 2026-09-02
completed: 2026-09-26
---

# Current Focus

## Active Priority
- Prove Replication Vector can model one primary matter resource and a first deterministic mining interaction without opening the rest of the gameplay loop.

## Why This Appears Current
- `FOCUS-INTERACT-0001` is complete: `WI-INTERACT-0001` resolved the controllable parent-probe tuning sandbox and `EV-0009` records browser/WebGPU smoke evidence.
- `ROADMAP-INITIAL` lists mining beam and one primary matter resource after slow inertial movement in Phase 2 Core Simulation.
- The adopted game design recommends a single first resource, matter, and a short-range mining beam that works best when close or docked.
- A narrow Rust-owned mining/resource slice is the next smallest step toward the core loop while shields, enemies, child-probe behavior, scoring, progression, and collision remain deferred.

## Priorities
1. Refine and execute `WI-MINING-0001`.
2. Keep the first mining/resource state deterministic and implemented in Rust.
3. Model one primary resource only: matter.
4. Keep any browser/render evidence narrow and opt-in, reusing the existing Velumin harness where useful.
5. Preserve the static scene, replay, and tuning sandbox paths unless a future item explicitly supersedes them.

## Non-Goals
- Do not implement asteroid collision, parent damage, shield construction, shield repair, enemies, child-probe construction, launch sequence, scoring, progression, or game-over states.
- Do not add a full resource economy, multiple resource types, inventory UI, production game UI, or balancing system.
- Do not add broad visual regression infrastructure, committed golden screenshots, or mandatory CI visual gates.
- Do not replace Velumin with an alternate rendering stack.
- Do not change Velumin unless a real API blocker is discovered and documented.

## Exit Criteria
- `WI-MINING-0001` is resolved.
- Evidence records whether deterministic mining transfers matter from a resource-bearing asteroid/source to the parent probe, or records the precise blocker.
- Documentation identifies how to validate the mining/resource slice and any opt-in browser artifact path added for evidence.
- Canonical validation and LRH validation remain green.
