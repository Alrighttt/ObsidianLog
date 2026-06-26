# 0001 — Record architecture decisions

- Status: Accepted
- Date: 2026-06-23

## Context

ObsidianLog is a multi-month, multi-crate project with a public grant and
monthly progress reports. Decisions about storage layout, cryptography, and the
Sia/indexd integration need to be discoverable and justifiable later — both for
contributors and for the Sia Foundation reporting.

## Decision

We record significant architectural decisions as ADRs in `docs/adr`, using the
lightweight Nygard format. Each ADR is immutable once accepted; changes are made
by adding a new ADR that supersedes the old one.

## Consequences

- New contributors can read the "why" behind the structure, not just the code.
- The Month 1 "Architecture Decision Record documenting finalized storage
  decisions" deliverable has a concrete home and format.
- A small amount of process overhead per significant decision.
