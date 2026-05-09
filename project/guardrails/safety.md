---
id: GUARDRAIL-SAFETY
title: Safety Guardrails
status: active
owner: project maintainers
---

# Safety Guardrails

## Repository Safety
- Preserve existing repository files unless a focused task explicitly requires edits.
- Keep bootstrap and planning artifacts auditable.
- Use TODO/unknown markers instead of inventing implementation facts.
- Avoid hidden dependencies on local Velumin checkouts unless explicitly documented and reviewed.

## Product Safety
- Keep the MVP mechanically readable and testable.
- Avoid visual clutter that obscures important game state.
- Avoid complex systems that make the launch decision obvious, automatic, or irrelevant.

## Testing Safety
- Prefer deterministic tests for core gameplay rules where practical.
- Treat render smoke tests as important because this project is also Velumin dogfood.
