# Technical architecture — proposal v1

**Status:** Proposed behavioral contracts plus implementation hypotheses, not a completed implementation. Outbound license and clean-room policy are decided in [ADR-0002](adr/0002-gplv3-only-and-clean-room.md). [ADR-0004](adr/0004-staged-delivery-and-recoverable-state.md) proposes the staged delivery baseline. Sections 9–13 clarify required outcomes without freezing data structures. Revisit implementation hypotheses after experiments; change a contract explicitly when evidence requires it.

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

A host adapter must not mutate VM internals through arbitrary shared pointers. The VM should use explicit events and host service calls. Section 10 defines the proposed pending-operation lifecycle; its representation and feasibility still require G3H evidence. Logical graphics/audio/storage state belongs to the recoverable runtime where supported; native devices and handles belong to the host. The diagram locates execution services, not permission to hide guest-visible state in the host.

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

## 9. Canonical state, safepoints and cache ownership

These project contracts enable recovery; they do not add requirements to the
Java specification or prove a particular implementation efficient.

| Owner | State required for continuation | Reconstructable / external material |
| --- | --- | --- |
| VM | Frames/PCs, locals/stacks, heap identities, statics, initialization and exception state, scheduler/monitor state | Decoded-instruction and resolution caches may be rebuilt if behavior is preserved |
| Java ME runtime | MIDlet lifecycle, event queues, mutable image contents/drawing state, supported media timeline, session-managed RMS | Immutable assets may be reloaded only with verified content identity |
| Host adapter | Delivery/cancellation tracking and actual device resources | Windows, GPU/audio handles, file descriptors and connections are never portable snapshot identities |
| Debugger/control layer | Explicit commands and inspection points | UI views are not guest state; pending commands must not leak into a restored timeline |

Use one authoritative owner for each logical state item. A cache may be omitted
from snapshots only if recreating it cannot change required guest-visible
behavior. A mutable image cannot be replaced by its original asset unless all
subsequent modifications are also reconstructed. Media seeking/decoder state
and completion callbacks require resource-specific proof; exact audio output
is not automatically preserved by remembering a playback timestamp.

Define the commit boundary of an instruction/runtime operation before allowing
pause or checkpoint there. At a safepoint, PCs, stacks, roots, exceptions and
pending operations describe one coherent execution state. Checkpoint latency
must account for long native helpers; helpers must finish within a declared
bound or expose a resumable logical operation. Do not serialize a Rust closure,
future, native stack or borrowed pointer as the sole pending continuation.

Failed restore must leave the prior active VM and its managed storage intact.
Validate a candidate state and stage resource reconstruction before publishing
it as active. If staging fails, release candidate resources; do not execute
guest-visible effects during validation. Coordinate storage and runtime commit
so failures cannot expose a half-restored session. These properties need tests,
not an assumption that swapping a heap alone is atomic.

## 10. Host requests and effect policy

Conceptual lifecycle: guest issues a request; its thread either obtains a
synchronous result or blocks with an explicit logical continuation; the host
returns a completion event; the VM accepts it at a defined scheduling point.
Request identity, operation type, necessary arguments and continuation must be
represented without host pointers. Guest exceptions and host failures remain
distinguishable. Host code must not re-enter and mutate the VM arbitrarily.

Use a session/timeline epoch plus request identity (or an equivalent proven
mechanism) to prevent old callbacks from being accepted after restore, rewind
or stop. An epoch must not be restored in a way that admits callbacks from the
discarded timeline. Within an active timeline, a completion can be consumed at
most once. This is a local delivery property, **not** an exactly-once guarantee
for an external server transaction.

Each resource/operation declares its recovery policy before support is claimed:

| Policy | Example / obligation |
| --- | --- |
| VM/runtime-owned | Include logical state, such as a pending timer or managed storage version |
| Reconstructable | Recreate a renderer resource from retained data; validate identities and report failure |
| Recorded result | Replay previously captured input/I/O completion without repeating the original effect |
| Unsupported while active | Reject checkpoint with a specific reason for an unrecoverable operation/resource |

For each supported policy, define disposal, cancellation, late completion and
failure behavior. Operation arguments are not sufficient evidence that a
request may safely be reissued. Network sends, purchases or external file
writes are not undone by rewinding local memory. Checkpoint refusal must be
reported; do not silently omit a resource or hang indefinitely waiting for it.

## 11. Time, replay and persistent storage

The initial deterministic envelope is a fixed semantics/build version, guest
and runtime content identities, device/API profile, configuration, initial
state and ordered input/completion stream. Define any host-dependent numeric,
rendering or decoder behavior outside that envelope explicitly. Portable
snapshot bytes alone do not establish cross-platform deterministic execution.

Execution budget, guest-observed time and display pacing are distinct concepts.
Define how timers advance and simultaneous events are ordered, including when
all guest threads are blocked. Real-time operation may sample the host through
this boundary; replay consumes recorded virtual-time/event decisions. Do not
let interpreter/JIT speed silently define guest elapsed time. A budgeted
scheduler still needs CLDC priority, monitor and memory-visibility semantics;
deterministic scheduling alone is not proof of conformance.

Rewind first uses measured full checkpoints and, if selected, recorded events.
Incremental snapshots/copy-on-write are deferred; admitting them requires a
complete mutation-tracking contract for all affected state. Restore the nearest
usable checkpoint and replay to the target without duplicating external side
effects. New input after rewind creates a new timeline; discard incompatible
future events and reject old completions. Bound history and explain eviction.

RMS persistence must survive ordinary application restart, while rewind must
keep guest memory and managed storage mutually consistent. Before RMS support,
specify how session storage versions/checkpoints, durable commits, crashes,
shared stores and concurrent writers interact. Isolated session storage with
versioned checkpoints is the initial candidate. Do not silently overwrite
external changes or claim shared/live resources can rewind. Select and test
the policy before G6 rather than treating storage as an ordinary host file.

## 12. Inspection and future execution engines

G2 introduces bounded stepping and read-only inspection of class/method/bytecode
location, frames, locals, operand stacks and heap identities. Inspection occurs
at a consistent stop point. Add breakpoint/exception stops for the supported
subset, and thread/block/monitor inspection as G4 introduces concurrency.
Source-line/name views depend on available debug metadata; obfuscated or
stripped JARs retain bytecode-level support. A debugger write is a deliberate
state mutation and timeline event, not an invisible inspection side effect.

Future JIT admission must identify live references for GC, preserve exception
and monitor behavior, honor scheduling/stop boundaries, and materialize
logical state at supported safepoints. Machine code is a disposable cache;
restore may resume in the interpreter. Compare optimized behavior with both
the applicable normative tests and interpreter results; agreement between two
wrong implementations is insufficient. Backend/IR design and any unsafe/FFI
boundary require a later focused decision under the existing safety policy.

## 13. Compatibility and format evolution

Record profile/API selections and evidenced compatibility quirks in snapshot
and replay identity. Keep quirks at the owning runtime/profile boundary; do not
add title-specific interpreter branches without a reviewed semantic reason.
API availability, snapshot support and replay support are separate capabilities.

Initially reject mismatched build/semantics, code or profile identities. Version
the explicit wire schema; never dump Rust layout. Snapshot migration and stable
embedding ABI are later commitments, with a tested supported-version matrix.
Integrity checks detect corruption but do not make untrusted snapshots safe:
all bounds, references and allocation budgets still require validation.
