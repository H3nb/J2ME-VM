# WP02: bounded integer execution proof

**Status:** [PR #3](https://github.com/H3nb/J2ME-VM/pull/3) and
[PR #16](https://github.com/H3nb/J2ME-VM/pull/16) integrated; **partial G2
proof only**. The relevant technical baseline is the
[JVMS Second Edition](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html)
(§§3.6, 4.7.3, 4.9 and chapter 6 instruction definitions).
Version-specific CLDC `StackMap` verification, linking and resolution are
**not** implemented; see [N01 evidence grades](specification-map.md).

## Entry and supported method shape

`ClassFile::parse_class` structurally reads version 45.x–48.0 and exposes
method `Code`. `IntProgram::prepare` independently checks the **selected
method**, not the whole class or loader dependencies. Exactly one method
matching the name is required; only static, concrete, unsynchronized
`()I` or `(I)I` methods are admitted. `IntMachine::new` invokes the
no-argument method; `IntMachine::with_int_arg` supplies the one integer
argument. Calling the wrong host constructor is a programming error that
panics, not a guest verifier error.

The supported method may have no exception handlers or nested `Code`
attributes. Project resource policies cap the method at **4096 code bytes,
64 operand slots and 64 local slots**. No class loading, class/field
resolution, initialization, object allocation or instance methods occur.

| Bytecode | Supported behavior |
| --- | --- |
| `iconst_m1`–`iconst_5`, `bipush`, `sipush` | Push an `int` (signed immediate sign-extension) |
| `iload`, `iload_0`–`iload_3` | Read a definitely initialized in-range integer local |
| `istore`, `istore_0`–`istore_3` | Store an `int` into an in-range local |
| `iinc` | Add signed 8-bit immediate to initialized integer local, wrapping as Java `int` |
| `iadd`, `isub` | Signed 32-bit arithmetic with two's-complement wraparound |
| `if_icmpeq`, `if_icmpne`, `if_icmplt`, `if_icmpge`, `if_icmpgt`, `if_icmple` | Pop two `int` values; evaluate signed comparison; branch relative to opcode PC |
| `goto` | Branch relative to opcode PC |
| `ireturn` | Return one `int` |

No `wide`, switch, floating-point, reference, invocation, monitor,
exception or dynamic-linking instructions are supported. All unsupported
opcodes fail before a machine is created; CLDC `StackMap` and modern
`StackMapTable` are **not interchangeable**.

## Method admission and explicit state

The proof decodes the entire instruction stream, validates operand widths
and local bounds, and records instruction boundaries. Branch targets must
point **inside Code, at an opcode boundary**, not into an operand.
A worklist traverses reachable control flow, validates operand-stack
underflow/overflow and equal depths at merges, intersects definitely
initialized local facts on joins (including back edges), rejects invalid
fall-through and unreachable instructions, and requires a reachable
`ireturn`. These are **subset-specific** structural/type checks, not
general JVMS or CLDC verification.

`IntProgram` owns the validated decoded instructions and their bytecode
PCs. Each `IntMachine` owns its own program, current instruction index,
`Vec<i32>` operand stack, `Vec<Option<i32>>` local slots and result.
Read-only accessors expose the next PC, stack and locals. `step()` consumes
one guest instruction; `run(budget)` executes at most
`min(budget, 65_536)` instructions per call. Exhausting this budget returns
`None` and leaves the frame resumable; there is **no lifetime-wide
execution cap**, process-independent checkpoint format or snapshot API.

## Empirical evidence and limitations

Original self-authored ECJ 3.26.0 Java 1.4 (**class 48.0**) fixtures are
recorded with SHA-256 in [fixture provenance](../tests/fixtures/README.md).
`MinimalClass.answer()I` returns 42;
`LocalArithmetic.evaluate(I)I` exercises locals and signed overflow;
`BranchLoop.evaluate(I)I` covers conditional and backward branches.
Positive and negative regression cases cover corrupt targets, invalid
stack/local state, all six signed comparison branches, step PC, independent
frames, bounded loops and resume.

[Final W4 PR CI](https://github.com/H3nb/J2ME-VM/actions/runs/40816652770),
[W3+W4 combined-tree CI](https://github.com/H3nb/J2ME-VM/actions/runs/40817525937)
and [post-merge main CI](https://github.com/H3nb/J2ME-VM/actions/runs/40817648432)
each succeeded. The resulting main Git tree exactly matches the validated
combined candidate; **22 Rust tests pass**. These tests do not include a
CLDC TCK, preverified class, external VM oracle comparison of CLDC behavior,
or a cross-process restore.

## Further gates

N01 still requires original final JSR 139 editions and legally reviewed
implementation terms before CLDC-admission assertions can be finalized.
Full G2 additionally needs the chosen class/descriptor format constraints,
general type verification where applicable, class loading and symbolic
resolution, calls/return frames, exception propagation, debugging stops and
non-`int` state. G3 separately requires a real **process A exits →
process B restores** proof. This execution prototype does not justify
a MIDP compatibility, Java certification or JL-Mod Plus integration claim.
