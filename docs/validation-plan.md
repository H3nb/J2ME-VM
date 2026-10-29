# Experiment plan and acceptance gates

**Rule:** Every milestone needs executable evidence and negative tests. Do not expand scope until its core feasibility question has been answered. If fundamentally unworkable, write a short conclusion and stop rather than stacking workarounds.

The [project plan](project-plan.md) owns requirements, work packages and change
control; [project status](project-status.md) owns the dated handoff. A failed
experiment first calls for rework or a narrower scope. A recommendation to stop
the project requires a demonstrated fundamental conflict and a preserved
resumption record; it is not an automatic consequence of a failed test.
G0–G7 identifiers are retained. G3H and G6R below are additional required proofs
for the proposed recovery-focused baseline, not renumbered existing gates.

| Gate | Experiment | Evidence required | Not proven by this gate |
| --- | --- | --- | --- |
| G0 — design & toolchain | Document VM/host boundary; build empty Rust lib on desktop | `cargo fmt`, `clippy`, `test` pass; specs and ownership explicit | VM functionality |
| G1 — class input | Parse original `.class` fixtures and bounded malformed inputs, reject invalid formats; select explicit opcode subset | Provenance-recorded original fixtures + malformed/truncated/overflow cases with precise errors | General CLDC compliance |
| G2 — execution | Execute a validated, explicit subset of arithmetic, branching, method calls, return and exceptions on owned frames; bounded step/inspect and breakpoint/exception stops | Clause-driven original fixtures; frame/PC inspection; reject unsupported or invalid execution; compatible Java runtime comparison only where appropriate | General CLDC verification or save states |
| G3 — single-thread checkpoint | Pause in a nontrivial call chain, save locals/stack/PC + heap, terminate process, restore in **new process**, continue | Two independent executions have matching final VM-visible values; invalid snapshots are rejected | Multithreaded game save state |
| G3H — early host feasibility | Suspend on a synthetic host request, checkpoint and restore its continuation; declare one unsupported resource | Controlled completion resumes once; stale/duplicate callbacks are rejected; unrecoverable checkpoint reports a reason; no native continuation leaks | Production media/network recovery |
| G4 — virtual concurrency | Add virtual threads, instruction-budget scheduling, monitor enter/exit, wait/notify, sleep, interrupt | Busy-loop fairness, blocked monitor, timeout and snapshot-while-blocked tests; no unresolved state leaks | All concurrency corner cases |
| G5 — runtime boundaries | Integrate necessary CLDC services with G4 threads and the G3H host lifecycle; explicit persistent/ephemeral resource policies | Concurrent blocked guest calls, virtual-clock completions, runtime roots, cancellation/cleanup and unsupported-resource rejection remain correct across checkpoint | Real MIDP compatibility |
| G6 — MIDP pilot | Original offline MIDlet/JAD, minimum lifecycle, selected UI input, mutable graphics and session-managed RMS | Cold launch, lifecycle pause/resume distinct from VM checkpoint, fresh-process session restore, storage consistency and resource accounting; explicit API exclusions | Broad J2ME game support or all media fidelity |
| G6R — rewind pilot | Bounded checkpoint history and controlled event replay; branch after rewinding | Matching state/observable trace, no duplicate external writes, coherent RMS branch, stale-event rejection and measured memory/latency | Live remote-server rollback or universal cross-platform replay |
| G7 — decision | Benchmark original JARs and lawful representative MIDlets; evaluate recovery/rewind overhead and future host options | Agreed target workloads/device budgets, compatibility matrix, performance/memory figures, maintenance estimate and go/rework/pause recommendation; no proprietary files in public CI | Automatic Android release or JIT approval |

## Required invariants for G3

1. Restored PCs refer to the correct validated bytecode and operand boundaries.
2. Object IDs, Java identity, static fields, and reference graph are preserved after restore.
3. New VM process never relies on a stale process pointer or system handle.
4. Snapshot contents are versioned, size-bounded, integrity-checked and incompatible inputs fail safely.
5. A paused execution and a restored execution given the same **controlled input events** have equivalent observable VM results.
6. A test must prove the old process ended before the new process started.

## Evidence expectations

Gate entry requires the preceding dependencies in the project plan, an explicit
behavioral scope, normative clauses for that scope, owned state, realistic
failure modes and a bounded experiment. G3H can use synthetic host requests;
it does not require a full CLDC/MIDP library or all G4 thread semantics.

Record pass, needs rework, blocked or not attempted, together with exact source
commit, fixture hashes, environment, commands, observed positive/negative
results, residual risks and reviewer identity. Link the evidence from project
status. Do not mark a gate passed from a PR description or another agent's
summary alone. A partial implementation PR is not a whole-gate pass.

For G3/G3H, compare continuation traces as well as final values and test failure
atomicity. For G6, inject a resource-recreation failure and check both prior VM
and managed storage remain intact; test ordinary RMS persistence separately
from rewind. For G6R, replay from multiple retained checkpoints, branch with
new input and reject late events from the discarded branch. Select meaningful
cases rather than an exhaustive cross-product.

Performance figures are observations until workloads and budgets are agreed.
Use the measurement contract in [project-plan.md](project-plan.md); report
snapshot/replay costs separately from guest execution. No universal speed,
compatibility or cross-platform determinism claim follows from a pilot.

Capture exact commands, toolchain versions, fixture provenance, observed outputs, and failures. A successful `cargo test` with zero behavior tests is **not** evidence that a VM exists. Compare against CLDC requirements, not only a modern JVM.

## Stop / reassess signals

- Core state cannot be reconstructed without native stack/OS resources for plausible Java ME workloads.
- Required scheduler semantics make snapshots unsafe or impossible within target memory/latency budgets.
- Host API resource model forces unsafe or undefined reconstruction of external state.
- Compatibility/performance costs negate the value compared with the existing JL-Mod Plus engine.

## Integration rule

No branch or code modification in JL-Mod Plus until G3 has been independently reproduced and the relevant architecture/maintenance costs reviewed. Android integration is a separate decision after proof of feasibility.
