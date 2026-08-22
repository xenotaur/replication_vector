# Design Backlog

## LRH Schema Portability And Proposal Lifecycle Metadata

Status: open
Priority for Replication Vector: low
Priority for LRH ecosystem: medium
Source: `AUDIT-CONTROL-PLANE-2026-08-22`

### Observation
Mature LRH practice has deeper schema and lifecycle expectations than Replication Vector currently encodes. In particular, mature work-item schemas use fields such as `created_on` and `updated_on`, while Replication Vector work items currently use a simpler `created` field. Mature proposal practice also benefits from distinguishing the design decision's lifecycle from implementation delivery status.

### Why This Is Not A Local Defect
Replication Vector's current control plane validates and is internally consistent enough for its size. Changing local metadata ad hoc would risk creating a project-specific dialect rather than solving the broader portability issue.

### Backlog Question
Should LRH tooling define a compatibility path for lightweight project control planes, including:

- accepted aliases or migrations for date fields such as `created` versus `created_on`;
- a standard proposal lifecycle shape that can represent proposed, adopted, superseded, abandoned, and implementation-backed states;
- validation messages that distinguish local project inconsistency from upstream schema evolution needs?

### Suggested Disposition
Handle this as an LRH design/tooling discussion before asking Replication Vector to migrate metadata. Replication Vector should keep its current schema shape unless LRH tooling provides a documented migration or compatibility rule.
