# Current project status

Public baseline reviewed on 2026-11-20 at `fa81687`. The
[roadmap](project-plan.md) owns development order and compatibility criteria.
[GitHub Issues](https://github.com/H3nb/Kilox/issues) own active assignments.

## Public main

- Bounded CLDC-era class-file structural parsing and a restricted integer,
  locals, branch and same-class static-call execution proof with VM-owned
  frames. [Execution contract](wp02-execution-contract.md)
- Bounded ZIP package admission, manifest main attributes and offscreen pixel
  surfaces. [Package archives](https://github.com/H3nb/Kilox/pull/36) ·
  [Manifest](https://github.com/H3nb/Kilox/pull/38) ·
  [Raster surfaces](https://github.com/H3nb/Kilox/pull/37)
- Original test fixtures and dated technical evidence for legacy CLDC
  verification and loading. Full legacy verification, general JVM execution,
  MIDP and fresh-process recovery remain release milestones.
  [CLDC evidence](https://github.com/H3nb/Kilox/pull/42)

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
now present in the local integration candidate. A fresh process resumed the
original Reader fixture while its virtual character read was suspended.
Three MIDP 2.0 variants now pass their previous legacy `StackMap` check
with the verifier using instruction-entry locals on exception edges.
They now reach a separate preparation limit on guest constructor calls
inside exception-handler ranges. The interactive Form sample still passes
input, command dispatch and fresh-process recovery.

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
