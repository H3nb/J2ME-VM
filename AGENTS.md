# J2ME-VM — contributor and coding-agent guidance

This file is the short, always-applicable contract. Use the [documentation map](docs/README.md) to find task-specific details; do **not** read every document for every change. Instructions guide judgment rather than impose a ritual.

## Mission and priorities

Build an independent, host-neutral Java ME VM in Rust, with correct execution and recoverable VM state. It is research toward a *possible optional* JL-Mod Plus engine, not a replacement mandate.

Prioritize, in order: **correct useful capability**, preserved safety/clean-room boundaries, maintainability, and efficient delivery. A working increment or a decision-changing experiment is more valuable than extra plans, test counts, or procedural output. Failure of a bounded experiment is an acceptable result.

This repository currently has a **class-file structural parser and bounded integer/static-call execution proof**, not a general VM, full CLDC verifier, MIDlet runtime, checkpoint/restore, or rewind. Never promote planned behavior to implemented capability.

## Authority and autonomous work

For behavior decisions, follow: (1) explicit user requirements and non-negotiable license/security/clean-room restrictions; (2) applicable, version-specific primary JVM/CLDC/MIDP specifications and their permitted use terms; (3) accepted [ADRs](docs/adr/) and current architecture contracts; (4) source and reproducible observations as implementation evidence; (5) prior proposals, AI output, and historical notes as nonbinding context. Identify unresolved conflicts rather than guessing.

Within an **authorized task**, independently inspect relevant code/contracts, choose a sound design, implement, refactor locally, run relevant checks, repair introduced failures, and prepare a focused PR. Do not stop at planning if the work can proceed. Do not ask again for authorization already granted. Escalate material scope/architecture changes, rights/security questions, or actions outside the authorization; do not expand permission by inference. Publication, merges, releases, and modifications to JL-Mod Plus require the applicable user authorization. Use [development guidance](docs/development.md) and [task continuity](docs/parallel-development.md).

Work from the current remote state and open assignments, not stale chat handoffs. Start with the relevant source/tests and only the documents owning the affected contract. Choose the next independent, ready task when another task is blocked; do not treat a local blocker as a global project freeze.

## Clean-room and rights — mandatory

This project's code is **GPL-3.0-only** and independently authored. Follow [licensing and provenance](docs/licensing-and-provenance.md) and the [specification map](docs/specification-map.md).

- **Allowed design inputs:** applicable authoritative JVM/Java ME specifications after checking access/use terms, official API contracts, original reasoning, and our independently authored experiments/fixtures.
- **Prohibited design inputs:** implementation source, decompilation, internal designs, implementation-specific algorithms/data structures, copied patches or tests from **other VMs/emulators**, including KEmulator, FreeJ2ME, MicroEmulator and JL-Mod Plus runtime internals. Do not search their implementations for solutions, even through AI retrieval.
- Specification-mandated public names, signatures, opcodes and layouts may be used for interoperability. This does not authorize adapting another implementation.
- Do not redistribute proprietary Java libraries, SDK/TCK/RI material, copyrighted games, unlicensed specifications, or third-party sample code. Offline SDK/compiler resources may be used only within their permitted interface/fixture scope; never as design sources for VM internals or public CI dependencies.
- If prohibited implementation material is encountered, do not use it; disclose the exposure and rederive affected work from allowed sources. Deleting attribution does not cleanse copied code.
- Identify the version and normative clause behind consequential guest behavior. Keep unknown final CLDC clauses and implementation-use terms explicit; do not claim Java certification, trademark rights, or legal clearance from GPL alone.

Use the Rust standard library when suitable. Review a new crate's maintenance, security, **transitive licenses**, notices, and actual need before adoption; not every GPL-licensed crate is GPLv3-compatible. Preserve `Cargo.lock`. No unapproved build-time downloads or telemetry.

## Correctness boundaries

