# WP02: bounded integer execution proof

**Status:** [PR #3](https://github.com/H3nb/J2ME-VM/pull/3),
[PR #16](https://github.com/H3nb/J2ME-VM/pull/16), and bounded invocation
[PR #23](https://github.com/H3nb/J2ME-VM/pull/23) integrated; **partial G2
proof only**. The relevant technical baseline is the
[JVMS Second Edition](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html)
(§§3.6, 4.7.3, 4.9 and chapter 6 instruction definitions).
Version-specific CLDC `StackMap` verification, linking and resolution are
**not** implemented; see [N01 evidence grades](specification-map.md).

The first sections below describe the **legacy single-method** `IntProgram::prepare` path. W7's invocation-aware `prepare_with_calls` and its additional supported opcode, frame, and admission rules are specified separately in the final section. Statements below that exclude invocation apply only to the legacy path.

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
Full G2 additionally needs broader class/descriptor constraints,
general type verification where applicable, class loading and symbolic
resolution beyond W7's same-class proof, object identity and typed references,
exception propagation, debugging stops and non-`int` state. W7 has already
proven **bounded** nested calls and return frames; that is not equivalent to
general invocation support. G3 separately requires a real **process A exits →
process B restores** proof. This execution prototype does not justify
a MIDP compatibility, Java certification or JL-Mod Plus integration claim.

## WP02-W7 bounded same-class invocation — experimental contract

This experimental increment was developed under [Issue #22](https://github.com/H3nb/J2ME-VM/issues/22) using the original fixtures from [W6 PR #20](https://github.com/H3nb/J2ME-VM/pull/20). It changes no general CLDC verifier, loader, or G2 gate claim.

A caller explicitly requests `IntProgram::prepare_with_calls(class, name, descriptor)`. The old `prepare(class, name)` remains a single-method compatibility path and still rejects invocation. This new entry must match **both** name and descriptor, and explicitly rejects special method entry names such as `<init>` and `<clinit>` (including other names starting with `<` in the experimental admission policy). It prepares the statically reachable same-class `CONSTANT_Methodref` calls, with only `()I` and `(I)I` descriptors; unused compiler-generated constructors are ignored. Unsupported or malformed reachable callees, invalid constant-pool kind/index, truncated `invokestatic` operands, stack-underflow at invoke, invalid return/CFG, and external calls are rejected before an `IntMachine` exists. Calls do not guess CLDC linkage errors or their timing.

Admission also enforces two **bounded format constraints** from JVMS Second Edition §§4.1 and 4.6 before guest execution: an ordinary class may not be both `final` and `abstract`, and a method may not have multiple visibility flags (`public`, `private`, `protected`). The class-only proof explicitly excludes `ACC_INTERFACE` classes as unsupported, not as universally malformed. Both the legacy one-method path and the invocation path apply the same relevant checks; transitive callees are checked before a machine is constructed. Other class-file verification rules remain outside this subset, and reserved access-flag bits are not blanket-rejected. These are experimental `PrepareError` diagnostics, not claimed CLDC guest exception types or timing.

The synthetic class environment is treated as already initialized **only for this proof**: classes declaring `<clinit>` are excluded, and the superclass is not initialized/loaded. This is not JVM class-initialization semantics. General class loading, fields, objects, `void` or two-argument methods, handlers, native/instance/synchronized methods and legacy CLDC `StackMap` remain unsupported.

A prepared `IntProgram` owns each reachable validated method once, and `IntMachine` owns explicit frames with method ID, position/PC, operand stack and locals. At `invokestatic`, the argument is moved from the caller into the new callee's local 0; the caller's remaining stack and next PC are retained. At callee `ireturn`, the callee frame is removed and the int value is pushed on the caller stack. These transitions each consume **one guest opcode**, with no recursive Rust interpretation. `run(budget)` counts across the complete stack and executes at most 65,536 instructions per call; a normal exhausted budget preserves resumable frames. At most **64 distinct entry/reachable methods** are prepared in one `IntProgram` (experimental method-table resource bound), independently of the maximum **64 active frames, including the entry**, admitted at runtime. A 65th-frame attempt reports `CallDepthExceeded`, separately from budget exhaustion, without performing a partial frame transition; it is a terminal experimental host-policy failure, **not** a Java `StackOverflowError`. Inspection exposes active frame locals/stacks/PCs. The existing exact-one-stack-item `ireturn` rule remains an **experimental restriction**, not a JVMS return requirement.

This increment depends on original 48.0 W6 fixtures; tests and CI must establish its actual behavior. It does **not** establish CLDC verification, heap support, snapshot/restore or G2 completion. Normative base: JVMS Second Edition §§3.6, 4.3.3, 4.4.2, 4.7.3, 4.9, 5.4.3.3, 5.5 and chapter 6 `invokestatic`/`ireturn`, with JSR 139 N01 gaps still pending in Issue #12.
