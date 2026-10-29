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
