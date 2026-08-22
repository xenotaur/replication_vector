# Workstreams

Workstreams group meaningful streams of project work across design, work items, execution records, evidence, and closeout. They are lighter than a roadmap phase and broader than a single work item.

Use a workstream when:

- multiple work items are likely to share one project outcome;
- a focus needs a coordination stub before implementation begins;
- design, evidence, and closeout need a stable place to point;
- the work is substantial enough that future agents should not infer continuity from scattered files.

Do not add workstream ceremony for tiny fixes that are fully explained by one work item, one evidence record, or one design proposal.

## Buckets

- `proposed/`: candidate or stub workstreams.
- `active/`: workstreams currently governing implementation.
- `resolved/`: completed workstreams with closeout notes.
- `abandoned/`: intentionally closed streams that should not continue.

## Metadata

Workstream frontmatter should stay small and explicit: `id`, `title`, `status`, `priority`, `owner`, `created`, related focus/roadmap/design links, related work items, and any known evidence. Backfilled stubs may be intentionally incomplete when they exist to give a controlling session a place to continue.
