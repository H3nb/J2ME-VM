# Project status and coordinator handoff

**Observed:** 2026-10-29; refresh live GitHub refs, canonical Issues and
Actions before taking another task. The [project plan](project-plan.md),
[parallel workflow](parallel-development.md) and
[quality rules](engineering-quality.md) remain authoritative.

## Integrated WP02 evidence

| Deliverable | Source | Evidence and limitations |
| --- | --- | --- |
| G1 parser and initial int proof | [PR #1](https://github.com/H3nb/J2ME-VM/pull/1), [PR #3](https://github.com/H3nb/J2ME-VM/pull/3) | Structural reading and bounded straight-line stepping, not a CLDC verifier |
| Original Java sources and 48.0 fixture classes | [PR #9](https://github.com/H3nb/J2ME-VM/pull/9), [PR #11](https://github.com/H3nb/J2ME-VM/pull/11) | GPLv3-only, independently authored; legacy CLDC preverification not established |
| W1 and W3 graded CLDC evidence | [PR #8](https://github.com/H3nb/J2ME-VM/pull/8), [PR #15](https://github.com/H3nb/J2ME-VM/pull/15) | Historical typechecker and JCP 2007 evidence; **N01 final clauses and implementation-use terms remain unresolved** |
| W4 local and branch execution subset | [PR #16](https://github.com/H3nb/J2ME-VM/pull/16) | Validated local state, CFG, Java overflow, stepping, bounded and resumable execution; **not full G2** |

After W3 squash `bc1c60a`, a coordinator candidate applied exactly the
W4 source/test blobs. [Combined CI #40817525937](https://github.com/H3nb/J2ME-VM/actions/runs/40817525937)
passed and the resulting [W4 squash `263b6326`](https://github.com/H3nb/J2ME-VM/commit/263b6326b5acc9504d6405615f72402e2155c998)
has **identical Git tree `1eb592e1d4f435a414dfdeab3852d9792a6c7c9d`**.
[Post-merge CI #40817648432](https://github.com/H3nb/J2ME-VM/actions/runs/40817648432)
passed format, Clippy and **22 tests**. Coordinator reviewed both PRs, but
no second independent review session reviewed the coordinator's own
integration decisions.

## Canonical tasks, boundaries and next action

- [W3 Issue #12](https://github.com/H3nb/J2ME-VM/issues/12):
  **research output integrated**, but **N01 final-source verification remains
  blocked**. Keep source-authored 2003 manuscript distinct from authenticated
  JSR final attachments and from 2012 Oracle correction. Do not impose
  2007 proposed resolution timing as proven final.
- [W4 Issue #13](https://github.com/H3nb/J2ME-VM/issues/13):
  **integrated**, with no claim that G2 passes or `StackMap` is implemented.
- Coordinator owns shared status and the
  [execution contract](wp02-execution-contract.md); W3/W4 code/doc ownership
  ended at successful integration. New worker sessions require new canonical
  assignment and separate branches.

**Recommended next work:** define a bounded verified *call-frame/exception
increment* only after corresponding format/type/entry rules are identified
and their fixture coverage is planned. Separately pursue final historical
JSR 139 text and terms. Avoid creating an additional code owner while the
previous integrated contract is under review; G3 snapshot work stays a
separate gate.

No JAR runtime, MIDP stack, general class loader, checkpoint/restore,
rewind, Android/JL-Mod Plus integration, or compatibility certification exists.
