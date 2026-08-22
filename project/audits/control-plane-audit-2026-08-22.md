---
id: AUDIT-CONTROL-PLANE-2026-08-22
title: Control-plane maturity comparison
status: accepted
date: 2026-08-22
audit_type: control_plane
compared_repositories:
  - logical_robotics_harness
  - LCATS
---

# Control-Plane Maturity Comparison

## Scope
Audit Replication Vector's `project/` control plane against more mature LRH-style repositories, especially `logical_robotics_harness` and `LCATS`, and rank differences by project importance.

## Bottom Line
Replication Vector's control plane is healthy for a young project. It has source-of-truth guidance, a current focus, a roadmap, guardrails, work items, evidence, proposal lifecycle buckets, chain defaults, and execution records. The biggest immediate issue was lifecycle drift around `DP-0003`, followed by missing workstream and audit structure as the project grows beyond single-slice implementation.

The mature repositories are useful patterns, but they should not be copied wholesale. Replication Vector should add only the light structure it needs now and record broader LRH schema/tooling questions as backlog rather than local churn.

## What Replication Vector Has

- `AGENTS.md` defines the control-plane authority model and says `project/context/` is derived rather than authoritative.
- `project/focus/current_focus.md` names `FOCUS-INTERACT-0001` as the current scope and exits on `WI-INTERACT-0001` plus evidence of live keyboard motion through Velumin.
- `project/status/current_status.md` accurately frames the next project state as a move from deterministic replay to controllable tuning sandbox.
- `project/work_items/proposed/WI-INTERACT-0001.md` is a strong narrow work item with acceptance criteria, non-goals, validation, and artifact expectations.
- `project/evidence/EV-0004.md`, `project/evidence/EV-0005.md`, and `project/evidence/EV-0006.md` prove the rendering spike; `EV-0007` and `EV-0008` prove the first deterministic motion and replay follow-on.
- `project/design/proposals/adopted/` already captures adopted decisions such as the game design, Velumin repository infrastructure, and render smoke artifact path.
- `project/config/chain-defaults.yaml` exists, which puts Replication Vector closer to current LRH practice than a bare documentation tree.

## What Was Missing

- No `project/workstreams/` directory existed, even though `WI-INTERACT-0001` is now a natural stream boundary that will likely connect design, implementation, evidence, and closeout.
- No `project/audits/` directory existed, so there was no durable place to store structured control-plane or documentation audits.
- No `project/README.md` existed, making orientation depend on `AGENTS.md` plus scattered project files.
- No project design backlog existed for cross-repository process questions that are real but not immediate implementation work.
- Replication Vector does not yet have mature assistant-role or session-index structure. That is acceptable for now because the project is still small.

## Practice Conflicts

### High: DP-0003 Lifecycle Drift
`project/design/proposals/proposed/DP-0003-parent-probe-motion-model.md` still had `status: proposed`, but `project/work_items/resolved/WI-SIM-0001.md` records the deterministic parent-probe model as resolved and `project/evidence/EV-0007.md` records the implementation and validation. The proposal should therefore live in the adopted bucket and say it was implemented by `WI-SIM-0001`.

### Medium: Roadmap Breadth Versus Current Focus
`project/roadmap/roadmap.md` describes broader Phase 2 Core Simulation work, while `project/focus/current_focus.md` deliberately limits the next slice to a tuning sandbox and excludes mining, shields, collision, enemies, child-probe behavior, and scoring. This is not a defect, but future agents should follow the focus when selecting implementation scope.

### Low: Schema Depth Compared With LRH
Mature LRH artifacts distinguish more lifecycle and metadata dimensions than Replication Vector currently needs. For example, mature proposal practice separates whether a design governs the project from whether its implementation has landed, while Replication Vector proposal frontmatter currently uses a simpler `status` field. This should be handled as an LRH tooling and schema design question, not as a one-off Replication Vector migration.

## Importance Ranking

1. Adopt `DP-0003` and update references to its adopted path.
2. Add a workstream stub for `WI-INTERACT-0001` so the controlling session can flesh out coordination without inventing a new directory convention.
3. Add `project/README.md` so humans and agents have a concise map of the control plane.
4. Save this audit under `project/audits/` and use future audits only when they answer a concrete maintenance question.
5. Record LRH schema/tooling issues in a design backlog rather than changing Replication Vector's local schema speculatively.
6. Defer assistant-role and session-index structure until the project has enough parallel agent activity to justify it.

## Follow-Up Decisions Captured By This PR

- `DP-0003` was moved to the adopted proposal bucket because `WI-SIM-0001` and `EV-0007` prove implementation.
- `WS-INTERACT-0001` was created as a stub workstream for the upcoming controllable parent-probe tuning sandbox.
- `project/design/backlog.md` now records the schema/lifecycle portability issue as an LRH ecosystem design topic.
