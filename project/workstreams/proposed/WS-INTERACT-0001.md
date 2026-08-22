---
id: WS-INTERACT-0001
title: Controllable parent-probe tuning sandbox
kind: planning_node
status: proposed
stage: planned
priority: high
owner: project maintainers
created: 2026-08-22
origin: audit_follow_up
summary: Coordinate the controllable parent-probe tuning sandbox authorized by FOCUS-INTERACT-0001 and WI-INTERACT-0001.
related_focus:
  - FOCUS-INTERACT-0001
related_roadmap:
  - ROADMAP-INITIAL
related_design:
  - project/design/proposals/adopted/DP-0000-replication-vector-game-design.md
  - project/design/proposals/adopted/DP-0003-parent-probe-motion-model.md
work_items:
  - WI-INTERACT-0001
evidence: []
---

# WS-INTERACT-0001: Controllable Parent-Probe Tuning Sandbox

## Purpose
Coordinate the narrow interactivity stream that turns the deterministic parent-probe motion model into a live, opt-in browser tuning sandbox through the existing Velumin harness.

## Stub Scope
- Backfilled to give `WI-INTERACT-0001` a workstream anchor before execution begins.
- Covers live keyboard thrust/turn, developer-facing tuning sliders, static asteroid spatial context, documentation, and browser evidence authorized by `FOCUS-INTERACT-0001`.
- Does not expand into mining, collision, shields, enemies, child-probe construction, scoring, progression, production UI, or CI visual gates.

## Current Work
- `WI-INTERACT-0001` is the proposed implementation item for this stream.

## Notes for Controlling Session
- Flesh out sequencing, acceptance notes, and closeout criteria before or during execution.
- Keep the stream grounded in adopted `DP-0003`; the sandbox should reuse the existing Rust motion model instead of duplicating motion logic in browser JavaScript.
- Record evidence for live keyboard motion through Velumin before moving this workstream out of `proposed`.
