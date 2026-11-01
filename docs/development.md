# Development, validation and delivery

This document owns **toolchain commands, CI policy and delivery mechanics**. Engineering decisions follow [AGENTS.md](../AGENTS.md); test/fixture scope follows [testing](testing.md); quality review follows [engineering quality](engineering-quality.md).

## Rust and CI baseline

Use maintained stable Rust with `rustfmt` and `clippy`. The toolchain is currently unpinned; record the version for reproducible experiments. `Cargo.lock` pins dependencies, **not** compiler/host. Regenerate it only when deliberately changing resolution.

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

`.github/workflows/rust.yml` runs these checks on PRs, `main` pushes and manual dispatch with read-only permissions. During implementation run the most relevant focused checks; run the applicable existing baseline on the final candidate. A docs-only change needs link/consistency review; don't add test infrastructure for it. Do not repeat full suites after every edit or pretend green Rust CI establishes full Java ME compatibility.

Add cross-platform CI, fuzzing, benchmarks, license scanners or extra jobs **when they protect a demonstrated risk**. Review third-party source/transitive licenses before adding dependencies. Public CI must not depend on private SDKs, proprietary guest binaries, downloaded commercial games, Java ME RI/TCK, or unpublished tools. Original bytecode fixtures require independently written Java source, compile recipe, expected result, version, hashes and usage rights, as detailed in [testing](testing.md).

### When to use [skip ci]

Default: **do not skip CI**. The marker `[skip ci]` is allowed only exceptionally for an independently inspected, isolated spelling/comment/nonbehavioral docs change that affects **no** engineering policy, normative behavior, acceptance contract, source, tests, fixtures/metadata, dependencies, lockfile or workflows.

Never skip for `AGENTS.md`, safety/licensing changes, test expectations, or uncertain scope. A skipped PR head can leave required GitHub checks **Pending**, blocking merge; never place a skip marker on the final commit requiring checks. A workflow manually dispatched after a skip does not retrospectively validate a different SHA. Do not use path filters to bypass required checks. See GitHub's [skip-run documentation](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/skip-workflow-runs) and [required-check behavior](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks).

## One-PR delivery

1. Check `main`, active PR/issues, the actual source/tests and relevant contract. For an assigned task, do not reopen settled authorization.
2. Implement the smallest **complete** behavior that advances the goal. Preserve unrelated work; do not introduce placeholder APIs or speculative frameworks.
3. Test representative behavior and important failure modes. Review the final effective diff for root-cause fixes, redundant layers, stale comments and contract drift.
4. Submit a focused PR: **result, rationale/contract, relevant verification (including tested SHA), limitations and next dependency**. Link the original Issue instead of repeating a task history.
5. Merge **only when authorized**, after required checks, current-head review and dependency checks. Verify accepted integration and follow-up CI where relevant; keep `main` stable. Follow [parallel-session coordination](parallel-development.md) only when actually needed.

Do not change another repository, publish releases or redistribute third-party Java artifacts without the appropriate explicit authorization. Status claims must separate implementation from proposals, experiments and unrun verification; see [current status](project-status.md) and [licensing](licensing-and-provenance.md).
