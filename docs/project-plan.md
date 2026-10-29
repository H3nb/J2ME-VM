# Project definition and staged delivery

**Status:** Proposed planning baseline, 2026-10-29. Adoption does not establish
technical feasibility or authorize an Android release. Current implementation
and the next action are recorded in [project-status.md](project-status.md).

## 1. Purpose and success

Build an independent Java ME runtime that can run original JARs and give its
host controlled execution, inspection, and reliable recovery of supported
sessions. It may later power JL-Mod Plus or a separate Android application.
Neither host is selected yet. Preserve the existing JL-Mod Plus engine.

The first useful outcome is a desktop research runtime that executes an
original guest program, exposes its state, and restores it in a fresh process
after the original process exits. The first user-facing pilot is an original
offline MIDlet with input, mutable graphics and RMS whose session can be
restored and rewound within an explicitly supported profile.

Broad game compatibility, production Android delivery and JIT are subsequent
outcomes, not prerequisites for demonstrating that first useful result.
No document, test count or green scaffold job is a substitute for that result.

## 2. Development approach and decision authority

Use risk-driven incremental development with evidence-based gates. The user's
FEL analogy maps to the following activities, not to a fixed-price EPC contract
or a claim that all requirements can be frozen before experiments:

| Planning stage | Deliverable / exit question | Commitment unlocked |
| --- | --- | --- |
| Definition (FEL-1 analogue) | Mission, user scenarios, exclusions, success criteria and known constraints agree | Investigate the selected concept |
| Concept / feasibility (FEL-2 analogue) | Compare materially different approaches; expose ownership, standards and host risks; plan falsifiable experiments | Build bounded feasibility increments |
| Increment preparation (FEL-3 analogue) | For the next increment: behavior, normative clauses, dependencies, state owner, failure cases and acceptance evidence are concrete | Implement that increment |
| Delivery and validation | Demonstrate the increment, review the effective diff, record remaining limitations | Accept evidence and prepare the next increment |

