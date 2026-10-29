# ADR-0003: Original guest fixtures for public CI, permission-cleared private corpus for compatibility

- **Date:** 2026-10-29
- **Status:** Accepted; execution infrastructure added incrementally

## Context

The VM needs reproducible tests of class parsing, instruction semantics,
checkpoint/restore and eventually Java ME API integration. A public Rust CI
must be runnable by outside contributors without possessing old proprietary
Java ME SDKs or redistributing commercial game JARs. The curated, privately
held J2ME Dev Kit includes API JARs and compilers, but its archives are not
cleared for redistribution and also include implementation material excluded
by this project's clean-room restriction.

## Decision

1. **Public CI:** Rust unit/integration tests and small, original Java bytecode
   fixtures authored for this project. Check in only reviewed, licensed source
   and necessary binary outputs, with build recipe, version, SHA-256 and
   normative/test provenance. Do not install historic proprietary SDKs or
   fetch MIDlet/game archives in PR CI.
2. **Staged test inputs:** raw class bytes and `.class` files at G1, executable
   authored classes at G2, JARs with loading, cross-process snapshot workloads
   at G3, and synthesized MIDlets/optional JSR probes only when supported.
3. **Local SDK use:** the Library J2ME Dev Kit can support **private, permitted
   fixture compilation**, but its API/runtime JARs, vendor binaries, sample
   programs, reference implementation and TCK must not be committed or used
   as implementation-design source. Its self-test is not a VM test.
4. **Compatibility corpus:** commercial JARs and permission-cleared third-party
   MIDlets belong outside the public repo, in a rights-reviewed and
   access-controlled environment. They are optional empirical tests, not
   normative implementation oracles.
5. **CI growth:** preserve the single lightweight Rust gate; add only proven
   distinct test needs. No external guest download or private corpus in normal
   public CI.

## Rationale and alternatives

- **Commit proprietary games:** rejected for redistribution and provenance
  risks, even when copies are publicly archived.
- **Download from Archive.org or mirrors per CI run:** rejected for rights,
  provenance, reproducibility and external availability risks.
- **Use SDK/JSR JARs as runtime test subjects:** rejected as a confused test
  oracle; those artifacts are principally compile/API references and may carry
  proprietary implementations. Test *our* classes against independently
  implemented contracts instead.
- **Generate everything during CI with a legacy SDK:** rejected initially
  because publicly redistributable tooling and legal access are not established.
  If appropriate open tooling later supports deterministic regeneration, it may
  be added based on evidence.

## Consequences

The initial codebase contains **no placeholder guest fixture or additional CI
job**. Fixture provenance becomes mandatory when the first executable guest
test is admitted. Real-world compatibility remains a separate, rights-reviewed
activity and cannot be used to claim conformance. The owning detailed policy
is [testing.md](../testing.md).
