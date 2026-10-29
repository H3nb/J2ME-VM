# Parallel development across agent sessions

This document owns multi-session coordination. [AGENTS.md](../AGENTS.md) owns
project-wide authority and constraints; [project-plan.md](project-plan.md) owns
scope and dependencies. These rules apply equally to human and AI contributors.
They enable parallel work within authorization, not automatic delegation or merge.

## Roles and shared records

Hendra opens sessions and sets product priorities. One named coordinator at a
time prepares ready assignments, resolves overlaps and sequences integration.
Each task has one current writer; a reviewer checks primary evidence independently
of the implementation claims. The coordinator may implement a separate task but
must label their review of their own work as self-review. Another session or
model is not inherently an independent or correct review.

Use one GitHub Issue as the canonical assignment for a multi-session task. An
existing PR may serve as the assignment for a small, already authorized change;
do not require a duplicate Issue or retroactively block existing work. The
coordinator records a direct user assignment rather than asking for it again.
Reference these records from the current increment's handoff; do not create
competing task lists in chat, Library, project status and new registry files.

| Record | Authority / owner |
| --- | --- |
| Accepted behavior and design | Owning specification/architecture document and ADR; changes reviewed through a PR |
| Task scope, assignment and dependencies | Canonical Issue or PR, maintained by the coordinator |
| Implementation and evidence | Task branch/PR, maintained by its current writer |
| Integrated progress and next action | `project-status.md`, maintained by the integrator |
| Actual merge, branch head and CI outcome | Live GitHub refs/results; verify dated summaries against them |

Separate sessions have no guaranteed shared memory or automatic notification.
GitHub records are shared evidence, not automatic synchronization. Hendra relays
task links, blockers and changed assignments when a session needs to act; agents
reread the relevant records at the checkpoints below. Never claim another session
was notified without evidence. Distinguish session IDs from a shared GitHub login.

## Assignment and readiness

The coordinator publishes a bounded assignment before concurrent writing:

```text
Task ID and canonical URL:
Goal / work package / observable result:
Current writer session ID; coordinator; reviewer or review still unassigned:
Target branch and starting commit SHA; task branch:
Contract revision (commit + document/section); dependency task/PR links:
Owned responsibilities/files; shared files and designated writer:
Scope exclusions; unresolved assumptions that block this task:
Acceptance evidence / relevant checks; practical experiment bound if needed:
Status; latest handoff; next action:
```

Keep it short and link to owning contracts instead of copying them. A task is
`Ready` only when its scope, writer, required dependencies and acceptance criteria
are sufficient for that work. A research task can be ready with an unresolved
question as its explicit output. Dependent implementation cannot treat the
question as settled. Fixtures use confirmed expected behavior, not guesses from
the implementation being tested.

The coordinator assigns tasks serially; workers do not race to claim an apparently
empty slot. Issue labels and comments are not atomic locks. A worker verifies the
assignment before starting, resuming, publishing or modifying shared contracts.
If ownership is ambiguous, pause writes to the disputed area and ask the
coordinator to reconcile it; continue independent authorized work.

States: `Ready -> In progress -> Review -> Integrated`. Use `Blocked` with the
specific missing evidence/decision and who can resolve it; review can return a
task to `In progress`. `Cancelled` records why and preserves useful work. Research
is integrated when its reviewed output is recorded; a code task is integrated
only after its accepted changes land. A finished chat or green CI is insufficient.

## Boundaries, branches and contract changes

- Use one branch and separate checkout/worktree per task. Do not share a mutable
  working directory, overwrite another worker's branch, or reset their work.
  Separate files do not guarantee independent behavior: trace state ownership,
  errors, data formats and caller/callee assumptions before splitting work.
- Agree only the contract needed for the current increment. For verifier and
  interpreter work, specify accepted input, validated guarantees, error timing
  and frame/PC ownership before dependent implementations diverge. Do not invent
  stable APIs, traits, crates or adapters solely to create more parallel tasks.
- Name a writer for shared wiring, manifests/lockfile, CI, architecture and status
  files when they are touched concurrently. Other tasks propose the necessary
  change in their handoff; no blanket ownership of a whole subsystem forever.
- Prefer dependent tasks branching from main after their prerequisite merges.
  Use stacked branches only for a justified benefit, recording the exact parent
  head and merge order. After a squash merge, reconcile the child with the new
  main and review its effective diff; do not reintroduce the parent's old commits
  or treat the parent PR's checks as validation of the child.
