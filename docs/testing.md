# Testing strategy and corpus policy

**Scope:** This document owns test selection, Java fixture provenance, and the
public/private corpus boundary. [Development](development.md) owns commands and
CI wiring; [validation gates](validation-plan.md) own acceptance criteria; the
[specification map](specification-map.md) owns normative references. A passing
test is evidence for its declared contract, **not** Java ME certification.

## Select tests for failure modes, not volume

Before adding a test, identify its version-specific clause or invariant, the
plausible incorrect behavior it would catch, why this layer is sufficiently
faithful, and whether existing coverage already catches that failure. Prefer
observable outcomes to private implementation layout. Negative cases should
represent distinct failures, not an exhaustive Cartesian product. Do not add
placeholder tests or test-only frameworks before their behavior exists.

| Boundary | Preferred evidence | Important limitation |
| --- | --- | --- |
| Class-file bytes and JAR container parsing | Rust tests with original valid, truncated, malformed and resource-limit inputs | Parsing does not imply bytecode verification or execution. |
| CLDC verification | Original `.class` files with well-defined accepted/rejected `StackMap` and version cases | Legacy CLDC `StackMap` differs from SE `StackMapTable`. |
| Bytecode and class loading | Tiny authored Java classes, then multi-class JARs; check VM-observable results and failures | A host JVM is only an auxiliary oracle for *matching* semantics. |
| Heap and GC | Object identity, reachability, graph invariants and verified reclamation | Avoid asserting internal pointer layout or a particular GC algorithm. |
| Snapshot/restore | Controlled guest workload and **two OS processes**, corrupt/version-incompatible snapshots and atomic failure | Same-process round trip cannot pass G3. |
| Virtual threads and host interaction | Controlled scheduler, clock, events, blocking and resource boundaries | Host media/I/O state is not implicitly restored with VM memory. |
| CLDC / MIDP / optional JSRs | Original purpose-built MIDlets compiled against an appropriate, isolated API profile | Compiling against API JARs proves neither VM execution nor JSR compliance. |
| Real-world compatibility / performance | Permission-cleared third-party MIDlets and private game corpus where lawful; reproducible environment and outcome | Game behavior is empirical evidence, not a substitute for a specification. |

Property testing and bounded fuzzing are useful for untrusted class/JAR/snapshot
parsers once implemented. Keep failing seeds reproducible. Do not require
coverage percentages, mutation scores, speculative test matrices, mock-heavy
harnesses, or permanent full fuzz/benchmark jobs without demonstrated value.

## Public corpus: original, reviewable fixtures

The **default CI must run without proprietary SDKs, commercial games, third-party
VM binaries, Java ME TCKs, or guest JAR downloads**. It may use ordinary,
licensed Rust build dependencies and official toolchain installers.

- Write minimal Java fixture sources **for this project** from the applicable
  versioned specification, not by copying SDK sample programs, emulator tests,
  game decompilations, or reference implementation code.
- Initially use raw bytes and original `.class` fixtures for G1. Add runnable
  authored classes at G2, multi-class JARs when loading is implemented, and
  MIDlet/JAD fixtures only when CLDC/MIDP lifecycle exists. A `.jar` is an archive
  input, not a prerequisite for testing an isolated interpreter instruction.
- Commit small *original, permission-cleared* `.class`/`.jar` fixtures only when
  they catch a real regression. Retain editable source, exact expected observable
  behavior, compiler and preverification recipe, relevant specification
  edition/clause, and binary SHA-256. Binary fixtures must **not** include CLDC,
  MIDP or vendor API class implementations/stubs copied from SDK JARs.
- Preserve exact fixture bytes once a failure is tied to them. A toolchain
  upgrade must not silently regenerate a golden binary; review diffs and
  expected behavior before replacing it.
- When the first checked-in binary fixture is introduced, create a concise
  `tests/fixtures/README.md` and machine-readable manifest (`manifest.json`
  or equivalent). For each fixture record: source path, provenance/license,
  compiler name/version and command, class-file version and verification step
  (if relevant), expected behavior, SHA-256, and target spec clause. Do not
  create an empty registry now.
