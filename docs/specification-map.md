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

- Acquire and catalog an appropriately licensed copy of the **exact CLDC 1.1 / 1.1.1 text** needed for implementation without distributing the PDF in this repo.
- Confirm referenced JVMS edition and map each supported bytecode, class format and verifier requirement before claiming CLDC-level conformance.
- Resolve Java specification implementation/TCK licensing and permitted branding before any public claim of standards compatibility.
- Create a clause-to-test matrix incrementally while implementing; an empty matrix is not evidence of correctness.
