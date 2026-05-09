---
id: WI-BOOTSTRAP-0001
title: Bootstrap LRH project control plane
status: done
priority: high
owner: project maintainers
created: 2026-05-09
---

# WI-BOOTSTRAP-0001: Bootstrap LRH Project Control Plane

## Objective
Create the initial LRH `project/` scaffold for Replication Vector without modifying existing source files or inventing unsupported implementation state.

## Scope
- Add standard LRH artifact categories under `project/`.
- Seed authoritative artifacts from the supplied design summary and observed repository evidence.
- Add derived human and agent context summaries.
- Record bootstrap assumptions, uncertainties, and evidence.

## Evidence
- `README.md` identifies the project as ReplicationVector and describes it as "Mine. Build. Replicate. Replication Vector: a retro space combat game."
- The supplied design summary describes Replication Vector as a retro vector arcade survival game and Velumin dogfood project.
- No pre-existing `project/` directory was present.

## Acceptance Criteria
- `project/` contains the requested standard LRH scaffold.
- No existing files are modified.
- Unknowns are called out explicitly.
- Derived context files do not introduce new commitments.

## Status
- Done for bootstrap.
