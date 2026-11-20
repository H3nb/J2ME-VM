# Java ME standards and Rust dependency audit

**Reviewed:** 2026-11-20. Public Kilox baseline `fa81687` (source, manifest and lockfile); [capability checklist](capability-inventory.md) was merged in [#52](https://github.com/H3nb/Kilox/pull/52). This is an **interface/architecture audit**, not a legal opinion, executed conformance test or claim of comprehensive signature-level verification. Recheck the live base before integration.

## Decision summary

- **Current critical path:** source-use/rights decision tracked in [M0 #43](https://github.com/H3nb/Kilox/issues/43), reproducible baseline [#43](https://github.com/H3nb/Kilox/issues/43), legacy `StackMap` verification and the CLDC loader/exception/constructor semantics [#44](https://github.com/H3nb/Kilox/issues/44). Do not fill these gaps with third-party VM internals.
- **No additional Rust crates now.** The existing `rawzip` + `miniz_oxide` + `adler2` graph covers current archive parsing. Most missing functionality is guest semantics and class libraries, not host-side dependency infrastructure.
- **Distinguish scope from delivery:** MIDP 1.0/2.0/2.1 is the declared legacy product target; optional JSRs/vendor behaviors need measured demand; MIDP 3.0 remains a future release. Offline MIDlet launching is a nearer engineering goal than claiming a standards-compliant phone.
- **Audit boundary:** CLDC 1.1 public API *type names* and the MIDP 2.0 documented additional types are inventoried below. Complete member signatures, normative exceptions, overloads and versioned behavior must still be reconciled against appropriately accessed final specifications before any full-support claim. Neither `allclasses-noframe.html` nor an API jar establishes conformance.

## Public implementation versus required runtime

| Layer | Evidence in public `main` | Unimplemented integration boundary |
| --- | --- | --- |
| Java class files | Bounded 45.x–48.0 structural parser, attributes/constant-pool references | Full format rules, bytecode verification, legacy `StackMap`, name/descriptor policy |
| Interpreter | Narrow integer/branch subset, VM-owned frames and same-class `invokestatic` | General bytecodes, class hierarchy/linkage, heap, arrays, exception propagation, object identity |
| Packages | Bounded ZIP/JAR, DEFLATE, CRC, manifest main attributes; `package_inventory` example | JAD reconciliation, executable classpath/suite admission, class loading, app selection |
| Pixels | Bounded offscreen ARGB surfaces with fill/blit/copy | LCDUI `Image`/`Graphics`, fonts, input, event delivery, presentation |
| Guest libraries / platform | No integrated general CLDC/MIDP class library, AMS, RMS or media stack | Everything needed to run a normal MIDlet through the public core |
| Persistence and recovery | Experimental evidence separately recorded, **not integrated** | Bounded durable checkpoints and fresh-process state/effect reconciliation |

Source inspection: [classfile.rs](../src/classfile.rs), [execution.rs](../src/execution.rs), [archive.rs](../src/archive.rs), [surface.rs](../src/surface.rs), [Cargo.toml](../Cargo.toml), [project status](project-status.md). A host-neutral pixel primitive is not the guest Java `Image` API.

## CLDC 1.1 API type inventory (81)

Baseline: Oracle-hosted [CLDC 1.1 package overview](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/overview-summary.html) and [all-classes index](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/allclasses-noframe.html). Counts below include interfaces, classes, errors and exceptions. **None of these entire guest package implementations is verified on public `main`**; Rust structs with similar names do not qualify.

| Package | Types | Names in the 1.1 API |
| --- | ---: | --- |
| `java.io` | 18 | `DataInput`, `DataOutput`, `ByteArrayInputStream`, `ByteArrayOutputStream`, `DataInputStream`, `DataOutputStream`, `InputStream`, `InputStreamReader`, `OutputStream`, `OutputStreamWriter`, `PrintStream`, `Reader`, `Writer`, `EOFException`, `InterruptedIOException`, `IOException`, `UnsupportedEncodingException`, `UTFDataFormatException` |
| `java.lang` | 41 | `Runnable`, `Boolean`, `Byte`, `Character`, `Class`, `Double`, `Float`, `Integer`, `Long`, `Math`, `Object`, `Runtime`, `Short`, `String`, `StringBuffer`, `System`, `Thread`, `Throwable`, `ArithmeticException`, `ArrayIndexOutOfBoundsException`, `ArrayStoreException`, `ClassCastException`, `ClassNotFoundException`, `Exception`, `IllegalAccessException`, `IllegalArgumentException`, `IllegalMonitorStateException`, `IllegalThreadStateException`, `IndexOutOfBoundsException`, `InstantiationException`, `InterruptedException`, `NegativeArraySizeException`, `NullPointerException`, `NumberFormatException`, `RuntimeException`, `SecurityException`, `StringIndexOutOfBoundsException`, `Error`, `NoClassDefFoundError`, `OutOfMemoryError`, `VirtualMachineError` |
| `java.lang.ref` | 2 | `Reference`, `WeakReference` |
| `java.util` | 10 | `Enumeration`, `Calendar`, `Date`, `Hashtable`, `Random`, `Stack`, `TimeZone`, `Vector`, `EmptyStackException`, `NoSuchElementException` |
| `javax.microedition.io` | 10 | `Connection`, `ContentConnection`, `Datagram`, `DatagramConnection`, `InputConnection`, `OutputConnection`, `StreamConnection`, `StreamConnectionNotifier`, `Connector`, `ConnectionNotFoundException` |
| **Total** | **81** | **Type names only, not member coverage** |

Edition obligations: CLDC 1.0 has no general `float`/`double` capability; CLDC 1.1 adds floating point and `java.lang.ref` weak references. Validate other changed and edition-specific API signatures instead of assuming that 1.1 behavior can be advertised as 1.0. Treat CLDC 1.1.1's linking/verifier maintenance deltas as **unresolved** pending exact final normative material ([N01](profile-evolution.md#4-outstanding-normative-work-must-not-be-silently-assumed)).

## MIDP 2.0 API additions, excluding the CLDC 1.1 list (66)

This table enumerates **newly listed types** relative to the CLDC 1.1 API above, derived from the [MIDP 2.0 overview](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/overview-summary.html), [all-classes index](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/allclasses-noframe.html) and individual package summaries. It does **not** say every connection protocol is mandatory or every listed type behaves identically across CLDC configurations.

| Package | Additional types | Names appearing beyond the CLDC 1.1 index |
| --- | ---: | --- |
| `java.lang` | 1 | `IllegalStateException` |
| `java.util` | 2 | `Timer`, `TimerTask` |
| `javax.microedition.io` | 9 | `CommConnection`, `HttpConnection`, `HttpsConnection`, `SecureConnection`, `SecurityInfo`, `ServerSocketConnection`, `SocketConnection`, `UDPDatagramConnection`, `PushRegistry` |
| `javax.microedition.lcdui` | 27 | `Choice`, `CommandListener`, `ItemCommandListener`, `ItemStateListener`, `Alert`, `AlertType`, `Canvas`, `ChoiceGroup`, `Command`, `CustomItem`, `DateField`, `Display`, `Displayable`, `Font`, `Form`, `Gauge`, `Graphics`, `Image`, `ImageItem`, `Item`, `List`, `Screen`, `Spacer`, `StringItem`, `TextBox`, `TextField`, `Ticker` |
| `javax.microedition.lcdui.game` | 5 | `GameCanvas`, `Layer`, `LayerManager`, `Sprite`, `TiledLayer` |
| `javax.microedition.media` | 6 | `Control`, `Controllable`, `Player`, `PlayerListener`, `Manager`, `MediaException` |
| `javax.microedition.media.control` | 2 | `ToneControl`, `VolumeControl` |
| `javax.microedition.midlet` | 2 | `MIDlet`, `MIDletStateChangeException` |
| `javax.microedition.pki` | 2 | `Certificate`, `CertificateException` |
| `javax.microedition.rms` | 10 | `RecordComparator`, `RecordEnumeration`, `RecordFilter`, `RecordListener`, `RecordStore`, `InvalidRecordIDException`, `RecordStoreException`, `RecordStoreFullException`, `RecordStoreNotFoundException`, `RecordStoreNotOpenException` |
| **Total additions** | **66** | **Type names only, not member coverage** |

**Index cross-check:** The standalone MIDP 2.0 `allclasses` index lists **145** type names. The union of the CLDC 1.1 index (81) and additions above (66) is **147** because MIDP's published `allclasses` index does not list CLDC 1.1's two `java.lang.ref` types. This is an artifact of separate published API indexes, **not evidence that MIDP 2.0 forbids weak references on a CLDC 1.1 device**. The selected configuration determines the core library surface. Do not add 81+145 or infer missing methods from the index alone.

Notable mandatory vs optional **behavior** distinctions from MIDP 2.0 package descriptions:

| Subject | Baseline behavior to validate | Do not infer |
| --- | --- | --- |
| `http:` | HTTP/1.1 HEAD, GET, POST and guest-visible stream/headers/errors | That all TCP socket or UDP schemes are required |
| `https:`, certificates | MIDP 2.0 secure networking and the X.509 `pki` contracts | That old algorithms/certificates can be trusted without a host security policy |
| `javax.microedition.media` | Built-in **audio-only** subset: `Manager`, `Player`, tone and controls | Full JSR 135 video/custom DataSource support |
| LCDUI/Game API | `GameCanvas`, `Sprite`, `LayerManager`, `TiledLayer` are MIDP 2.0 APIs | That these existed in MIDP 1.0 |
| RMS | Versioned record identity, store lifecycle, listeners, permissions, persistence | That saving an RMS file automatically makes whole-VM rewind safe |
| Push/AMS/security | Registration, lifecycle, permission failures, trusted/untrusted execution | That every optional inbound physical transport is present |

Sources: [GCF](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/io/package-summary.html), [media](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/media/package-summary.html), [lifecycle](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/midlet/package-summary.html), [RMS](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/rms/package-summary.html), [PKI](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/pki/package-summary.html).

## Edition reconciliation still required

| Edition | Audited here | Open source / behavior question |
| --- | --- | --- |
| CLDC 1.0 / JSR 30 | Configuration boundary, no floating point; basic Java packages + GCF | Full 1.0 type/member delta against 1.1 and 1.0-specific verifier/instructions |
| CLDC 1.1 / JSR 139 | Five-package, 81-type public interface map | Every constructor/field/method descriptor, exceptions, overloads and normative VM behavior |
| CLDC 1.1.1 / JSR 139 maintenance | Version is distinct, linking/verifier change records known | Authenticated **final** maintenance wording and exact deltas; do not promote draft changes to law |
| MIDP 1.0 / JSR 37 | Four profile package families (`lcdui`, `rms`, `midlet`, `io`) | Complete 1.0 member/behavior delta, packaging, LCDUI, HTTP and security sandbox |
| MIDP 2.0 / JSR 118 | Published index/66 additional type names and key mandatory-vs-optional boundaries | Full API member matrix, lifecycle/event/error contracts, packaging, security and media behavior |
| MIDP 2.1 / JSR 118 maintenance | Relevant MR2 change log and profile study | Exact accepted final 2.1 clauses and differences (including JPEG/image, property reporting, error timing). Do not import 2.2 changes |
| MIDP 3.0 / JSR 271 | Architecture-awareness only | Separate future full API/behavior audit, not a legacy completion gate |

Official entrypoints: [JSR 30](https://jcp.org/en/jsr/detail?id=30), [JSR 37](https://jcp.org/aboutJava/communityprocess/final/jsr037/), [JSR 139](https://jcp.org/aboutJava/communityprocess/final/jsr139/), [JSR 118](https://jcp.org/aboutJava/communityprocess/final/jsr118/), [JSR 118 maintenance](https://jcp.org/aboutJava/communityprocess/mrel/jsr118/index.html). Historical materials' access and implementation-use rights remain unresolved; see [M0 #43](https://github.com/H3nb/Kilox/issues/43) and the [licensing record](licensing-and-provenance.md).

### Required member-level verification method

For **each selected edition** maintain an independently derived, version-aware entry for every public type's **constructor, field/constant, method signature, inherited required behavior, exceptions, permissions and profile-specific additions/removals**. Classify each as `absent`, `declared only`, `behavior implemented`, or `verified`, with a link to original positive/negative evidence and the tested commit. A dispatch stub must not count as implemented. Derive interface facts from permitted primary API/JSR materials, not implementation jars or other VMs.

This audit has **not** enumerated or verified all member signatures. A whole-edition checkbox remains open until that separate audit is completed. Avoid producing thousands of unreviewed placeholder Issues or automatic claims from compiler symbol resolution.

### Member-level spot checks (non-exhaustive)

These representative signatures were checked against the published Oracle API pages. They demonstrate the **kind of exact contract** required by the full member audit. Their guest implementations are **not integrated** on public `main`; the separate local experiment reported for `String` and `Reader` is not a checked public capability.

| Guest owner / examples | Required observable distinction | Current public-main result |
| --- | --- | --- |
| [`java.lang.String`](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/lang/String.html): `indexOf(String)`, `indexOf(String,int)` | Overloads, null argument, start offset and empty-substring behavior are distinct from the `int` character overloads | Not integrated |
| [`java.io.Reader`](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/io/Reader.html): `Reader()`, `Reader(Object)`, `read()`, `read(char[],int,int)`, `close()` | Abstract methods, blocking/error semantics, synchronization lock, character count and EOF | Not integrated |
| [`java.lang.Thread`](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/lang/Thread.html): `start()`, `run()`, `join()`, `interrupt()`, `sleep(long)` | Lifecycle, scheduling, interruption and monitor interaction rather than host OS thread substitution | Not integrated |
| [`MIDlet`](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/midlet/MIDlet.html): `startApp()`, `pauseApp()`, `destroyApp(boolean)`, `getAppProperty(String)`, `checkPermission(String)`, `platformRequest(String)` | Lifecycle error paths, trusted/untrusted JAD-manifest precedence, permission states and host request results | Not integrated |
| [`Display`](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/lcdui/Display.html): `getDisplay(MIDlet)`, `callSerially(Runnable)`, `flashBacklight(int)`, `vibrate(int)` | One Display per MIDlet, serialized callback ordering, capability-based behavior and background restrictions | Not integrated |
| [`RecordStore`](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/rms/RecordStore.html): `openRecordStore(String,boolean)` and MIDP 2.0 shared-store overloads | Suite identity, exceptions, authorization, record-name limits, object reuse and durability | Not integrated |

This is a **spot check**, not a six-class conformance certificate; all inherited members, overloads and normative errors must still be audited for the selected edition.

## Core execution and verifier coverage (group-level)

| Required family / contract | Public main observation | Next proof |
| --- | --- | --- |
| Operand stack, locals, `int` arithmetic, comparisons/branches | **Partial** bounded `int` subset with explicit frames | General return/slot/type semantics, full applicable opcode families |
| Long/double, floats, shifts, conversions, remaining arithmetic | Not in public integer proof | Original Java numeric-edge fixtures per edition |
| Object allocation, fields, arrays, identity, class initialization | Not integrated | Loader/ref heap/GC + constructor/initializer tests |
| Invocation and exceptions | Same-class static calls only | Virtual/special/interface dispatch; exception edges/handler entry |
| Legacy `StackMap` bytecode verification | Structural attribute exposure only | Positive/negative type states, handler edges, uninitialized references and correct errors |
| Virtual threads, monitors, memory visibility | Not integrated | Scheduler/monitor/interrupt/safepoint invariants |
| Complete guest libraries and host requests | Not integrated | Versioned CLDC/AMS/GCF/RMS/media guest behavior |
| Checkpoint/restore | Separate unpublished experiments | New-process verified, atomic/validated restore and host effect policies |

Do not equate a verified class *parser* with a verified *class*, or present an opcode count as guest compatibility. Treat target-edition opcode exclusions explicitly rather than supporting newer SE features accidentally. The existing [validation gates](validation-plan.md) already own acceptance and remain authoritative.

## Dependency audit: current lockfile

Actual [Cargo.toml](../Cargo.toml) has **two unique direct crates** and a feature-expanded dev use of `rawzip`; [Cargo.lock](../Cargo.lock) resolves only **three** external crates. There is no `serde`, Tokio, GC crate, graphics toolkit, TLS implementation, mobile binding, or Java SDK runtime dependency yet.

| Crate | Locked | Use / feature decision | License choice and current decision |
| --- | --- | --- | --- |
| `rawzip` | 0.5.2 | ZIP record parsing; default features disabled; `std` enabled only as a dev feature | MIT; keep (current function and small dependency graph). Declares MSRV Rust **1.85** |
| `miniz_oxide` | 0.9.1 | Raw DEFLATE decode; default features disabled | MIT OR Zlib OR Apache-2.0; keep for bounded decompression |
| `adler2` | 2.0.1 | Transitive dependency of `miniz_oxide` | 0BSD OR MIT OR Apache-2.0; keep transitively |

Metadata: [rawzip 0.5.2](https://docs.rs/crate/rawzip/0.5.2/source/Cargo.toml.orig), [miniz_oxide 0.9.1](https://docs.rs/crate/miniz_oxide/0.9.1/source/Cargo.toml.orig), [adler2 2.0.1](https://docs.rs/crate/adler2/2.0.1/source/Cargo.toml.orig). A permissive selectable license is **potentially compatible** with GPL-3.0-only; preserve exact notices and check redistribution obligations. The crate's `#![forbid(unsafe_code)]` does not automatically ban `unsafe` in transitive dependencies.

**Security review status:** inspect bounds and decompression limits at the archive boundary, but a current RustSec/OSV audit and source-level review of these exact crate versions were **not executed** in this documentation pass. No blanket claim of zero advisories, safety or current version is made. Prefer `cargo audit` when available for a targeted check, and only upgrade with tests/provenance review. Do not confuse advisories concerning the unrelated `zip` crate with `rawzip`.

## Named optional graphics/media tracks

The following are now **explicit planned compatibility targets**, not merely examples in an optional API list. Their *existence* in the roadmap is a product decision; supported versions, device capabilities, implementation sequence and release evidence remain subject to the relevant contracts and tests.

| Track | Guest API / important boundary | Dependency and acceptance policy |
| --- | --- | --- |
| [JSR 135 MMAPI #55](https://github.com/H3nb/Kilox/issues/55) — M3 | Full selected-edition `javax.microedition.media`, `javax.microedition.media.control`, `javax.microedition.media.protocol`; includes media lifecycle, controls, optional formats and `DataSource` where specified. MIDP 2.0 has a **subset**, not full MMAPI. | Preserve guest `Player`/events/permission semantics independently of audio or video backends. Evaluate codec, MIDI, capture and device/audio output dependencies **per supported format** and platform. |
| [JSR 184 M3G #56](https://github.com/H3nb/Kilox/issues/56) — M3 | Standard `javax.microedition.m3g`: 3D scene graph, loading, animation, transforms, texturing, immediate/retained rendering and MIDP graphics interoperation. | Select and verify host-neutral render contracts before considering software/GPU libraries. GPU resources cannot be serialized as stable snapshot handles. JSR 297/M3G 2.0 is a separate version decision. |
| [Mascot Capsule Micro3D v3 #57](https://github.com/H3nb/Kilox/issues/57) — M4 | Proprietary HI Corporation `com.mascotcapsule.micro3d.v3` with separately evidenced API, asset/effect semantics and handset variants; **not** JSR 184. | Review allowed documentation/implementation rights and clean-room provenance. Share internal render primitives with M3G **only if equivalent behavior is proved**, never import another VM/emulator implementation. |
 
Standards/reference entrypoints: [JCP JSR 135](https://jcp.org/en/jsr/detail?id=135), [Oracle full MMAPI API](https://docs.oracle.com/javame/config/cldc/opt-pkgs/api/mm/jsr135/overview-summary.html), [MIDP 2.0 media subset](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/javax/microedition/media/package-summary.html), [JCP JSR 184](https://jcp.org/aboutJava/communityprocess/final/jsr184/), [Oracle M3G explanation](https://www.oracle.com/technical-resources/articles/javame/3dgraphics.html). Mascot Capsule v3 is proprietary: historical Sony Ericsson documentation is supporting interface context, **not** a grant to copy engine sources or proprietary API libraries.

**No new Cargo dependencies admitted by this planning change.** Required decoder, rendering or native-host components should be chosen when a concrete tested feature calls for them, with transitive license/security/MSRV review and a verified state/host boundary.

## Future dependency decisions (no additions authorized by this audit)

| Need / owning milestone | First-choice approach | Consider an external crate **only when** |
| --- | --- | --- |
| JVM heap, GC, class lookup, virtual scheduler (M1) | Small explicit Rust-owned representations, bounded state and original tests | A measured correctness or maintenance gap justifies one; generic GC crates may conflict with stable snapshot identities |
| Library strings/UTF, collections, math (M1) | Correct guest-visible implementation from verified contracts using Rust primitives | A demonstrable encoding/normalization/math need cannot be met reliably with existing code; never import proprietary Java guest libraries |
| Checkpoint encoding (M1/M2) | Explicit versioned schema, bounded parser, staged atomic restore | Serialization crate can satisfy all validation/identity requirements; `serde` alone does **not** provide safe snapshots |
| PNG/JPEG decode and text/font raster (M2) | Host-neutral image semantics and original pixel tests; independently evaluated pure-Rust codec if needed | Actual mandatory image formats and exact error/alpha/colour behavior require it; review transitive licenses and hostile-input risk |
| RMS persistence (M2) | Minimal versioned files with atomicity and suite isolation first | A demonstrated concurrency/transaction need makes a storage engine worthwhile across supported hosts |
| HTTP/HTTPS and certificates (M2) | Explicit host request boundary, permissions, timeouts and errors | Selected desktop/mobile transport and trust policy demand a TLS/HTTP backend; investigate host TLS or independently reviewed Rust libraries then |
| Audio/media (M2) | Guest player/tone state + host PCM/event interface | A real mandatory format needs decoding; an audio output or MIDI crate does not automatically implement MMAPI semantics |
| Android/iOS FFI and desktop UI (M2/M5) | Decide at actual host adapter boundary | ABI, safety, ownership, toolchain, platform policy and cross-platform maintenance are demonstrated |
| JSR 135 MMAPI (M3) | Implement selected guest media/control/protocol contracts separately from host playback/recording | Codec, MIDI, video, capture or audio backend is justified by a supported format and tested policy |
| JSR 184 M3G (M3) | Independently verified guest scene graph and graphics semantics | Software or GPU rendering backend is needed for a supported behavior; inspect host/asset limits and licensing |
| Mascot Capsule Micro3D v3 (M4) | Independent proprietary API contracts and versioned device semantics | A shared rendering primitive proves compatible; never import another VM/renderer implementation |
| Other optional JSRs/vendor behavior (M3/M4) | Prioritize remaining gaps from observed corpus blockers and versioned API contracts | Required feature cannot be expressed with the existing core and supported host services; evaluate separately |

**Rejected for now:** wholesale JVM/CLDC/MIDP implementation dependencies from other VMs (clean-room prohibition); Tokio/async executor merely to schedule guest threads; broad graphics/game engines; automatic Kotlin/Swift/FFI binding frameworks; a database or serializer before a proven format/state contract; release CI tools without a distinct current verification need.

## Evidence and next implementation order

1. Resolve or explicitly scope the implementation-use decision in [M0 #43](https://github.com/H3nb/Kilox/issues/43) and the authentic final CLDC `StackMap`/linking requirements before publishing dependent runtime logic.
2. Keep public-original structural/bytecode fixtures and a permission-cleared corpus. Use `package_inventory` for admission counts **only**, then isolate one failing verifier/constructor/reference edge with an independently authored fixture.
3. Verify loading, exceptions, object/class state and the CLDC library methods demanded by a minimal **authored** MIDlet; establish a headless guest execution path. Only then enlarge LCDUI, RMS, GCF and host services.
4. Add dependency crates **when the feature first needs them**, with exact license, MSRV, audited feature flags, security notices and representative positive/negative tests recorded in that capability PR.
5. Before any `CLDC 1.1`/`MIDP 2.1 supported` claim, complete the exact-edition class/member/opcode and scenario matrix described above. **This audit is not that final claim.**

This audit updates [capability inventory](capability-inventory.md) coverage planning; it does not change code, `Cargo.toml`, `Cargo.lock`, build workflows, legal conclusions or the M0–M6 milestones.
