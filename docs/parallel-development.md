# Autonomous work and parallel-session coordination

This document owns **task selection, continuity and multi-session coordination**. [AGENTS.md](../AGENTS.md) owns engineering boundaries; [project plan](project-plan.md) owns dependencies; [development](development.md) owns CI and merge checks. The default is **one capable implementer**, not a standing hierarchy of coordinators.

## Default: one bounded engineering loop

1. **Resume:** inspect current `main`, open PRs/issues, and the short [project status](project-status.md). Continue an existing authorized task before starting a duplicate. Prefer a ready item that advances executable VM capability or removes a real blocker.
2. **Define:** record the observable outcome, essential contract/dependency, scope exclusions and the smallest credible verification. A GitHub Issue is useful for multi-session or nontrivial work; an existing PR or a direct, clear assignment is enough for small work. No mandatory planning document.
3. **Implement:** work on a task branch, choose routine technical solutions autonomously, make focused changes and validate incrementally. Avoid parallel work on shared state ownership/contracts unless coordination is explicit.
4. **Accept:** inspect the complete effective diff and relevant checks. Resolve introduced correctness, safety and contract failures. Prepare a PR with a short result, tests actually run, and limitations; do not create an extra PR solely for routine coordination or evidence.
5. **Continue:** after **authorized** integration, check the resulting `main` and CI once where relevant. Update the owning status/contract only when materially changed, then choose the next ready task. An external runner/scheduler is necessary for unattended recurrence; this document does not by itself execute jobs.

Do not optimize for the number of sessions, commits, tests, or PRs. One substantive result is better than repeated documentation-only handoffs.

## Prioritization and blocked work

Prefer work by dependency readiness, value toward a runnable/recoverable VM, risk reduction and implementation cost. Useful preparatory refactoring is allowed when it removes a demonstrated obstacle; avoid speculative infrastructure.

Distinguish:
- **Local blocker:** one behavior lacks a spec, legal basis, ownership decision or reliable test; record it and pursue independent authorized work.
- **Global blocker:** every useful next increment genuinely depends on the unresolved matter; report the concrete missing decision/evidence instead of looping.
- **Exploratory result:** a bounded local experiment may disprove a design. Record only the conclusion and reproducible evidence needed for subsequent decisions; do not publish restricted implementation material.

For foundational JVM/CLDC behavior, missing normative evidence or unresolved implementation-use rights cannot be solved by assumption or renamed as a harmless edge case. Relevant restrictions in [licensing](licensing-and-provenance.md) remain in force.

Repeated failed attempts without new evidence trigger a smaller experiment, a revised design, or a recorded blocker—not endless retries. Do not chase extremely unlikely scenarios unless they threaten correctness, input security, recovery integrity or a documented compatibility requirement.

## When parallel agents are useful

Use parallel sessions **only** for independent, ready tasks whose benefit outweighs review and integration cost. Start small. Assign a distinct current writer for each task and a single integrator for shared changes. Coordination uses the existing GitHub Issue/PR; do not invent another task registry.

A bounded parallel assignment needs only:
- Public task/Issue or PR, expected result and explicit authorizations.
- Starting base/branch, dependent contract and owned boundaries (including shared files).
- Verifiable acceptance criteria, unresolved assumptions and current writer.
- Integrator/reviewer when relevant, and next handoff.

Each writer uses an isolated branch/worktree; never share a mutable checkout, overwrite another writer's work, or silently change a shared contract. Separate files may still share semantics (state ownership, errors, formats, caller/callee assumptions). Resolve those dependencies before concurrent implementation. The maintainer need not approve routine steps already authorized.

If a writer is interrupted or replaced, preserve the branch and transfer responsibility explicitly through the canonical Issue/PR. Old sessions must recheck ownership before writing. A chat session or GitHub login is **not** an ownership lock.

## Review, integration and handoff

Review against the current target, actual contracts, final diff, and relevant executable evidence—not the implementer's claims or historical commit count. An independent reviewer is valuable for high-risk state, verification, security and snapshot changes, but **not mandatory for every small PR**. Label self-review honestly; resolve blockers with evidence, not votes among agents. See [engineering quality](engineering-quality.md).

One integrator acts at a time. Confirm dependencies, up-to-date target/head, permissions and required checks before a permitted merge. If the base changed, reassess the affected diff and checks instead of reflexively rerunning everything. After a squash merge, downstream branches must reconcile against the accepted main tree. Never force-push another worker's branch or infer merge authority from a task assignment.

Handoff in the existing Issue/PR (or final report when publication is unavailable): **result, current head, checks and limitations, blocker, next action**. Use links for evidence rather than copying long logs. Update [project status](project-status.md) only for material integrated capability, important blockers or priorities. Do not copy private session identifiers/paths into public records.
