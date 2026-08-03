# CLAUDE.md

Bridge file for Claude Code and Cowork. Delegates to the canonical agent guidance
in `AGENTS.md` and the LRH control plane, with Claude-specific additions below.

---

## Step 1 — Read canonical agent guidance

Read `AGENTS.md` before taking any action in this repository.

## Step 2 — Read the LRH source-of-truth chain

In precedence order:

1. `project/principles/principles.md`
2. `project/goal/project_goal.md`
3. `project/roadmap/roadmap.md`
4. `project/focus/current_focus.md`
5. `project/work_items/` (in-scope items only)
6. `project/guardrails/`
7. `project/status/current_status.md`
8. `project/memory/decision_log.md`

Treat `project/context/` as derived and informative, not authoritative.

## Step 3 — Claude-specific behavior

**Task tracking:** Use the TodoList tool for any request that involves more than one
tool call. Mark tasks in_progress before starting, completed when done.

**Clarification:** Use AskUserQuestion before starting multi-step or ambiguous work.

**Validation:** Run `scripts/validate` (or the task-phase sequence in AGENTS.md)
before committing. Do not commit changes that break validation.

**Shell paths:** Use the current shell working directory for bash commands and
`<local-checkout>` as the generic repository root when documenting paths. Do not
record developer-specific absolute paths in repository files.

**Control plane updates:** Follow the trigger rules in the "Control Plane Updates"
section of AGENTS.md when completing work items or making significant observations.

**Memory:** Save non-obvious project facts to Cowork session memory so future
sessions start with useful context.

**Dependency navigation:** See `project/context/repository_map.md` for entry points
into the game crate, web harness, and the local Velumin checkout.
