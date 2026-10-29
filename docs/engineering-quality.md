# Preventing code quality erosion and AI slop

This document owns focused quality review for authored and AI-assisted work.
It supplements [AGENTS.md](../AGENTS.md), without replacing the specification,
[testing policy](testing.md) or [parallel workflow](parallel-development.md).
Judge behavior and maintenance cost, not whether text looks AI-generated.

## Concrete rules

| Failure pattern | Required response |
| --- | --- |
| Repeated patches bypass the same unclear owner or lifecycle | Trace the state and fix the responsible mechanism; consolidate superseded paths within scope. Explain any compatibility path that must remain. |
| New wrapper, trait, flag, cache, dependency or framework without a current consumer/need | Prefer existing mechanisms or direct code. Introduce the abstraction only for a demonstrated requirement, with its cost and invariant explained. |
| Several representations of the same state or duplicated behavior across worker PRs | Establish one owner/contract and reconcile the combined implementation before acceptance. Do not paper over disagreement with adapters. |
| Broad error swallowing, fabricated defaults, retries or successful no-ops | Preserve explicit failure/unsupported behavior and diagnostic context. Recovery needs a defined failure case, bounded policy and meaningful test. |
| Placeholder implementations or speculative future APIs in a claimed complete feature | Complete the authorized slice or exclude unsupported behavior explicitly. Do not merge unreachable stubs merely to reserve architecture. |
| Defensive checks without an identified boundary/invariant | Explain their purpose or simplify. Preserve necessary hostile-input validation, bounds checks, atomic restore and runtime safety; minimal code is not the objective. |
| Tests mirror implementation, assert private layout, or only count calls | Test the observable contract and credible incorrect behavior at a faithful layer; reuse existing coverage. See testing.md for exceptions and fixture provenance. |
| Tests/expectations/lints changed merely to make CI green | Establish why the old expectation is wrong from primary evidence before changing it. No blanket suppression, hidden failure or weakening of acceptance criteria. |
| Large unrelated cleanup, naming churn or whole-file formatting obscures a fix | Keep the effective diff reviewable. Separate necessary preparatory refactoring from unrelated debt; preserve other workers' changes. |
| Verbose comments, copied checklists and duplicated design prose drift apart | Keep one owning contract. Explain non-obvious reasons, invariants and tradeoffs; remove commentary that only restates the code. |
| Confident claims of performance, compatibility, security or completion without evidence | State the exact scope, source/experiment, observed result and limits. Mark hypotheses and unrun checks explicitly. |

These are review criteria, not a ban on all duplication, guards or abstractions.
Small intentional duplication can be clearer than an unstable shared abstraction;
retain it when justified. A change should reduce total conceptual complexity while
meeting the full contract. Do not replace sound explicit code with clever generic
code, or redesign a subsystem just because an agent prefers a different style.

## Evidence before implementation and acceptance

Before a material change, identify the need, owning state, applicable contract,
failure modes and existing mechanism. AI output, previous-agent summaries and
another model's agreement are not primary authority. Verify claims against the
source/specification and reproducible evidence. All sessions retain the project's
clean-room constraints; parallelization does not permit studying other VM internals.

Review the final combined behavior, including deletions and superseded code, not
only added lines or individually green PRs. Use these questions proportionately:

- Does each new path serve the assigned behavior with clear state/lifetime ownership?
- Are errors, cleanup and relevant recovery/debug implications coherent across callers?
- Did this change leave duplicate mechanisms, stale fallbacks, temporary diagnostics,
  dead code or comments that no longer match behavior?
- Do tests catch meaningful wrong behavior without assuming the implementation is right?
- Are claims tied to inspected clauses or actual runs with the right base/head?

A reviewer reports confirmed defects with location, triggering condition,
contract/invariant, consequence and available reproduction. Separate introduced
defects, existing out-of-scope debt, unresolved questions and optional suggestions.
No quota of findings, tests or comments; "no confirmed findings" is valid. Different
agent sessions can share the same mistaken assumption, so disagreement is resolved
with evidence rather than votes or the number of reviewers.

Record each substantive finding's disposition: fixed with evidence, rejected
with rationale/evidence, or deferred with an owning task and reason. Confirmed
violations of the agreed behavior, safety invariants or required acceptance
evidence block the affected change until corrected or the authorized scope is
explicitly revised. A deferral label or resolved review thread alone does not
clear such a blocker. Unanswered questions block only the behavior that depends
on them; stylistic preferences and unrelated debt do not become implicit merge
conditions. Verify the fix at the revised head before closing a blocker.

## Keeping quality from eroding over increments

Fix relevant introduced defects before acceptance and remove mechanisms made
obsolete by that fix. A temporary workaround needs the concrete reason it cannot
yet be removed, bounded impact, an owning task and a removal condition. "For future
proofing" or an unowned TODO is insufficient. Do not add a fallback solely to
hide an unresolved contract disagreement between parallel tasks.

Record unrelated debt only when evidence establishes a real impact; link a
follow-up with owner/priority or an explicit deferral reason. Avoid both silently
discarding findings and expanding every task into a cleanup project. At each
increment boundary, the integrator checks touched boundaries for duplicated
ownership, accumulated workarounds and stale contracts. This is a focused part of
integration, not a mandatory whole-repository rewrite/audit after every PR.

Repeated regressions or rising rework trigger a narrower next slice, corrected
contract or justified simplification before more features are layered on top.
Use observed integration delay, rework, regressions and measured resource cost;
do not optimize lines generated, coverage percentages or number of agents.

## Documentation and reports

Lead with the concrete result and behavior. Include rationale, verification and
material limitations in proportion to the change. Use plain language and stable
project terms; avoid filler, invented jargon, unearned superlatives and claiming
"production-ready", "future-proof" or "fully compatible" from a small proof.
Do not copy chat transcripts into permanent policy or inflate checklists to look
thorough. Update the owning document and link evidence instead of duplicating it.
An honest bounded result is useful; unsupported certainty is not.
