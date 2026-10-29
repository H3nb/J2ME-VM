# ADR-0004: Staged delivery around recoverable execution state

- **Date:** 2026-10-29
- **Status:** Proposed; effective when this planning baseline is accepted

## Context

The project needs a durable development direction for save/load, rewind,
debugging and eventual JIT, with Android hosting undecided. Unbounded API work
can postpone discovering an unrecoverable host boundary; speculative frameworks
can consume effort before a useful result. Existing G0–G7 gates already provide
a foundation and must not be replaced or mistaken for passed milestones.

## Decision

- Use the requirements, bounded packages and continuity/change-control process
  in [project-plan.md](../project-plan.md). Keep evidence and the next unfinished
  dependency in [project-status.md](../project-status.md).
- Treat recoverable logical state, consistent safepoints, explicit host effects
  and bytecode-level inspection as architectural constraints. Concrete layouts,
  GC, codecs and JIT backends remain experimental choices.
- Preserve G0–G7 identifiers. Add G3H for early host-boundary feasibility and
  G6R for rewind. Neither is satisfied by documentation or core-only restore.
- Keep interpreter execution as the initial baseline. Future JIT must preserve
  the same semantics and reconstruct logical state at supported safepoints.
- Make each increment's requirements ready before implementation; do not wait
  for a complete emulator design. Do not restart a project without evidence,
  consideration of repair/migration and the product owner's direction.

## Consequences

Small, useful demonstrations become prerequisites for larger commitments.
Host and storage risks are exposed before broad MIDP/JSR work. Snapshot/replay
support must be stated per resource/profile. Extra state bookkeeping and
checkpoint latency require measurement; no performance or schedule guarantee
is made. Existing licensing, clean-room and authorization rules still apply.

## Alternatives

- Implement a broad emulator and add recovery afterward: rejected because
  hidden host state may invalidate the core objective.
- Freeze every component before coding: rejected because recovery and
  performance need executable evidence, not exhaustive paper design.
- Build JIT, plugin framework or multi-crate workspace now: deferred until a
  measured requirement justifies their maintenance cost.
- Require a new permission prompt for every gate: rejected; honor existing
  authorization while requiring evidence for capability claims and escalation
  for material direction changes.
