# WP02-W6 — original invocation fixture contract and test recipes

**Assignment:** [Issue #19](https://github.com/H3nb/J2ME-VM/issues/19).
**Baseline:** `4fa0cbf381fc9635870e5f6b5dd78f12ae46787c` (`main` when the work began).
**Scope:** guest-fixture evidence, **not** a change to the VM execution
contract or a claim of JVM/CLDC conformance.

## Provenance, compiler, and admission boundary

Both Java sources were written independently for this project and are
`GPL-3.0-only`. They derive from Java primitive semantics and the
[JVMS Second Edition](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html):
§3.6 (frame/local model), §§4.3.3, 4.4.2, 4.7.3, 4.9 (descriptors,
method references, Code and constraints), and chapter 6 (`invokestatic`,
`ireturn`, `iadd`, `isub`, integer comparisons). No SDK sample, VM test
suite, decompiled game, SquirrelJME content, vendor classes or runtime
implementations were inspected or copied.

A permitted private copy of **ECJ 3.26.0** was used only as the compiler:
`Eclipse Compiler for Java v20210609-0549, 3.26.0`,
SHA-256 `ac0ba5876eaf7ebb47749a0d1be179c51f194b9dd0b875d1c09e1b530f5a2db5`.
It is **not** included in this repository, the guest classes or public CI.
Host JDK was OpenJDK `21.0.12.1`. Rebuild from the repository root (with a
lawfully obtained ECJ 3.26.0 JAR):

```sh
java -jar ecj-3.26.0.jar -source 1.4 -target 1.4 -g:none \
  -d tests/fixtures/compiled \
  tests/fixtures/java/NestedStaticCalls.java \
  tests/fixtures/java/RecursiveStaticCalls.java
javap -classpath tests/fixtures/compiled -v -c -p \
  NestedStaticCalls RecursiveStaticCalls
sha256sum tests/fixtures/compiled/{NestedStaticCalls,RecursiveStaticCalls}.class
```

| Binary | Version | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `NestedStaticCalls.class` | 48.0 | 344 | `519ef34a1c7038b21f94168eaca17cc8b86d81bc228c82654c091da9ba37e4aa` |
| `RecursiveStaticCalls.class` | 48.0 | 207 | `19882d5f8a9c7b8d35dad9cf54467d428839da34cef7db3e17875083b9faeedf` |

Both class files were independently rebuilt with the documented relative-path
command and compared byte-for-byte. `javap -v -c -p` confirmed 48.0, the
declared `Code` lengths, constant-pool tags, descriptor references and
instructions below. Each contains an ordinary compiler-generated unused
default constructor (`invokespecial java/lang/Object.<init>`); this
constructor is **outside** the bounded static-entry execution target.
No exception handler, third-party API class or external class implementation
is embedded in the class files. There is **no legacy CLDC `StackMap`** and
no CLDC preverification; modern `StackMapTable` is not an alternative.

## Positive behavior derived without this VM

### `NestedStaticCalls.evaluate(I)I`

- `increment(v) = v + 3`; `adjust(v) = increment(v) + 2 = v + 5`.
- `adjust() = 4` is a **different descriptor** from `adjust(I)I`.
- `evaluate(x) = adjust(x) + adjust(x + 1) - adjust()`
  `= (x+5) + (x+6) - 4 = 2*x + 7` modulo `2^32`, interpreted as
  signed Java `int`. `x + 1` and all intermediate arithmetic wrap likewise.

| Input `x` | Expected `evaluate(x)` |
| ---: | ---: |
| `Integer.MIN_VALUE` (-2147483648) | 7 |
| -3 | 1 |
| -1 | 5 |
| 0 | 7 |
| 4 | 15 |
| `Integer.MAX_VALUE` (2147483647) | 5 |

The `evaluate` Code is 20 bytes (`max_stack=2`, `max_locals=3`):
`invokestatic #12` at PCs 1 and 8 resolves to
`NestedStaticCalls.adjust:(I)I`; PC 15 `invokestatic #15`
resolves to `NestedStaticCalls.adjust:()I`. The `adjust(I)I` Code
calls `increment(I)I` using `invokestatic #18`.
Thus a call chain `evaluate -> adjust(int) -> increment(int)`
requires **three simultaneously active frames**, with a later call to
`adjust()` and repeated `adjust(int)` calls on distinct arguments.
A correct implementation returns the callee's single integer to the
calling operand stack and resumes the caller at the instruction following
the invoke; the callee's locals/operand stack must not be shared.

### `RecursiveStaticCalls.evaluate(I)I`

`evaluate(n) = 0` for `n <= 0`; for `n > 0`,
`evaluate(n) = evaluate(n-1) + 1`. Induction gives `evaluate(n)=n`
for small nonnegative `n` when execution finishes normally.

| Input | Expected result if completed | Max simultaneous `evaluate` frames |
| ---: | ---: | ---: |
| -3 | 0 | 1 |
| 0 | 0 | 1 |
| 1 | 1 | 2 |
| 4 | 4 | 5 |
| 8 | 8 | 9 |

The method Code is 18 bytes (`max_stack=2`, `max_locals=2`);
PC 4 uses `if_icmpgt` (already in the WP02 integer subset);
PC 12 uses `invokestatic #12` targeting this class's
`evaluate:(I)I`. The explicit `base` local in the authored source
deliberately produces a two-operand comparison; no unagreed unary branch
opcode is assumed.

A host-JVM diagnostic run of the authored classes confirmed all vectors,
but is **auxiliary corroboration**, not the normative oracle or evidence
that `j2me-vm` can execute them.

## Test design for W5/coordinator (no Rust API assumptions)

**Positive:** select `evaluate:(I)I` by *name and descriptor*;
exercise each input above and different inputs in successive independent
invocations. Assert correct nested return value, caller resume, two calls
of `adjust(I)I` with different argument values, `()I` vs `(I)I`
resolution, and no local/operand state bleed across frames or machines.
For recursion, test `-3, 0, 1, 4` to validate base, return propagation,
and finite unwinding. Any PC/step assertions must follow the **accepted**
W5 call-frame/budget contract, not assumptions encoded in this document.

**Bytecode mutation recipes** refer to the *verified 48.0 binaries*, not
additional redistributed goldens. Apply to a fresh binary copy with a
class-file-aware editor that preserves or recomputes enclosing lengths.
Index/PC values are from the ECJ `javap` inspection above:

| ID | Reproducible mutation | Intended negative condition; avoid premature error-class claims |
| --- | --- | --- |
| N1 | `NestedStaticCalls.evaluate(I)I`, `invokestatic` at bytecode PC 1: change its two index operands at PCs 2–3 from `00 0c` to `00 00` | Invalid zero constant-pool index; reject before transfer. Invalid instruction operand, not a truncated physical class header. |
| N2 | Same opcode at PC 1: replace operands with `00 0b` (`#11` is `Utf8 "(I)I"`) | Wrong constant kind rather than `CONSTANT_Methodref`; bytecode constraint failure, not a missing target method. |
| N3 | Change `NestedStaticCalls` CP `#13` (`NameAndType`) **name_index** from `#14` (`adjust`) to `#2` (`NestedStaticCalls`), retain descriptor `#11` (`(I)I`); file bytes 111–112: `00 0e -> 00 02` | Well-formed symbolic reference `NestedStaticCalls.NestedStaticCalls:(I)I` with no matching method. This is **resolution/linking**, not an invalid CP tag; timing/class per final CLDC contract are unresolved. |
| N4 | Replace `adjust(I)I` method Code instructions with `b8 00 12 ac` (`invokestatic #18; ireturn`), `max_stack=1`, `max_locals=1` | Valid-width invoke, **zero** operand-stack arguments despite target `increment:(I)I` needing one; type/stack verification failure. |
| N5 | Replace `adjust(I)I` Code with `1a b8 00` (`iload_0; invokestatic` missing last index byte); update `code_length=3` and enclosing attribute length | Malformed/truncated invoke operand, distinct from linking failures. |
| N6 | Separately recompile a source *variant* replacing `increment(value)` in `adjust(int)` with `java.lang.Math.abs(value)`; do not commit variant binary | Properly encoded **external** static method reference, unsupported until an explicit loader/host policy exists; it is not an invalid CP shape. |
| N7 | With a hypothetical **maximum 8 active call frames**, compare recursion inputs 7 (requires 8 frames) and 8 (requires 9) | Boundary admission vs deterministic resource exhaustion without host stack overflow; whether 8 includes the entry frame is part of W5's contract. |
| N8 | With a small, explicit guest-instruction budget, step `evaluate(4)` through nested calls; resume only if supported by W5 contract | Budget exhaustion must be bounded; no exact step count or API is fixed here. |

N1/N2/N4/N5 are invalid guest bytecode shapes or constraints, whereas
N3 is a plausible symbolic resolution error. N6 is a valid external
reference but **outside the proposed bounded same-class invocation scope**.
N7/N8 are resource-policy conditions, not malformed classes. No
`ClassFormatError`, `VerifyError`, `NoSuchMethodError` or timing is
mandated here while version-specific CLDC N01 research remains incomplete.
Mutations are intentionally **not** checked-in binaries or a new framework.

## Handoff / remaining dependencies

W5/coordinator should decide: same-class method identity and descriptor
admission; call-frame ownership of locals, operand stack, PC and return
address; resolution and validation order; bounded frame depth; budget scope
and suspension behavior; and explicit handling for unsupported external,
nonstatic and constructor calls. The fixtures require no new arithmetic
or branch opcodes beyond current WP02, only `invokestatic` and the
corresponding frame/resolution semantics. Compiler-emitted constructors are
not an entrypoint test.

**Checks performed:** ECJ 3.26.0 Java 1.4 compilation, independent
recompilation/byte comparison, `javap` bytecode/CP inspection, binary
version and SHA-256, and auxiliary host-JVM vector run. **Not performed:**
CLDC preverification, G2 invocation execution, negative mutation runs
inside the VM or runtime CI. Rust `cargo`/Clippy/rustfmt were not
available in the fixture-authoring container; verify GitHub Actions
separately and record actual run status in the Issue/PR.
