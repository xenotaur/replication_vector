# Decision Log

## 2026-05-09: Bootstrap decision

### Summary
- Created a standard LRH `project/` scaffold for Replication Vector because no pre-existing `project/` directory was present.

### Decisions
- Classified the repository as `new`.
- Seeded the scaffold from the supplied Replication Vector design summary and observed repository evidence.
- Treated implementation details as unconfirmed where no source, tests, scripts, CI, package metadata, or Velumin configuration were observed.
- Kept all created files under `project/`.
- Marked derived context files as non-authoritative.

### Rationale
- The request explicitly asked for an LRH bootstrap and required non-destructive operation.
- The README identifies the project as a retro space combat game with the core "Mine. Build. Replicate." phrase.
- The supplied design summary gives enough product direction for a useful scaffold while repository implementation evidence remains minimal.

### Uncertainty / Follow-ups
- Runtime platform is not confirmed.
- Package layout is not confirmed.
- Exact Velumin dependency strategy is not confirmed.
- Human owners and review responsibilities are not confirmed.
- The moral/narrative framing of the self-replicating probe remains open.

### Status
- Accepted (Bootstrap Phase)

## 2026-08-03: Focus realigned to rendering spike

### Summary
- Archived the bootstrap focus and made the first Velumin rendering spike the active LRH focus.

### Decisions
- Created `FOCUS-RENDER-0001` as the active focus.
- Added `WI-RENDER-0001` as the proposed next implementation work item.
- Added `WI-CONTROL-0003` and `EV-0003` to record the control-plane housekeeping.

### Rationale
- The bootstrap and infrastructure scaffolds are complete enough for implementation work to begin.
- `STATUS-CURRENT` already identified the first Velumin rendering spike as the next recommended action.
- The next implementation should remain bounded to proving project-owned vector scene data through Velumin before broader gameplay systems.

### Uncertainty / Follow-ups
- The current Velumin browser renderer may not yet accept external `VectorCommand` scenes directly.
- `WI-RENDER-0001` should record any Velumin API gap discovered by the spike.

### Status
- Accepted (Control Plane Housekeeping)
