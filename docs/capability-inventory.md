# Kilox capability inventory

**Purpose:** This is a **capability and readiness checklist**, not another issue/task registry. It covers runtime features, engineering evidence and decisions; their acceptance criteria differ. [Project roadmap](project-plan.md) owns milestones and priorities; [GitHub Issues](https://github.com/H3nb/Kilox/issues) own work and acceptance; [validation gates](validation-plan.md) own experiments; [project status](project-status.md) owns the headline public baseline.

**Baseline:** public `main` at `fa81687`, reviewed 2026-11-20. A checked box means the *precisely worded, bounded outcome* is integrated or accepted with appropriate evidence: original executable tests for runtime behavior, reproducible checks for tooling, and recorded source/decision evidence for research or rights work. It does **not** imply full JVM/CLDC/MIDP support. Unchecked means the whole stated outcome is not complete on public `main`, even if preparatory code or unpublished experiments exist; optional and future unchecked items are **not** mandatory release blockers.

**Scope:** CLDC 1.0/1.1 with tracked 1.1.1 deltas, MIDP 1.0/2.0/2.1, optional JSRs and vendor APIs prioritized by evidence, shared Rust VM on desktop/mobile, and Kilox-specific fresh-process recovery. MIDP 3.0 is deferred. Do not assume modern Java SE semantics or that all optional device protocols are mandatory.

**How to update:** A PR may propose checking a box when final-head evidence appropriate to the item supports it; runtime behavior ordinarily needs original positive/negative guest-visible tests, while documented decisions and toolchain checks need their own reproducible evidence. A proposed check counts as completed only after acceptance into public `main`. Keep evidence in the owning PR/Issue and update [project status](project-status.md). Split broad tasks in their owning Issue when necessary, not into an Issue for every checkbox. Add new normative requirements with their source and exact edition. Unchecked lines may have partial support.

**Source-level audit:** [Java ME standards and Rust dependencies](standards-dependency-audit.md) inventories CLDC 1.1 types, MIDP 2.0 additions, version gaps, current dependencies and deferred dependency decisions. It is **not** a completed member-level conformance matrix.

**Primary entrypoints:** [CLDC 1.1 API](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/overview-summary.html), [MIDP 1.0 API](https://docs.oracle.com/javame/config/cldc/ref-impl/midp1.0/jsr037/overview-summary.html), [MIDP 2.0 API](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/overview-summary.html), [CLDC 1.1.1 maintenance](https://jcp.org/aboutJava/communityprocess/mrel/jsr139/index.html), [MIDP 2.1 maintenance](https://jcp.org/aboutJava/communityprocess/mrel/jsr118/index.html), [MIDP 3 JSR 271](https://jcp.org/aboutJava/communityprocess/final/jsr271/index.html), [version/rights map](specification-map.md), [profile study](profile-evolution.md) and [provenance policy](licensing-and-provenance.md). Follow edition-specific terms before implementation; API documentation is not an implementation-use license.

## Capability checklist
### Scope, legal basis & reproducibility

*M0 / G0*

- [x] Buildable Rust crate, checked-in Cargo.lock and documented fmt/Clippy/test commands
- [x] Original test fixtures and bounded public parser/execution regressions
- [ ] Resolve the applicable specification implementation-use and distribution rights; record the decision in the licensing/specification documents under [M0 #43](https://github.com/H3nb/Kilox/issues/43)
- [ ] Authenticate final versioned JVM/CLDC/MIDP clauses and outstanding N01/N03 differences
- [ ] Reproduce a fresh-checkout baseline on a second machine without proprietary inputs
- [ ] Freeze permission-cleared public fixtures and separately governed private MIDlet corpus
- [ ] Record reproducible admission/verification/launch/interactivity baseline per target profile
- [ ] Independently review unpublished experiments before accepting any public integration


### Classfile format & legacy verifier

*M1 / G1–G2*

- [x] Parse bounded 45.x–48.0 class structures, constant-pool references and Code attributes
- [ ] Complete version-correct names, descriptors, flags, attributes, sizes and structural constraints
- [ ] Decode/validate applicable opcode operands, branches, switches and instruction boundaries
- [ ] Implement CLDC legacy StackMap reading and verifier type propagation (not StackMapTable)
- [ ] Verify stack/local types, joins, exception handlers and uninitialized object types
- [ ] Correctly distinguish ClassFormatError, VerifyError, linkage failure and unsupported features
- [ ] Create an edition-specific opcode/format coverage inventory with original pass/fail fixtures


### Loading, linkage & class lifecycle

*M1 / G2–G5*

- [ ] Resolve classpath/resource names and packages with correct lookup, isolation and precedence
- [ ] Load class/interface hierarchy, access rules, assignability and per-context identities
- [ ] Prepare static fields, constants and runtime class state
- [ ] Resolve fields, methods, interfaces, strings and classes with correct failure timing
- [ ] Implement class initialization, constructors, recursion/failure and synchronization
- [ ] Implement class/resource APIs and distinguish guest errors from host/runtime failures


### Bytecode, values & exceptions

*M1 / G2–G3*

- [x] Execute bounded integer constants/locals/add/subtract/branches/return with step budgets
- [x] Execute bounded same-class invokestatic including nested/recursive VM-owned frames
- [ ] Support required primitive values, category-2 slots, Java conversions and stack operations
- [ ] Match Java arithmetic, overflow, signed shifts, divide-by-zero and floating-point/NaN semantics
- [ ] Implement object identity, heap, instance/static fields, allocation and constructors
- [ ] Implement primitive/reference arrays, array bounds/store/type checks and applicable multiarray ops
- [ ] Implement invokevirtual/invokespecial/invokestatic/invokeinterface dispatch and returns
- [ ] Implement references, null branches, checkcast, instanceof and relevant constants
- [ ] Implement all applicable jump/switch/wide forms and precise bytecode PC semantics
- [ ] Implement athrow, exception-table selection, stack unwinding, catchability and specified errors
- [ ] Preserve error and side-effect ordering under linkage, allocation and invocation


### GC, threads, synchronization & isolation

*M1 / G3–G5*

- [ ] Implement GC, root discovery, stable Java identity and reachable/weak objects
- [ ] Enforce bounded heap, stacks and predictable guest-visible resource errors
- [ ] Implement Thread/Runnable lifecycle, priorities, interrupts, sleep/yield/join
- [ ] Implement virtual preemptive scheduler, safepoints, clock and runnable/blocked queues
- [ ] Implement synchronized methods, monitors, reentrancy, wait/notify/notifyAll
- [ ] Meet applicable CLDC memory-visibility/class-initialization synchronization rules
- [ ] Prove independent VM contexts cannot leak mutable classes, objects or callbacks


### CLDC class libraries and edition differences

*M1 / G5*

- [ ] Audit **every required API class and member** for CLDC 1.0, 1.1 and relevant 1.1.1 deltas
- [ ] Implement java.lang Object, Class, String/StringBuffer, Character, System, Runtime and wrapper types
- [ ] Implement java.lang Math and edition-correct floating-point and numeric APIs
- [ ] Implement Java Throwable/Error/Exception hierarchy and exception messages/identities
- [ ] Implement Thread/Runnable and required runtime utilities without importing Java SE-only APIs
- [ ] Implement java.io byte/data streams, read/write, modified UTF, character streams and errors
- [ ] Implement java.util collections/enumerations, date/calendar/time zones, random and timers
- [ ] Implement java.lang.ref Reference/WeakReference and GC interaction for the required edition
- [ ] Implement CLDC javax.microedition.io base Generic Connection Framework abstractions
- [ ] Provide edition-correct system properties, resource lookups, encoding and locale/time policy
- [ ] Verify CLDC 1.0 vs 1.1 vs 1.1.1 admission, permitted behavior and missing-member errors


### JAR/JAD, installation and AMS

*M0–M2 / G5–G6*

- [x] Read bounded ZIP/JAR stored and DEFLATE entries, validation and CRC
- [x] Read main JAR manifest attributes and folded UTF-8
- [x] Run the public package inventory CLI to count bounded JAR/class structures and presence of legacy `StackMap` attributes (not verification)
- [ ] Parse JAD attributes, validate JAR size/version/vendor, conflicts and manifest reconciliation
- [ ] Select MIDlet-N entrypoints, configuration/profile requirements, classpath and resources
- [ ] Implement install/update/uninstall, suite identity, protected namespaces and invalid-package errors
- [ ] Implement applicable OTA provisioning and notification behavior with controlled test services
- [ ] Implement startApp/pauseApp/destroyApp, notifyPaused/notifyDestroyed/resumeRequest and failures
- [ ] Implement MIDlet.checkPermission and platformRequest, including versioned AMS and host effects
- [ ] Preserve versioned suite sharing versus cross-suite isolation and correct AMS concurrency
- [ ] Separate guest lifecycle, VM debugger pause and host OS background/foreground transitions
- [ ] Report supported guest-visible profile and configuration only, never advertise unavailable APIs


### High-level LCDUI

*M2 / G6–G6C*

- [ ] Implement Display, Displayable, Screen, foreground/visibility, setCurrent, callSerially and Alerts
- [ ] Implement Form, List, TextBox and command navigation
- [ ] Implement StringItem, ImageItem, ChoiceGroup, TextField, DateField, Gauge
- [ ] Implement Ticker and version-specific CustomItem/Spacer and specialized item behaviors
- [ ] Implement Command/CommandListener, ItemCommandListener, ItemStateListener and ordering
- [ ] Implement text editing, constraints, focus, selection, item changes and softkeys
- [ ] Implement form layout, alignment, scrolling, font measurement and resize policies
- [ ] Implement key, pointer and device input mapping without violating guest-visible semantics
- [ ] Implement versioned Display vibration/backlight capabilities and appropriate unsupported behavior


### Canvas, graphics and Game API

*M2 / G6–G6C*

- [x] Provide bounded ARGB pixel surfaces, clipped fill and overlap-safe copy primitives
- [ ] Implement Canvas/GameCanvas painting, repaint/serviceRepaints, flush and key states
- [ ] Implement Graphics geometry, text, fonts, clip, translation, anchor and compositing
- [ ] Implement immutable/mutable Images, RGB data, regions, transforms and resource decoding
- [ ] Implement mandatory PNG support, edition-specific JPEG requirements and explicitly optional image formats
- [ ] Implement Sprite, LayerManager, TiledLayer collisions, frames, transforms and rendering
- [ ] Implement screen size, pixel properties, input capabilities, font metrics and repaint timing
- [ ] Add original pixel-accurate regressions for clipping, signed coords, alpha and region copies


### RMS storage

*M2 / G6–G6R*

- [ ] Implement RecordStore open/create/close/delete and add/get/set/delete records
- [ ] Implement enumeration, filters, comparators, listeners and stable record IDs
- [ ] Implement version-specific suite sharing, access checks and concurrent-open semantics
- [ ] Persist RMS across normal process restarts and handle crash/update/error cases
- [ ] Define quotas, mutation atomicity, corrupted storage recovery and error reporting
- [ ] Reconcile managed RMS versions with checkpoints and forbid unsafe shared-store rewind


### GCF, networking, security & PushRegistry

*M2 / G5–G6C*

- [ ] Implement Connector/protocol dispatch, streams/datagrams, URI rules and exceptions
- [ ] Implement MIDP-required HTTP/1.1 HEAD/GET/POST and header/body/error semantics
- [ ] Implement profile-required HTTPS/secure connection and certificate information where applicable
- [ ] Track optional socket/datagram/server schemes separately from required transports
- [ ] Implement PushRegistry registrations, inbound events, alarm-driven activation and cleanup
- [ ] Implement permissions, trusted/untrusted domains, user consent and denial semantics
- [ ] Implement signed suite verification/trust policy and javax.microedition.pki API surface
- [ ] Keep network calls and callbacks bounded, authorized, cancelable and restorable or rejectable


### MIDP media and host audio

*M2 / G6–G6C*

- [ ] Implement MIDP 2.0 base media API independently from optional full JSR 135 MMAPI
- [ ] Implement Manager tone generation, supported type/protocol discovery and minimum audio rules
- [ ] Implement Player lifecycle, media time, listeners, looping and error ordering
- [ ] Implement required ToneControl/VolumeControl and defined supported formats
- [ ] Isolate audio output handles from VM state and specify checkpoint policy for playing media
- [ ] Measure behavior on host and guest lifecycle transitions with reproducible tests


### VM inspect, snapshot, restore & rewind

*M1–M2 / G2–G6R · Kilox-specific, not Java ME standard requirements*

- [ ] Support consistent read-only frame/PC/locals/stack/heap inspection and breakpoint/exception stops
- [ ] Specify checkpoint consistency for frames, objects, statics, scheduler, monitors and guest runtime state
- [ ] Encode bounded versioned snapshots with code/profile identities and no raw native handles/pointers
- [ ] Validate corrupt/incompatible snapshots completely before activation; preserve old state on failure
- [ ] Restore a paused nontrivial guest in a **different OS process** and compare traces
- [ ] Represent pending host requests with owner, timeline epoch and stale/duplicate callback rejection
- [ ] Implement VM pause/resume and adjustable speed without fabricating MIDlet lifecycle events
- [ ] Decide explicit reconstruct/replay/cancel/refuse policy for external effects and resources
- [ ] Implement bounded rewind/event replay, branch invalidation and RMS consistency only after restore proof
- [ ] Measure snapshot size/latency/resource budget and report actual recoverable scope


### Host services and portability

*M2/M5*

- [ ] Add headless runner with deterministic virtual-time/input/service harness
- [ ] Define thin host-neutral display/input/audio/storage/transport API and ownership
- [ ] Run interactive pilot MIDlets on Windows/Linux with correct input, display and lifecycle
- [ ] Implement Android integration with lifecycle, input, storage, permission and Rust/native ownership
- [ ] Implement iOS adapter and separately assess OS execution and distribution restrictions
- [ ] Test device suspend/resume, process death, loss of resources, scaling and network cancellation
- [ ] Keep logical guest coordinates/clocks independent of host pixel density, wall time and frame pacing
- [ ] Diagnose errors and reclaim resources without leaking mutable VM state across sessions


### Optional JSRs & vendor profiles

*M3/M4 · On demand, not all mandatory*

- [ ] Inventory optional JSR name/version/packages/members and blocked MIDlets before prioritizing
- [ ] Implement [JSR 135 MMAPI #55](https://github.com/H3nb/Kilox/issues/55) beyond MIDP 2.0 base media: selected edition's full guest API surface, controls, protocol/DataSource behavior and resource lifecycle
- [ ] Verify JSR 135 selected audio/video/MIDI/recording capabilities, formats, permission/error semantics, host interactions and checkpoint policy with original fixtures
- [ ] Implement [JSR 184 M3G #56](https://github.com/H3nb/Kilox/issues/56) `javax.microedition.m3g`: required scene/animation/mesh/material/texture/loader API and exact guest object semantics
- [ ] Verify M3G immediate/retained rendering, MIDP Graphics/Image interoperation, pixel output, resource errors and restore policy
- [ ] Implement [Mascot Capsule Micro3D v3 #57](https://github.com/H3nb/Kilox/issues/57) `com.mascotcapsule.micro3d.v3` as a distinct vendor API, including evidenced model/texture/motion and rendering contracts
- [ ] Verify Mascot Capsule v3 device/API variants, 2D/3D compositing, pixel/animation outcomes, bounded assets and restore behavior
- [ ] Decide scope of remaining optional JSRs (e.g., JSR 75, JSR 120/205, JSR 226) from measured demand
- [ ] Decide scope of JSR 82 Bluetooth, JSR 179 Location, JSR 211 CHAPI, SATSA and additional JSRs from measured demand
- [ ] Inventory device/vendor packages and behaviors (Nokia, Sony Ericsson, Samsung, Siemens, Motorola, SoftBank, etc.)
- [ ] Use versioned device profiles for fonts/keymap/graphics/media/timing/storage/quirks
- [ ] Keep unsupported optional APIs and unavailable external services visible to users/test reports
- [ ] Validate each supported extension with original regressions and permitted representative workloads


### Conformance inventory, compatibility & release

*M2–M6 / G6C–G7*

- [ ] Maintain a versioned matrix of API and behavioral differences across all targeted CLDC/MIDP editions
- [ ] Audit every required *package/type/member* and permitted opcode + verifier family against authoritative edition-specific material
- [ ] Attach committed positive/negative guest behavior evidence to each claimed completed capability
- [ ] Freeze scenario corpus, eligibility, duplicate policy, version, device and permission boundaries
- [ ] Validate launch, meaningful interaction, lifecycle, storage, media/network as demanded by scenario
- [ ] Keep errors and undetermined cases visible; separate optional coverage, host performance and recovery
- [ ] Track first root cause of blocked apps and continuously rerun a fixed baseline after fixes
- [ ] Report pass/fail/undetermined by profile and platform; do not equate checked lines with compatibility %
- [ ] Validate 99.99% only for defined population with defensible sampling and uncertainty
- [ ] Audit licenses, fixture provenance, release dependencies, trademark and distribution scope before publishing


### MIDP 3.0 future (not a legacy-release requirement)

*Future after G7*

- [ ] Authenticate JSR 271 final and inventory its mandatory APIs and normative behavior
- [ ] Support per-MIDlet context ownership and backward-compatible legacy lifecycle only after clause review
- [ ] Implement LIBlets, dependency binding and expanded AMS/security/sharing as required
- [ ] Validate versioned CLDC 1.1.1/CDC contexts, concurrency, media and new profile semantics
- [ ] Run separate MIDP 3 evidence while retaining all MIDP 1–2.1 regressions
- [ ] Revisit JIT, new rendering engines and snapshot migration only against measured goals

## Completeness and acceptance gate

This inventory deliberately does **not** claim a complete list of every Java ME method, operand, security rule, or device obligation. Before claiming an entire edition supported, reconcile its authentic final **normative clauses** and required **classes/interfaces/members** against a separate version-aware audit, including allowed classfile/opcode/verifier behavior, errors, packaging/AMS, security, lifecycle, media, networking, and host capabilities. The same required function can have different semantics in MIDP 1.0, 2.0, 2.1 and 3.0. Keep unresolved clauses as blockers, not inferred requirements.

Passing Rust checks prove only the tested Rust contracts. The repository's automated Rust workflow was removed from `main` at `e1f4076`; do not assume a passing historical PR workflow is a current merge/release gate. A checklist percentage is **not** a compatibility score or a conformance claim. A completed broad subsystem requires relevant guest-observable tests on the accepted `main` commit, negative cases, documented limitations and profile-specific evidence. Use [validation gates](validation-plan.md) and the [roadmap measurement contract](project-plan.md#compatibility-measurement), not arbitrary issue counts, for release decisions.

Current dependencies: [M0 #43](https://github.com/H3nb/Kilox/issues/43) (including unresolved implementation-use rights), [M1 #44](https://github.com/H3nb/Kilox/issues/44), and [object experiment #33](https://github.com/H3nb/Kilox/issues/33). Any privately demonstrated capability stays unchecked until approved public integration.
