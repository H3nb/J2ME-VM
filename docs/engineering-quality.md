# Engineering quality — proportional, evidence-based review

This document owns focused quality review for human- and AI-authored work. Apply [AGENTS.md](../AGENTS.md), the task's actual contract, and [testing policy](testing.md). Judge **observable correctness and total maintenance cost**, not code volume, test count, how much reasoning was narrated, or whether prose looks AI-generated.

## Risk sets the depth

| Risk | Examples | Proportionate evidence |
| --- | --- | --- |
| Low | Names, isolated documentation, local simplification without behavior change | Inspect affected content/diff; focused checks if applicable |
| Medium | Supported opcode, parser behavior, bounded optimization, API contract | Trace relevant semantics, meaningful positive/negative checks, existing Rust CI |
| High | Untrusted-input boundary, CLDC verification, object identity, class initialization, concurrency, snapshot/restore, legal or clean-room rules | Inspect primary contract and ownership, bounded adversarial/restore evidence where applicable, independent review if practical |

Risk is determined by consequences, **not diff size**. High-risk cases justify careful analysis; rare hypotheticals without a credible trigger do not justify prolonged investigation. Do not bypass input-validation, state-integrity or license boundaries merely to be faster.

## Before and during implementation

Establish the behavior, relevant code path, owner/lifecycle, failure modes and applicable version-specific contract. Read **only needed** sources. Compare alternatives when there is a real architectural trade-off; do not create comparison matrices or research documents for minor changes.

Prefer direct, complete solutions over:
- speculative traits, crates, caches, flags, adapters or fallback paths without a current consumer;
- duplicated state or logic created by competing agents;
- errors silently swallowed, invented defaults or successful no-op placeholders;
- stacked workarounds instead of correcting the owner;
- comments that restate code, copied instructions or status diaries.

Some duplication or defensive checks are justified. Keep them when they protect a real invariant or make the code easier to maintain. Never simplify away essential hostile-input validation, explicit unsupported errors, or atomic restore semantics.

Correctness and test expectations come from applicable specifications and independently reproduced behavior, **not from prior AI output or green tests alone**. Test observable contracts and plausible wrong implementations; remove redundant or brittle tests when justified. Do not add exhaustive combinatorics, mocks of internals, new CI jobs or benchmarks without a distinct risk.

## Final review and stopping rule

Inspect the **effective final change**, including deleted code, replacement of old behavior, shared ownership, regression paths and relevant checks. Stop when the authorized behavior is implemented, necessary invariants hold, and a credible set of checks passes or the remaining gap is accurately reported.

Classify findings as:
- **Blocking:** demonstrated incorrect behavior, security/rights violation, broken required contract or essential verification gap.
- **Nonblocking:** worthwhile simplification/maintenance issue with no demonstrated violation, suitable for a bounded follow-up.
- **Unproven:** hypothesis requiring evidence; do not present it as a confirmed bug.

Give concrete reproduction/contract/effect for blockers. Fix introduced defects before acceptance. Defer unrelated debt only when its impact justifies tracking. Do not produce findings for quota, create an extra approval ritual, or rerun unaffected costly checks after every minor edit.

If repeated patches increase complexity, consolidate superseded paths at the relevant boundary. If an experiment contradicts assumptions, correct the contract rather than add silent fallbacks. A small, verified result or an honest negative finding is acceptable.

## Documentation and reports

Update one owning document when accepted behavior or a consequential design decision changes. ADRs record durable choices; research notes explicitly remain proposals until accepted. GitHub PRs, issues, commits and CI own historical activity. Give concise result, rationale, exercised checks and material limitations; avoid duplicated prose and unsupported production-readiness/conformance claims. Parallel-session rules live in [parallel development](parallel-development.md).
