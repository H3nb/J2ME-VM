# Current project status

Public baseline reviewed on 2026-11-20 at `8b14b82`. The
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

## Experimental evidence

A separate experimental runtime has demonstrated bounded object execution,
virtual scheduling, portions of CLDC and MIDP, and several guest-state
checkpoints restored in a new process. In a local 23-package smoke run, one
MIDP 2.0 package reached an interactive Form, paused, resumed and was destroyed.
Three variants advanced through additional bindings to a shared legacy
`StackMap` type mismatch. All 11 MIDP 2.1 packages stopped during preparation.
The run measured a specific local candidate and corpus, with public
integration and broad compatibility still pending.

## Active dependencies

- [#30](https://github.com/H3nb/Kilox/issues/30) tracks the implementation-use
  and distribution basis. Keep final CLDC technical evidence and legal rights
  traceable independently. See the [specification map](specification-map.md)
  and [licensing policy](licensing-and-provenance.md).
- [#33](https://github.com/H3nb/Kilox/issues/33) tracks the original local
  typed-reference feasibility experiment. Review it in the CLDC milestone
  alongside other independently authored candidate changes.
- Establish the reproducible baseline first. Then diagnose the first shared
  verification failure with a small original fixture and continue with the
  highest-impact CLDC and MIDP gaps.
