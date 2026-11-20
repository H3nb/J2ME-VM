# Current project status

Public code baseline reviewed on 2026-11-20 at `3b43c43`. The
[roadmap](project-plan.md) owns development order and compatibility criteria.
[GitHub Issues](https://github.com/H3nb/Kilox/issues) own active assignments.

## Capability checklist — integrated public main

A checked item means the **bounded behavior named in that item** is implemented
and exercised by original tests on public `main`. It does not imply full JVM,
CLDC or MIDP compliance. An unchecked item means the *complete* capability is
not yet integrated, even when supporting pieces or local experiments exist.

### Class files and execution

- [x] Parse bounded CLDC-era class-file structures (45.x–48.0), including
  structural reference and input validation.
- [x] Execute an explicit integer subset (constants, locals, addition,
  subtraction, branches and return) with VM-owned state and step budgets.
- [x] Execute bounded same-class `invokestatic` calls with nested/recursive
  VM-owned frames.
- [ ] Load, link, initialize and verify guest classes, including legacy CLDC
  `StackMap` semantics.
- [ ] Support general JVM execution: references/heap, fields, arrays, method
  dispatch and exceptions.
- [ ] Support garbage collection, virtual threads, monitors and scheduling.

### Packages and graphics

- [x] Read bounded ZIP/JAR entries (stored and DEFLATE), checking package
  structure, content length and CRC.
- [x] Read the main attributes of a package manifest.
- [x] Allocate bounded offscreen ARGB surfaces and perform clipped fills/copies.
- [ ] Resolve JAD/JAR metadata, load classes and launch a MIDlet suite.
- [ ] Implement MIDP display, graphics, commands and input with host presentation.

### Runtime and portability

- [ ] Implement the targeted CLDC 1.0/1.1 class libraries.
- [ ] Implement MIDP 1.0/2.0/2.1 lifecycle, RMS, GCF and media behavior.
- [ ] Save and validate versioned VM checkpoints; restore execution in a
  different process.
- [ ] Run the integrated MIDlet runtime on Windows/Linux and Android/iOS hosts.

Implementation and test evidence: [execution contract](wp02-execution-contract.md),
[package archives](https://github.com/H3nb/Kilox/pull/36),
[manifest](https://github.com/H3nb/Kilox/pull/38),
[raster surfaces](https://github.com/H3nb/Kilox/pull/37), and
[CLDC research/fixture evidence](https://github.com/H3nb/Kilox/pull/42).
For *next work*, use the [roadmap](project-plan.md) and its linked GitHub
Issues rather than copying task checklists here.

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
