---
id: FOCUS-REPLAY-0001
title: Deterministic parent-probe motion replay evidence
status: completed
priority: high
owner: project maintainers
started: 2026-08-11
completed: 2026-08-14
---

# FOCUS-REPLAY-0001: Deterministic Parent-Probe Motion Replay Evidence

## Active Priority
- Prove Replication Vector can replay deterministic parent-probe motion through Velumin and save inspectable PNG/JSON capture artifacts.

## Why This Was Current
- `FOCUS-SIM-0001` was complete: `WI-SIM-0001` resolved the first deterministic parent-probe motion model and `EV-0007` records the validation evidence.
- The existing Velumin browser harness and smoke artifact path could already save inspectable PNG/JSON output for a static scene.
- `WI-REPLAY-0001` was proposed and prompt-ready for the narrow depth-first replay artifact slice.
- Mining design remained deferred while maintainers prioritized discoverability of current deterministic motion state.

## Priorities
1. Execute `WI-REPLAY-0001`.
2. Keep the replay fixed, scripted, deterministic, and sourced from the Rust simulation model.
3. Render replay-derived parent-probe pose through Velumin's browser harness.
4. Save inspectable local PNG/JSON artifacts without turning the capture into a broad visual regression gate.

## Non-Goals
- Do not add browser keyboard, pointer, controller, or input loops in this focus.
- Do not implement mining, matter resources, shield construction, collision, parent integrity, enemies, child-probe construction, launch sequence, scoring, progression, or UI.
- Do not add live gameplay loops or player-controlled motion.
- Do not add broad visual regression infrastructure, committed golden screenshots, or mandatory CI visual gates.
- Do not replace Velumin with an alternate rendering stack.
- Do not change Velumin unless a real API blocker is discovered and documented.

## Exit Criteria
- `WI-REPLAY-0001` is resolved.
- Evidence records whether deterministic parent-probe motion replay rendered through Velumin and saved inspectable PNG/JSON artifacts, or records the precise blocker.
- Documentation identifies how to run the replay capture command and where to inspect generated artifacts.
- Canonical validation and LRH validation remain green.

## Completion Evidence
- `WI-REPLAY-0001` resolved the deterministic parent-probe motion replay slice.
- `EV-0008` records the deterministic parent-probe replay artifact path and validation evidence.
- `scripts/README.md` documents the opt-in replay capture command and ignored PNG/JSON output paths.
