# W7 preparation audit and method lookup correction

**Scope:** [audit #24](https://github.com/H3nb/J2ME-VM/issues/24),
[fix #28](https://github.com/H3nb/J2ME-VM/issues/28); baseline
`37e32cd179223378fa6f271fd48fc9b33622a394`, 2026-10-29.
This is a host-input resource correction, not a new Java instruction or a G2 pass.

## Confirmed finding

W7 scanned every declared method for every decoded `invokestatic`, including
repeated references. The 64-prepared-method limit did not bound those scans:
unused declarations still participated in every lookup. An original 48.0 class
with 40,000 declarations, 64 reachable methods and 1,023 calls per reachable
method caused 2,618,880,000 candidate visits. Its 1,731,493 bytes and 4,094-byte
reachable methods fit the existing limits, and preparation **succeeded**. Guest
execution was not needed to trigger the cost. The independent audit classified
this synchronous preparation amplification as P2.

The correction builds one temporary `BTreeMap` keyed by borrowed method-name and
descriptor bytes. Duplicate keys retain an ambiguous outcome; missing entry and
missing callee diagnostics remain distinct. Only selected/reachable duplicates
fail admission, preserving the existing proof's exclusion of whole-class
duplicate validation. The index is discarded after preparation and adds no
serialized or guest state. The map has at most one entry per declared signature;
the parser's `u16` method count bounds it. Lookup no longer multiplies each call
by the entire declaration table. There is no dependency or persistent cache.

The Java behavior remains the [bounded WP02 contract](wp02-execution-contract.md).
Relevant historical interfaces are JVMS Second Edition §§4.3.3, 4.6 and 5.4.3.3
([class format](https://docs.oracle.com/javase/specs/jvms/se6/html/ClassFile.doc.html),
[resolution](https://docs.oracle.com/javase/specs/jvms/se6/html/ConstantPool.doc.html)).
The lookup structure and resource policy are original project choices, not Java
requirements. This does not establish full verification or CLDC resolution timing.

## Reproduction and observed cost

[`examples/method_lookup_probe.rs`](../examples/method_lookup_probe.rs) generates
both raw classes independently from the JVMS Second Edition chapter 4 binary
layout. It contains no external library class, compiler output or third-party VM
material. Source and generated fixture bytes are GPL-3.0-only. Run from the repo
root; an optional **existing** output directory preserves bytes for checksumming:

```text
cargo run --locked --release --example method_lookup_probe -- <output-directory>
```

For a baseline comparison, compile this same example in a temporary standalone
Cargo package whose only dependency is `j2me-vm` at the baseline checkout. Do not
replace source in that checkout. No public CI timing assertion is introduced.

Coordinator measurements: Windows x86_64 MSVC, AMD Ryzen 7 7445HS, Rust 1.99.1
(`58cf8f9c3`), Cargo 1.99.1, release builds. Timers cover parsing and preparation
separately, excluding fixture construction, optional file writes and compilation.
These are single-run observations on one host, not a general speed guarantee.

| Input | Bytes | Baseline prepare | Indexed prepare | Outcome in both |
| --- | ---: | ---: | ---: | --- |
| 40,000 methods, one call | 1,468,976 | <1 ms | 13 ms | Accepted |
| 40,000 methods, 64 reachable, 65,472 calls | 1,731,493 | 5,926 ms | 51 ms | Accepted |

Building the index costs time for the single-call control. This tradeoff bounds
repeated lookup work; no claim is made that every small input becomes faster.
Peak memory was not measured. The independent auditor separately observed the
accepted amplification case at 5,973 ms and 5,933 ms on the baseline.

SHA-256 of generated class bytes, identical in the coordinator's baseline and
indexed runs:

- Control: `bcda51aac9e8aaae638f3f8ccfa2d0241abfadf382e548a4a3b4c3eac7be5234`.
- Amplification: `41d49150971aceb1d6b20885e533a0994a8c8a062e8a703de6ec60e182d0a2ee`.

## Validation and remaining boundaries

Coordinator local format, Clippy with `-D warnings` and `cargo test --locked`
passed with 30 tests. The added observable test protects duplicate entry/callee
ambiguity, its call-site error PC, and unaffected unused duplicate declarations;
existing tests exercise overloads, missing/unsupported targets and frame limits.
The independent auditor reviewed the source/test correction separately from its
author and found no confirmed defect. Final commit and CI evidence belong to the
PR and canonical issues; local checks alone are not a CI result.

A separate original scratch probe accepted 64 prepared methods and rejected 65;
that boundary was not added as another permanent test. Audit also observed that
legacy single-method `prepare` can select a specially named method in a mutated
class. That predates W7 and remains outside this fix; it is not evidence of full
class verification. Objects, exceptions, general loading, snapshot and CLDC
admission remain unsupported. Technical audit does not clear specification-use
rights under [the licensing contract](licensing-and-provenance.md).
