# WP05 virtual concurrency contract and prerequisites

**Status:** W16 design contract only. This defines the bounded concurrency
backend and its dependencies; it is not implementation evidence, G4 acceptance,
or a Java ME thread-support claim. Public `main` remains single-threaded. The
local W14/W15 checkpoint and host-request experiments are separate evidence.
Implementation observations below refer to unpublished local W14/W15
experiments, not capabilities or source on public `main`. This contract is
standalone research and does not establish API acceptance or rights clearance.

## Authority and scope

The target remains CLDC 1.1 and MIDP 1.0–2.1, with versioned profile
contracts; JSR 271 / MIDP 3 behavior is not inferred. CLDC 1.1.1
maintenance changes and CLDC amendments take precedence over inherited JVM
rules. The applicable final JSR 139 text and implementation-use rights remain
unresolved; this contract does not close N01 or issue #30. Public API pages are
useful interface evidence, not permission to implement, certify, or distribute
a compatible VM.

Primary reference points:

- Oracle-hosted [CLDC 1.1 `Thread` API](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/lang/Thread.html):
  `Thread` extends `Object` and implements `Runnable`. Its API covers
  construction, priority, `start`/`run`, `yield`, `sleep`, `interrupt`, `join`,
  liveness, names, and current/active-thread queries. `start` begins `run` and
  rejects a second start; higher priorities are preferred, but the page sets
  no quantum or starvation bound. `sleep(long)` retains monitors and clears
  interrupt status when it throws. CLDC does not require interrupt to cancel
  or clean up pending I/O.
- Oracle-hosted CLDC 1.1 [`Runnable`](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/lang/Runnable.html)
  and [`Object`](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/lang/Object.html)
  APIs: `run()V`; monitor ownership for `notify`/`wait`; wait releases all
  claims on that object and reacquires them before returning or throwing;
  `notify` selects an arbitrary waiter without handing off the monitor.
  `Object` also exposes `equals`, `getClass`, `hashCode`, and `toString`; their
  class-identity and string requirements remain beyond the backend.
