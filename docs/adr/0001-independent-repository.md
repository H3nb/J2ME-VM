# ADR-0001: Independent Rust VM repository

- **Date:** 2026-10-29
- **Status:** Accepted for experimentation; integration remains conditional

## Context

An optional VM with explicit ownership of frames, heap, virtual time and threads could eventually enable state capture that a host-managed runtime does not generally expose. This is a hypothesis, not an existing feature.

## Decision

Develop `j2me-vm` as an independent, host-neutral Rust library in a **separate repository**. Start with one crate and desktop checks, not a speculative workspace. Do not modify or replace the existing JL-Mod Plus engine. Treat future Android/JNI embedding as a separate design and licensing decision, contingent on cross-process restore evidence and maintenance cost.

## Consequences

Independent CI/history, an explicit host boundary and no Android dependencies are benefits. A second runtime would incur substantial compatibility and integration effort; no commitment to ship is implied. Engineering rules and clean-room provenance are defined in [AGENTS.md](../../AGENTS.md) and [ADR-0002](0002-gplv3-only-and-clean-room.md).

## Alternatives

Long-lived development branch inside JL-Mod Plus (rejected: coupling and merge complexity); immediate replacement of existing engine (rejected: regression risk); multi-crate workspace from day one (deferred until actual dependency boundaries justify it).
