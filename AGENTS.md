# j2me-vm — contributor and coding-agent contract

This is the root guidance for humans and AI agents. Its requirements apply to every change. Keep this file durable and concise; follow [the documentation map](docs/README.md) for task-specific contracts. **This project has no functional VM yet.** Never infer capabilities from a roadmap.

## 1. Mission, scope, and authority

Build an **independent, host-neutral Java ME virtual machine in Rust** to investigate reliable VM checkpoint/restore and, eventually, rewind. It is a possible *optional* JL-Mod Plus engine, not a mandate to replace its existing runtime. Desktop research and independently repeatable proof precede Android work.

Use this authority order when deciding behavior:

1. Explicit user requirements and this repository's license, clean-room, and security constraints.
2. Applicable **version-specific primary specifications** (the selected CLDC revision / JSR 139, MIDP 1.0 / JSR 37 or 2.0–2.1 / JSR 118, and the JVMS edition they reference; consult JSR 271 for the future MIDP 3.0 contract), subject to legal access and use terms. CLDC-specific amendments prevail for the CLDC target.
3. Accepted, current [architecture decisions](docs/adr/) and the maintained behavior contract in [architecture.md](docs/architecture.md).
4. Current source, executable tests, and reproducible observations as **implementation evidence**, not as authority over the intended contract.
5. Prior commits, proposals, notes, and agent output as nonbinding history.

Never convert an unverified assumption or test expectation into a normative requirement. When sources conflict, identify the exact section/version, record the unresolved conflict, and avoid implementing a guessed answer.

## 2. Mandatory clean-room design

**All project implementation code must be independently authored from allowed standards and original reasoning.** This restriction is stronger than what the GPL alone requires.

- **Allowed inputs:** authoritative publicly accessible JVMS/JSR normative materials after confirming applicable access/implementation terms; official Java ME API contracts; original design analyses; original bytecode fixtures; compiler-produced fixtures whose license permits use; documented observations from our own test programs.
- **Prohibited inputs:** source, decompiled output, internal design descriptions, implementation-specific algorithms, data structures, function/class naming patterns, generated code, or patches from **other VM/emulator implementations** (including KEmulator, FreeJ2ME, MicroEmulator, and similar projects). Do not browse or search those implementations to solve a problem, including via AI retrieval, code suggestions, or copied test suites. Do not port JL-Mod Plus runtime internals into this VM.
- **Interoperability exception:** exact Java-defined names, signatures, opcodes, error classes, and binary layouts *required by the specification* may be used. These are external interface contracts, not permission to reproduce another project's implementation.
- Never paste lengthy specification text, proprietary JSR source code, reference implementation binaries, TCK materials, proprietary JARs, or unlicensed game assets into this repository. Paraphrase rules and cite the exact primary source section in the relevant design or test.
- For every significant behavior change, record its **normative reference** (specification + edition/revision + clause), design rationale, and provenance of fixtures. Record any empirical assumption explicitly. A short source comment or test note is enough when the context is simple; do not create one provenance form per trivial edit.
- If prohibited implementation material is encountered accidentally, **do not use or adapt it**. Disclose the exposure in the handoff; isolate the affected task and arrange an independent re-derivation from allowed sources before accepting code. Removing attribution does not sanitize copied code.
- Do not claim Java certification, TCK compliance, trademark rights, or that GPLv3 by itself grants specification implementation rights. Follow [licensing and provenance](docs/licensing-and-provenance.md).

## 3. Before changing code

For continuity, read [project-status.md](docs/project-status.md), verify current
remote heads/open work, and select the next unfinished dependency from the
[project plan](docs/project-plan.md). Preserve existing work. Update the handoff
with exact evidence and the next action when an increment ends; do not restart
or expand the project silently. This adds no routine approval requirement and
does not override the user's existing authorization.

1. Read `README.md`, this file, and only the task-relevant links in [docs/README.md](docs/README.md). Inspect actual source, tests, configuration, and the current branch; check whether the proposed feature exists.
2. Define the externally visible behavior and versioned specification scope. Trace which component owns the relevant state, its lifetime, loading/verification, concurrency, and host boundary.
3. State important preconditions, postconditions, invariants, and realistic failure paths. Distinguish *verified findings* from hypotheses. Use a small reversible experiment if the design hinges on an unresolved property.
4. Compare only materially different designs. Choose the solution with **lowest total conceptual complexity** that meets the contract—not the shortest diff. No speculative modules, crates, adapters, caches, fallback layers, feature flags, or configuration knobs.
5. Implement the complete authorized behavior; do not hide gaps behind placeholders, silent success, undefined behavior, or unsupported compatibility claims. If a foundational assumption fails, reassess instead of layering workarounds.

Keep the crate independent from Android and JL-Mod Plus. A change to their repositories, native ABI, or license/integration contract is **separately authorized work**. No implementation should depend on another emulator being present.

## 4. JVM correctness and hard boundaries

