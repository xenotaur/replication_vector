---
id: FOCUS-INTERACT-0001
title: Controllable parent-probe tuning sandbox
status: active
priority: high
owner: project maintainers
started: 2026-08-14
---

# Current Focus

## Active Priority
- Prove Replication Vector can expose a controllable browser sandbox for tuning slow, heavy parent-probe thrust and rotation through the existing Rust simulation model and Velumin render path.

## Why This Appears Current
- `FOCUS-REPLAY-0001` is complete: `WI-REPLAY-0001` resolved deterministic parent-probe motion replay through Velumin and `EV-0008` records the replay artifact evidence.
- The Rust simulation model already exposes normalized thrust/turn input and tunable motion configuration.
- The browser harness already renders project-owned Velumin frames, but does not yet support live parent-probe control.
- A narrow tuning sandbox lets maintainers evaluate heavy motion feel before committing to mining, collision, shield, enemy, or child-probe gameplay.

## Priorities
1. Execute `WI-INTERACT-0001`.
2. Keep the authoritative parent-probe motion step in Rust and deterministic where practical.
3. Add browser keyboard thrust/turn input only for the tuning sandbox.
4. Add a small designer-facing tuning surface for weight, inertia, and responsiveness.
5. Keep one or two static asteroid outlines as spatial references, not interactive objects.

## Non-Goals
- Do not implement mining, matter resources, shield construction, collision, parent integrity, enemies, child-probe construction, launch sequence, scoring, progression, or game-over states.
- Do not implement asteroid interaction; asteroids are visual landmarks only in this focus.
- Do not add production game UI beyond minimal developer tuning controls needed for this sandbox.
- Do not add broad visual regression infrastructure, committed golden screenshots, or mandatory CI visual gates.
- Do not replace Velumin with an alternate rendering stack.
- Do not change Velumin unless a real API blocker is discovered and documented.

## Exit Criteria
- `WI-INTERACT-0001` is resolved.
- Evidence records whether keyboard-controlled parent-probe motion rendered through Velumin with live tuning controls, or records the precise blocker.
- Documentation identifies how to run the tuning sandbox and what the controls/sliders do.
- Canonical validation and LRH validation remain green.
