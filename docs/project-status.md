# Current project status

Public code baseline reviewed on 2026-11-20 at `3b43c43`. The
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
registry.

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
