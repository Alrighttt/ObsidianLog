# 0003 — Per-service hash chains

- Status: Accepted
- Date: 2026-06-26

## Context

Tamper-evidence comes from hash-chaining chunks: each chunk records
`prev_hash = SHA-256(previous chunk)`, so any deletion, reorder, or modification
breaks the chain at a detectable position.

A single global chain across all services would force a global write order:
every chunk, regardless of which service produced it, would have to append after
the current global head. That serializes all ingest behind one lock and couples
unrelated services' throughput — a poor fit for a service that ingests many
independent log streams in parallel.

## Decision

We maintain **one independent hash chain per service**. The manifest stores a
map of `service -> chain`, each with its own head and its own `GENESIS` origin
(`Manifest::chains`, `Manifest::head(service)`). Writes are serialized *within* a
service, but services are mutually independent and ingest in parallel.

This pairs with ADR-0002: the per-service partition is also the scope of the
monotonic nonce counter.

## Consequences

- Parallel ingest across services with no global write lock; contention is
  bounded to a single service's chain.
- `verify` walks each service's chain independently and can be scoped to one
  service (`obsidianlog verify --service <name>`).
- Tamper-evidence is per service: a break is localized to and reported within the
  affected service's chain.
- There is no single global ordering across services; cross-service ordering, if
  ever needed, must come from timestamps in the index, not the chain.
