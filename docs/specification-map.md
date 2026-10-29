# Normative specification map

**Purpose:** Use version-correct, primary technical contracts for an independent implementation. This page identifies sources and gaps, **not** a blanket grant of access, patent rights, certification, or reproduction rights. Before using downloaded materials to implement the standard, assess their applicable terms; see [licensing-and-provenance.md](licensing-and-provenance.md).

| Contract | Primary source | Use / version caveat |
| --- | --- | --- |
| J2ME CLDC 1.1 | [JSR 139 final release (2003)](https://www.jcp.org/aboutJava/communityprocess/final/jsr139/) and [maintenance release (2007)](https://www.jcp.org/aboutJava/communityprocess/mrel/jsr139/index.html) | **Baseline target.** Resolve whether a clause originates in 1.1 or 1.1.1 maintenance text; confirm implementation-use terms before claiming rights. |
| MIDP 2.0 | [JSR 118 final release](https://www.jcp.org/aboutJava/communityprocess/final/jsr118/) | Later profile; versioned scope distinct from CLDC VM. Its documentation explicitly references a license agreement. |
| JVM structure, bytecode and format | [JVMS Second Edition (Oracle-hosted HTML)](https://docs.oracle.com/javase/specs/jvms/se6/html/Preface.doc.html) and [Oracle JVMS SE 8 HTML](https://docs.oracle.com/javase/specs/jvms/se8/html/) | **Do not equate SE 8 with CLDC 1.1.** Use SE 8 for navigational/informative assistance only when the normative older clause and CLDC amendment have been independently checked. |
| Oracle historical API contracts | [CLDC 1.1 API](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/overview-summary.html) and [MIDP 2.0 API](https://docs.oracle.com/javame/config/cldc/ref-impl/midp2.0/jsr118/overview-summary.html) | API behavior reference; not reference-implementation source code. |
| Later CLDC behavior (comparison only) | [CLDC 8 VM differences](https://docs.oracle.com/javame/8.0/api/cldc/api/doc-files/CLDCvm.html) and [byte code typechecker overview](https://docs.oracle.com/javame/8.0/api/cldc/api/doc-files/CLDCtypechecker.html) | **Not authoritative for 1.1.** Useful to locate subjects but re-check against JSR 139 before implementation. |

## Required interpretation workflow

1. Specify the target configuration/profile and **revision** (e.g. CLDC 1.1 vs 1.1.1; MIDP 2.0), then locate its exact normative clause.
2. Determine whether the CLDC clause **modifies, removes, or constrains** a JVMS behavior. The edition referenced by CLDC takes precedence over a modern reinterpretation.
3. Note applicable binary/class-file version, preverification and `StackMap` constraints, supported libraries and exceptions. **Legacy CLDC `StackMap` is not modern `StackMapTable`.**
4. For a code/test change, cite a clause in the local design/test or PR rationale. Use your own words; do not copy sample implementation code or large protected text.
5. If normative text is inaccessible or licensing terms are unclear, record a gap and defer the affected implementation or obtain an appropriate legal review—do not substitute an emulator's code or a random third-party summary.

## Verification work still required

The [profile evolution study](profile-evolution.md) adds source findings and
open questions N01–N05. Product delivery targets MIDP 1.0–2.1 first; JSR 271 and
CLDC 1.1.1 inform architectural constraints now. Do not treat a new API version
as a drop-in semantic replacement for an older execution environment.

- Acquire and catalog an appropriately licensed copy of the **exact CLDC 1.1 / 1.1.1 text** needed for implementation without distributing the PDF in this repo.
- Confirm referenced JVMS edition and map each supported bytecode, class format and verifier requirement before claiming CLDC-level conformance.
- Resolve Java specification implementation/TCK licensing and permitted branding before any public claim of standards compatibility.
- Create a clause-to-test matrix incrementally while implementing; an empty matrix is not evidence of correctness.

## G1 parser traceability (implementation scope, not verification)

- **JVMS Second Edition chapter 4 / [Oracle-hosted class format](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html) §§4.1, 4.4, 4.5, 4.6, 4.7:** big-endian header, constant pool indices and two-slot Long/Double, members, bounded attributes, Code layout and modified UTF-8.
- **JSR 139 CLDC 1.1 §5.3.1:** class-file range 45.x through 48.0 for G1. Older minor-version 45.x and major 46–48 minor 0 are accepted; newer tag variants remain unsupported. The fixture is an original ECJ-produced class file 48.0; see [`tests/fixtures/README.md`](../tests/fixtures/README.md).
- **G1 additionally enforces** the method `Code` cardinality and `native`/`abstract` exclusion from JVMS Second Edition §4.7.3. **Deliberately deferred:** remaining format and name/descriptor constraints, non-`Code` attribute cardinality and locations, bytecode instruction decoding and validation, symbolic resolution, and CLDC 1.1 `StackMap` verifier (§5.2.1 and Appendix 1). Structural parsing success must never enable guest execution by itself.
- **Host resource policy:** input size capped at 16 MiB, independent of normative class-file limits. Errors carry an offset. Parser borrows attribute/constant bytes and does not normalize guest strings with lossy UTF-8 conversion.

Any work extending verification must independently check exact JSR 139 normative clauses and applicable terms before claiming compliance.

## Expanded profile sources and evidence levels

| Contract | Source | Current evidence boundary |
| --- | --- | --- |
| MIDP 1.0 / JSR 37 | [Oracle MIDlet API](https://docs.oracle.com/javame/config/cldc/ref-impl/midp1.0/jsr037/javax/microedition/midlet/package-summary.html) | Lifecycle source inspected; complete profile/version clauses still mapped incrementally |
| MIDP 2.1 / JSR 118 | [Final maintenance download](https://download.oracle.com/otndocs/jcp/midp-2.1-mrel-oth-JSpec/), [MR2 change log](https://www.jcp.org/aboutJava/communityprocess/maintenance/jsr118/JSR_118_MR2_Changelog.pdf) | Accepted/rejected status inspected, including runtime declaration and behavior deltas; final full text must close N03. MIDP 2.2 changes must not be attributed to 2.1 |
| CLDC 1.1.1 / JSR 139 | [Maintenance release](https://jcp.org/aboutJava/communityprocess/mrel/jsr139/index.html), [accepted review index](https://jcp.org/aboutJava/communityprocess/maintenance/jsr139/139ChangeLog.html), [change text](https://jcp.org/aboutJava/communityprocess/maintenance/jsr139/CHANGE_LOG.txt) | Change text addresses resolution failure timing in sections 5.4/5.3.4 but retains a proposed heading; reconcile final wording before dependent implementation (N01) |
| MIDP 3.0 / JSR 271 | [Final release](https://jcp.org/aboutJava/communityprocess/final/jsr271/index.html), [Oracle final artifacts](https://download.oracle.com/otndocs/jcp/midp-3.0-fr-oth-JSpec/) | Acquisition points identified; complete final chapters have not been validated in this planning increment |
| MIDP 3 API descriptions | [Mirrored final Javadoc](https://nikita36078.github.io/J2ME_Docs/docs/midp-3_0-fr-javadoc/overview-summary.html), individual sources S05–S08 in the study | Primary-authored API text on third-party hosting; research evidence only until artifact identity/final chapter cross-check. No emulator implementation was inspected |

When closing a gap, record edition, section/API member, retrieval provenance and
the specific expected behavior. Do not assign an invented clause number. The
study's original architecture proposals are not themselves normative Java rules.

## WP02 first executable proof: exact source boundaries

The scope and executable entry preconditions are owned by
[wp02-execution-contract.md](wp02-execution-contract.md).

- [JVMS Second Edition §3.6](https://docs.oracle.com/javase/specs/jvms/se6/html/Overview.doc.html):
  explicit method frame/operand stack and PC behavior; exact host memory
  layout is not prescribed.
- [JVMS Second Edition §4.7.3 and §4.9](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html):
  method Code bounds and static/structural bytecode constraints.
- [JVMS Second Edition opcode map](https://docs.oracle.com/javase/specs/jvms/se6/html/Mnemonics.doc.html)
  and its instruction descriptions: iconst_m1..iconst_5, bipush, sipush,
  iadd, isub and ireturn. The project uses signed immediates and 32-bit
  two's-complement wrapping arithmetic.
- **CLDC status:** This is deliberately an execution *proof* of a subset,
  not a CLDC 1.1 / 1.1.1 verifier. The fixture predates preverification,
  lacks legacy CLDC StackMap, and cannot establish a conformant class loader.
  N01 remains open for full CLDC execution admission and linking semantics.

## WP02-N01: historical CLDC verifier and linking evidence (2026-10-29)

**Scope and result:** This is research, not a VM conformance claim or an accepted
verifier design. The final CLDC 1.1 (2003) and CLDC 1.1.1 (2007) *complete
normative specification PDFs* were not directly obtained in this review.
Accordingly, N01 is **partially answered, not closed**. In particular, the
proposed 1.1.1 linking clause must not be silently promoted to final normative
wording. Do not use CLDC 8, modern `StackMapTable`, or the current experimental
`src/execution.rs` subset as a substitute for this evidence.

### Sources, edition and evidence grade

| ID / grade | Source, edition / section | What it actually establishes |
| --- | --- | --- |
| N01-A / **direct primary** | [JVMS Second Edition, Oracle-hosted Chapter 5](https://docs.oracle.com/javase/specs/jvms/se6/html/ConstantPool.doc.html), §§5.3.5, 5.4.1–5.4.3 and 5.5 | The inherited base distinction between class-file loading/format errors, verification, preparation, symbolic resolution and initialization. CLDC amendments take precedence. The Oracle-hosted HTML is an edition-aware reading point, not evidence that a later JVMS applies. |
| N01-B / **official release metadata, not full clauses** | [JSR 139 Final Release](https://www.jcp.org/aboutJava/communityprocess/final/jsr139/) (2003); [JSR 139 Maintenance Release](https://jcp.org/aboutJava/communityprocess/mrel/jsr139/index.html) (2007); [JSR status](https://www.jcp.org/en/jsr/detail?id=139) | Establishes separate 1.1 final and 1.1.1 maintenance artifacts. The maintenance page distinguishes evaluation/app-use access from implementation access; the final text and relevant grant were **not** accepted/downloaded or adjudicated in this task. |
| N01-C / **official accepted-review record; draft wording** | [JSR 139 change index](https://www.jcp.org/aboutJava/communityprocess/maintenance/jsr139/139ChangeLog.html) (updated 28 Nov 2007), Maintenance Review 1 approved 28 Aug 2007; [linked CHANGE_LOG.txt](https://jcp.org/aboutJava/communityprocess/maintenance/jsr139/CHANGE_LOG.txt), lines concerning proposed §§5.4 and 5.3.4 | Official index establishes approval of the review. The linked file **still begins “Proposed Change Log for CLDC 1.1.1”**; it proposes deferring observable resolution-related `LinkageError` until an executed instruction uses the reference, even if the implementation resolves eagerly. Exact *final* 1.1.1 §§5.4/5.3.4 wording remains unverified. |
| N01-D / **later primary-authored, edition mismatch** | [Oracle-hosted CLDC Byte Code Typechecker Specification](https://docs.oracle.com/javame/8.0/api/cldc/api/Appendix1-verifier.pdf), dated **13 Apr 2012**, §§1.2, 2, 2.1 and 3; [Oracle CLDC 8 typechecker overview](https://docs.oracle.com/javame/8.0/api/cldc/api/doc-files/CLDCtypechecker.html) | Documents legacy `StackMap` format and errors, implicit empty maps, type safety checks, and off-device preverification followed by runtime verification. Its history explicitly records a **2012 correction for `goto_w`**. Neither this corrected appendix nor CLDC 8's support rules prove the exact 2003/2007 text unchanged. |
| N01-E / **mirrored/indexed excerpt only** | [March 2003 CLDC 1.1 Sun-authored PDF, academic mirror](https://vsis-www.informatik.uni-hamburg.de/oldServer/teaching/ws-05.06/dbms/materialien/j2me/CLDCSpecification1.1.pdf) (retrieval returned HTTP 403; search-index contents only) | Index places class-file verification and off-device/runtime stack maps at §§5.2–5.2.1; format/loading at §§5.3–5.3.4. It is **not** proof that the full PDF contents were inspected or that particular detailed rules are final. |

### Clause-to-observable-behavior matrix

The stage below means the *earliest applicable semantic boundary*, not a
promise of an exact instruction/time before the full CLDC revision is verified.
Tests are proposed examples, **not** existing W2 fixtures or executed tests.

| Input / example | Boundary and expected observable result | Evidence and limitation |
| --- | --- | --- |
| Bad class-file magic/invalid top-level binary structure; unsupported class version | **Loading/format admission**, rather than successful parse or execution: `ClassFormatError` for invalid format; `UnsupportedClassVersionError` for an unsupported version. | JVMS Second Edition §5.3.5 (N01-A). The currently recorded CLDC 1.1 version 45.x–48.0 boundary is in the G1 trace above (§5.3.1); confirm exact historical minor-version wording against accessible JSR 139 final before widening admission. |
| Well-formed `Code` without a `StackMap` attribute, e.g. a straight-line static `int` return | **Runtime verification** is still mandatory. Missing attribute is **not automatically** `ClassFormatError`: the later typechecker §2.1 models it as an implicit *zero-entry* map. It verifies only if the method needs no additional maps and otherwise satisfies type rules. | N01-D, not yet verified against the exact 2003/2007 appendix; do **not** generalize to all non-preverified methods. |
| Conditional branch to another block or nontrivial exception handler with absent/inconsistent map frame | **Verification before executing the admitted method/class** must establish valid types at control-flow joins/handlers. The legacy typechecker may fail, yielding `VerifyError`. A branch target being in range does not establish type safety. | N01-D §§2.1, 3 and CLDC 8 typechecker description. Confirm historic frame-placement and error details before W2 fixture expectations become normative. |
| Duplicate or structurally malformed legacy `StackMap` in a method's `Code` attribute | **Attribute parsing/format**: the corrected typechecker §2.1 specifies `ClassFormatError` for malformed/duplicate `StackMap`; invalid *type relationships* instead lead to verification failure (`VerifyError`, §3). | N01-D, **historical revision comparison pending**. `StackMap` is a subattribute of `Code`, **not** `StackMapTable`. |
| `jsr`, `jsr_w`, `ret`, or `wide ret` within guest method | The legacy CLDC preverification pipeline **inlines subroutines off-device** before in-device verification. Do not simply treat parser acceptance as execution safety or require a VM to perform preverification itself. | Oracle's later typechecker overview (N01-D); exact final 1.1/1.1.1 rejection/error wording remains unverified. |
| Valid class refers from a nonexecuted method to an **absent optional class**; only a separate safe path runs | **Resolution versus loading/verification** must remain distinct. The change proposed for CLDC **1.1.1** disallows a resolution-caused `LinkageError` becoming observable *before an instruction uses that symbolic reference*, including in an eager resolver. A guarded `new`/`invokestatic`/field use is a suggested future observable test, not evidence that all missing dependencies may be ignored. | N01-C plus JVMS Second Edition §§5.4.1–5.4.3 (N01-A). **2007 final wording not verified; do not enforce as finalized 1.1.1 contract yet or retroactively ascribe it to 1.1.** |
| Class has a missing or invalid **direct superclass**; class has an invalid own bytecode; a method merely mentions a missing class in a constant pool entry | **Different failure categories:** superclass resolution is part of loading (§5.3.5); type-unsafety is verification (§5.4.1); a symbolic reference used by an instruction triggers resolution (§5.4.3). The presence of an unused symbol alone does not prove that the referring class can safely run; mandatory dependency/verification checks still apply. | N01-A for inherited distinction; CLDC 1.1.1 final changes require independent cross-check. |
| Valid `getstatic` or `invokestatic` reaching a reference after successful verification | **Resolve, then initialize when required**, rather than executing class initializer merely because the class file was parsed. | JVMS Second Edition §§5.4.3 and 5.5 (N01-A), subject to applicable CLDC exceptions. |

### Hand-off boundary for WP02 and W2

- **Confirmed from inspected primary text:** JVMS Second Edition separates
  structural loading, verifier failure, symbolic resolution and initialization.
  Oracle's later *CLDC Byte Code Typechecker* specifies an optional legacy
  `StackMap` (implicit empty map), format-versus-type errors, and runtime type
  checking. This appendix is **not** proof of historical text identity.
- **Confirmed only as an approved maintenance item, not the final clause text:**
  the 2007 JCP review index and linked draft describe a stricter observable
  resolution-error timing rule (including eager linking). The exact final
  maintenance artifact remains the blocking source for declaring N01 closed.
- **Required before affected implementation or definitive W2 fixture assertions:**
  obtain the complete 2003 final CLDC 1.1 and 2007 final CLDC 1.1.1 documents
  under appropriately reviewed terms; compare §§5.2.1, 5.3.1, 5.3.4, the
  appended verifier revision and proposed new §5.4 **word for word privately**
  (do not commit specification text); log any actual deltas with precise
  clause/page locators. Separately resolve implementation-use conditions.
  The 2012 `goto_w` corrigendum is not automatically a 2007 change.
- **No shared-contract or runtime change is authorized by this report.** The
  coordinator owns decisions about how to sequence any new tests or behavior
  changes. W2 may create original fixtures, but expectations dependent on the
  unresolved clauses should remain explicitly provisional.

No repository runtime, test fixture, CI, or emulator implementation was read or
modified as part of this research increment.
