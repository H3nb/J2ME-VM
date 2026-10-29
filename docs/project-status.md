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
| W6 original invocation fixtures | [PR #20](https://github.com/H3nb/J2ME-VM/pull/20), squash `12b91eba` | Original Java 1.4 class-file 48.0 nested/recursive static call fixtures; provenance and negative mutation recipes. Not CLDC-preverified. |
| W7 VM-owned static call frames | [PR #23](https://github.com/H3nb/J2ME-VM/pull/23), squash `bc371f1` | Bounded same-class descriptor-aware `invokestatic` and `ireturn`, explicit nested frames, 64-frame and 64-method limits, subset-level access checks; **partial G2 only**. |

After W3 squash `bc1c60a`, a coordinator candidate applied exactly the
W4 source/test blobs. [Combined CI #40817525937](https://github.com/H3nb/J2ME-VM/actions/runs/40817525937)
passed and the resulting [W4 squash `263b6326`](https://github.com/H3nb/J2ME-VM/commit/263b6326b5acc9504d6405615f72402e2155c998)
has **identical Git tree `1eb592e1d4f435a414dfdeab3852d9792a6c7c9d`**.
[Post-merge CI #40817648432](https://github.com/H3nb/J2ME-VM/actions/runs/40817648432)
passed format, Clippy and **22 tests**. Coordinator reviewed both PRs, but
no second independent review session reviewed the coordinator's own
integration decisions.

W6 post-merge [CI #40880712729](https://github.com/H3nb/J2ME-VM/actions/runs/40880712729) and W7 post-merge [CI #40881324313](https://github.com/H3nb/J2ME-VM/actions/runs/40881324313) succeeded. W7's final merged state passed format, Clippy and **29 Rust tests**. Review of W7 was a coordinator **self-review**, not a separately authored independent review; the outstanding post-merge review is tracked in [Issue #24](https://github.com/H3nb/J2ME-VM/issues/24).

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

**Next parallel handoffs:** [#24](https://github.com/H3nb/J2ME-VM/issues/24) is an *independent post-merge W7 audit* (not yet completed); [W8 #25](https://github.com/H3nb/J2ME-VM/issues/25) owns the **standards/execution contract research** for minimum object-reference identity and prerequisites for exception propagation; [W9 #26](https://github.com/H3nb/J2ME-VM/issues/26) owns **independent original Java 1.4 fixture/test design**. These are research/fixture tasks, **not an authorization to introduce an object heap or `athrow` implementation** before the required constructor, resolution, type and handler contracts are demonstrated. Do not turn W7's `Vec<i32>` stack into a speculative object framework. Final CLDC N01 uncertainties remain in [#12](https://github.com/H3nb/J2ME-VM/issues/12); G3 snapshot remains a separate later gate.

No JAR runtime, MIDP stack, general class loader, checkpoint/restore,
rewind, Android/JL-Mod Plus integration, or compatibility certification exists.
