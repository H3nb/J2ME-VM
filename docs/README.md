# Documentation map

This index selects an **owning document** rather than introducing duplicate policy. Start with [`AGENTS.md`](../AGENTS.md), then read only the relevant contract.

| Question / task | Owning document |
| --- | --- |
| Purpose, non-goals, status, getting started | [`README.md`](../README.md) |
| VM/host ownership, execution model, snapshot semantics and open design questions | [`architecture.md`](architecture.md) |
| Milestones, evidence, stop criteria | [`validation-plan.md`](validation-plan.md) |
| Java ME / JVMS editions, source selection, clause-level traceability | [`specification-map.md`](specification-map.md) |
| GPLv3, clean-room provenance, third-party dependency review, release cautions | [`licensing-and-provenance.md`](licensing-and-provenance.md) |
| Test selection, guest fixture provenance, public CI and private compatibility corpus | [`testing.md`](testing.md) and [ADR-0003](adr/0003-public-fixtures-private-compatibility-corpus.md) |
| Commands, CI, reproducibility, pre-commit checklist | [`development.md`](development.md) |
| Consequential accepted decisions and alternatives | [`adr/`](adr/) |

Documents differentiate **requirements**, **accepted decisions**, **hypotheses**, and **observed evidence**. Experiments do not silently promote a proposal to a contract. Only claim capabilities demonstrated by tests at the relevant gate.
