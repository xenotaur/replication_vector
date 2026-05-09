---
id: GUARDRAIL-APPROVALS
title: Approval Guardrails
status: active
owner: project maintainers
---

# Approval Guardrails

## Default Approval Rules
- Bootstrap changes may add LRH artifacts under `project/` only.
- Source code, package files, CI, scripts, and existing documentation require normal review before modification.
- Destructive operations, file deletion, renaming, or restructuring require explicit maintainer approval.

## Escalation Points
- Changing the project goal or core design thesis.
- Choosing the runtime platform, package layout, or Velumin dependency strategy.
- Introducing CI, publishing, release, or distribution behavior.
- Expanding MVP scope beyond the mine, defend, replicate, and launch loop.

## Notes
- This file constrains execution; it does not authorize work by itself.
