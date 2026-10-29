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

W6 post-merge [CI #40880712729](https://github.com/H3nb/J2ME-VM/actions/runs/40880712729) and W7 post-merge [CI #40881324313](https://github.com/H3nb/J2ME-VM/actions/runs/40881324313) succeeded. W7's final merged state passed format, Clippy and **29 Rust tests**. Original integration used coordinator **self-review**. The subsequent separate Luna xHigh audit is now recorded in [Issue #24](https://github.com/H3nb/J2ME-VM/issues/24); it found a preparation resource issue tracked in [#28](https://github.com/H3nb/J2ME-VM/issues/28), with a proposed fix in [PR #29](https://github.com/H3nb/J2ME-VM/pull/29), not yet merged.

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

## Current coordinator handoff

Hendra assigned coordinator chat `01a187f4-7a2b-7383-ad54-4302bc79d1ff`
continued maintenance/development and Luna xHigh helpers. No competing active
assignment or open PR existed at dispatch. Three read-only workers reported
through the coordinator; only the coordinator wrote/published shared files.

- **Baseline main:** `37e32cd179223378fa6f271fd48fc9b33622a394`, clean and equal
  to the observed remote. [Exact-head CI #40883600344](https://github.com/H3nb/J2ME-VM/actions/runs/40883600344)
  passed. Local Windows MSVC, Rust/Cargo 1.99.1: fmt, Clippy and 29 tests passed.
- **Audit #24 completed:** confirmed repeated full-method-table scans during
  preparation. [PR #29](https://github.com/H3nb/J2ME-VM/pull/29), head
  `bbca3ee4f68ab80d7884511d3846337bcbc7ef1e`, replaces them with one temporary
  signature index. The coordinator measured identical admitted input at
  5,926 ms before / 51 ms after; the single-call control costs <1 ms / 13 ms.
  The separate auditor reviewed the entire exact-head diff, reproduced the
  input hashes and ran fmt/Clippy/30 tests. [Head CI #40894710139](https://github.com/H3nb/J2ME-VM/actions/runs/40894710139)
  passed. **Review, not integrated**; #28 remains open. No general speed or
  memory-use claim follows.
- **W8 #25 / W9 #26 research:** [reference/exception prerequisites](wp02-reference-exception-research.md)
  and [original source-only fixture evidence](../tests/fixtures/reference-exception-w9.md)
  are prepared for review. Coordinator independently rebuilt the diagnostic
  48.0 outputs, checked hashes and zero results on JDK 17, and checked a
  deliberately broken null-return variant. No binary is newly checked in and
  neither candidate runs on this VM. These tasks are not a heap or `athrow`
  implementation authorization. Their live issues/PR carry final review state.
- **Standards/rights:** final CLDC N01 remains open in #12. Historical JVMS
  implementation terms also need a recorded applicable basis in
  [#30](https://github.com/H3nb/J2ME-VM/issues/30); see
  [licensing and provenance](licensing-and-provenance.md). Research does not
  settle those terms or establish certification.

**Next action:** review/integrate the focused #29 fix only within merge authority,
then reconcile this handoff with actual main/CI. Complete review of W8/W9 evidence
and resolve the affected rights/specification gaps before a bounded bootstrap
class/type/initialization contract and reference implementation. Preserve the
future caller call-site PC requirement for exception propagation. G2 is still
partial; G3 snapshot remains a later gate. Do not start a speculative heap,
general loader or null-only framework from these research notes.

No JAR runtime, MIDP stack, general class loader, checkpoint/restore,
rewind, Android/JL-Mod Plus integration, or compatibility certification exists.
