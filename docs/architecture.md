# Technical architecture — proposal v0

**Status:** Working hypothesis, not a completed implementation. Outbound license and clean-room policy are decided in [ADR-0002](adr/0002-gplv3-only-and-clean-room.md); implementation hypotheses remain open. Revisit after each proof-of-concept result.

## 1. The question we must answer

Can a purpose-built Rust VM execute relevant Java ME bytecode while owning **all recoverable VM state**, such that a checkpoint can restore into an entirely new VM instance (ideally another process) and produce equivalent subsequent VM-visible behavior? A reliable *game* save state requires extra control over host effects and resources.

## 2. Boundary of responsibility

```text
+------------------------- optional hosts --------------------------+
| Desktop test harness / future Android host adapter (not yet made) |
| UI, graphics, sound, input, wall clock, file/network, device APIs   |
+-------------------------------+-----------------------------------+
                                | explicit calls + event protocol
+-------------------------------v-----------------------------------+
| CLDC / MIDP runtime libraries (future, independently implemented)  |
+-------------------------------+-----------------------------------+
| Rust VM core                                                       |
| class loading & validation | interpreter | linking/initialization  |
| typed frames & operand stack | logical object identities + heap    |
| virtual threads, monitors & scheduler | exceptions | checkpoints    |
+-------------------------------------------------------------------+
```

A host adapter must not mutate VM internals through arbitrary shared pointers. The VM should use explicit events and host service calls; exactly how to serialize pending host operations is **unresolved**.

## 3. Early target and exclusions

- Target **CLDC 1.1 semantics** as an investigation baseline, with MIDP 2.0 as a later profile; CLDC 1.0 behavior must be evaluated separately (e.g., floating-point restrictions).
- Prove class-file parsing and a **small, explicit opcode subset** first. Do **not** assert support for all Java SE class-file versions. Select accepted versions from CLDC requirements and real fixtures, then reject unsupported versions transparently.
- Verify class-file structure, instructions, constant-pool indexes, code boundaries, and stack/local types before execution. CLDC has its own verification/preverification model; supporting Java SE bytecode validation alone is insufficient.
- Start single-threaded to prove restore, then expand to virtual threads and monitor semantics. Do not market single-thread success as full CLDC threading support.
- No JNI, reflection beyond target API, native JIT, full Java SE library, or manufacturer-specific JSR at this phase.

## 4. Execution state (proposed ownership)

The VM must own, or explicitly reconstruct, the following (enumeration incomplete until proven):

| State | Examples / invariants |
| --- | --- |
| Loaded classes | Class identities, resolved constant-pool entries, initialization status, static fields, code/fixture hashes |
| Threads | Stable virtual IDs, run/block/wait status, current PC, invocation frames, local variables, operand stacks, exception state |
| Heap | Stable logical object IDs, types, arrays/fields, identity/hash invariants, GC roots, allocation bookkeeping |
| Synchronization | Monitor owner, reentrancy count, waiting/entry queues, wakeup deadlines |
| VM services | Virtual time, deterministic PRNG state when owned by VM, queued input/events, permission/context metadata |
| Host boundary | Pending operations represented by resumable descriptors **only where possible**; external resources need explicit rebind/recreate policy |

**Important:** A Rust address, reference, heap index that may be reused, OS thread ID, audio handle, GPU texture ID, file descriptor, socket, or wall-clock reading is **not** a durable snapshot identity.

### Object reference hypothesis

Use stable logical handles rather than serialized raw pointers. Consider an ID plus generation to detect stale handles, but benchmark overhead and correctness first. Object identity and `==` must remain stable over GC and restore. The exact heap and collector design is **not yet selected**.

### Interpreter hypothesis

Explicit VM-owned frames with PCs, typed locals/operand stacks, and Java exception propagation make checkpointing more tractable than borrowing the host's native call stack. Rust integer operations must reproduce Java semantics (overflow, signed shifts, divide-by-zero, conversions); double-slot long/double, float behavior, and type safety need targeted validation.

### Scheduler hypothesis

Use VM-managed virtual threads with **bounded instruction-budget preemption / safepoints**, not a purely voluntary yield mechanism: tight loops can otherwise starve other threads. This is an *investigation*, not a claim of matching every JVM scheduling detail. Monitors, memory visibility, `wait`/`notify`, sleeps, interrupts and class initialization locks need explicit conformance tests. Snapshot can occur only at a globally consistent VM-owned safepoint with host callbacks quiesced or representable.

## 5. Snapshot and restore contract

There are **two distinct milestones**:

- **Core snapshot:** restore bytecode-visible state wholly owned by VM. This proves PC/frames/heap/classes/virtual threads can continue from a new instance.
- **Game snapshot:** core snapshot **plus** MIDP/JSR runtime, media/renderer state, device input, timers and persistent/external resource reconciliation. This is **not guaranteed** by core snapshot success.

Proposed durable snapshot envelope (not yet a chosen wire format):

```text
magic | format_version | VM_semantics_version | fixture/runtime hashes
      | configuration/permissions | virtual clock/event epoch
      | classes/statics | heap+object identities | threads/frames/monitors
      | VM service state | host-rebind manifest | integrity check
```

- **No direct binary dump of Rust structs.** Use an explicit versioned encoding with clear endian, bounds and schema rules.
- Validate snapshot integrity and compatibility **before** mutating active VM state; restoration should be atomic or leave prior state untouched.
- Restores must reject incompatible code/runtime versions and malformed references rather than guess.
- No active unmanaged host I/O may silently survive; define what is replayed, recreated, cancelled, or declared unsnapshotable.
- Time and input nondeterminism must be virtualized or recorded to make continuation comparable. Rewind adds rolling checkpoints or event replay; memory/latency tradeoffs must be measured later.
- Security: Treat JARs and snapshots as untrusted. Enforce length, nesting, allocation, instruction and resource limits; fuzz parsers once present.

## 6. Proposed interface (behavioral, not committed API)

A future embedding API should support separate operations akin to `load`, `step(instruction_budget)`, `inject_event`, `checkpoint`, `restore`, and `inspect_status`, returning structured errors and host requests. **These are conceptual operations, not Rust functions currently available.** The host contract, ABI and concurrency model remain open.

## 7. Open decisions — resolve with experiments

1. Which CLDC-permitted class-file versions and verifier semantics do real target JARs require?
2. Can frames and monitors be checkpointed at instruction/safepoint granularity without unacceptable performance loss?
3. How should native CLDC methods / asynchronous host calls suspend and resume? What makes a resource unsnapshotable?
4. What is the minimum deterministic virtual clock, scheduling and I/O recording model?
5. How much memory and time do full vs incremental snapshots consume on realistic games?
6. Which host capabilities are needed before MIDP testing, and how do we map failures to user-visible messages?
7. How do we resolve implementation-use terms of JSR 139 / JSR 118, TCK/branding requirements and GPLv3 obligations for any future integration?

## 8. Specification references and provenance

- CLDC 1.1 (JSR 139): https://www.jcp.org/aboutJava/communityprocess/final/jsr139/
- CLDC 1.1 API: https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/overview-summary.html
- MIDP 2.0 lifecycle (JSR 118): https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/midlet/package-summary.html
- General JVM format/bytecodes (used with CLDC caveats): https://docs.oracle.com/javase/specs/jvms/se8/html/

See [specification-map.md](specification-map.md) for the precise version hierarchy and [licensing-and-provenance.md](licensing-and-provenance.md) for implementation-access caveats. The modern JVMS site is **informative** until reconciled against the historical JVMS edition referenced by JSR 139. Do not copy reference implementation sources, libraries, TCKs or proprietary game assets into the repository.