FEL-3 repeats for each increment. Do not wait for an exhaustive design of the
entire emulator, and do not label FEL-2 technically complete before experiments.
The process borrows the evidence/risk principle of the
[Incremental Commitment Spiral Model](https://boehmcsse.org/tools/icsm/);
this lightweight project process is not a claim of formal ICSM compliance.

Hendra owns product priorities, material scope changes, release/integration,
and any decision to abandon or replace the project. Contributors and agents
make routine implementation choices within authorized scope and prepare
evidence and recommendations. Reviewers check primary contracts and behavior
independently of implementation claims. A separate reviewer is preferred for
foundational gates; self-review must be labelled as such. Do not create an
extra permission prompt for every routine task or gate when work is already
authorized. Existing authorization never turns missing evidence into a pass.

## 3. Required scenarios and traceability

These are project requirements/proposed acceptance contracts, not statements
that any feature exists. Technical gate definitions live only in
[validation-plan.md](validation-plan.md).

| ID | Scenario / requirement | Acceptance evidence | Gate |
| --- | --- | --- | --- |
| R01 | Accept only the declared input and execution subset | Bounded valid/invalid fixtures; parsing distinguished from verification; unsupported behavior fails explicitly | G1, G2 |
| R02 | Execute guest code with VM-owned state | Original programs exercise calls, branches, exceptions, object identity and inspected frames against applicable clauses | G2 |
| R03 | Continue a saved computation after process death | Old process exits before restore; continuation trace and values match a controlled uninterrupted run; malformed restore leaves target unchanged | G3 |
| R04 | Recover pending operations without stale host activity | A synthetic host event survives checkpoint or is explicitly rejected; late and duplicate completions cannot mutate a restored session | G3H, G5 |
| R05 | Support controlled concurrent guest execution | Busy-loop scheduling, monitor ownership/reentrancy, blocking, interrupt and timeout cases survive restore | G4 |
| R06 | Restore an offline MIDlet session coherently | Input, lifecycle, mutable image state, virtual time and session-managed RMS agree after a fresh-process restore | G6 |
| R07 | Rewind within a declared deterministic envelope | Checkpoint plus recorded events reaches matching observable state; branching discards future events and does not repeat external writes | G6R |
| R08 | Inspect and debug supported MIDlets | Bytecode-level stepping, frame/heap inspection and exception stops; thread/block inspection when concurrency exists | G2, G4, G6 |
| R09 | Preserve a path to JIT without weakening semantics | Design review of safepoints, roots and reconstructable state now; execution equivalence and performance evidence before admitting JIT | G2 design, G7 decision |
| R10 | Be embeddable without Android owning guest state | Headless tests have no Android dependency; future host can pause/stop/recreate resources via the defined boundary | G3H, G6, G7 |
| R11 | Maintain a reproducible, independent implementation | Normative clause and fixture provenance; public tests without private SDK/game access; bounded hostile input; clean-room and dependency policy | Every relevant increment |

Priority is deliberate: R01-R04 and basic R08 are the first feasibility slice;
R05-R07 turn core recovery into session recovery. R09 is an architectural
constraint now, an optimization project later. Optional JSRs/vendor profiles
follow measured compatibility demand, not an exhaustive API checklist.

## 4. Scope boundaries and concept selection

### Baseline constraints

- Rust, host-neutral library, initially one crate; GPL-3.0-only and the existing
  clean-room rules remain in force. No borrowed emulator/runtime internals.
- CLDC 1.1 investigation baseline, then MIDP 2.0; identify exact revisions and
  normative clauses before implementing behavior. A modern JVM is not authority
  over CLDC. See [specification-map.md](specification-map.md).
- Original public fixtures; private compatibility assets stay outside the repo
  and public CI under [testing.md](testing.md).
- Explicit state and host boundaries, structured failure, safe Rust core,
  deterministic research inputs and a headless execution path.

### Deliberately deferred

Full Java SE, physical phone/firmware emulation, broad vendor/JSR coverage,
production UI, network rewind, guest parallelism on multiple host CPUs,
advanced GC, compressed/incremental checkpoints, IDE protocol integration,
native JIT and stable public ABI. No universal cross-build snapshot promise.
Deferral protects the first outcome; it does not prohibit a later justified ADR.

| Choice | Selected direction and reason | Revisit trigger |
| --- | --- | --- |
| Own bytecode engine vs depend on host Java execution | Own interpreter: makes guest frames and scheduling available to recovery and inspection | A demonstrated fundamental obstacle, with alternatives that still meet R03/R06 |
| Whole emulator first vs vertical increments | Small executable slices with negative tests: expose wrong assumptions before API breadth grows | No useful evidence emerging from the current slice |
| OS-thread-per-guest vs VM-managed threads | VM-managed, budgeted execution initially: explicit blocking and checkpoint ownership | Measured workload that cannot meet an agreed budget; preserve guest semantics |
| Opaque native media vs recoverable logical services | Runtime-owned logical media/storage state; host resources recreated where supported | Fidelity or resource cost demonstrated by G5/G6 |
| JIT immediately vs interpreter baseline | Interpreter first: establish semantics and actual bottlenecks | G7 profiling shows execution is a material bottleneck |
| Full design freeze vs evolving implementation | Freeze requirements/invariants; keep layout/algorithms provisional | New evidence invalidates a decision, documented in an ADR |

These are original comparisons of project options, not descriptions derived
from another VM implementation. [Architecture](architecture.md) owns the actual
state, execution and host contracts; do not duplicate them in task checklists.

## 5. Work packages and dependency order

One primary foundational implementation package should be active at a time.
Independent review, specification research or fixture work may proceed where
it does not race on state ownership or the same files. This is a coordination
rule, not permission to spawn agents or start unrelated work.

| Package | Concrete output | Depends on / completion boundary |
| --- | --- | --- |
| WP00 — planning baseline | This plan, owned architecture contracts, gates and resumable status | Current scope reconciled with existing PRs; no implementation gate claimed |
| WP01 — input boundary | Review existing G1 parser PR; resolve applicable class/verification clauses and limitations | Preserve PR #1 work; G1 evidence reviewed at exact head |
| WP02 — executable subset | Prepared instruction/verification contract, original guest fixture, owned frames and minimal inspect/step support | WP01; G2; every executed instruction path is validated for the declared subset |
| WP03 — core recovery | Nontrivial calls, heap/statics, versioned bounded snapshot and two-process harness | WP02; G3, including atomic invalid restore |
| WP04 — host feasibility | Synthetic resumable request, completion event and explicitly unsupported resource | WP03; G3H before substantial runtime/API breadth |
| WP05 — virtual concurrency | Scheduler, monitor, blocked-thread and virtual-time behavior | WP04; G4; no blanket CLDC concurrency claim |
| WP06 — runtime services | Necessary CLDC services and host lifecycle/resource contracts | WP05; G5; expand selected functions only |
| WP07 — offline MIDlet pilot | Original JAR/JAD, lifecycle/input, simple mutable graphics and RMS restore | WP06; G6; no broad game compatibility claim |
| WP08 — rewind pilot | Measured rolling checkpoints/replay and session-storage branching | WP07; G6R |
| WP09 — product decision | Compatibility/performance evidence and Android host/integration options | WP08; G7; shipping/JIT require their own bounded plan |

Dependencies refer to acceptance evidence; a package may contain several
reviewable PRs. A partial PR may merge without declaring its entire gate passed.
Necessary runtime primitives may be built earlier to enable a gate; these are
not permission to grow the whole CLDC library ahead of proof.

### Prepared next increment: WP01 review and input boundary

- **Input baseline:** refresh PR #1 and its target; use the exact heads rather
  than the dated hashes in project status. Scope is the effective parser/fixture
  diff plus its contracts and CI, not an unsolicited verifier or interpreter.
- **Behavior to establish:** the documented structural subset is parsed within
  declared limits; malformed, truncated and unsupported input fails with a
  useful error; successful parsing does not authorize execution.
- **Review questions:** version/minor rules, constant-pool tag/index types and
  long/double slots, modified UTF-8, count/length arithmetic, bounded attributes
  and Code structure, borrowed-input lifetime, fixture bytes and provenance.
  Separate constraints intentionally deferred to verification from defects in
  the structural contract. Verify the exact clauses in the specification map;
  do not accept the PR's interpretation as primary authority.
- **Evidence:** full effective diff review, current `fmt`/`clippy`/tests results
  at that head, original valid fixture plus distinct negative boundary cases.
  Reuse meaningful tests; add a case only for a real uncovered failure mode.
- **Completion:** findings disposition, precise G1 scope/limitations and next
  WP02 contract recorded. Review/CI failure returns to the same package. Merge
  remains subject to existing authorization; no new successor branch/project
  is a substitute for fixing the existing work.
- **Bound:** one review of the complete current diff, then focused follow-up
  on changed or unresolved findings. A missing specification/toolchain result
  is recorded as a blocker, not guessed or retried indefinitely. This is a
  review scope bound, not an invented delivery-time estimate.

## 6. Risks and unresolved decisions

Risk level expresses consequence and uncertainty, not a calculated probability.
All rows are open at baseline. A passing related test reduces only its stated
risk; record closure or residual risk in the gate evidence.

| ID / level | Risk and consequence | Next reduction / decision deadline | Owner role |
| --- | --- | --- | --- |
| K01 / high | Wrong historical CLDC/verifier contract makes accepted bytecode unsafe or incompatible | Resolve precise clauses and accepted subset in WP01/WP02 before execution | Standards/implementation contributor |
| K02 / high | Hidden native continuation or object identity makes restore incomplete | G3 two-process proof, identity graph and failed-restore cases | Core contributor + reviewer |
| K03 / high | Host callbacks/media cannot be reconciled after restore | G3H request lifecycle experiment before runtime expansion; G5/G6 resource-specific proof | Host/runtime contributor |
| K04 / high | RMS and replay effects diverge from rewound memory | Define versioned session storage and branch/commit policy before WP07; test in G6/G6R | Runtime contributor |
| K05 / high | Virtual scheduling/clock changes semantics or reproducibility | G4 clause-driven blocking/priority tests; separate elapsed-time policy from execution budgets | Core contributor |
| K06 / high | JSR terms or later integration license obligations remain unresolved | Follow existing licensing policy before affected use/publication; no technical gate clears this risk | Maintainer; qualified review where needed |
| K07 / medium-high | Interpreter, checkpoints or recovery exceed memory/latency needs | Measure G3/G6 baseline, select target device and budgets before G7 acceptance; profile before optimizing | Performance contributor + product owner |
| K08 / medium-high | Vendor quirks consume unbounded compatibility effort | Limit pilot profile; classify each mismatch, add explicit evidenced profile behavior only | Runtime contributor + product owner |
| K09 / high | Agent/session discontinuity causes duplicate work, silent scope drift or restart | Exact-head status, one next package, owned documents and change-control rule below | Every contributor |
| K10 / medium-high | JIT later cannot reconstruct roots/state or respect scheduling | G2 architecture review; future JIT admission proves GC roots, safepoints and equivalence | Core contributor + reviewer |

Open choices intentionally remain open: GC algorithm, handle representation,
snapshot encoding/compression, renderer/audio backend, JIT backend, minimum
Android/device target, release ABI and cross-version snapshot migration.
Choose each when an increment needs it; record rationale and measured tradeoff.

### Measurement contract

Do not invent a completion date, percentage-complete score or speed claim.
Before each experiment, bound its workload, input sizes and expected resource
use; record actual elapsed work/CI cost and what uncertainty it resolved.
If an experiment reaches its declared bound without an answer, stop that
experiment, record why, and choose a narrower test rather than loop indefinitely.

At G3 measure snapshot bytes, save/restore duration and peak memory for declared
fixture sizes. At G6/G6R also measure host reconstruction, event-log growth,
rewind memory per second of history and replay latency. Record CPU/OS/toolchain,
build mode, input hashes, workload and measurement method. Before G7 is judged,
Hendra and the technical contributor select representative lawful workloads,
target Android device class and acceptable playability/memory/latency/energy
budgets. These are **TBD**, not passed or silently unlimited. Research can
continue with stated bounds before product budgets are chosen.

## 7. Change control, continuity and completion

For a new feature or design change, identify the requirement, owning contract,
affected state, gate and maintenance cost. Routine fixes proceed within scope.
A feature that does not serve the current package goes into the deferred list
with a reason; it must not silently expand the package.

Revisit a settled decision only with a concrete changed requirement, conflicting
primary evidence, failed experiment, measured bottleneck or security/licensing
finding. A newly fashionable tool, model or framework is not sufficient.
Compare the smallest corrective change, a bounded redesign and replacement;
include migration cost, retained tests and rollback. Prefer repairing the
current repository where it can still meet the mission. Never start a successor
repo, abandon the project or replace the engine merely because a task is hard.
Material direction changes belong to Hendra; prepare evidence first.

A gate outcome is **pass**, **needs rework**, **blocked** or **not attempted**.
Only pass unlocks dependent capability claims. A failed experiment normally
means narrower scope or rework, not project cancellation. A fundamental conflict
with the mission may justify a pause or a recommendation to stop; preserve
source, tests, evidence, unresolved blockers and an exact resumption step.
Avoid both impulsive restarts and continuing solely because effort was spent.

For every implementation increment, completion means:

1. Observable behavior and normative scope are explicit, including exclusions.
2. State ownership, failure behavior and snapshot/debug impact are reviewed.
3. Relevant positive/negative tests were actually run; environment and exact
   commit are recorded, with unrun tests and residual risks stated.
4. Effective diff and fixture/dependency provenance are reviewed.
5. Owning documentation and [project-status.md](project-status.md) are updated.

Record durable decisions in ADRs, not transcripts. Record evidence in the
relevant PR/test report and link it from status rather than copy whole logs.
At each session end, preserve the baseline/head, active package, completed
evidence, blockers, next concrete action and any changed decision. On resumption,
read that status, verify remote heads and open work, then continue the earliest
unfinished dependency. Never infer implementation progress from this plan.
