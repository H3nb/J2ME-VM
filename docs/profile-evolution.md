# MIDP 1.0–2.1 delivery and MIDP 3.0 architecture study

**Date:** 2026-10-29. **Status:** Planning study; no new runtime capability.
The product direction is MIDP 1.0–2.1 compatibility first, with a foundation
that can evolve toward MIDP 3.0 on CLDC. This does not replace the small G2/G3
proof with a full MIDP 3 implementation. [Architecture](architecture.md) owns
the resulting contracts; [validation-plan.md](validation-plan.md) owns gates.

## 1. Evidence and limitations

Read only specifications/API descriptions and original project material, not
emulator implementations. Some MIDP 3 API pages below are mirrors of the
Motorola/Sun final-release Javadoc, not official hosting. They are useful
primary-authored API text, but archive identity and the complete normative
chapters must be checked against an appropriately accessed final release
before implementing affected behavior. No specification PDF, SDK class or
third-party sample is copied into this repository.

| ID | Source inspected / locator | Finding used in this plan |
| --- | --- | --- |
| S01 | [MIDP 1.0 MIDlet package](https://docs.oracle.com/javame/config/cldc/ref-impl/midp1.0/jsr037/javax/microedition/midlet/package-summary.html), lifecycle definitions/states | Lifecycle is an application/AMS protocol, distinct from suspending VM execution. |
| S02 | [MIDP 2.0 MIDlet package](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/midlet/package-summary.html), MIDP Execution Environment | MIDlets in a suite share an execution environment and class/object state. |
| S03 | [MIDP 2.1 MR2 change log](https://www.jcp.org/aboutJava/communityprocess/maintenance/jsr118/JSR_118_MR2_Changelog.pdf), accepted rows 3, 22, 23, 36, 38 | Changes include LayerManager error ordering, runtime declaration, exception ownership, advertised-profile rules and JPEG support. Rejected rows are not requirements. |
| S04 | [CLDC maintenance record](https://jcp.org/aboutJava/communityprocess/maintenance/jsr139/139ChangeLog.html) and [linked change text](https://jcp.org/aboutJava/communityprocess/maintenance/jsr139/CHANGE_LOG.txt), proposed sections 5.4/5.3.4 | The accepted-review entry links text still headed “Proposed”. It calls for resolution LinkageError to become observable at use, even with eager linking. Reconcile final text before coding. |
| S05 | [MIDP 3 MIDlet package](https://nikita36078.github.io/J2ME_Docs/docs/midp-3_0-fr-javadoc/javax/microedition/midlet/package-summary.html), MIDlets / Execution Environment / Security (mirrored final API) | Execution/class state is per MIDlet; LIBlet binding executes in its consumer context. CLDC 1.1.1 support is required; CDC is optional. |
| S06 | [MIDP 3 MIDlet class](https://nikita36078.github.io/J2ME_Docs/docs/midp-3_0-fr-javadoc/javax/microedition/midlet/MIDlet.html), pauseApp, notifyPaused, destroyApp (mirrored final API) | Pause semantics differ: AMS does not call pauseApp; notifyPaused has a compatibility action, not a generic no-op. |
| S07 | [MIDP 3 RMS package](https://wupp.cc/j2meapi/JSR_271_Mobile_Information_Device_Profile_3_0/javax/microedition/rms/package-summary.html), Record Store / sharing (mirrored final API) | Persistent namespaces and shared-store updates/listeners cross application boundaries; VM isolation does not eliminate storage coupling. |
| S08 | [MIDP 3 overview](https://nikita36078.github.io/J2ME_Docs/docs/midp-3_0-fr-javadoc/overview-summary.html), Sound; [JCP technical overview](https://jcp.org/aboutJava/communityprocess/ec-public/materials/2010-01-1213/MIDP_3.0_technical_overview.pdf), slides 3–6 | Full MIDP 3 includes MMAPI 1.1 and concurrency requirements; a handful of new classes is not full support. Slides are informative, not implementation authority. |

Official acquisition points: [MIDP 2.1](https://download.oracle.com/otndocs/jcp/midp-2.1-mrel-oth-JSpec/),
[MIDP 3 final](https://jcp.org/aboutJava/communityprocess/final/jsr271/index.html),
[CLDC maintenance](https://jcp.org/aboutJava/communityprocess/mrel/jsr139/index.html).
Existing [licensing/provenance policy](licensing-and-provenance.md) still applies.
The later MIDP 2.2 maintenance changes are not evidence for 2.1 requirements.

## 2. Consequences for our design (original analysis)

Do not equate a suite, a MIDlet instance, an execution context, a guest thread
and a host process. Those identities serve different lifetimes. Runtime policy
maps applications into contexts; the core owns execution inside a context.
The exact legacy-on-MIDP-3 compatibility mapping remains an open normative
question, not an excuse to guess a rule from the JAD version alone.

| Decision | Prepare now | Implementation intentionally deferred |
| --- | --- | --- |
| Context ownership | Per-instance VM state; explicit class/statics/heap owner; no process-global active MIDlet or resolver | General multi-application AMS, quotas and concurrent MIDlet product UI |
| Class identity | Separate immutable class bytes from mutable loaded-class state; references resolve in their owning context | LIBlet installation, dependency validation, binding rules and shared-code optimizations |
| Linking | Keep structural validity, verification, resolution and initialization distinct; record errors at the contractually correct execution boundary | Full CLDC 1.1.1 class library and unneeded optional APIs |
| Runtime selection | Distinguish declared requirements, selected semantics/revision, device capabilities and vendor quirks | Universal version switch or automatic per-title heuristics |
| Lifecycle | Separate AMS transitions, display visibility, guest thread blocking and VM control pause | MIDP 3 background/auto-start/event APIs |
| Host security | Carry owner identity through requests and async continuations; authorize before effects | Complete certificates, trust domains and class-based permissions |
| Recovery | Identify a session's state dependencies, not just one heap | Live multi-application group rewind or external transaction rollback |

Start with owned fields and small explicit arguments. No speculative context
trait hierarchy, plugin system, workspace split or security-framework stubs.
Two independently created core instances are a useful isolation test; they do
not prove MIDP 3 concurrent application conformance or decide host-process layout.

### Versioned behavior, without accidental substitution

Resolve an application to a documented runtime contract at admission. Record
the selected semantics, configuration/revision, API set, device behavior and
code identities for diagnostics and recovery. Unsupported required contracts
must fail explicitly. Do not announce MIDP 3 because some classes are present.
Do not construct microedition.profiles by concatenating all supported historical
versions: validate guest-visible reporting against the selected contract (S03).

Keep legacy execution tests distinct from tests of legacy content on a future
MIDP 3 runtime. The former does not prove the latter. A modern runtime may have
explicit compatibility rules that differ from a historical device's behavior.
Do not remove a legacy callback because its MIDP 3 counterpart is deprecated.

### Recovery consistency domain

Our proposed recovery unit is a session plus the state needed to continue it
coherently: contexts, pending completions, queued messages, managed storage
versions and shared logical resources. This is a design requirement, not a
MIDP-defined save-state feature. Initially support an isolated session only.

If A has sent a message consumed by B, restoring A alone may repeat a consumed
effect. If A and B share a writable store, rewinding one heap may leave it
inconsistent with that store and pending notifications. Before enabling either
interaction, select one supported policy: checkpoint the participating state
together, provide a proven isolated/replay boundary, or reject recovery while
the unsupported coupling exists. Merely pausing peers during restore does not
repair already-consumed messages. Never silently undo another active session's
persistent data. Unknown dependencies make checkpoint support unavailable.

## 3. Bounded experiments and decision deadlines

These are test designs, not executed tests or reasons to build APIs early.
Implement each only when its owning state exists; reuse the normal test gate.

| ID | Experiment and distinguishing failure | When / acceptance boundary |
| --- | --- | --- |
| E01 | Run the same original class in two core instances; mutate statics/objects in one and stop it. The other retains its values and remains usable. | G2 as mutable class/heap state appears; repeat restoration isolation at G3. Catches global state leakage, not MIDP conformance. |
| E02 | Original optional-class reference: unused reference does not fail prematurely; executing the guarded use exposes the correct catchable linkage failure. Also reject malformed/invalid executed input normally. | Before selecting resolver behavior in WP02; implementation after final CLDC clauses are resolved. Catches conflating resolution failure with structural rejection. |
| E03 | Freeze one context with a pending host request; stop/restore it while another exists; deliver a stale completion and verify no cross-context mutation. | G3H; owner plus timeline identity required. No MIDP scheduler needed. |
| E04 | Trace lifecycle calls under the selected legacy contract; separately specify MIDP 3 traces after normative closure. Verify VM debugger pause does not invent application callbacks. | G6, then G6C; MIDP 3 trace execution deferred. Includes repeated start/pause and failure paths as applicable. |
| E05 | Two legacy MIDlets in one suite exercise the confirmed sharing contract; a distinct suite cannot acquire their private references. Separate MIDP 3 fixture expects its confirmed per-MIDlet context rules. | G6C for legacy behavior; MIDP 3 implementation later. Never reuse one expectation across versions without authority. |
| E06 | Admission with unsupported profile/configuration fails; supported fixture sees correct properties and API behavior. MIDP 2.1 boundary case checks a selected accepted MR2 change. | G5 contract, G6C proof; record exact editions and exclusions. |
| E07 | Managed-store write and pending notification are checkpointed; a coupling outside the declared recovery unit causes explicit refusal. Failed restore leaves current data and sessions intact. | G6/G6R for supported local storage; full cross-application messaging test before later admission. |

Legacy release readiness means a versioned behavior/capability matrix, representative
lawful workload evidence and no unresolved release-blocking defects in the declared
scope. It does not mean every historical game runs, nor is a percentage of passing
games a standards conformance result. Classify failures into core semantics,
profile/API behavior, vendor extensions, host effects and unknown causes.

## 4. Outstanding normative work (must not be silently assumed)

| ID | Unresolved question | Needed evidence / deadline |
| --- | --- | --- |
| N01 | Exact final CLDC 1.1 versus 1.1.1 verification, linking and library deltas | Versioned clause comparison, especially final 5.4/5.3.4; before affected WP02 behavior |
| N02 | Legacy suite sharing versus running legacy declarations under MIDP 3, including concurrency/lifecycle compatibility rules | Full JSR 271 final concurrency and compatibility chapters; before AMS/context mapping is fixed, no later than WP06 design |
| N03 | Complete MIDP 2.1 accepted changes and their final wording | Final 2.1 text plus maintenance status, not a generic “2.x” bucket; before affected runtime API and G6C claims |
| N04 | Binding/class resolution order, resource lookup and permission propagation for LIBlets | Final LIBlet/binding/security chapters and API contracts; before LIBlet implementation; current ownership model must permit the eventual rule |
| N05 | Complete MIDP 3 mandatory/optional capability and resource obligations | Final requirement inventory, including media, concurrency and deployment; before a MIDP 3 implementation milestone or support claim |

These gaps do not block all work. Pure parser fixes and unrelated proven bytecode
semantics can continue. They block dependent assumptions. Record closure in the
owning specification/architecture contract with clauses, rationale and evidence.

## 5. Delivery sequence

Preserve G2 → G3 → G3H → G4 → G5 → G6 → G6R. Add **G6C**, a legacy
compatibility/stabilization gate for the declared MIDP 1.0/2.0/2.1 scope, before
the G7 product decision. Architectural research runs alongside these increments
at the deadlines above; no parallel MIDP 3 implementation branch is needed.
After legacy evidence and the G7 review, prepare a separate bounded MIDP 3
increment in this repository. Retain legacy regression suites throughout.

No JIT, CDC or OSGi requirement is introduced by this plan. JIT remains a measured
optimization decision. Supporting MIDP 3 requires its own runtime contracts and
evidence; faster bytecode execution is not proof of those capabilities.
