# j2me-vm

**Experimental, independent Java ME VM research in Rust** — investigating whether a VM that owns its execution state can support reliable checkpoint/restore and eventually rewind. Intended as a possible *optional* engine for JL-Mod Plus, **not** a replacement for its existing Android runtime.

> **Current status — architecture/scaffold only.** There is **no bytecode interpreter, MIDlet runtime, functioning snapshot, or rewind**. No Java ME compatibility or TCK certification is claimed.

## Research goals and gates

1. Validate class input against version-specific CLDC 1.1 requirements, including correct distinction between parsing and verification.
2. Implement a small bytecode interpreter with VM-owned frames and observable specification-correct results.
3. Prove a bounded, versioned snapshot can reconstruct VM-owned heap, PC, frames and class state in a **new process**, with the old process terminated.
4. Extend proof to virtual threads, monitors, timers, and explicit host callbacks without guessing at unsnapshotable resources.
5. Evaluate CLDC 1.1 → MIDP 2.0 in phases, then consider whether integration into JL-Mod Plus is viable.

A **VM core snapshot is not a complete game save state**: display, audio, I/O, host resources, and external time must be reconciled separately. Failure of the proof is an acceptable research outcome; we will not paper it over with compatibility patches or overbuilt frameworks.

## Scope and structure

- A **single Rust library crate**; desktop first. No Android/JNI, JIT, full Java SE, proprietary device extensions or speculative crate workspace at this stage.
- [`AGENTS.md`](AGENTS.md): mandatory engineering and clean-room rules for humans and coding agents.
- [`docs/README.md`](docs/README.md): documentation map; use it to locate the owning architecture, standards, testing, validation and licensing policy.
- [`docs/architecture.md`](docs/architecture.md): current technical hypotheses and state ownership.
- [`docs/validation-plan.md`](docs/validation-plan.md): gates with concrete success/failure evidence.
- [`docs/testing.md`](docs/testing.md): original guest fixture policy, public CI and private compatibility corpus.
- [`docs/adr/`](docs/adr/): accepted architectural decisions.

## Development

With a current stable Rust toolchain, rustfmt and clippy installed:

```sh
cargo generate-lockfile # only before committing the initial lockfile
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

These are **scaffold checks**, not proof that a VM exists. The crate does not currently depend on external packages. Generate and commit `Cargo.lock` before the initial commit; the CI deliberately requires it. See [development guidance](docs/development.md).

## Clean-room and licensing

**License:** [GNU General Public License v3.0 only](LICENSE) (`GPL-3.0-only`). Our implementation must be written independently from *official version-appropriate JVMS/JSR technical contracts and original tests*, **never by copying, studying or adapting code/implementation internals from an existing VM/emulator**. External Java-defined names needed for interoperability are dictated by the specifications. Third-party crates must pass dependency/attribution review; none are included yet.

The GPL license **does not by itself grant permission** to reuse copyrighted Java class libraries, proprietary JSR PDFs/reference implementations/TCKs, or to claim Java certification/trademark rights. The legal conditions for implementing applicable JSR specifications require separate review before public conformance claims or integration. See [licensing and provenance](docs/licensing-and-provenance.md) and [specification map](docs/specification-map.md).

This project is not affiliated with or endorsed by Oracle or Java platform owners.
