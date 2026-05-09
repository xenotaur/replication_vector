---
id: WI-INFRA-0002
title: Adapt Velumin-style repository infrastructure
status: done
priority: high
owner: project maintainers
created: 2026-05-09
---

# WI-INFRA-0002: Adapt Velumin-Style Repository Infrastructure

## Objective
Adopt Velumin-compatible guidance, scripts, CI, and a minimal Rust/WASM/Vite skeleton so Replication Vector can validate as a Velumin ecosystem project.

## Scope
- Add root agent/style/review guidance files.
- Add canonical validation scripts.
- Add CI that uses the same script contract as local validation.
- Add a minimal Rust/WASM/Vite skeleton that consumes Velumin through a scripted checkout.
- Record the adopted design proposal.

## Non-Goals
- No gameplay loop.
- No rendering spike.
- No release automation or supply-chain policy gates.

## Evidence
- `project/design/proposals/adopted/DP-0001-velumin-repo-infrastructure.md`
- `project/evidence/EV-0002.md`

## Status
- Done for infrastructure bootstrap.
