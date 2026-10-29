# ADR-0005: Legacy MIDP first, with profile-aware execution contexts

- **Date:** 2026-10-29
- **Status:** Product direction agreed; technical contracts proposed for review

## Context

The user prioritizes MIDP 1.0–2.1 compatibility and wants foundations capable of
evolving toward MIDP 3.0. The [profile study](../profile-evolution.md) identifies
different execution/lifecycle contracts and CLDC linking implications. Simply
exposing more APIs or globally isolating every MIDlet risks breaking legacy
behavior. API mirrors/maintenance notes do not close all normative questions.

## Decision

- Deliver the declared legacy scope first; use JSR 271 and CLDC 1.1.1 now to
  identify architectural constraints and deadlines, not to start full MIDP 3.
- Keep suite, MIDlet, context, thread and host process identities distinct.
  Select a runtime contract explicitly and preserve its behavior in recovery.
- Keep mutable class/heap state owned by a context. The runtime's mapping of
  MIDlets to contexts follows the selected, verified contract; it is not fixed
  to one-MIDlet-per-context for every profile.
- Keep resolver failure timing, AMS lifecycle, display state and VM pause
  independently modeled. Host requests retain owner and timeline identity.
- Recovery support depends on a declared consistent set of participating state;
  reject unsupported shared effects rather than silently restore one heap.
- Preserve existing gates and add G6C for legacy stabilization. Track normative
  gaps and minimal experiments in the study; do not invent implementation policy
  where final compatibility clauses remain unread.

## Consequences and alternatives

Small explicit ownership boundaries cost some bookkeeping but reduce later
rewrites. Multiple core instances and synthetic events can test those boundaries
before an AMS exists. Complete MIDP 3, LIBlets, class-based permissions, CDC,
OSGi and native JIT remain separate work. No ABI or cross-version snapshot
guarantee follows.

Reject a global single-game runtime, unconditional MIDP 3 semantics for legacy
applications, immediate full MIDP 3 implementation, and separate engines/repos
per profile. Reuse one core with verified runtime policies; introduce physical
module boundaries only when real dependencies justify them.