- A contract change names the reason/evidence, affected tasks, revised behavior
  and migration needed. The coordinator identifies which work must pause and
  records the accepted revision. Workers reread it before continuing affected
  work. Do not bridge incompatible assumptions with silent fallbacks or duplicate
  models. Unaffected work may continue; material product changes remain Hendra's.

Start with a coordinator and two bounded worker tasks. Increase concurrency only
when review capacity and independent ready work justify it. Measure time to an
integrated result, rework, regressions and CI cost, not agent/commit/line counts.
Unfinished work beyond review capacity is a reason to finish or narrow tasks,
not launch more. The coordinator role must remain transferable through records.

## Handoff, interruption and reassignment

At a useful stopping point, leave the following in the canonical Issue/PR (within
authorized publication scope), then give Hendra the link:

```text
Task/session ID; status:
Baseline SHA; branch; current head SHA; PR:
Implemented or researched result and its limits:
Contract changes / affected tasks, or none:
Checks actually run: command/environment, tested SHA, result, evidence URL:
Unrun checks and remaining risks:
Blocker and required decision, or next concrete action:
```

If publishing is unavailable, supply this handoff in the final response and state
that it is not yet stored in GitHub. Do not pretend other sessions can read it.
An abandoned or interrupted session does not release ownership automatically.
The coordinator verifies/preserves its branch, records reassignment and gives the
replacement the handoff. A resumed old session must recheck ownership and stop
writing if replaced. The same explicit transfer applies to the coordinator.

## Review and serialized integration

1. The reviewer reads requirements, applicable primary clauses, the full effective
   diff and actual checks against the current target. Apply
   [engineering-quality.md](engineering-quality.md). Verify cross-task behavior,
   not only textual conflicts; record the reviewed base/head SHAs and limitations.
2. Recheck live target/head, prerequisite merges and outstanding findings before
   integration. If either changed since review, examine the intervening diff and
   repeat affected review/checks. Reuse still-applicable evidence with explicit
   provenance; do not rerun unrelated expensive work by reflex.
3. Validate the candidate combined result with the relevant existing checks.
   Separate successful branch checks do not prove compatible integration. Record
   what tree/commit was exercised, including a CI merge candidate if applicable;
   never substitute an old successful run for the current candidate.
4. One integrator merges at a time, within Hendra's authorization. Use an expected
   PR head guard where available. Recheck the target immediately before merging;
   if it changed, reassess the candidate. A rejected update requires rereading refs,
   not a blind retry or force push. GitHub settings may enforce stronger guards;
   this document does not claim they are configured.
5. Verify the merge result, inspect the applicable main CI, and update integrated
   status/dependent assignments. Preserve pending or failed checks explicitly;
   investigate failures before dependent acceptance. Do not declare a whole gate
   passed from a partial PR. Record merged PRs and observed commits without trying
   to embed a document's own future commit SHA. Reconcile adoption/status wording
   at integration or in a named follow-up, not an unowned perpetual stale note.

Independent review is preferred for foundational changes. If unavailable, report
self-review and residual risk honestly; do not fabricate another reviewer or add
an approval loop where existing authorization suffices. CI/skip policy remains
owned by [development.md](development.md); do not weaken checks to speed integration.

## Session launch prompts for Hendra

The coordinator fills in URLs/IDs before handing over these prompts. Hendra only
needs to open the sessions, paste the prompts and relay completion/blocker links.

**Coordinator:**

> Coordinate https://github.com/H3nb/J2ME-VM. Read AGENTS.md, current remote work
> and the project plan. Preserve existing assignments. Select the current
> increment, record ready tasks/dependencies and prepare complete worker prompts
> using docs/parallel-development.md. Distinguish work ready now from blocked
> work. Prepare integration recommendations; merge only within my authorization.

**Worker:**

> Work as session [ID] on [canonical task URL] in H3nb/J2ME-VM. Verify ownership,
> current refs and the linked contract before writing. Use the assigned branch
> and separate workspace. Complete the bounded task, relevant checks and PR
> within the assignment's authorization. Do not take other tasks, change shared
> contracts unilaterally or merge. Record evidence/blockers and a resumable handoff.

**Reviewer:**

> Independently review [PR URL] against the current target and [task URL]. Read
> AGENTS.md and the owning contracts. Check the entire effective diff and primary
> evidence, including interaction with [related tasks/PRs]. Separate confirmed
> defects from suggestions and existing debt; report reviewed SHAs and unverified
> claims. Do not modify the author's branch or merge without an assigned mandate.

**Replacement:**

> You replace session [old ID] as [new ID] for [task URL], as recorded by the
> coordinator. Verify that transfer and the latest handoff/refs. Preserve and
> continue existing work rather than restarting it; follow the worker procedure.
