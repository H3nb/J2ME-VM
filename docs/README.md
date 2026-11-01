# Documentation map and authority

Start with [AGENTS.md](../AGENTS.md), the relevant source/tests and [current status](project-status.md). Read **only the documents that own the task's contracts**; do not load the entire planning/research archive before every implementation.

| Need | Owning source |
| --- | --- |
| Project mission, actual features and basic build | [Root README](../README.md) |
| Current integrated capabilities, major blockers and next step | [Project status](project-status.md); verify against live GitHub |
| Requirements, work packages, dependency order and scope changes | [Project plan](project-plan.md) |
| Accepted VM/host boundaries and architecture hypotheses | [Architecture](architecture.md) and accepted [ADRs](adr/) |
| Version-specific Java ME/JVMS source evidence and unresolved gaps | [Specification map](specification-map.md) |
| License, clean-room boundaries, permitted source use | [Licensing and provenance](licensing-and-provenance.md) |
| Tests, original fixtures, public/private corpus rules | [Testing](testing.md) |
| Build, CI, PR and integration checks | [Development](development.md) |
| Risk-proportional implementation/review quality | [Engineering quality](engineering-quality.md) |
| Autonomous task continuity and optional multi-agent assignments | [Parallel development](parallel-development.md) |
| Experiment gates and acceptance conditions | [Validation plan](validation-plan.md) |
| MIDP-version differences and future evolution | [Profile evolution](profile-evolution.md); [ADR-0005](adr/0005-legacy-midp-first-with-profile-aware-contexts.md) |

## Feature-specific contracts and research

| Scope | Evidence / document | Authority |
| --- | --- | --- |
| Existing bounded integer/static-call behavior | [WP02 execution contract](wp02-execution-contract.md) | Actual **partial** implementation contract; compare with source |
| Method lookup optimization | [WP02 evidence](wp02-method-lookup-evidence.md) | Experiment-specific evidence |
| Reference/exception prerequisites | [WP02 reference/exception research](wp02-reference-exception-research.md) | Research, **not** accepted runtime capability |
| Virtual concurrency, monitors and restore | [WP05 concurrency contract](wp05-concurrency-contract.md) | **Proposal**, not implemented or a passed gate |
| Fixture provenance and bounded tests | [Fixture README](../tests/fixtures/README.md) and related fixture notes | Test evidence, not Java certification |

**Authority distinction:** a requirement says what is needed; an ADR records an accepted consequential decision; a proposal describes a possible design; a successful experiment provides bounded evidence; only integrated implementation plus relevant verification supports a capability claim. Do not infer that all proposals are approved simply because they are in `docs/`.

Keep existing paths stable for historical links. When consolidating, edit the owning document rather than duplicating rules or creating new workflow registries. GitHub Issues, PRs, commits and CI already own task history.
