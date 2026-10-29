# WP02 reference identity and exception prerequisites

**Research reviewed:** 2026-10-29 against
`37e32cd179223378fa6f271fd48fc9b33622a394`;
[W8 #25](https://github.com/H3nb/J2ME-VM/issues/25) and
[W9 #26](https://github.com/H3nb/J2ME-VM/issues/26).
This report prepares a later contract; it does not expand the executable
[WP02 subset](wp02-execution-contract.md) or pass G2.

## Finding and smallest useful next dependency

The original W9 candidates exercise alias identity, distinct allocation, null
transport, and the identity of an exception caught locally or in a caller.
They require capabilities absent from the current VM: class identities and
resolution, class initialization, typed references, allocation, constructor
execution and uninitialized-value verification. The exception candidate also
requires `Throwable`, subtype matching and abrupt frame completion. Adding a
reference-shaped integer to W7's stack cannot establish those contracts.

The next implementation preparation should therefore specify a **bounded
bootstrap class environment**, beginning with `java.lang.Object`, class identity,
superclass rules, resolution/initialization state, and constructor behavior.
Choose its representation only when these obligations and applicable use terms
are clear. The specification does not require an on-disk `Object.class`:
JVMS Second Edition chapter 4 permits other class representations, and §3.7
does not fix object layout. Original class bytes or an original internal bootstrap
representation must still satisfy the selected behavior; neither permits skipped
constructor execution or fabricated host-owned guest references.

A smaller null-only experiment could validate reference/int separation using
typed locals/stacks, `aconst_null`, reference loads/stores and comparisons. It
would not show non-null identity or remove the bootstrap dependency. It is an
available prerequisite experiment, **not selected as a new implementation here**.
No generic heap, collector, class-loader framework or snapshot codec is selected.

## Clause-backed obligations

The linked Oracle `se6/html` [preface](https://docs.oracle.com/javase/specs/jvms/se6/html/Preface.doc.html)
identifies this text as the **Second Edition**; the hosting path does not turn
its clauses into a modern JVM contract. These are inherited JVMS findings;
unresolved CLDC amendments and final verifier terms remain in
[N01](specification-map.md#wp02-n01-historical-cldc-verifier-and-linking-evidence-2026-10-29).

| Boundary | Required behavior and evidence |
| --- | --- |
| Values and frames | References include null and aliases to objects; locals, operand stacks and frame transfer must distinguish them from integers. VM-owned logical IDs are a project option, not mandated object layout. [§§2.4.6, 3.2, 3.4, 3.6, 3.7](https://docs.oracle.com/javase/specs/jvms/se6/html/Overview.doc.html); [§2.4.6](https://docs.oracle.com/javase/specs/jvms/se6/html/Concepts.doc.html). |
| Allocation and initialization | `new` resolves the target and can trigger class initialization; allocation yields an uninitialized instance. `dup` duplicates a category-1 value. `<init>` executes through `invokespecial`. The verifier tracks allocation-site-specific uninitialized values and promotes matching aliases only after the appropriate constructor call. [§§3.9, 4.9.4](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html), [new](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc10.html), [dup](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc3.html), [invokespecial](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc6.html). |
| Class environment | Class creation, superclass resolution, linking and initialization have distinct behavior; the root Object class is special. Absence of a fixture's `<clinit>` does not establish that its dependencies are loaded and initialized. [§§5.3.5, 5.4, 5.5](https://docs.oracle.com/javase/specs/jvms/se6/html/ConstantPool.doc.html). |
| Identity transport | Reference loads/stores, calls and `areturn` preserve the referenced identity; comparisons test references. Each allocation must have fresh identity within the live VM state. Durable identities and instance isolation must be defined before recovery. [§§3.4, 3.6](https://docs.oracle.com/javase/specs/jvms/se6/html/Overview.doc.html), [aload/astore/areturn](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc.html), [if_acmp](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc6.html). |
| Throw and handlers | `athrow` uses an actual Throwable/subclass reference; null requires a NullPointerException. Search handlers in table order, using `[start_pc, end_pc)` and compatible thrown type (or catch-all `catch_type=0`). A matched handler receives the same thrown reference on an otherwise cleared stack. Unmatched frames complete abruptly and propagate that same reference. [§3.10](https://docs.oracle.com/javase/specs/jvms/se6/html/Overview.doc.html), [§4.7.3](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html), [athrow](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc.html). |

The official CLDC 1.1 API entries for
[Object](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/lang/Object.html)
and [Throwable](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/lang/Throwable.html)
are interface/behavior references, not executable runtime implementations supplied
by this project. Constructor side effects and failures need their own evidence;
W9's bare Object allocation does not test them.

## Ownership, failures and exception call sites

A later accepted slice must give the VM context ownership of class/initialization
state and allocated identities, and frames ownership of typed locals, stacks and
PCs. Verification facts about uninitialized allocation sites are distinct from
runtime identity. All aliases must resolve to the same VM-owned object; no Rust
pointer or host object becomes a durable identity. Identity reuse, roots,
allocation limits, failure behavior and per-instance isolation must be concrete
before admitting a heap. The current 64-frame/64-method policy is not a heap limit.

Current W7 advances the caller continuation before pushing a callee. That is
appropriate for its normal-return-only contract. Future exception search in a
suspended caller must use the **invocation instruction's PC**, rather than that
already advanced continuation. Preserve the call site or change the transition
deliberately; do not infer it from a fixed opcode length. This is a future
exception prerequisite, not a current W7 defect.

The parser exposes exception tables but does not prove handler instruction
boundaries, Throwable subtype validity or typed handler entry state. Structural
range checks and full execution admission remain separate. Host resource errors
must not masquerade as guest thrown objects, and unknown CLDC resolution error
timing must not be guessed.

## Fixture and review evidence

[W9 fixture provenance](../tests/fixtures/reference-exception-w9.md) records the
original source, ECJ 3.26.0 recipe, diagnostic 48.0 hashes, bytecode dependencies
and bounded negative recipes. The coordinator independently inspected and rebuilt
the source and checked the auxiliary JDK result. Host-JVM execution and the
absence of stack-map attributes do not prove CLDC admission. No candidate is
claimed to execute on this VM; no new binary is admitted into the public corpus.

A bootstrap class need not be an on-disk class file, and null transport must be
checked against null itself rather than only against two other objects. These
boundaries were checked against the primary text and an original negative probe,
rather than accepted from worker agreement alone.

## Rights and clean-room boundaries

The historical JVMS notice has conditional implementation terms; the final CLDC
1.1/1.1.1 texts and terms remain unresolved. See the owning
[licensing contract](licensing-and-provenance.md). Technical feasibility does not
clear those questions. No new object/exception implementation is accepted here.

Research used primary specifications/API contracts and original fixtures, not
third-party VM source or SDK runtime implementations. During coordinator browsing,
automatic extraction of the `new`/`newarray` specification page also displayed a
nonnormative vendor implementation note. It was not used in any design or code.
No array storage design is accepted here; later array representation work must
be independently derived by an unexposed contributor from allowed contracts.
