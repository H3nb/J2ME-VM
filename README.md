# Kilox

Kilox is an independent Java ME runtime project written in Rust. The goal is
to run MIDlets through a shared VM core on Windows, Linux, Android, iOS and
future hosts, with inspectable execution state and recovery in a new process.
Its initial compatibility scope is CLDC 1.0/1.1 and MIDP 1.0/2.0/2.1.

Public `main` currently contains a bounded class-file reader, a restricted
integer and static-call execution proof, JAR and manifest readers, and an
offscreen pixel surface. The planned MIDlet runtime, host interfaces and
compatibility target are tracked in the [roadmap](docs/project-plan.md).
See [project status](docs/project-status.md) for the integrated baseline and
the limits of experimental results.

## Build and collaborate

Use stable Rust with rustfmt and Clippy. Keep `Cargo.lock` in the checkout.

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Start from [GitHub main](https://github.com/H3nb/Kilox/tree/main), choose
a tracked task in the [milestones](https://github.com/H3nb/Kilox/milestones),
and link the PR to its Issue. A contributor should be able to resume using a
Git ref, the Issue and reproducible commands. See the
[documentation map](docs/README.md) and [development guide](docs/development.md).

## Inspect a package locally

```sh
cargo run --locked --example package_inventory -- path/to/application.jar
```

The report counts archive files, structurally readable classes grouped by
class-file version, and methods carrying a legacy `StackMap` attribute. It
also checks the main manifest when present. An unsuccessful run reports the
first failing stage. Keep suite files in the permitted local corpus. The
output omits package paths, entry names, and guest data. Use a runtime probe
to establish linkage, verification, lifecycle, and restore outcomes.

## AI-assisted development

This project is developed with help from AI coding agents. Agents assist with
research, implementation, original fixtures, testing and review under
[AGENTS.md](AGENTS.md). The same clean-room, provenance and verification
requirements apply to human and AI-assisted contributions. Maintainers are
responsible for reviewing changes, checking evidence and approving publication.

## License and provenance

Project code is licensed under [GNU General Public License v3.0 only](LICENSE).
The project develops its own implementation using permitted specification
inputs and original fixtures. The source tree excludes proprietary Java
libraries, reference VMs, test kits and commercial MIDlets. See
[licensing and provenance](docs/licensing-and-provenance.md) for implementation-use
and distribution constraints. Java ME certification has not been established.
