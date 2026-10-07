---
id: FOCUS-MINING-0002
title: Browser mining interaction and resource feedback
status: active
priority: high
owner: project maintainers
started: 2026-10-07
---

# Current Focus

## Active Priority
- Prove Replication Vector can expose the deterministic matter-resource boundary through a narrow browser mining interaction without opening the rest of the gameplay loop.

## Why This Is Current
- `FOCUS-MINING-0001` and `WI-MINING-0001` completed the Rust-owned matter-resource boundary.
- `WI-MINING-0002` is the next explicit Phase 2 slice: browser keyboard mining input, a Velumin-rendered source and beam, and inspectable active, inactive, out-of-range, and depleted-source behavior.
- The adopted design and existing evidence support a single matter resource and a narrow short-range mining interaction while collision, shields, enemies, child-probe behavior, scoring, progression, and game-over remain deferred.

## Priorities
1. Keep browser input and visual feedback backed by the Rust mining boundary.
2. Preserve explicit active, inactive, out-of-range, and depleted-source behavior.
3. Preserve the static scene, replay, and parent-probe tuning sandbox paths.
4. Keep browser evidence opt-in and reuse the existing Velumin harness.

## Non-Goals
- Do not implement asteroid collision, docking, damage, parent integrity, shields, enemies, child probes, launch, scoring, progression, or game-over.
- Do not add multiple resources, economy systems, inventory UI, production UI, or balancing systems.
- Do not duplicate authoritative mining, transfer, depletion, or capacity logic in JavaScript.
- Do not replace Velumin or add broad visual-regression infrastructure.

## Exit Criteria
- `WI-MINING-0002` is resolved with browser mining evidence.
- Keyboard input activates and deactivates mining through the Rust-owned boundary.
- Browser evidence covers active in-range, inactive, out-of-range no-op, and depleted-source behavior.
- Existing static scene, replay, and parent-probe tuning sandbox paths remain available.
- Canonical validation and LRH validation remain green.
