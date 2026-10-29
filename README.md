# j2me-vm

**Experimental, independent Java ME VM research in Rust** — investigating whether a VM that owns its execution state can support reliable checkpoint/restore and eventually rewind. Intended as a possible *optional* engine for JL-Mod Plus, **not** a replacement for its existing Android runtime.

> **Current status — G1 and bounded WP02 static invocation proof integrated (partial G2).** G1 reads bounded CLDC-era class structures. WP02 validates and steps a restricted integer `()I` / `(I)I` subset with locals, branching, nested/recursive same-class `invokestatic`, and explicit VM-owned call frames. It is **not** full class-file/CLDC verification. There is **no general bytecode interpreter, object heap, exception propagation, MIDlet runtime, functioning snapshot, or rewind**. No Java ME compatibility or TCK certification is claimed.

## Research goals and gates

1. Validate class input against version-specific CLDC 1.1 requirements, including correct distinction between parsing and verification.
2. Implement a small bytecode interpreter with VM-owned frames and observable specification-correct results.
3. Prove a bounded, versioned snapshot can reconstruct VM-owned heap, PC, frames and class state in a **new process**, with the old process terminated.
4. Extend proof to virtual threads, monitors, timers, and explicit host callbacks without guessing at unsnapshotable resources.
5. Deliver declared MIDP 1.0–2.1 compatibility in phases, keeping foundations suitable for later MIDP 3.0 on CLDC; evaluate Android hosting after evidence. See the [profile evolution study](docs/profile-evolution.md).

A **VM core snapshot is not a complete game save state**: display, audio, I/O, host resources, and external time must be reconciled separately. Failure of the proof is an acceptable research outcome; we will not paper it over with compatibility patches or overbuilt frameworks.

## Scope and structure

Development follows the adopted [staged project plan](docs/project-plan.md):
define the next increment, reduce its main risks, and accept executable evidence
before expanding scope. Read [project status](docs/project-status.md) to resume
existing work without duplicating open PRs. Architecture and gates describe
targets, not capabilities already implemented.

- A **single Rust library crate**; desktop first. No Android/JNI, JIT, full Java SE, proprietary device extensions or speculative crate workspace at this stage.
- [`AGENTS.md`](AGENTS.md): mandatory engineering and clean-room rules for humans and coding agents.
- [`docs/README.md`](docs/README.md): documentation map; use it to locate the owning architecture, standards, testing, validation and licensing policy.
- [`docs/architecture.md`](docs/architecture.md): current technical hypotheses and state ownership.
- [`docs/validation-plan.md`](docs/validation-plan.md): gates with concrete success/failure evidence.
- [`src/classfile.rs`](src/classfile.rs): G1 bounded, borrowed class-file reader (versions 45.x–48.0), not a verifier.
- [`src/execution.rs`](src/execution.rs): WP02 bounded integer execution with VM-owned nested frames, same-class static calls, stepping and budgets; see [scope/limitations](docs/wp02-execution-contract.md).
- [`docs/testing.md`](docs/testing.md): original guest fixture policy, public CI and private compatibility corpus.
- [`docs/adr/`](docs/adr/): accepted architectural decisions.

## Development

With a current stable Rust toolchain, rustfmt and clippy installed:

```sh
cargo generate-lockfile # only when deliberately updating dependency resolution
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

These validate the Rust crate and the exercised parser/experimental execution tests, **not** complete JVM/CLDC compatibility or recovery. The crate does not currently depend on external packages. `Cargo.lock` is committed for reproducibility; CI deliberately requires it. See [development guidance](docs/development.md).

## Clean-room and licensing

**License:** [GNU General Public License v3.0 only](LICENSE) (`GPL-3.0-only`). Our implementation must be written independently from *official version-appropriate JVMS/JSR technical contracts and original tests*, **never by copying, studying or adapting code/implementation internals from an existing VM/emulator**. External Java-defined names needed for interoperability are dictated by the specifications. Third-party crates must pass dependency/attribution review; none are included yet.

The GPL license **does not by itself grant permission** to reuse copyrighted Java class libraries, proprietary JSR PDFs/reference implementations/TCKs, or to claim Java certification/trademark rights. The legal conditions for implementing applicable JSR specifications require separate review before public conformance claims or integration. See [licensing and provenance](docs/licensing-and-provenance.md) and [specification map](docs/specification-map.md).

This project is not affiliated with or endorsed by Oracle or Java platform owners.
