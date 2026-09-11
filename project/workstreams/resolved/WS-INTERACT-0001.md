---
id: WS-INTERACT-0001
title: Controllable parent-probe tuning sandbox
kind: planning_node
status: resolved
stage: closed
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
evidence:
  - EV-0009
exit_criteria:
  - WI-INTERACT-0001 is resolved.
  - EV-0009 records the controllable sandbox browser/WebGPU evidence.
  - The static scene, replay path, and tuning sandbox remain available.
  - LRH validation passes with no control-plane errors or warnings.
---

# WS-INTERACT-0001: Controllable Parent-Probe Tuning Sandbox

## Purpose
Coordinate the narrow interactivity stream that turns the deterministic parent-probe motion model into a live, opt-in browser tuning sandbox through the existing Velumin harness.

## Scope
- Backfilled to give `WI-INTERACT-0001` a workstream anchor before execution began.
- Covers live keyboard thrust/turn, developer-facing tuning sliders, static asteroid spatial context, documentation, and browser evidence authorized by `FOCUS-INTERACT-0001`.
- Does not expand into mining, collision, shields, enemies, child-probe construction, scoring, progression, production UI, or CI visual gates.

## Completed Work
- `WI-INTERACT-0001` is resolved in `project/work_items/resolved/`.
- `EV-0009` records the browser/WebGPU smoke evidence for keyboard motion, tuning sliders, and the retained render paths.

## Closeout Notes
- The stream stayed within adopted `DP-0003` and reused the Rust motion model instead of duplicating motion logic in browser JavaScript.
- The interaction focus is complete; the next authorized stream is the mining/resource slice under `FOCUS-MINING-0001`.
