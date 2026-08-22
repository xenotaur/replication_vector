# Project Context (Human-Oriented)

## One-line Description
- Replication Vector is a retro vector arcade survival game about mining, defending, building a child probe, and choosing when to launch the next generation.

## Overview
- The repository currently appears to be an early-stage game project.
- `README.md` identifies it as "ReplicationVector" and says: "Mine. Build. Replicate. Replication Vector: a retro space combat game."
- The supplied design summary describes a Velumin dogfood project where the player controls a self-replicating von Neumann probe under attack by anti-replicator machines.

## Goals and Direction
- Goal: build a tight vector arcade survival game and Velumin downstream validation project.
- Near-term focus: expose a controllable parent-probe tuning sandbox through the existing Rust simulation model and Velumin render path, then use that evidence to guide the playable core loop.
- Core loop: arrive, mine, defend, allocate matter, build child probe, launch, continue as successor.

## Design Snapshot
- The parent probe should feel heavy, powerful, and vulnerable rather than nimble.
- Shields are spatial vector arc segments, not just abstract hit points.
- The child probe is the central objective and should be visible and mechanically meaningful.
- The launch decision is the key differentiator: the level ends because the player decides the successor is ready enough to risk launch.
- Velumin should be exercised through vector lines, arcs, rings, procedural asteroid outlines, beams, particles, projectile trails, and render tests.

## Current Status Snapshot
- Health: yellow.
- Concept direction and repository infrastructure are established, and the active focus is now `FOCUS-INTERACT-0001`.
- The repository now includes Velumin-style guidance files, canonical scripts, CI, and a minimal Rust/WASM/Vite skeleton.
- `DP-0001` records the adopted decision to consume Velumin through a scripted checkout at `.deps/velumin`.
- The rendering spike is complete: `WI-RENDER-0001`, `WI-RENDER-0002`, and `WI-SMOKE-0001` are resolved, with evidence in `EV-0004`, `EV-0005`, and `EV-0006`.
- The first project-owned scene already exists as Velumin `VectorCommand` data, downstream Velumin browser rendering already exists through `WebGPU.renderFrame(frame)`, and an opt-in `scripts/render-smoke` artifact path already saves local PNG/JSON captures under `replication_vector/web/smoke-out/`.
- `WI-INTERACT-0001` is the current proposed next implementation work item.

## Known Unknowns
- Packaged game target beyond the current Rust/WASM/Vite browser harness.
- Human ownership and review responsibilities.
- Exact mining interaction, child-probe placement, inheritance rules, launch thresholds, and render-test strategy.
- Exact live-input feel for the slow, heavy parent probe under keyboard control and tuning sliders.
- Moral/narrative framing of the self-replicating probe.

## Notes
- Derived summary only (non-authoritative).
- `project/context/repository_map.md` is also derived: use it as a navigation aid,
  not as a source of project commitments.
- Authoritative intent lives in `principles/`, `goal/`, `roadmap/`, `design/`, `focus/`, `work_items/`, `guardrails/`, `evidence/`, `status/`, and `memory/`.
