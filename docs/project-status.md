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

A separate, unpublished runtime branch contains the implementation used for
these experiments. A fresh clone of public `main` can reproduce the structural
inventory above. Reproducing the runtime results requires the experimental
branch and a permitted local corpus.

The experimental branch has demonstrated bounded object execution, virtual
scheduling, portions of CLDC and MIDP, and several guest-state checkpoints
restored in a new process. In a local 23-package smoke run, one MIDP 2.0
package reached an interactive Form, paused, resumed and was destroyed.
Both CLDC `String.indexOf` overloads and the abstract `Reader` binding are
present in the local integration candidate. Fresh-process restore resumes
the original fixture while inherited `read` or `skip` has a pending guest
character read. A separate fixture verifies that a guest reference published
before a constructor throws remains usable after a catch and restore.

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

An isolated 4,096-record diagnostic build produces visible Canvas pixels for
all three variants. After four seconds of simulated host time they remain
Active with 1,174–1,438 records and a quiescent scheduler. The 1,024-record
diagnostic runs exhaust their heap before drawing a visible frame.
A fresh process restores one visible Canvas from the 4,096-record diagnostic
and writes back an identical checkpoint.

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
