# Development, validation and delivery

This document owns **toolchain commands, validation and delivery mechanics**. Engineering decisions follow [AGENTS.md](../AGENTS.md); test/fixture scope follows [testing](testing.md); quality review follows [engineering quality](engineering-quality.md).

## Local Rust validation

Use maintained stable Rust with `rustfmt` and `clippy`. The toolchain is currently unpinned; record the version for reproducible experiments. `Cargo.lock` pins dependencies, **not** compiler/host. Regenerate it only when deliberately changing resolution.

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Run focused checks during implementation and the applicable checks above on the final candidate. Review links and consistency for docs-only changes. Passing Rust checks do not establish Java ME compatibility.

Add cross-platform checks, fuzzing, benchmarks or license scanners when they protect a demonstrated risk. Review third-party source and transitive licenses before adding dependencies. Public validation must be reproducible without private SDKs, proprietary guest binaries, downloaded commercial games, Java ME RI/TCK or unpublished tools. Original bytecode fixtures require independently written Java source, compile recipe, expected result, version, hashes and usage rights, as detailed in [testing](testing.md).

## One-PR delivery

1. Check `main`, active PR/issues, the actual source/tests and relevant contract. For an assigned task, do not reopen settled authorization.
2. Implement the smallest **complete** behavior that advances the goal. Preserve unrelated work; do not introduce placeholder APIs or speculative frameworks.
3. Test representative behavior and important failure modes. Review the final effective diff for root-cause fixes, redundant layers, stale comments and contract drift.
4. Submit a focused PR: **result, rationale/contract, relevant verification (including tested SHA), limitations and next dependency**. Link the original Issue instead of repeating a task history.
5. Merge **only when authorized**, after required checks, current-head review and dependency checks. Verify accepted integration and follow-up CI where relevant; keep `main` stable. Follow [parallel-session coordination](parallel-development.md) only when actually needed.

Do not change another repository, publish releases or redistribute third-party Java artifacts without the appropriate explicit authorization. Status claims must separate implementation from proposals, experiments and unrun verification; see [current status](project-status.md) and [licensing](licensing-and-provenance.md).
