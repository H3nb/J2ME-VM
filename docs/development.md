# Development and pre-commit checks

## Rust toolchain and CI

Use maintained Rust stable with `rustfmt` and `clippy`. The toolchain is intentionally unpinned; record the exact compiler version for repeatable experiments. A committed `Cargo.lock` fixes dependency resolution, not the compiler version or host.

```sh
cargo generate-lockfile
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Commit the Cargo-generated lockfile. `--locked` makes checks fail when the lockfile is missing or stale. The single permanent workflow at `.github/workflows/rust.yml` runs on main pushes, pull requests and manual dispatch. It uses an Ubuntu runner, read-only permissions, rustfmt and clippy.

The initial `Cargo.lock` was generated and validated with a temporary GitHub Actions bootstrap workflow because the scaffold environment lacked Rust. That workflow was removed after successful validation and lockfile incorporation; the permanent Rust workflow owns ongoing validation.

### CI evidence boundary

The initial crate has no interpreter or guest behavior tests. Green Rust checks prove formatting, compiler lint and compilation hygiene only. They do not prove Java ME compatibility, save/restore, or certification. Add original contract tests when an actual VM gate is implemented; do not add placeholder tests or extra jobs just for a green badge.

### Java ME guest fixtures and SDK separation

Public CI deliberately needs no JDK, proprietary JAR, Java ME SDK or access to ChatGPT Library. The Library `/J2ME Dev Kit/` is available for private, licensed, local fixture compilation, **not** as a runtime implementation source and not as a public CI download. When a VM test first needs bytecode, admit original guest fixtures with authored Java source, compile recipe, version, SHA-256, rights and expected result per [testing.md](testing.md). No live downloads from Archive.org, no commercial MIDlets and no vendor API JARs in public CI.

Review source and transitive licenses before introducing crates. Add automated dependency advisory/license checks when real dependencies warrant them, and add cross-platform, benchmark or fuzz CI only when protecting distinct implementation risks.

## Before material changes and releases

1. Review [AGENTS.md](../AGENTS.md), the [documentation map](README.md), and relevant ADRs.
2. Check license metadata, ownership/visibility and full GPL-3.0-only text. Treat JSR implementation rights and trademark claims as separate open legal questions.
3. Exclude third-party emulator code, proprietary API JARs, SDK samples, TCK, secrets and generated binaries without provenance.
4. Generate `Cargo.lock` with real Cargo and validate it; review effective changes.
5. Commit within the authorized scope; do not publish releases, distribute third-party materials, or merge unrelated work without authorization.

## Change review and handoff

Use the specification clause and current code to identify state owners, error boundaries and invariants before changing architecture. Review the final effective diff for unnecessary layers, stale fallbacks, superseded approaches and weak tests. Report executable evidence separately from hypotheses. Do not consult other emulator implementations for debugging; see the [clean-room policy](licensing-and-provenance.md).
