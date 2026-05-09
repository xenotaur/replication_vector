# Project Context (Human-Oriented)

## One-line Description
- Replication Vector is a retro vector arcade survival game about mining, defending, building a child probe, and choosing when to launch the next generation.

## Overview
- The repository currently appears to be an early-stage game project.
- `README.md` identifies it as "ReplicationVector" and says: "Mine. Build. Replicate. Replication Vector: a retro space combat game."
- The supplied design summary describes a Velumin dogfood project where the player controls a self-replicating von Neumann probe under attack by anti-replicator machines.

## Goals and Direction
- Goal: build a tight vector arcade survival game and Velumin downstream validation project.
- Near-term focus: establish the LRH control plane, then move toward a minimal rendering spike and playable core loop.
- Core loop: arrive, mine, defend, allocate matter, build child probe, launch, continue as successor.

## Design Snapshot
- The parent probe should feel heavy, powerful, and vulnerable rather than nimble.
- Shields are spatial vector arc segments, not just abstract hit points.
- The child probe is the central objective and should be visible and mechanically meaningful.
- The launch decision is the key differentiator: the level ends because the player decides the successor is ready enough to risk launch.
- Velumin should be exercised through vector lines, arcs, rings, procedural asteroid outlines, beams, particles, projectile trails, and render tests.

## Current Status Snapshot
- Health: yellow.
- Concept direction is strong, but repository implementation evidence is minimal.
- No source code, tests, scripts, CI, package metadata, or Velumin configuration were observed during bootstrap.
- The LRH scaffold records intent, constraints, evidence, current status, and immediate follow-up direction.

## Known Unknowns
- Exact Velumin consumer shape: package dependency, local development path, browser harness, packaged game target, and CI integration details.
- Human ownership and review responsibilities.
- Exact mining interaction, child-probe placement, inheritance rules, launch thresholds, and render-test strategy.
- Moral/narrative framing of the self-replicating probe.

## Notes
- Derived summary only (non-authoritative).
- Authoritative intent lives in `principles/`, `goal/`, `roadmap/`, `design/`, `focus/`, `work_items/`, `guardrails/`, `evidence/`, `status/`, and `memory/`.
