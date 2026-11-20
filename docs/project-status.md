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

Three MIDP 2.0 variants now pass their previous legacy `StackMap` and protected
constructor limits. They complete preparation and construction, enter
`startApp`, and fault during startup. Two reach the 64-object heap cap after
collection. The third now opens a 1,738,360-byte JAR resource through
`Class.getResourceAsStream`. Its own `new byte[]` of that size exceeds the
current 1 MiB element limit. An original fixture reads a DEFLATE entry of
the same size and restores two independent streams in a fresh process. The
interactive Form sample still passes input, command dispatch and fresh-process
recovery.

All 11 MIDP 2.1 packages stopped during preparation. Eight need interfaces
outside the current bundle. Seven of those first need interfaces from Payment,
AMMS, Content Handler, Location, SIP or RMI. The other three packages stop on
unsupported field descriptors. These results describe the local corpus and
candidate. Public runtime integration remains subject to the rights review.
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
