# WP02: bounded arithmetic execution proof

**Status:** Accepted limited research proof via PR #3; partial evidence for G2
only. Normative references: [JVMS Second Edition §3.6 and §3.11](https://docs.oracle.com/javase/specs/jvms/se6/html/Overview.doc.html),
[§4.7.3/§4.9](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html),
[opcode listing](https://docs.oracle.com/javase/specs/jvms/se6/html/Mnemonics.doc.html).
CLDC 1.1/1.1.1 applicability and StackMap obligations remain governed by JSR 139,
not modern Java SE verification.

## A deliberately narrow entry point

Input is a structurally parsed G1 `ClassFile`. The prototype selects exactly
one method with a specific name and requires **static, concrete,
unsynchronized, exact `()I` descriptor**. It accepts no locals,
exception handlers or nested Code attributes; a 4096-byte code/64-slot stack
limit is a project resource budget. It rejects absent/ambiguous names and
unsupported method shapes. It neither loads, links nor initializes the class;
all other methods are outside this proof.

Supported bytecodes only:

| Bytecode | Meaning | Acceptance rule |
| --- | --- | --- |
| `iconst_m1`–`iconst_5` | Push signed int constant | Stack does not exceed declared max |
| `bipush`, `sipush` | Sign-extended 8-/16-bit immediate | All operand bytes present |
| `iadd`, `isub` | Pop 2 ints, push result | Prior stack depth ≥ 2; wrap as Java 32-bit int |
| `ireturn` | Return exactly 1 int | Stack depth exactly 1; final instruction |

Every instruction of the method is decoded and the straight-line stack depth
checked **before a machine can be created**. Unsupported opcode, truncation,
underflow, overflow, missing return, or bytes after return fail closed with an
offset. No branch target, dataflow join or local-variable type rule is claimed:
those instructions are intentionally rejected instead of partially verified.

The validator creates an owned `IntProgram`; `IntMachine` owns its program,
operand stack, next instruction position and result. Stepping exposes the
bytecode PC and read-only operand stack. No native stack continuation, process
global class table, JIT or host resources are involved. This is a **single
invocation proof**, not yet a class loader, separate Java thread, general
debugger or snapshot API. An instruction position may be mapped to PC in the
verified program; no Rust pointer becomes a durable reference.

## Distinct validation and execution guarantees

G1 parsing is not a `VerifiedClass`; the returned `IntProgram` is not a CLDC
certification token. The Java 1.4 fixture was not preverified and its enclosing
class may contain methods not checked for execution. No public API should
present this proof as acceptance of arbitrary guest classes or MIDlets.

The first vertical test parses the original `MinimalClass.class` from G1,
checks only `answer()I`, steps `bipush 42; ireturn`, inspects PC/stack and
returns 42. Focused tests reject forbidden bytecode and incomplete operands,
exercise signed integer arithmetic and independent instances. A green CI
demonstrates only these cases.

## Required next contract extension

1. Resolve affected N01 final CLDC preverification/StackMap clauses before
   claiming conformant execution admission. Do not treat `StackMapTable`
   as interchangeable with legacy `StackMap`.
2. Add exact descriptor and method/class structural constraints, locals and
   definite assignment, variable-length instruction decoding and validated
   branch target boundaries with dataflow joins. Reject unsupported paths.
3. Add explicit VM-owned call frames and error propagation as calls and
   exceptions arrive. Class resolution and initialization remain distinct;
   do not move observable resolution failure earlier than normative rules.
4. Extend instruction stepping to bounded inspection/stops and only then
   design serialization around logical PC, code identity and owned state.

**Exit:** A reviewer confirms source provenance and effective diff; permanent
GitHub CI actually checks formatting, Clippy, positive/negative test cases
and confirms there are no new dependencies or third-party fixtures. Do not
mark the full G2 gate passed on this milestone.