- Prefer pinned, openly redistributable build tools for reproducible generation.
  If generating a historic CLDC fixture requires an SDK not cleared for public
  CI, generate it offline in a permitted environment, verify that **only our
  own compiled bytecode** is present, and commit just the cleared fixture with
  its provenance. Public CI consumes the known-good fixture without fetching
  that SDK. A compiler-generated binary is not automatically redistribution-
  cleared: inspect embedded classes/resources and source/tool terms first.

Tests should assert guest-observable results (return value, exceptions, event
trace or restored state), not merely that a Java compiler produced a file or a
JAR was loaded. For VM semantics where Java SE and CLDC differ, the versioned
CLDC contract wins; never take the output of an existing emulator as normative.

## Private compatibility corpus

Commercial MIDlets, proprietary games, vendor sample binaries, Java ME API
implementations, reference VMs and TCKs **must not** enter the public Git tree,
Git LFS, CI artifacts, test logs or caches without independently verified rights.
Archive.org availability or possession of a copy does not establish permission
to redistribute or execute it on a hosted runner. Do not download arbitrary
JARs from public mirrors during PR CI.

When there is a concrete compatibility question, use a **separately maintained,
access-controlled corpus** only where storage and execution are permitted. Keep
an inventory outside the public repo: title, provenance, version, license/use
basis, SHA-256, required device/API profile, permitted environments, expected
observations and observed failures. Record results in text only when doing so
reveals no protected content or user data. Never commit full game JARs or
publisher assets to document a failure.

A private/manual compatibility run may augment (not replace) the public spec-
based tests. If private GitHub Actions are ever proposed, separately review
hosting rights, secret handling, artifact retention and fork-PR isolation before
creating a workflow. No private-corpus workflow is justified at scaffold stage.

## Private offline compiler and API references

Permitted private SDK/toolkit material is a **separate research resource, not a
repository dependency**. Possession and private availability do not establish
redistribution rights or permission to use it in public CI. Keep private storage
locations and inventories out of public documentation.

- Use the exact official JVMS/JSR *version and clause* as behavior authority.
  Indexed API documentation and SDK JAR signatures may assist research, but are
  not a normative substitute for the specification.
- For private fixture compilation where permitted, use a **single compatible
  profile**. Do not stack incompatible profiles, duplicate a self-contained API
  set or wildcard optional JSR JARs. Select only required API declarations and
  record the relevant compiler/API versions and SHA-256 in fixture provenance.
- Treat SDK API JARs as **compile-time references only**. Their classes are not
  guest test programs and their presence does not prove an implemented runtime.
  Do not redistribute them or embed their contents in generated fixture JARs.
- Private toolkits may include historical runtime implementation/source material,
  vendor materials and third-party samples. **Do not inspect or use their
  implementation code, internal algorithms or tests** when designing this VM.
  Use only cleared compiler tooling and interface documentation as permitted
  by the project [clean-room policy](licensing-and-provenance.md).
- A toolkit's own build self-test checks its compilation path, not
  VM correctness. It must never be counted as a passing `j2me-vm` test.

No private storage credential or third-party binary should be required
by an external contributor or the public CI to reproduce core project tests.

## Snapshot proof and CI growth

Distinguish (1) pause/resume in one VM, (2) restoration into another VM in one
process, and (3) restoration in a **new OS process after the original ended**.
Only (3) passes G3. Given controlled host input, compare the uninterrupted and
restored VM-visible traces, including frame/PC, object identity, static data and
exceptions. Reject corrupt, oversized or incompatible snapshots *without
mutating* the running target VM.

The default GitHub Actions job remains a fast `fmt` + `clippy` + `cargo test`
gate with `--locked`. When a milestone introduces meaningful Rust tests or
original checked-in fixtures, include them through ordinary `cargo test` so
regressions fail the same job. Do not add a Java SDK install, Archive.org
network step or proprietary JSR JAR download to the default workflow. Add an
independent optional job only when a distinct needed boundary cannot be tested
faithfully with the existing gate. Run expensive fuzzing or measured benchmarks
separately when their value and reproducibility are demonstrated.

Always report exact tests **actually run**, which contract they covered, and
what remains unverified. A green scaffold CI proves no VM functionality.
