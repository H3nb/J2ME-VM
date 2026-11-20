# Development across machines and agents

Use the public J2ME-VM repository as the upstream for accepted code, Issues,
milestones and PRs. Start from the current GitHub Issue and fetch the upstream
before selecting a base. The [roadmap](project-plan.md) owns scope and priority.
[AGENTS.md](../AGENTS.md) owns authority and clean-room boundaries.

## One handoff per task

1. Choose the next ready checklist item. Record the writer and the intended
   outcome in the owning Issue when coordination is needed.
2. Use one short-lived branch from the current authorized base. An isolated
   worktree is useful for genuinely parallel writers or to preserve unrelated
   local changes. A single writer owns each branch and shared runtime contract.
3. Review the effective diff against the current base. Run the smallest checks
   that prove the behavior and the applicable integration checks. Link a
   permitted Git ref or draft PR to its Issue before handing work to another
   machine.
4. Record the base and head SHAs, exact reproduction commands, verified
   result, remaining limit and next dependency. A fresh clone should be able
   to recover the work with those inputs. Keep private corpus setup references
   sanitized and stored in the permitted location.
5. After accepted integration, check the resulting main and CI where relevant.
   Remove obsolete branches and worktrees after verifying the merge or
   preserving unique work. Keep dirty or still-assigned work until its owner
   transfers or retires it.

Use the same flow for human contributors and AI agents. Prefer one implementer
for a cohesive change and parallelize work with distinct dependencies, tests or
ownership. A single integrator reviews any shared state, error or snapshot
contract and accepts changes in dependency order.

## Restricted work and recovery

Follow the [licensing policy](licensing-and-provenance.md) for experimental
implementation, restricted specifications, hosted execution and publication.
If a change cannot enter upstream, record a sanitized status and blocker in the
Issue. Preserve the exact commits through an approved access-controlled
handoff or a verified Git bundle, with a documented restore command and
expected ref. Keep unpublished work recoverable before pruning a branch
or removing a checkout. Store protected corpus data independently.

Keep product decisions in the repository's owning documents and tracked work
in GitHub. Use chat for discussion and agent coordination. Resolve ownership
and dependencies from the current Issue, ref and PR before editing.