- [JVMS Second Edition, Oracle-hosted `Threads and Locks`](https://docs.oracle.com/javase/specs/jvms/se6/html/Threads.doc.html),
  §§8.1–8.9 and 8.12–8.14, covers per-thread execution, shared variables,
  locks, synchronization actions, wait sets, and the base lock model. It says
  it was adapted with minimal changes from Java-language material; apply the
  relevant CLDC amendments and do not use it to invent absent CLDC APIs.
- [JVMS Second Edition instruction semantics](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc9.html)
  defines `monitorenter`/`monitorexit`. The [structure chapter](https://docs.oracle.com/javase/specs/jvms/se6/html/Overview.doc.html),
  §§3.11.8, 3.11.11, and 3.12, covers invocation/return, monitors, and VM
  support for class libraries. `Concepts.doc.html` §2.19 is only an overview.
  The current Rust API's detached-fork rule follows the [standard-library
  `Clone`/`Eq` expectation](https://doc.rust-lang.org/std/clone/trait.Clone.html#clone-and-partialeqeq):
  an equal value's clone is expected to remain equal, so `IntMachine` uses an
  explicit fork operation rather than `Clone`.
- [JVMS Second Edition class-file §4.5](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html)
  defines the field `ACC_VOLATILE` flag needed by the proposed progress signal.
  Its execution meaning remains subject to the unresolved CLDC memory-model
  crosswalk.

The official CLDC 1.1 [API class index](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/allclasses-noframe.html)
lists `Thread` and `Runnable`, but not `ThreadGroup`; do not import the Java SE
`ThreadGroup` surface just because the JVMS thread/lock chapter discusses it.
Resolve any additional CLDC or MIDP runtime threading contract from its exact
versioned primary source before claiming support.

## State ownership and scheduling model

One VM owns the shared guest heap, class/static state, virtual clock, monitor
table, host-event boundary, and a bounded table of logical threads. Retain the
current VM-wide 64-frame and 64-object limits for the first backend, add a
64-record logical-thread limit, and retain the 1 MiB checkpoint cap. A blocked
context may have linked indexes in more than one queue (for example, an
`Object.wait` wait-set entry and its timeout timer); each individual index is
bounded by the 64-thread limit. Existing method, code, local and operand-stack
limits remain in force. Frames remain subject to the aggregate VM-wide cap, so
each started live thread consumes at least one of the 64 frame slots. These are
project resource limits, not CLDC maxima; changing them requires compatibility
and restore review. Exhaustion stops private launch setup before execution;
mapping Java API allocation or start exhaustion to guest-visible errors remains
a dependency. A thread owns its frame chain, locals, operand stacks, interrupt
status, priority, blocking cause, and terminal outcome. Logical thread IDs are
VM-owned and monotonic within one execution timeline; do not wrap or reuse an ID
while its VM state is live. Restore replaces the timeline and its counter, so IDs from a
discarded future are no longer valid. Exhaustion is checked before creation
changes the VM. Thread frames cannot read another thread's locals. Guest
references in frames, statics, heap fields, monitor records, and thread-object
associations use the VM's durable object IDs; no Rust address or OS thread ID
is a guest identity. Execution uses one host-side
interpreter driver, not one OS thread per guest thread.

The single-root `IntMachine` has one frame chain, one machine outcome and one
`Option<PendingHostCall>`. A concurrent machine needs one frame chain and
outcome per logical thread, plus shared object/static storage. Keep the existing
interpreter, verifier and instruction meanings; scheduler selection chooses
which thread's next already-validated instruction the interpreter executes.
An instruction transition is atomic with respect to other guest threads. No
native helper may retain an invisible Rust stack across a scheduling boundary.

There are two materially different starting points: implement the Java
`Thread`/`Runnable` API immediately, or first prove the scheduler with a private
launch table. The API-first path depends on class hierarchy, virtual/interface
dispatch, Thread objects and guest error identities that the current engine
lacks. Choose the private launch table as the smaller backend prerequisite; it
must not impersonate `Thread.start()` or count as API support. One interpreter
driver keeps guest scheduling state visible to checkpoints and avoids hidden
OS-thread stacks.

The backend needs one multi-root preparation, not several single-root programs
merged or cloned after preparation. It accepts ordered `(name, ()I)` static
entry tuples. Duplicate tuples are rejected; each tuple must resolve to exactly
one declared static method with that descriptor, and every root is validated.
If any root or reachable helper is missing, ambiguous, or unsupported,
preparation rejects the whole launch before execution. The preparer computes
and validates the union of the roots' supported same-class static call
closures. It assigns one shared deterministic method table; each method has
one program method ID across all roots and helper calls. Launch order is
separate from the shared code table, while the complete ordered root tuple
list is bound into program and snapshot identity.

Each thread has one primary state: `New`, `Ready`, `Running` (the selected
context), `Blocked` with exactly one cause, or `Terminated` with a normal or
uncaught-exception outcome. The eventual CLDC `Thread` API maps to these states:
a `Thread` object may be New before start, and a started context may be ready,
running, blocked entering a monitor, waiting on an object's monitor, sleeping
until a deadline, joining a target, awaiting a modeled host request, or
terminated. CLDC 1.1 declares `Thread()` / `Thread(String)` and
`Thread(Runnable)` / `Thread(Runnable,String)`; `run()` dispatches to the target `Runnable` when
present, and otherwise the base implementation returns. A thread object starts
at most once. `Thread.start()` creates the runnable execution of that object's
dynamically selected `run()V`; directly invoking `run()` is an ordinary method
call. The API also includes `activeCount`, `currentThread`, name and priority
queries, `isAlive`, `join`, `setPriority`, `toString`, `sleep`, `yield`, and
`interrupt`; these observable operations remain prerequisites beyond the
backend. The thread record retains the associated `Thread` object ID,
`Runnable` target/dispatch identity and started state; the current private
launch backend has no guest `Thread` object. An uncaught guest
exception terminates only its own context; it must not corrupt another thread's
frames or the VM's shared state. Application/runtime policy for reporting an
uncaught exception and deciding when a MIDlet session ends is a later WP06
dependency.

Keep runtime execution failures separate from guest exceptions. If a supported
instruction reaches a non-guest execution/resource failure, such as a bounded
frame or object limit, the driver stops globally with a structured
`ExecutionFailed { thread_id, pc, reason }` machine outcome. The failing
instruction is atomic: it changes no frame, PC, operand, heap or static state.
This is a machine failure, not a thread-termination transition; the selected
context remains at the failing PC. Other contexts and shared state remain
inspectable as they stood at failure, but no peer continues after it. Do not
turn the error into a guest `Throwable`, return a normal result, or retry the failing instruction. This terminal
execution-failure state is not checkpoint-eligible; a prior valid checkpoint
remains the recovery point.

CLDC `Object.wait` requires monitor ownership; a non-owner call reports
`IllegalMonitorStateException`, while a null receiver raises
`NullPointerException`. Negative timeout or out-of-range nanosecond
arguments report `IllegalArgumentException`; an interrupt while waiting throws
`InterruptedException` only after the monitor state is restored. `Thread.start`
on an already-started object reports `IllegalThreadStateException`, and
`Object.notify` and `notifyAll` also require ownership and report
`IllegalMonitorStateException`. `setPriority` validates the CLDC priority
range. Implement these as guest
exceptions with their required identities, never as host errors. The CLDC 1.1
API defines `NORM_PRIORITY` as the default-priority constant. Verify from the
final JSR text whether a new thread inherits its creator's priority before API
admission; the constant alone does not settle that constructor behavior.

Scheduling is project policy. The CLDC API says higher-priority threads are
preferred but gives no quantum or starvation bound. Every ready thread has a
CLDC priority. At a scheduling point choose the highest-priority ready thread,
using deterministic round-robin order among threads at that priority. A
priority change affects the next selection point. A selected thread runs at
most 32 guest instructions before rotation; calls and returns count as
instructions. A thread that blocks or terminates leaves the ready queue
immediately. This
gives a continuously runnable equal-priority thread a bounded opportunity to
run when the ready set is finite. A continuously runnable higher-priority
thread may starve a lower-priority one; no all-priority fairness or real-time
latency claim follows. The quantum is a versioned experiment constant, not a
guest option or an elapsed-time unit. `yield()` forces a scheduler point; if
another context is ready, select the highest-priority ready context other than
the yielding one for the next quantum. This project policy prevents `yield()`
from becoming a no-op. Confirm it against the final CLDC contract before API
admission.

Machine-wide instruction budgets count guest instructions across all threads.
Budget exhaustion leaves the selected thread, remaining quantum, queue order,
frames and PCs resumable. When no thread is runnable, the scheduler returns a
structured quiescent stop rather than spinning, with known timer and host-event
wake sources. It reports deadlock only if it can establish that no guest or
host event can make progress; mere absence of runnable threads is not proof.
Completion means every started context is terminal; one context's return is not
VM completion while another remains live.

The initial backend is sequentially consistent: each guest instruction reads
and writes the one shared VM-owned memory state in scheduler order. This is a
deliberate, stronger project execution policy for reproducible experiments,
not a claim that the JVM requires a unique interleaving or that every CLDC
memory-model detail is settled. Synchronization still must enforce mutual
exclusion and the memory effects required by the selected CLDC/JVMS contract.
Do not infer a Java concurrency memory model from a Rust mutex or from a green
single-thread test.

## Monitors, waiting, time and interruption

Each guest object has one logical monitor identified by its object ID. Monitor
state is `(owner_thread, reentry_count, entry_queue, wait_set)`. Acquisition by
an unowned monitor sets owner and count 1; re-entry by the owner increments
the count. `monitorexit` requires the owning thread, decrements the count, and
releases at zero. If another thread owns a monitor, `monitorenter` blocks at
that instruction: it has not completed, so its PC and reference operand remain
available for retry. A null receiver for a synchronized instance call also
raises `NullPointerException`. Implicit monitor acquisition for a synchronized
call is part of that call transition: if contended, the caller remains at the
invocation with its operands intact, without a checkpointable half-entered
callee. A null monitor operand raises `NullPointerException`; a non-owner exit
raises `IllegalMonitorStateException`.

Monitor reentry counters use checked finite arithmetic; overflow stops before
the instruction changes monitor ownership, operand stack or PC. The guest-
visible mapping for this resource exhaustion remains a prerequisite.

`ACC_SYNCHRONIZED` methods acquire their instance monitor before the body and
release it on normal or abrupt method completion. Static synchronized methods
use the class object's monitor; this requires real `Class` identities.
Concurrent class initialization separately requires class-initialization lock
state; neither exists in the current experiment. Static synchronized methods
and concurrent class initialization are therefore excluded until those
prerequisites exist. Explicit `monitorenter`/`monitorexit` and instance
synchronized methods must share the same monitor owner and reentry count.

`Object.wait()` is valid only for the owner. It stores the current reentry
count, releases all claims on that object and enters its wait set; it does not
release other monitors held by that thread. `notify()` moves one waiter chosen
by the implementation to the monitor-entry competition; `notifyAll()` moves
all waiters. A deterministic FIFO choice is the experiment policy for the
otherwise arbitrary `notify` choice. Neither operation transfers ownership.
Timeout and interrupt also move a waiter to monitor-entry competition. FIFO
queue order is the project policy for deterministic monitor-entry competition;
it does not create a monitor handoff or override scheduler priority. The thread
resumes only after reacquiring the monitor, with the prior reentry count
restored; an interrupt exception is delivered after reacquisition and clears
the interrupted status as specified by the API. `Thread.sleep(long)`
does not release monitors. `join()` waits for target termination and is
interruptible, but its exact monitor behavior must be cross-checked against the
applicable CLDC text; do not infer it from `Object.wait()` or release the
caller's unrelated monitors.

The VM owns a monotonic virtual clock measured in nanoseconds. Time advances
only from an explicit, ordered host time input; interpreter speed and wall-clock
reads inside guest execution do not define elapsed guest time. Reject negative,
overflowing or out-of-policy host deltas before changing clock state. Due timers
are delivered as scheduler transitions before the next guest instruction, in
deadline order and then by a nonwrapping registration sequence. Real-time hosting
may translate monotonic host deltas into these inputs; replay supplies the
recorded deltas. Convert `sleep(long)` milliseconds and `wait(long,int)` with
checked arithmetic; the latter uses the API's millisecond-plus-nanosecond
interval. `wait(0)` and `wait(0,0)` are indefinite. Choose a fixed maximum time
horizon and its overflow policy before admitting timed APIs. Use bounded
deadlines and a deterministic timer tie order. Timer-registration sequence
exhaustion refuses the wait before PC, operands, monitor ownership or timer
state change. Clock input, value, timer order and the next timer-registration
sequence are snapshot state. CLDC APIs describe elapsed real-time waits, so
virtual-time tests alone do not establish real-time behavior. `sleep(long)`,
`wait(long)` and `wait(long,int)` additionally require
category-2 `long` values, which the current interpreter does not support.

The VM records interrupt status per logical thread. The CLDC 1.1 API exposes
`interrupt()` and interruption through `sleep`, `wait` and `join`; its listed
surface does not expose the later `isInterrupted()`/`interrupted()` queries.
Pending I/O need not be cancelled by `interrupt`. A request retired because its
thread terminates or its VM timeline is replaced cannot accept a late callback;
retirement does not claim to cancel external I/O. An OS signal does not mutate
guest interrupt status implicitly; any host-originated interrupt must be an
explicit, ordered host input. The precise CLDC behavior for
interrupting monitor-entry waits, class initialization, and an outstanding
native host request must be checked against the applicable final text before
those cases are admitted. No exception or cancellation behavior may be
approximated by a successful native stub.

JVMS Second Edition §8.13 permits, but does not require, enforcement of its
structured-locking rules. The first monitor implementation should use the
base ownership/reentrancy checks and not add a second verifier rule for
structured locking; guest `monitorexit` ownership is still checked. This choice
must be revisited if the exact CLDC verifier/preverification contract imposes
a stronger rule. Guest exception handlers must run the guest's monitor cleanup
code; only the implicit monitor of an `ACC_SYNCHRONIZED` method is released by
method completion itself. Resolve from the applicable primary source what
happens to explicit monitor ownership if a thread terminates uncaught before
cleanup; do not invent automatic release or retain behavior.

## Host requests and callback authority

W15 currently admits one machine-wide request and one `AwaitingHost` machine
outcome. That representation cannot let other threads run while a caller is
blocked, nor can it distinguish several suspended callers. Before native host
calls are reachable from multiple guest threads, make the logical pending
continuation per thread and allow a bounded set of outstanding requests. Each
record identifies its owning logical thread, operation, arguments, caller,
call-site PC and continuation; its arguments are consumed and its result slot
is absent until completion. A nonwrapping VM-wide request sequence makes
each record unique under the existing `(owner, activation, sequence)` callback
token; delivery maps the sequence to exactly one blocked thread and makes only
that thread runnable. Host events are injected between VM transitions, never
by reentrant concurrent mutation. An event must match all three token fields;
wrong-owner, stale-activation, wrong-sequence and duplicate events change no
thread, queue, outcome or event-acceptance state.

Missing binding or exhausted sequence is a host-boundary refusal before the
call changes its PC, operands or thread state. The call remains selected at its
invocation and the VM pauses with a structured `step()` reason; `run()` keeps
its existing `None` stop result and returns immediately instead of retrying the
same call. It is not a guest exception or a blocked request. A caller may
rebind after a missing-binding refusal if the host contract permits; exhausted
sequence is never wrapped or reused and requires stopping that VM instance.

Restore preserves logical pending pure calls but excludes host callback
identities, reserves a fresh activation only after all state validates, and
reissues each request under that activation. Reissue must be a non-failing
enqueue or an all-or-nothing batch; if the host cannot guarantee that, restore
is refused before commit. The host supplies an owner identity for each
independent machine and a fresh activation on every bind or restore; it must
guarantee uniqueness across process lifetimes. The VM can reject collisions
with its current binding but cannot prove global freshness from a counter or
process ID. Old and duplicate events remain
stale. Any active `unsupportedResource` still vetoes a checkpoint unless a
separately specified resource policy resolves it. Restore onto a target with
such an active operation is refused before replacement; it cannot silently
discard that operation. `IntMachine::fork_detached` continues to be the explicit
logical fork: preserve per-thread logical state,
drop bound callback authority, require a fresh owner/activation binding before
delivery or request reissue, keep `Retired` closed, and retain full equality.
The machine does not implement Rust `Clone`; Rust's documented `Clone`/`Eq`
expectation is that an equal value's clone remains equal, which cannot
represent dropping host authority.

The first scheduler-backend increment below rejects host-hook methods from its
multi-entry closure. This avoids giving the existing singleton callback false
multi-thread semantics. The per-thread host model is a prerequisite for the
later concurrent host/service integration, not a test-only success return.

## Safepoints and checkpoint contract

The global safepoint is between completed guest instructions and scheduler or
host-event transitions. No Rust/native call is suspended. The backend image
identity binds exact guest class bytes, preparation mode and the ordered launch
entry set. Once general loading exists, identity must also cover every loaded
class definition and the bootstrap-library/runtime semantics. A threaded image
uses a new format and runtime-semantics version; W15 v2 bytes are rejected
rather than guessed or reinterpreted. Include every admitted state in the
image; bump the schema/semantics version when its representation or
interpretation changes.

A complete threaded image preserves shared statics and heap IDs, including
VM-owned special object state such as the bounded Throwable null-message value
and logical creation trace; every thread record's ID, priority, interrupt
status, `Thread`/`Runnable` associations and lifecycle outcome. For each live
execution context, preserve frames, locals, stacks, current PC,
call-site/return continuation, blocked cause, wait reentry count and deferred
wake result. A `New` or `Terminated` context has no execution frames; retain
each terminated context's ID and outcome while peers remain live, even after
its frames are removed;
ready-queue order and selected-thread quantum; monitor owners, recursion
counts, entry queues and wait sets; virtual clock, deadlines, timer order and
the next timer-registration sequence; the next thread ID and host-request
sequence; and all logical pending host operations. Restore these counters with
their timeline; fresh callback activations prevent old events from colliding
with reused request sequence values after restore.
Host owner/activation identities, OS thread IDs, native stacks and external
handles are never serialized.

Decode into bounded candidate state, then validate before commit. Check limits,
unique thread IDs and a saved next-ID greater than all IDs in the image;
validate the frame/PC/call chain and verifier facts for each live
`Running`/`Ready`/`Blocked` context, and exactly one primary state per record.
`Running` matches the selected slot; `Ready` appears once in the ready queue.
`New` and `Terminated` contexts have no execution frames and appear in no
runnable or blocked index. Validate a `New` context's unstarted state and a
`Terminated` context's outcome; an uncaught-exception outcome must reference a
valid guest `Throwable`. At a quiescent safepoint with no runnable context, the
selected slot is empty; while guest execution is runnable, exactly one context
is `Running` and selected. Every queue and timer index has unique, live thread
IDs and no orphan registrations.

A blocked thread has exactly one logical cause, with all indexes required by
that cause. Monitor entry has one entry-queue membership. `Object.wait` has one
wait-set membership and, when timed, one timer registration as well; notify,
timeout or interruption removes the wait-set and timer indexes atomically and
transfers the thread to monitor-entry competition. A sleeping thread has one
timer only, a joiner resolves to its target's termination wait, and a host wait
resolves to one pending request. Preserve the wait's saved reentry count and
deferred completion result while the thread waits to reacquire the monitor.
Reject missing, duplicate or incompatible memberships rather than collapsing
them into one generic queue slot. Monitor owners and waiters must exist,
recursion counts must be coherent, and each blocked reason must agree with its
associated indexes. Object and class references must resolve and agree with the
prepared layouts. Due timers are delivered as ordinary scheduler transitions
before a requested capture; serialization itself does not advance time or deliver
events. Special-object metadata such as Throwable creation traces must resolve
to program method/PC identities and any logical thread references the format
actually records, never host stacks. Callback sequences must be
unique, monotonic and attached to one awaiting thread; the saved next-request
sequence exceeds every pending request sequence. No in-progress or
uninitialized allocation, active constructor, unsupported resource,
half-applied event or host native stack may disappear from the image.

Images over 1 MiB are rejected before writing or decoding; checkpoint failure
does not change the running VM. Any invalid or incompatible image leaves the
target's full logical state,
binding, callback acceptance and event queue unchanged. For a valid restore,
reserve fresh callback authority only after validation, then atomically commit
the VM state and reissue supported pending requests. If reservation fails, the
old machine remains usable under its old activation. Time and ordered input
events needed for deterministic continuation are state or an explicit replay
log, never inferred from the new process's wall clock.

## Dependency order and first increment

| Dependency | Needed before | Current limitation / scope rule |
| --- | --- | --- |
| Exact CLDC 1.1 / 1.1.1 and MIDP threading crosswalk plus reviewed use terms | Any compatibility or implementation-rights claim | The API pages and inherited JVMS clauses do not settle final JSR amendments or rights. |
| Per-thread call stacks, outcomes, shared VM state, inspection and budgeted scheduler | Any concurrent guest execution | Current `IntMachine` has one frame chain and one outcome. The backend can be prototyped before guest Thread APIs, but only proves that backend. |
| Generic admission and ordered access for supported volatile fields | Busy-wait visibility in the first fixture | Current field admission rejects `ACC_VOLATILE`; the first backend must admit the flag generically for supported field types, not by special-casing the fixture name. The global instruction order supplies this slice's access order. |
| Class hierarchy/linkage, `invokevirtual`, `invokeinterface`, `Runnable`, `Thread` objects and exact `Thread.start()` / `run()V` behavior | Real CLDC `Thread`/`Runnable` support | Current preparation admits one guest class and static helper calls; no Thread class/API or dynamic dispatch exists. Do not replace these signatures with test intrinsics. |
| Required Thread/Object exception identities and propagation (`IllegalThreadStateException`, `IllegalMonitorStateException`, `InterruptedException`, null/runtime failures) | Observable API errors from start, wait, interrupt or monitor operations | Host execution failures are not guest exceptions. Add only exact Java ME API errors needed by the admitted behavior. |
| Reentrant object monitors and implicit synchronized-method entry/exit; `Class` objects and initialization locks for static synchronization | `synchronized`, `wait`/`notify`, joins and concurrent class initialization | Current objects have no monitor state; class initialization is a completed precondition and no `Class` objects exist. |
| Category-2 `long`, timer input, deadlines and wake ordering | `sleep(long)`, timed `wait` and real-time mapping | Current values contain only int/reference/null. Virtual-time evidence is not elapsed-real-time compatibility. |
| Per-thread host pending records and activation-safe multi-request delivery | Any host/native call reachable by multiple guest threads | W15 stores one pending request. First backend rejects host hooks in its multi-entry program. |
| Versioned whole-scheduler snapshots and atomic restore | G4 checkpoint claims | W14/W15 formats are single-threaded; new scheduler/block/monitor/time state requires incompatible versioned encoding and hostile-state validation. |

The proposed first implementation is a **scheduler-backend feasibility slice**,
not Java `Thread` support. Its private launch plan selects two to 64 unique
static `()I` entry tuples from one multi-root-prepared same-class program. One
atomic launch transition creates every logical context and root frame before
guest execution. At the first checkpointable state, the first ordered entry is
`Running` and selected with a full 32-instruction quantum; the others are
`Ready` in launch order. No intermediate state with runnable work but no
selected thread is checkpointable. Each context has the same project baseline
priority, a stable logical ID and an independent terminal result; all share the
prepared program's heap, statics and method table. The complete launch plan is
part of program and snapshot identity. It is a harness/backend contract, not a
Java-defined constructor or fake `Thread.start()` implementation.

The slice uses the fixed 32-instruction round-robin quantum, shared-state
instruction order, per-context frames/outcomes, thread inspection, bounded
execution and version-3 process checkpoint/restore for states with no owned or
contended monitors, waiters, timers, host hooks, in-progress constructors or
uninitialized allocations. It explicitly rejects reachable monitor operations,
synchronized methods, Java Thread/Runnable API references, native hooks, class
initialization and unsupported value/exception forms. It does not implement
`Thread`, `Runnable`, priorities other than the fixed test
priority, start-once behavior, wait/wakeup, sleep, interrupt, or any MIDP
lifecycle API. It does not pass G4.

Use an original source fixture with at least two entries made runnable
together. Its ordered launch list starts the busy-waiting entry first; it waits
on a volatile static flag until its peer publishes progress and a result value.
The single global instruction order implements these volatile reads/writes;
no broader Java memory-model claim follows. Inspection must show the peer publishing while
the first context remains live in the loop, then both finish under the
equal-priority budget with inspectable per-context results and frame traces.
Add a third entry that exits by uncaught guest exception while a peer continues
to protect outcome isolation. In a separate bounded launch, exercise object-
limit `ExecutionFailed`: assert its thread ID, PC and reason, no failed-step
mutation, unchanged peer/shared state, and checkpoint refusal; do not expect a
guest `OutOfMemoryError`. Stop at a selected global scheduler boundary with
multiple contexts live and a nontrivial ready-queue order. At that stop,
require the selected thread to have 1–31 instructions remaining in its quantum,
and retain any entry that has already returned or terminated by uncaught guest
exception alongside the live contexts. Producer A writes, flushes and closes
the image, then continues to record its post-stop trace and outcomes and exits
successfully. A supervisor with no VM waits for that exit before it starts
fresh process B to restore and continue. Compare the single global ordered
continuation trace of `(thread ID, method ID, PC)` steps, as well as per-thread
traces, shared fields, and terminal outcomes. In fresh process C, restore a
valid-checksum image captured after the publisher has stored the flag and
result but before the waiting context consumes that result, with the saved
result field changed; continuation must observe that saved value,
proving the decoder uses image state rather than restarting the launch plan.
Include checksum/truncation plus correctly checksummed
invalid PC, duplicate/missing run-queue membership, wrong launch identity,
dangling guest reference and thread/frame-limit cases; each failed restore
must preserve the running target and allow it to continue. This evidence is
only for the launch-plan scheduler backend and core-state subset. It does not
prove CLDC Thread API behavior, monitor semantics, time/interrupt behavior,
full G4, G5, G3 acceptance or rights.

## Unresolved before API or gate claims

- Reconcile CLDC 1.1 and 1.1.1 final rules for thread creation and initial
  priority, sleep argument/zero behavior, join monitor behavior, interruption,
  verification, class initialization and explicit-monitor ownership at uncaught
  termination. The
  JVMS Second Edition thread/lock material is a base, not a substitute for
  unavailable or amended CLDC text.
- Verify the target's exact `Thread`/`Runnable` and `Object` API surface, error
  behavior, and required runtime lifecycle. The public docs establish the
  listed signatures but not a redistributed implementation or TCK right.
- Define how virtual monotonic input maps to CLDC elapsed real time in a real
  host and how the MIDlet runtime controls initial and terminal threads.
- Resolve the per-thread host-request contract before a thread can block on a
  native/service operation. No scheduler-only result can establish G5.
- Close the applicable JSR implementation-use review before public conformance
  claims or distribution. This document is not a legal determination.
