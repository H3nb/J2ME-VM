# ADR-0002: GPLv3-only license and specification-first clean-room development

- **Date:** 2026-10-29
- **Status:** Accepted for this repository; legal review remains open

## Context

The Rust VM is being developed as an independent implementation from Java ME/JVMS requirements. A clear outbound license, provenance discipline and dependency policy are needed before the first commit. Open-source status alone does not prove a source is clean-room or that specification implementation rights are granted.

## Decision

- License all original project code and documentation under **GNU GPL version 3.0 only** (`GPL-3.0-only`); ship the canonical license text and aligned manifest/source identifiers.
- Derive all VM implementation behavior independently from version-appropriate official specifications and original experiments. Prohibit reviewing/reusing implementation internals of other VMs/emulators or JL-Mod Plus runtime code. Exception: exact external API names/layouts mandated by normative specifications.
- Use third-party Rust crates only after license, security, maintenance, feature and transitive dependency review. Prefer MIT/Apache-2.0/GPL-3.0-compatible dependencies. No dependency currently needed.
- Keep distinct a code license, permissions to implement JSR specifications, TCK/certification access, Java trademarks, and legal consequences of later integration into JL-Mod Plus. Resolve these questions at release gate; do not claim they are settled today.

## Consequences

GPLv3 copyleft and notices apply to distribution and potential later linking; integrations require separate legal/architecture review. The clean-room restriction limits shortcuts and requires traceable original tests and specification clauses. These are deliberate constraints rather than a guarantee of legality or conformance.

## Alternatives

- MIT/Apache-2.0 as outbound project license: rejected by explicit project decision.
- GPL-3.0-or-later: deferred; version flexibility was not explicitly selected.
- Port or adapt an existing VM: rejected under the clean-room rule regardless of source license.