- Treat class files, bytecode, JAR/JAD data, snapshots, and guest-controlled inputs as hostile. Validate versions, indexes, sizes, arithmetic and bounds; fail explicitly on unsupported behavior. Parsing is **not** CLDC verification.
- Follow targeted Java numeric, linking/initialization, reference, exception and bytecode semantics rather than Rust defaults. Distinguish CLDC `StackMap` from later `StackMapTable` and avoid guessing error timing.
- VM-owned execution state must have clear lifetime and ownership. Never serialize raw pointers, host stacks, OS threads/locks, file descriptors, GPU/audio handles, or other unowned native resources as durable identities.
- Snapshot/restore, when implemented, must be bounded, versioned, validated **before state mutation**, and proven in a **different process**. VM-state restoration alone does not prove a complete game save; external effects need explicit host policy.
- `unsafe` is forbidden by current crate policy. A change requires explicit authorization, a focused architecture decision and reviewed safety invariants.

## Engineering and verification

Choose the **lowest total conceptual complexity** satisfying the current contract. Fix root causes; reuse sound mechanisms; retire superseded layers and workarounds within scope. Avoid speculative crates, traits, fallbacks, compatibility flags, mocks, guards, and exhaustive edge-case analyses without a credible requirement. Safety-critical input and restore checks are **not** optional micro-details.

Scale investigation and testing to risk. Each new test should catch a plausible incorrect result, normative violation, or material regression not already covered. Test observable behavior; avoid test-volume targets and unhelpful CI expansion. Original public bytecode fixtures require rights, compiler recipe, version, expected result and hashes; private game/vendor corpus stays out of public repository, workflows and artifacts. See [testing](docs/testing.md) and [quality review](docs/engineering-quality.md).

Use focused checks during iteration and the relevant existing validation before acceptance. Never weaken production behavior or tests just to turn CI green; state exact checks run, tested SHA, and important gaps. A green Rust workflow is not Java ME conformance.

## Code clarity and comments

Write cohesive Rust code with explicit ownership, understandable data flow and error paths. Prefer one clear source of truth and the least conceptual machinery that meets the contract; remove obsolete layers within scope rather than stacking workarounds. Do not trade away hostile-input checks, Java semantics or VM-state invariants for shorter code. Follow [engineering quality](docs/engineering-quality.md) for risk-proportional review.

- Let names, types and structure explain **how** code works. Comment only when useful intent or constraints are not apparent: **why** a design exists, invariants, ownership/lifetime boundaries, non-obvious failure modes, or deliberate compatibility and resource limits. Do not narrate obvious operations or generate boilerplate comments.
- Distinguish specification-mandated guest behavior from project-defined host policy and experimental limitations. For consequential JVM/CLDC behavior, identify the applicable version and normative clause when known; never copy or adapt another VM's implementation explanation.
- Use Rust `///` and `//!` documentation when callers need API or module contracts, preconditions, errors, side effects or limitations that signatures do not convey. Trivial internal helpers need no obligatory rustdoc.
- Keep comments concise and next to the behavior they qualify. Prefer simplifying genuinely confusing code, but do not introduce abstractions solely to remove useful comments.
- Treat existing comments as claims to verify against source and applicable contracts, not authoritative proof. Update or remove stale comments with affected changes; leave unrelated bulk comment cleanup out of scope.
- Write TODOs only for concrete, actionable gaps with enough context. During review, judge comments and code by correctness, clarity and maintenance value—not by comment coverage, line counts, stylistic quotas or test volume.

## Git, documentation, and handoff

Use focused branches and PRs, preserve unrelated work, and review the **final effective diff** against the current base. Respect dependencies and the integration boundary; merging is not automatic authorization. Multiple agents are optional; use the [parallel-development rules](docs/parallel-development.md) only when tasks genuinely overlap or run concurrently.

Update an **owning document** only when accepted behavior, a durable decision, or a material blocker changes. Keep research hypotheses separate from accepted contracts. Do not write activity diaries, duplicate PR histories, or create new policies for one-off problems. Keep public records free of private paths, session identifiers, secrets and unrelated personal details.

A concise handoff is enough: **what now works, verification evidence, remaining blockers/limits, and next actionable step**. No manufactured findings or mandatory PR/test/document quota.
