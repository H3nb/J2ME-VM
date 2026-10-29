# W9 original reference and exception fixture design

**Scope:** [Issue #26](https://github.com/H3nb/J2ME-VM/issues/26), reviewed against
`37e32cd179223378fa6f271fd48fc9b33622a394`, 2026-10-29. Only original Java source
is admitted here. Diagnostic binaries remain outside the repository; the current
VM cannot execute either candidate. The [W8 research](../../docs/wp02-reference-exception-research.md)
owns the runtime prerequisites, not this fixture note.

## Provenance and independently expected results

Both sources were authored for this project under task W9 and
reviewed by the coordinator. They are GPL-3.0-only; no SDK sample, runtime source,
API stub, game, TCK or third-party VM test was copied. The coordinator required
the explicit null check after finding that comparisons against two other objects
alone would allow an incorrectly returned third object to pass.

| Source | Expected `evaluate()I` | Meaningful failure |
| --- | --- | --- |
| [ReferenceIdentity.java](java/ReferenceIdentity.java) | `0` | `1`: alias changed across a call; `2`: two allocations conflated; `3`/`4`: null conflated with an allocation; `5`: null argument returned as a non-null value. |
| [ExceptionIdentity.java](java/ExceptionIdentity.java) | `0` | `1`: local catch changed identity; `2`: throwing helper returned normally; `3`: caller catch changed identity. |

The basis is JVMS Second Edition §§3.4, 3.6, 3.9, 3.10, 4.7.3, 4.8.1, 4.9.4
and chapter 6 reference instructions
([structure](https://docs.oracle.com/javase/specs/jvms/se6/html/Overview.doc.html),
[class format](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html),
[instruction index](https://docs.oracle.com/javase/specs/jvms/se6/html/Mnemonics.doc.html)).
Official CLDC 1.1 [Throwable](https://docs.oracle.com/javame/config/cldc/ref-impl/cldc1.1/jsr139/java/lang/Throwable.html)
confirms the constructor API, not an available executable runtime. Final CLDC
verification/StackMap and resolution timing remain unresolved in N01.

## Actual compilation and inspection

Private offline compiler: ECJ 3.26.0, `v20210609-0549`, SHA-256
`ac0ba5876eaf7ebb47749a0d1be179c51f194b9dd0b875d1c09e1b530f5a2db5`;
same compiler identity as [W6](invocation-w6.md). Used only for compilation;
neither it nor SDK/API classes are redistributed. Both worker and coordinator
rebuilt the final sources without warnings on Microsoft OpenJDK 17.0.20+8-LTS.
With an existing temporary output directory and a permitted ECJ copy:

```text
java -jar <ecj-3.26.0.jar> -source 1.4 -target 1.4 -g:none -d <output> tests/fixtures/java/ReferenceIdentity.java tests/fixtures/java/ExceptionIdentity.java
javap -classpath <output> -v -c -p ReferenceIdentity ExceptionIdentity
```

| Diagnostic output | Version | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| ReferenceIdentity.class | 48.0 | 323 | `457827a5674af9fcc245baa61a13ba6c8b34198db3145e9930aaa2ec086fedce` |
| ExceptionIdentity.class | 48.0 | 353 | `fcc6795c9a96d9170f28250ab6c9de4c8c5c15bef61154f340cdb011f0ae6f94` |

`javap` confirmed no fields, `<clinit>`, `StackMap` or `StackMapTable`. Both
classes extend Object and have generated default constructors invoking
`Object.<init>`. A temporary host-only harness called both static methods,
checked their zero results and printed `ReferenceIdentity=0; ExceptionIdentity=0`.
The coordinator also changed only a temporary copy of `echo` to return a fresh
Object for null and observed status `5`, confirming that the revised null oracle
detects that credible incorrect behavior. No corrupted class-file mutations or
Rust VM execution were performed. JDK 17 is an auxiliary observation, not the
CLDC verifier or an oracle for CLDC-specific behavior. Diagnostic bytes are not
added to `manifest.json`, which inventories checked-in binaries.

## Observed bytecode and missing runtime contracts

ReferenceIdentity requires `new`, `dup`, `invokespecial Object.<init>`, reference
locals, `aconst_null`, reference-preserving `invokestatic echo(Object)Object`,
`areturn`, `if_acmp*`, `ifnull`, and integer status returns. Its constructor calls
are executable guest instructions, not a reason to skip construction. The first
alias comparison is at PC 28, distinct comparison at PC 35, null-versus-allocation
comparisons at PCs 42/49, and explicit null preservation uses `ifnull` at PC 55.
The fixture does not observe constructor side effects.

ExceptionIdentity requires Object/Throwable class and constructor behavior,
typed reference frames, `athrow`, ordered handler matching, and propagation
through `rethrow(Throwable)V`. Its `evaluate` handlers are `[8,10) -> 10` and
`[18,22) -> 24`, both catching Throwable; PC 9 throws locally and PC 19 invokes
the throwing helper. Each handler begins by storing its sole thrown reference.
The helper has an `Exceptions` metadata attribute; metadata is not a substitute
for `athrow` execution. This fixture tests matching Throwable catches, not subtype
selection, handler precedence, uncaught termination or `athrow null`.

Current admission rejects these candidates: reference-producing opcodes/types,
constructors, object-return/void calls and exception handlers are outside W7.
Neither file supplies runtime library classes, loading, initialization or
preverification. Class version 48.0 is not CLDC-preverification evidence.

## Bounded negative recipes, not executed tests

Coordinates apply only to the exact diagnostic bytes/hashes above. Use a
class-file-aware mutation when sizes change; never overwrite a golden fixture.

| Mutation | Expected boundary, without inventing CLDC exception timing |
| --- | --- |
| ReferenceIdentity `new` PC 0: CP operand `#3` (Object Class) to `#8` (constructor Methodref), or zero | Instruction/CP admission rejects the wrong kind or invalid index. A raw structural parser need not validate the opcode operand. |
| ReferenceIdentity `if_acmpeq` PC 28: offset `00 05` to `ff ee` | Target PC 10 lies inside the `invokestatic` operand; reject instruction-boundary verification/admission. |
| Replace that reference-compare opcode `a5` with `9f` (`if_icmpeq`) | Reference operands cannot be consumed as integers; reject type admission, not a guest runtime result. |
| ExceptionIdentity first handler target `10` to `34` (code length); separately second start `18` to `20` (inside PC 19's invoke operand) | The first is structurally out of range; the second needs instruction-boundary verification beyond the parser's current range checks. |
| ExceptionIdentity first handler `catch_type #12` (Throwable Class) to `#13` (Utf8) | Reject wrong constant-pool kind in the exception table. |
| Change rethrow descriptor `#18` from `(Ljava/lang/Throwable;)V` to `(Ljava/lang/Object;)V`, retaining `aload_0; athrow` | Reject Object-typed `athrow` during type admission; rebuild UTF-8 lengths/offsets correctly. |

A separate well-formed symbolic reference to an absent class is a resolution
case, not malformed CP data. Exact CLDC failure timing remains open. These
recipes must be reviewed against the eventual accepted slice before adding
executable Rust tests; they are not assertions that current W7 admits the input.