- **Untrusted input:** class files, JAR/JAD contents, snapshot blobs, and guest-controlled resources may be hostile. Reject malformed or unsupported versions/structures deterministically; check integer overflows, length arithmetic, recursive/nesting/size limits and allocation before use. A parser success does not imply bytecode verification.
- **Semantics:** implement Java's numeric, conversion, exception, class-loading, initialization, reference identity, monitor, and bytecode rules from the targeted specifications; do not substitute Rust's defaults where they differ. Reject unsupported bytecode and APIs explicitly rather than approximate.
- **CLDC verification:** treat preverification/StackMap requirements as a distinct contract. Do not confuse the legacy CLDC `StackMap` with modern JVMS `StackMapTable`; identify target versions and behavior before implementation.
- **State ownership:** only durable logical identities may cross a checkpoint. No serialized Rust pointers/references, native stack frames, host locks/threads, handles, file descriptors, GPU/audio handles, or assumptions about process address space.
- **Restore:** use a bounded, explicit, versioned format. Validate schema, code/runtime identity, graph consistency, resource policy, and integrity **before committing** state to a VM. Failed restore must not partially replace a running VM.
- **Scheduler:** before claiming thread support, define instruction-budget fairness, monitor ownership/reentrancy, blocking/wakeup, time, interrupt, and checkpoint safepoints. Explicitly document nondeterministic input and host I/O.
- **Save state vs rewind:** a VM-owned snapshot is **not** proof of complete game state preservation. External graphics/audio/files/network/time require a host contract; unknown or live non-rebindable resources must not be silently treated as restored.
- **Rust safety:** `unsafe` is forbidden by crate policy. Do not remove the prohibition without a focused architectural decision, safety invariants, and explicit authorization. Prefer owned data and ordinary Rust lifetime discipline to clever aliasing.

## 5. Dependencies, licenses and security

The repository is **GNU GPL v3.0 only** (`GPL-3.0-only`); see `LICENSE`. For each new crate, justify the feature, assess maintenance/security, inspect the **effective transitive license set** and required notices, and prefer widely maintained Rust crates with compatible **MIT, Apache-2.0, or GPLv3-compatible** terms. Do not assume every crate with `GPL` in its license expression is compatible (GPL-2.0-only is a known counterexample). Do not copy dependency source into our implementation as a clean-room shortcut.

Prefer the standard library when it adequately solves the problem. Pin the resolution with `Cargo.lock` for reproducible research; commit it when the toolchain generates it. Document dependencies and attributions as needed. Do not add `unsafe`, build-time downloads, networked telemetry, or binary blobs casually. Review security advisories and updates when dependencies change. A license scanner is an aid, not a legal guarantee.

No redistributed copyrighted ROM, game JAR, proprietary Java libraries, TCK, reference implementation, or specification PDF in the repo without explicit verified rights. Validate release/legal questions separately from technical success.

## 6. Test philosophy: evidence, not test volume

Follow [testing.md](docs/testing.md). Each test must protect a normative behavior, a real invariant, or a credible failure mode; it should fail under a meaningful incorrect implementation. Prefer focused observable tests over mocks of internals. Use original, provenance-recorded bytecode fixtures; cover positive, malformed, unsupported, limits, and cross-process restore behavior at the appropriate gates.

- A green scaffold does **not** prove a VM, compatibility, snapshot/restore, or rewind.
- The **public CI corpus** uses authored, rights-cleared Java source and necessary reviewed `.class`/`.jar` fixtures, with specification provenance, expected results, compiler/preverification recipe, version and SHA-256. Start with raw/class fixtures; add executable JARs only when the VM can consume them. Never include third-party SDK/API class implementations in guest fixture artifacts.
- The **private compatibility corpus** is separate: commercial games and vendor binaries are not committed, fetched from Archive.org/mirrors during public CI, or uploaded as public workflow artifacts. Private testing also requires a documented lawful storage/execution basis; public availability alone is not permission.
- ChatGPT Library `/J2ME Dev Kit/` is a private offline **compilation/interface reference**, not a clean-room design source or public CI dependency. Its historical VM/runtime implementations and third-party sample source must not be inspected for VM algorithms or copied into fixtures. Its build self-tests do not validate this VM.
- Compare results against the **applicable specification**; a modern JVM can be an auxiliary oracle only when its version/semantics genuinely match the test, and never overrides CLDC rules.
- Test nondeterminism deliberately: control virtual clock, schedule, seed, and host events where necessary. Avoid flaky sleep/retry tests.
- For G3, assert the old OS process really exited and a distinct process independently restored; same-process cloning is insufficient.
- Do not create tests merely to increase counts, coverage, CI jobs, or AI output. Stop after enough distinct failure modes are protected. Record tests not run and limitations honestly.

## 7. Git, review, documentation and handoff

- Work in focused feature/experiment branches; keep `main` reviewable. **Do not create an initial commit, publish, merge, or alter JL-Mod Plus without explicit authority.** Local non-destructive checks are allowed.
- Scope changes to the request; if unrelated debt appears, report evidence and impact without unsolicited refactoring. Preserve user work and attribution. No historical code porting.
- Update the *owning* contract when behavior or a decision changes. Use ADRs for consequential, durable architectural choices; keep experiments and evidence in the validation plan or a dedicated report, not a growing activity log in `AGENTS.md`.
- Inspect the entire effective diff before requesting review. Check correctness, ownership, missing failure paths, licensing/provenance, stale decisions, duplicated abstractions, and regression risk. Retire superseded workarounds instead of stacking them.
- Run relevant `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, and `cargo test --locked` when available. Report exact observed outcomes, the environment, and what **was not run**. Do not claim CI passed without a workflow result.
- Handoff a concise statement of what changed, why it is correct under the cited contract, remaining hypotheses/risks, and evidence. An investigation with no justified findings is acceptable; never manufacture defects or improvements.
