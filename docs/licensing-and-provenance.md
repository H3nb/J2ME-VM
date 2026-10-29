# Licensing, clean-room provenance, and release policy

## Project license

New, original code and authored documentation in this repository are published under **GNU General Public License version 3.0 only** (`GPL-3.0-only`), not `GPL-3.0-or-later`. The authoritative full text is [`LICENSE`](../LICENSE). The Cargo manifest and source SPDX identifiers must agree. The license choice does not alter third-party rights in externally sourced assets or docs.

A future change to **or-later**, dual licensing, contributor requirements, or integration licensing is a separate reviewed decision. Do not silently choose the more permissive version range. Project contributions must carry sufficient rights to be distributed under GPL-3.0-only; preserve contributor copyright and upstream notices.

## Legal distinctions that must not be collapsed

- **Copyright clean-room**: independently authoring our implementation and avoiding protected source helps prevent unauthorized copying; this is not an automatic legal warranty.
- **Specification implementation**: official JSR materials can impose separate conditions for implementing the specification, potentially including namespace and TCK-related conditions. In particular, the [JSR 139 maintenance page](https://www.jcp.org/aboutJava/communityprocess/mrel/jsr139/index.html) distinguishes evaluation access from implementation, and the [JSR 118 final page](https://www.jcp.org/aboutJava/communityprocess/final/jsr118/) refers to a license. **Do not assume a downloadable specification is a permissive open-source license.** Seek qualified legal review of the exact terms before publication/claims.
- **Historical JVMS terms**: the Oracle-hosted [JVMS Second Edition copyright notice](https://docs.oracle.com/javase/specs/jvms/se6/html/Copyright.doc.html), checked 2026-10-29, also contains a conditional implementation grant. Its conditions concern completeness, required Java 2 SE functionality, namespace restrictions, testing and independent authorship. Public access and clean-room authorship alone do not establish that this experimental CLDC subset meets that grant. Record applicable rights or another lawful basis before relying on it for further implementation or distribution; this observation is not a legal conclusion that the project is permitted or prohibited. [Issue #30](https://github.com/H3nb/J2ME-VM/issues/30) tracks this review; the JSR 139 final-text and terms questions remain separate.
- **Java branding/certification**: no claim of Oracle endorsement, certification, Java trademark license, or successful TCK completion. Describe the project factually as *experimental Java ME bytecode VM research*.
- **External code/assets**: do not bundle the proprietary Java class libraries, JSR reference implementations, TCK suites, sample programs lacking redistribution rights, games, or copyrighted manuals. Link to official docs rather than bundling files.
- **GPL integration**: linking a GPL-3.0-only VM into a redistributed app can trigger distribution obligations for the combined work depending on how integrated. This requires a separate compatibility/legal review with the target JL-Mod Plus distribution; a distinct repository does not by itself remove those obligations.

This is engineering policy, not legal advice or a conclusion that all JSR implementation conditions have been met.

## Clean-room evidence

Each substantive VM behavior should be traceable to an **original** implementation rationale and a version-specific primary normative clause (see [specification-map.md](specification-map.md)). Maintain original fixture provenance: who created it, original source or generator/compiler version, permitted license, and what behavior it exercises. For fixture source, prefer small program text authored for this project. Binary fixtures should be reproducibly generated, or otherwise carry documented permission and hashes.

Do **not** inspect, compare against, translate, adapt, or rename source/code from any third-party VM/emulator (including their decompilations, pull requests, reverse-engineered internals, or copied tests). Do not use such code through an AI retrieval intermediary. The names and signatures explicitly mandated by the Java specification are exempt as **interface constraints**, not as an implementation source.

If a contributor has encountered prohibited material: stop using it, document the exposure privately with maintainers, and route affected work through a newly derived specification-first design by someone not relying on the exposed implementation. Mere rewriting or renaming is insufficient. Do not add the prohibited material to issue trackers, PRs, or commit history.

## Guest test assets and permitted observations

A guest test program written specifically for this project is distinct from a
third-party VM implementation. Authored Java classes can enter the public tests
when their source and any compiler-generated class/JAR bytes have documented
rights and provenance. An official SDK API JAR is ordinarily only a local
compile reference: do not place its classes into published fixtures. The
Library `/J2ME Dev Kit/` bundle is not redistributable by default and its
runtime implementation source must not guide this clean-room VM.

Commercial JARs, vendor sample code, JSR reference implementations and TCKs
belong outside the repository unless explicit rights are independently verified.
Permission to store or run an item in private/hosted CI is not implied by its
availability on Archive.org. For the maintained public/private corpus contract,
see [testing.md](testing.md) and
[ADR-0003](adr/0003-public-fixtures-private-compatibility-corpus.md).

## Rust dependency admission

For every new direct dependency:

1. Show why standard library or existing crates cannot solve the identified problem adequately.
2. Verify current maintenance/security, Rust toolchain requirements, and license expressions for **all transitive dependencies** (including features/build tools where relevant).
3. Prefer proven crates under **MIT, Apache-2.0, or GPL-3.0-compatible** terms with appropriate notices. **GPL-2.0-only is not GPLv3-compatible.** A crate's title or a single SPDX substring is not a sufficient check.
4. Record nontrivial third-party attributions and preserve notices when redistributing. Lock dependencies with `Cargo.lock`, and scan/audit on updates when a tool is available.
5. Do not import other VM/emulator implementation crates as a way around the clean-room restriction, even when technically license-compatible.

**Initial dependency inventory:** none (Rust standard library only). No third-party attribution file is needed until a dependency or asset is admitted.

## Release gate

Before first public code release or compatibility marketing, verify the source license and notices; specification/JSR implementation and trademark terms; binary fixture provenance; dependency obligations; packaging of corresponding source; and integration implications. Record open legal issues as blockers rather than declaring them resolved from technical tests.
