# Grant progress — ObsidianLog

Monthly progress reports for the Sia Foundation grant. Each milestone's tasks are
listed with the pull request(s) — or commit(s) — that implement them, following
the Foundation's Grants Development Guide.

> **Reporting note.** Early scaffolding work predates the project's switch to a
> PR-based flow, so those rows link **commits** rather than PRs (permitted for
> pre-existing grants by the guide's caveat). Every task from here on lands as a
> pull request and links the PR number.

## Milestones

Milestones follow the proposal's monthly plan (see the README roadmap):

- **Milestone 1 — Core storage library + HTTP ingest server**
- **Milestone 2 — Query tooling, `obsidianlog init` wizard, cross-platform binaries**
- **Milestone 3 — Reusable GitHub Actions workflow, docs site, live demo**

## Progress report

| Milestone | Task | Pull Request(s) / Commit(s) | Additional Notes |
| --- | --- | --- | --- |
| 1: Core storage + ingest | Workspace scaffold: 4 crates (core/store/ingest/cli), mock-first backends, CI, ADRs, docs | `3ceb7ca` | Pre-PR scaffolding (commit-linked per caveat). Pipeline is stubbed with `todo!()`; not yet functional. |
| 1: Core storage + ingest | Commit-convention docs — add `core` scope | `132c0c3` | |
| 1: Core storage + ingest | _zstd compression (`compression::{compress,decompress}`)_ | _pending_ | |
| 1: Core storage + ingest | _AES-256-GCM encryption + deterministic nonces_ | _pending_ | Implements ADR-0002. |
| 1: Core storage + ingest | _SHA-256 per-service hash chaining + manifest_ | _pending_ | Implements ADR-0003. |
| 1: Core storage + ingest | _`LocalBackend` (filesystem) — unblocks Sia-free tests_ | _pending_ | Makes `tests/pipeline.rs` runnable. |
| 1: Core storage + ingest | _HTTP ingest server (`/ingest`, `/health`)_ | _pending_ | |

## How to update

When a task's PR merges (or, for early work, a commit lands):

1. Add a row: the milestone, the task, a link to the PR (`#123`) or commit (short
   SHA), and any notes (difficulties, partial completion, follow-ups).
2. Remove the leading `_italics_`/`_pending_` once a planned task is done.

At month end, the completed rows for that month's milestone are the report
submitted to the Foundation.
