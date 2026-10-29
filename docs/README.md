# Documentation map

This index selects an **owning document** rather than introducing duplicate policy. Start with [`AGENTS.md`](../AGENTS.md), then read only the relevant contract.

| Question / task | Owning document |
| --- | --- |
| Purpose, non-goals, status, getting started | [`README.md`](../README.md) |
| Mission, requirements, staged commitment, work packages, risks and change control | [`project-plan.md`](project-plan.md) |
| Current baseline, open work, evidence pointers and next action | [`project-status.md`](project-status.md) |
| MIDP version differences, evidence limits and evolution experiments | [`profile-evolution.md`](profile-evolution.md) and [ADR-0005](adr/0005-legacy-midp-first-with-profile-aware-contexts.md) |
| VM/host ownership, execution model, snapshot semantics and open design questions | [`architecture.md`](architecture.md) |
| Milestones, evidence, stop criteria | [`validation-plan.md`](validation-plan.md) |
| Java ME / JVMS editions, source selection, clause-level traceability | [`specification-map.md`](specification-map.md) |
| GPLv3, clean-room provenance, third-party dependency review, release cautions | [`licensing-and-provenance.md`](licensing-and-provenance.md) |
| Test selection, guest fixture provenance, public CI and private compatibility corpus | [`testing.md`](testing.md) and [ADR-0003](adr/0003-public-fixtures-private-compatibility-corpus.md) |
| Commands, CI, reproducibility, pre-commit checklist | [`development.md`](development.md) |
| Consequential accepted decisions and alternatives | [`adr/`](adr/) |

Documents differentiate **requirements**, **accepted decisions**, **hypotheses**, and **observed evidence**. Experiments do not silently promote a proposal to a contract. Only claim capabilities demonstrated by tests at the relevant gate.

When resuming work, read project status and verify current remote refs before
selecting a package. The project plan is the development baseline proposal;
it does not supersede existing license, clean-room or specification contracts.
