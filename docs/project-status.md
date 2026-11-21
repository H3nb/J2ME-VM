# Current project status

Public code baseline reviewed on 2026-11-20 at `fc89ce9`. The
[roadmap](project-plan.md) owns development order and compatibility criteria.
[GitHub Issues](https://github.com/H3nb/Kilox/issues) own active assignments.

## Integrated public main

The [detailed capability inventory](capability-inventory.md) records checked
behaviors and remaining JVM, CLDC, MIDP and host gaps. On the reviewed public
`main`, verified bounded features include a structural classfile parser,
integer and same-class static-call execution proofs, ZIP/manifest readers and
an offscreen pixel surface. These are **foundations**, not a complete class
loader, CLDC runtime, MIDP environment or new-process save-state system.

For current implementation tasks use the [milestone roadmap](project-plan.md)
and its linked GitHub Issues; do not treat the inventory as a second task
registry. The [standards and dependency audit](standards-dependency-audit.md)
records type-level API coverage, unresolved member-level requirements and crate
admission decisions.


The public [package inventory](../examples/package_inventory.rs) reports
bounded structural archive, manifest and class-file results without exposing
suite names. A local run completed for all 23 corpus JARs, each with methods
carrying legacy `StackMap` attributes. This stage measures structural
admission only. Runtime preparation and execution are reported below.

## Local runtime experiments

The implementation used for these experiments is on the local
`codex/cldc-midp-runtime` branch. A fresh clone of public `main` can reproduce
the structural inventory above. Reproducing the runtime results requires
access to the local implementation and a permitted corpus. Public integration
is tracked in the [baseline milestone](https://github.com/H3nb/Kilox/issues/43).

The experimental branch has demonstrated bounded object execution, virtual
scheduling, portions of CLDC and MIDP, and several guest-state checkpoints
restored in a new process. In a local 23-package smoke run, one MIDP 2.0
package reached an interactive Form, paused, resumed and was destroyed.
Both CLDC `String.indexOf` overloads and the abstract `Reader` binding are
present in the local integration candidate. Fresh-process restore resumes
the original fixture while inherited `read` or `skip` has a pending guest
character read. A separate fixture verifies that a guest reference published
before a constructor throws remains usable after a catch and restore.

The local CLDC binding now covers `String.replace`, `String.regionMatches`,
and eight `String.valueOf` overloads, including binary32 formatting through
`Float.toString`, as well as `StringBuffer` array
append, insertion, and copying. Original guest fixtures check UTF-16,
mutation independence, invalid ranges, and fresh-process recovery.

The local default `InputStreamReader` converts ISO-8859-1 through virtual
guest stream calls. An original fixture resumes a pending read in a fresh
process. The named-encoding constructor remains open. All 69 package/profile
outcomes and diagnostics match the earlier run after these library changes.

Guest code can read the emulated configuration, profile, platform, and
default encoding through `System.getProperty`. Its identity hashes survive
collection and fresh-process restore. For untrusted MIDlets,
`getAppProperty` reads JAD values before manifest fallback. A new process
restores the fixture with the same JAD. A changed JAD is rejected before
the target session changes.

Three MIDP 2.0 variants pass their previous legacy `StackMap` and protected
constructor limits. With the default heap, two still reach the 64-record cap
during `startApp`. The third opens a 1,738,360-byte DEFLATE resource but
exceeds the default limit when allocating a byte array of the same size.

The opt-in expanded profile allows 256 object records and permits a byte
array to use the existing 4 MiB aggregate array budget. An original fixture
verifies the resource byte for byte across fresh-process restore. All three
variants return from `startApp` and reach a Canvas under this profile. Their
active Canvas state restores in a fresh process before the first visible
frame. Continued execution reaches the 256-record cap. The interactive Form
sample passes input, command dispatch and fresh-process recovery with both
profiles.

The local runtime now has an opt-in `Large` profile with 4,096 object records
and a shared budget of 262,144 user-instance field slots. All three Canvas
variants draw visible frames. For each variant, a fresh process restores its
first visible frame and runs another six seconds of simulated time with the
same timed keypad events. The frame-sequence checksums and final checkpoints
match uninterrupted execution. The interactive Form also passes its lifecycle
and input checks with `Large`. The 46 legacy and expanded package/profile
outcomes and diagnostics are unchanged.

All 11 MIDP 2.1 packages stopped during preparation. Eight need interfaces
outside the current bundle. Seven of those first need interfaces from Payment,
AMMS, Content Handler, Location, SIP or RMI. The other three first declare
fields typed as `Formatter`, `EGL10`, or `APDUConnection`.
[JSR 238](https://www.jcp.org/en/jsr/detail?id=238) ·
[JSR 239](https://www.jcp.org/en/jsr/detail?id=239) ·
[JSR 177](https://www.jcp.org/en/jsr/detail?id=177)

Four MIDP 2.0 packages first stop on fields typed as `SensorConnection`,
`VideoControl`, `Graphics3D`, or `World`. Both heap profiles report the same
first missing field types at preparation.
[JSR 256](https://www.jcp.org/en/jsr/detail?id=256) ·
[JSR 135](https://www.jcp.org/en/jsr/detail?id=135) ·
[JSR 184](https://www.jcp.org/en/jsr/detail?id=184)

These results describe the local corpus and candidate. Public runtime
integration remains subject to the rights review.

A separate test of 94 additional JARs declaring CLDC 1.1 and MIDP 2.1 in the
manifest stopped at preparation. Skipping opaque Code metadata let one
archive reach method resolution. Two archives first need String
`ConstantValue`.

At local runtime commit `4b4d9db`, original guest tests cover `Vector`
constructors, protected storage fields, positional operations, and
fresh-process restore while an insertion holds the monitor. Equality-based
search and removal, enumeration, and text conversion remain open.
All 94 archives still stopped at preparation. A temporary diagnostic at the
same commit showed that all 34 originally blocked on a `Vector` field reached
the next field check. Across the 94, the first unsupported field types are
now `Sprite` or `Sprite[]` in 58, `Hashtable` in 20, `Stack` in one, and
`TextBox` in one.

Local runtime commit `0bac52a` adds the protected `Writer` constructors
and lock field, UTF-16 convenience writes, and dispatch to a guest subclass.
An original fixture resumes the guest override in a fresh process while it
holds its configured lock. The concrete `OutputStreamWriter` remains open.

At local runtime commit `1c1d844`, virtual `Object.equals` and `hashCode`
reach guest overrides and the String, Integer and Long implementations.
Inherited defaults preserve object identity for ordinary and array receivers.
Commit `b89340a` adds CLDC 1.1 weak references. The collector clears
weak-only referents together and keeps ordinary subclass fields strongly
reachable. Fixtures recover live and cleared references in a separate
process. The combined candidate passed 714 locked offline Cargo tests and
strict all-target Clippy on Windows MSVC.

Commit `6b17efd` adds `Vector` search and removal through virtual guest
equality. The original fixture restores while equality is pending and
verifies that an exception releases the Vector monitor. This candidate
matched all 69 prior
package/profile diagnostics and reproduced three Canvas runs through
fresh-process recovery, plus interactive Form input and lifecycle recovery.

Local commit `48f6fc3` binds the five declared Stack methods and
`EmptyStackException`. An original fixture restores a pending search through
the inherited Vector implementation. In the additional MIDP 2.1 corpus, the
one archive previously stopped by a Stack field now reaches method admission,
where it needs support for a private synchronized method. All 94 archives
still stop during preparation.

Local commit `96506dd` initializes String-valued `ConstantValue` fields
through the interned literal pool before the declaring initializer. Both
previously blocked archives reach method resolution. Their next unresolved
references are `Display.callSerially(Runnable)` and
`IOException.toString()`. The other 92 outcomes match the Stack candidate.

Local commit `fccf0c3` adds `Vector.elements()`. Its original fixture checks
independent cursor positions, live growth, null elements, exhaustion, and
fresh-process recovery after collection. Vector text conversion remains
open.

The [CLDC milestone](https://github.com/H3nb/Kilox/issues/44) tracks the
shared verification and library work.

## Active dependencies

- Establish the implementation-use and distribution basis for public runtime
  integration. The [licensing and provenance record](licensing-and-provenance.md)
  tracks this review. The [specification map](specification-map.md) tracks
  technical evidence for the final CLDC clauses.
- [#33](https://github.com/H3nb/Kilox/issues/33) tracks the original local
  typed-reference feasibility experiment. Review it in the CLDC milestone
  alongside other independently authored candidate changes.
- Establish the reproducible baseline first. Then diagnose the first shared
  verification failure with a small original fixture and continue with the
  highest-impact CLDC and MIDP gaps.
