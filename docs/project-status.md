# Current project status

Public code baseline reviewed on 2026-11-20 at `6aa0c86`. The
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
Both CLDC `String.indexOf` overloads and the abstract `Reader` binding are
now present in the local integration candidate. A fresh process resumed the
original Reader fixture while its virtual character read was suspended.
Three MIDP 2.0 variants pass substring linkage and now share a legacy
`StackMap` type mismatch.

All 11 MIDP 2.1 packages stopped during preparation. Eight need interfaces
outside the current bundle. Seven of those first need interfaces from Payment,
AMMS, Content Handler, Location, SIP or RMI. The other three packages stop on
unsupported field descriptors. These results describe the local corpus and
candidate. Public runtime integration remains subject to the rights review.
The [CLDC milestone](https://github.com/H3nb/Kilox/issues/44) tracks the
shared verification and library work.

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
