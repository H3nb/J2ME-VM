# Experiment plan and acceptance gates

**Rule:** Every milestone needs executable evidence and negative tests. Do not expand scope until its core feasibility question has been answered. If fundamentally unworkable, write a short conclusion and stop rather than stacking workarounds.

| Gate | Experiment | Evidence required | Not proven by this gate |
| --- | --- | --- | --- |
| G0 — design & toolchain | Document VM/host boundary; build empty Rust lib on desktop | `cargo fmt`, `clippy`, `test` pass; specs and ownership explicit | VM functionality |
| G1 — class input | Parse original `.class` fixtures and bounded malformed inputs, reject invalid formats; select explicit opcode subset | Provenance-recorded original fixtures + malformed/truncated/overflow cases with precise errors | General CLDC compliance |
| G2 — execution | Execute deterministic arithmetic, branching, method calls, return, exceptions on owned frames | Compare program outputs with a trusted compatible Java runtime where appropriate; document spec differences | Save states |
| G3 — single-thread checkpoint | Pause in a nontrivial call chain, save locals/stack/PC + heap, terminate process, restore in **new process**, continue | Two independent executions have matching final VM-visible values; invalid snapshots are rejected | Multithreaded game save state |
| G4 — virtual concurrency | Add virtual threads, instruction-budget scheduling, monitor enter/exit, wait/notify, sleep, interrupt | Busy-loop fairness, blocked monitor, timeout and snapshot-while-blocked tests; no unresolved state leaks | All concurrency corner cases |
| G5 — runtime boundaries | Minimal CLDC classes and synthetic host calls; explicit persistent/ephemeral resource policies | Checkpoint with pending host event, virtual clock, and deliberate unsnapshotable resource; deterministic/reject behavior | Real MIDP compatibility |
| G6 — MIDP pilot | Implement minimum MIDlet lifecycle and selected UI input/event contracts; run an original, source-available minimal MIDlet compiled against appropriate API declarations | Cold launch, pause/resume, fresh-process restore, resource accounting and documented unsupported APIs | Broad J2ME game support |
| G7 — decision | Benchmark original test JARs plus rights-cleared representative private MIDlets and snapshot/restore overhead | Compatibility matrix, performance/memory figures, maintenance estimate, go/no-go decision; no proprietary files in public CI | Automatic Android release |

## Required invariants for G3

1. Restored PCs refer to the correct validated bytecode and operand boundaries.
2. Object IDs, Java identity, static fields, and reference graph are preserved after restore.
3. New VM process never relies on a stale process pointer or system handle.
4. Snapshot contents are versioned, size-bounded, integrity-checked and incompatible inputs fail safely.
5. A paused execution and a restored execution given the same **controlled input events** have equivalent observable VM results.
6. A test must prove the old process ended before the new process started.

## Evidence expectations

Capture exact commands, toolchain versions, fixture provenance, observed outputs, and failures. A successful `cargo test` with zero behavior tests is **not** evidence that a VM exists. Compare against CLDC requirements, not only a modern JVM.

## Stop / reassess signals

- Core state cannot be reconstructed without native stack/OS resources for plausible Java ME workloads.
- Required scheduler semantics make snapshots unsafe or impossible within target memory/latency budgets.
- Host API resource model forces unsafe or undefined reconstruction of external state.
- Compatibility/performance costs negate the value compared with the existing JL-Mod Plus engine.

## Integration rule

No branch or code modification in JL-Mod Plus until G3 has been independently reproduced and the relevant architecture/maintenance costs reviewed. Android integration is a separate decision after proof of feasibility.

## G1 execution record

A separate `feature/g1-classfile-parser` branch adds an original compiler-produced 48.0 class-file fixture, a byte reader, typed constant-pool references, Code attribute structure reading, and format rejection tests. It intentionally leaves verifier semantics for the next reviewed scope. **Do not mark this gate complete before GitHub Actions has actually executed the new tests successfully.** The 16 MiB input cap is a host security policy, not a claim about the JVM specification. Source/fixture provenance is documented in `tests/fixtures/README.md`.
