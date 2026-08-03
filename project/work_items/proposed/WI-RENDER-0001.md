---
id: WI-RENDER-0001
title: Render the first Replication Vector scene through Velumin
type: deliverable
status: proposed
priority: high
owner: project maintainers
created: 2026-08-03
blocked: false
blocked_reason: null
resolution: null
---

# WI-RENDER-0001: Render the First Replication Vector Scene Through Velumin

## Objective
Create the smallest rendering spike that proves Replication Vector can render project-owned vector scene data through Velumin.

## Scope
- Represent a parent probe outline, asteroid outline, shield arcs, and simple projectile lines as Velumin-facing scene data.
- Validate the scene through the existing Rust/WASM/Vite project shape.
- Document whether the current Velumin API accepts external scene commands directly or requires an upstream Velumin API change.
- Keep the spike focused on rendering primitives and smoke validation.

## Non-Goals
- No gameplay loop.
- No physics, collision, mining, enemies, child-probe construction, or input handling.
- No alternate rendering stack.
- No broad visual regression framework unless required to prove the spike.

## Acceptance Criteria
- A Replication Vector-owned scene exists with parent probe, asteroid, shield arc, and projectile primitives.
- The scene is validated through canonical scripts.
- Any Velumin API gap discovered by the spike is recorded in evidence and status.
- The work remains consistent with `FOCUS-RENDER-0001`.

## Evidence
- TODO: Add evidence when implementation begins.
