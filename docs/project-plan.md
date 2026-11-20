# Compatibility roadmap

This document owns milestone order, dependencies and release evidence.
[Project status](project-status.md) records implemented behavior.
[GitHub milestones](https://github.com/H3nb/Kilox/milestones) own assignments and
live checklists. [Validation gates](validation-plan.md) retain the detailed
execution and recovery experiments from the original work packages.

## Target and scope

Build a clean-room Rust core that runs Java ME MIDlets with the advertised
configuration and profile. Target CLDC 1.0/1.1 and MIDP 1.0/2.0/2.1 with
version-aware behavior. Track CLDC 1.1.1 maintenance differences explicitly.
Address optional JSRs and vendor extensions by observed impact on real
applications. Keep MIDP 3.0 as a later target and preserve the profile boundaries
needed to add it. [CLDC JSR 139](https://jcp.org/en/jsr/detail?id=139) ·
[MIDP JSR 118](https://jcp.org/en/jsr/detail?id=118) ·
[MIDP 2.1 changes](https://jcp.org/aboutJava/communityprocess/maintenance/jsr118/JSR_118_MR2_Changelog.pdf) ·
[Profile evolution](profile-evolution.md)

The long-term target is **99.99% end-to-end compatibility for an explicitly
defined MIDlet population**. Record platform and device profile for every
result. Standard profile behavior, optional APIs, vendor behavior and external
services have distinct requirements and evidence. The target makes no
certification or distribution claim.

## Development order

| Milestone | Acceptance outcome | Checklist |
| --- | --- | --- |
| [M0 · Reproducible baseline](https://github.com/H3nb/Kilox/milestone/1) | Review the candidate, resolve the applicable rights and document a second-machine handoff and repeatable corpus baseline. | [#43](https://github.com/H3nb/Kilox/issues/43) |
| [M1 · CLDC execution and libraries](https://github.com/H3nb/Kilox/milestone/2) | Verify class loading and legacy StackMap, complete core execution, and bind the CLDC APIs exercised by the corpus. | [#44](https://github.com/H3nb/Kilox/issues/44) |
| [M2 · Standard MIDP and desktop host](https://github.com/H3nb/Kilox/milestone/3) | Prove an interactive standard MIDlet on Windows and Linux with lifecycle, LCDUI, RMS, GCF, media and recovery. | [#45](https://github.com/H3nb/Kilox/issues/45) |
| [M3 · Optional standard Java ME APIs](https://github.com/H3nb/Kilox/milestone/4) | Add the named JSR 135 MMAPI and JSR 184 M3G tracks; expand other optional APIs based on measured compatibility impact and controlled host services. | [#46](https://github.com/H3nb/Kilox/issues/46), [#55](https://github.com/H3nb/Kilox/issues/55), [#56](https://github.com/H3nb/Kilox/issues/56) |
| [M4 · Device and vendor compatibility](https://github.com/H3nb/Kilox/milestone/5) | Add Mascot Capsule Micro3D v3 and other measured vendor/handset behavior behind versioned device profiles. | [#47](https://github.com/H3nb/Kilox/issues/47), [#57](https://github.com/H3nb/Kilox/issues/57) |
| [M5 · Mobile hosts and portability](https://github.com/H3nb/Kilox/milestone/6) | Use the same core on Android and iOS and verify lifecycle, input, effects and recovery on devices. | [#48](https://github.com/H3nb/Kilox/issues/48) |
| [M6 · Measured compatibility release](https://github.com/H3nb/Kilox/milestone/7) | Measure a named release candidate against the defined population and report per-platform evidence and limitations. | [#49](https://github.com/H3nb/Kilox/issues/49) |

Prioritize a proven blocker shared by many distinct MIDlets, then the smallest
semantically complete change that removes it. Resolve shared loader, verifier,
scheduler or host invariants before layering APIs over them. Use original
regressions to confirm the cause. Compare results against the same frozen
corpus before choosing the next task. Preserve rights and prerequisites as
constraints on each work item.

M0 is the immediate priority. While the implementation-use review recorded in
[licensing and provenance](licensing-and-provenance.md) is unresolved, permitted
work includes source and rights evidence, original fixtures, corpus design and
review of local experiments. Review runtime changes in dependency order before any
approved publication. JSR 135, JSR 184 and Mascot Capsule v3 are explicitly planned for M3/M4; their edition coverage and implementation sequence still depend on prerequisite runtime capability and observed tests. Other M3/M4 features follow measured corpus blockers.
M5 can start once M2 exercises the host contract. M6 evaluates a named
candidate and its supported host matrix.

## Shared Rust core and host interfaces

| Layer | Responsibility |
| --- | --- |
| Core | Package and class admission, verification, Java execution, object and class state, virtual scheduling, deterministic resource budgets, and bounded versioned checkpoint encoding. |
| CLDC and MIDP runtime | Guest classes, suite identity and lifecycle, logical display and input, resource semantics, permissions, guest-visible I/O and recoverable references. |
| Host | Package import, surfaces and presentation, keyboard/touch mapping, audio output, persistent storage, transport and device services under declared capabilities. OS objects stay in the host. |

Start with a headless runner and a minimal Windows/Linux frontend to validate
the boundary. Pass input and time through explicit events. Exchange bounded
frame and audio outputs and explicit storage or network requests. Specify
cancellation, failures, permission decisions and rebind behavior before an
adapter relies on a host resource. Keep logical guest coordinates and
timestamps independent of display scaling and host wall time. Reuse the same
core for Android and iOS, then verify input, suspend/resume, and process-death
recovery on physical devices. Choose FFI ownership and bindings when a host
requires them.

VM recovery includes the program identity, frames, heap, classes, threads and
virtual devices within the documented snapshot domain. Test restore in a
different process before claiming recoverability. Treat durable RMS and
external effects as host contracts with explicit commit/rebind policies.
Track memory, frame pacing, input latency and snapshot overhead with named
workloads and hardware once a runnable pilot exists. Keep optimized execution,
rewind and new render backends behind measured needs.

Technical iOS reachability and distribution approval have separate
acceptance criteria. Review the intended distribution route against Apple's
rules on executable content and emulators before committing to a store release.
[Architecture](architecture.md) · [Apple App Review Guidelines, §§2.5.2 and 4.7](https://developer.apple.com/app-store/review/guidelines/)

## Compatibility measurement

1. **Define the population before the run.** Publish the corpus revision, target
   profiles, acquisition and eligibility rules, duplicate policy and scenario
   inventory. Stratify by profile, use case, vendor/device family and key API
   dependencies. Record license and permitted test environment privately where
   needed. Group sibling ports and builds whose code gives correlated results.
2. **Count an application pass at the scenario boundary.** For each included
   version, exercise installation or selection, startup, meaningful input and
   usable output, lifecycle and clean exit. Exercise persistence, media,
   networking and other required behaviors when the scenario uses them.
   Report fresh-process recovery and performance separately with an explicit
   test scope. A package that merely reaches `startApp` has passed a launch
   stage only.
3. **Keep the denominator visible.** Report pass, fail, and undetermined counts,
   and the reason for any predeclared exclusion. Keep unavailable servers,
   hardware, permission or unsupported dependencies visible. For a verified
   corpus score use passes divided by eligible cases, with undetermined cases
   receiving no pass credit. Report coverage by platform and by stratum
   alongside the aggregate. Do not combine reruns or duplicate builds to inflate
   the denominator.
4. **Separate corpus score and inference.** A fixed corpus reports the observed
   fraction only. A wider population claim needs a representative sampling
   design, independent evidence or a method accounting for clusters, and a
   one-sided lower confidence bound of at least 99.99%. For independent
   representative Bernoulli trials with zero failures, a one-sided exact 95%
   lower bound is `0.05^(1/n)`. At least **29,956** such trials are needed to
   reach 99.99%. A small convenience or private development corpus cannot
   substantiate that claim. [NIST exact binomial confidence limits](https://itl.nist.gov/div898/software/dataplot/refman2/auxillar/exacbino.htm)
5. **Make regressions reproducible.** Record commit, toolchain, OS/device,
   guest profile, corpus version, inputs, budgets, actual outcome, and first
   causal failure. Use permission-cleared original fixtures in public CI and a
   separate held-out corpus for release validation. Keep protected binaries,
   data and paths out of GitHub. [Testing and corpus policy](testing.md)

A milestone closes after the linked checklist has an accepted, reproducible
result and recorded limits. Open follow-up work for remaining gaps in its
owning milestone or defer with the reason recorded in GitHub.
