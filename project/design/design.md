# Design

## Purpose
- Build a retro vector arcade survival game about defending a self-replication cycle under escalating attack.
- Use the game as a downstream dogfood project for the Velumin vector graphics library.

## Scope
- Current scope is a bootstrap-level design control plane, not an implemented game.
- MVP scope should prove one playable loop with one parent probe, asteroid mining, matter allocation, shield arcs, child-probe construction, launch, and a small enemy set.

## Core Structure
- Intent layer: `principles/`, `goal/`, `roadmap/`
- Execution layer: `focus/`, `work_items/`, `contributors/`
- Constraint layer: `guardrails/`
- Truth layer: `evidence/`, `status/`, `memory/`

## Gameplay Model
- Player role: a heavy, vulnerable self-replicating probe under siege.
- Core loop: arrive -> mine -> defend -> allocate matter -> build child probe -> launch -> continue as successor.
- Main tension: spend matter to survive now or invest in the future child probe.
- Level ending: the player chooses when to launch the successor, rather than simply clearing the screen or escaping.

## System Concepts
- Mining system: short-range or close-contact beam extraction from asteroids, with visible resource flow.
- Resource model: one primary MVP resource, likely matter or mass.
- Shield system: geometric shield arc segments with position, integrity, construction, and repair states.
- Child-probe system: visible construction state and launch thresholds.
- Enemy system: anti-replicator machines that pressure shields, parent integrity, and the child-probe objective.
- Rendering/testing: vector scenes, shield arcs, procedural asteroid outlines, particles, beams, projectile trails, and smoke tests should exercise Velumin.

## Precedence and Interpretation Notes
- Interpret work through: principles -> goal -> roadmap -> focus -> work_items -> guardrails/runtime context.
- Status claims should be backed by `evidence/` entries.
- Derived context files are summaries only and must not add new commitments.

## Current Implementation Boundary
- Repository evidence currently shows only `README.md`, `LICENSE`, and `.gitignore`.
- No source package, tests, scripts, CI, Velumin dependency, or runtime implementation were observed during bootstrap.
- The supplied design summary provides rich product direction, but implementation details remain unvalidated until code is added.

## Future Extensions (Non-binding)
- Roguelite progression, upgrades, sector variety, additional enemies, generated audio, richer visual effects, packaging, and reusable Velumin testing helpers.
