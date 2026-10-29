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

### When to use [skip ci]

GitHub Actions supports `[skip ci]` (also `[ci skip]`) in commit messages
for `push` and `pull_request` events. This is a **manual, exceptional
optimization**, not a substitute for validation. The default is to run CI.

- **Permitted only for an isolated, non-behavioral documentation-only or
  comment-only commit**, after reviewing the diff and confirming that no
  executable code, compiler inputs, tests, fixture bytes/metadata, manifests,
  `Cargo.lock`, `AGENTS.md` engineering policy, GitHub workflow, or normative
  behavior/acceptance contract was changed. Prefer running CI when uncertain.
- **Never** skip CI for Rust/source, tests, checked-in class/JAR fixtures,
  dependencies, workflow permissions, security/licensing or acceptance behavior,
  or changes where the scope cannot be established.
- **Do not put a skip marker on a PR's final/head commit when required
  status checks are configured.** GitHub leaves skipped workflow checks
  *Pending*, blocking the merge. Push a genuine non-skipped validation commit
  and await a successful run if a skip was used earlier. The squash commit
  title/body must not inherit a skip marker if the resulting main push needs CI.
- `workflow_dispatch` can still run the ordinary Rust workflow when a manual
  verification is needed. Never claim a skipped commit was CI-validated; report
  the exact previously checked SHA and the verification gap.

Example for an independently reviewed spelling-only change when no required
check will be left pending: `docs: correct typo [skip ci]`. If any condition
above is unclear, **omit the marker**. Do not introduce path filters merely
to force skipping because required checks may become permanently Pending.

References: [GitHub: skip workflow runs](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/skip-workflow-runs)
and [required checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks).

### CI evidence boundary

The initial crate had no interpreter tests. The G1 parser and WP02 proof now have their own tests; green Rust checks prove only the behavior exercised by those checks. They do not prove Java ME compatibility, save/restore, or certification. Add original contract tests when an actual VM gate is implemented; do not add placeholder tests or extra jobs just for a green badge.

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

For multiple sessions, use the assignment, handoff and integration procedure in
[parallel-development.md](parallel-development.md). Apply the focused review
criteria in [engineering-quality.md](engineering-quality.md); avoid duplicating
those policies in PR templates. Changes to engineering policy or acceptance
contracts require relevant validation and must not use a CI skip marker.

Use the specification clause and current code to identify state owners, error boundaries and invariants before changing architecture. Review the final effective diff for unnecessary layers, stale fallbacks, superseded approaches and weak tests. Report executable evidence separately from hypotheses. Do not consult other emulator implementations for debugging; see the [clean-room policy](licensing-and-provenance.md).
