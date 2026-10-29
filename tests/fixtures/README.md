# G1 original class-file fixture

- Class: `compiled/MinimalClass.class`, 171 bytes, version **48.0** (Java 1.4).
- Source: `java/MinimalClass.java`, independently authored for this project; **GPL-3.0-only**.
- Compiler: Eclipse Compiler for Java **3.26.0**, executed using Java 21. Compiler binary is **not redistributed**.
- Rebuild: `java -jar ecj-3.26.0.jar -source 1.4 -target 1.4 -g:none -d tests/fixtures/compiled tests/fixtures/java/MinimalClass.java`.
- SHA-256: `2b86770edc7dcb28515c361627b73030873ed2291c7123671b6d1bf461a73a49`.
- Preverification: **none**; G1 proves structural reading, not CLDC `StackMap` verification.
- Expected observation: `answer()I` Code contains `bipush 42; ireturn`; not executed.
- Contract: JVMS Second Edition §§4.1, 4.4, 4.6, 4.7, plus CLDC 1.1 legacy class-file format range.

The class is built entirely from our Java source, not copied from an SDK or emulator. Do not regenerate this binary on CI; review binary, checksum and source changes together.

## WP02-W2 source fixtures (Issue #6; not yet CLDC bytecode fixtures)

Both new Java files are independently authored for this project and licensed
`GPL-3.0-only`. They use only language primitives; no SDK, library, JAR, emulator
source, or proprietary assets were copied. Each intentionally exposes one public
static `evaluate(int)` method; a Java compiler also generates an unused default
constructor. The expected values below follow the Java `int` and control-flow
rules, **not** the current Rust execution implementation.

| Source | Method and independently derived results | Coverage intended for a later WP02 slice |
| --- | --- | --- |
| `java/LocalArithmetic.java` | `evaluate(x) = 2*x + 2` modulo 2^32, interpreted as signed `int`; inputs `5 -> 12`, `-3 -> -4`, `2147483647 -> 0` | Integer parameter/local loads and stores, addition, subtraction, return, overflow |
| `java/BranchLoop.java` | `evaluate(-2) -> 0`, `evaluate(0) -> 0`, `evaluate(2) -> 1`, `evaluate(3) -> 0`, `evaluate(5) -> 7` | Zero iterations, signed comparison, both conditional paths, loop back-edge, local increment, return |

The branch loop starts `counter` at zero, adds each value except that `2`
subtracts one instead, and stops when `counter` reaches `iterations`. For
nonpositive inputs it returns zero. For nonnegative `iterations`, `counter`
never overflows before the terminating comparison. These are guest-observable
method results, **not guaranteed compiler instruction sequences**.

**Normative trace:** [JVMS Second Edition §3.6](https://docs.oracle.com/javase/specs/jvms/se6/html/Overview.doc.html)
(method frames and locals); [Second Edition instruction descriptions](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc6.html)
(`iload`, `istore`, `iinc`, `if_icmp*`, `iadd`, `isub`, `ireturn`), and
[branch instructions](https://docs.oracle.com/javase/specs/jvms/se6/html/Instructions2.doc5.html).
These establish signed comparisons, local-variable operations, and 32-bit
integer wraparound. CLDC 1.1/1.1.1 class admission and legacy `StackMap`
requirements are **unresolved** pending W1 (Issue #5).

### Compilation status and reproducibility

A local **diagnostic-only** compile succeeded using OpenJDK `javac 21.0.12.1`:

```sh
mkdir -p /tmp/wp02-w2-diagnostic
javac --release 8 -g:none -d /tmp/wp02-w2-diagnostic \
  tests/fixtures/java/LocalArithmetic.java \
  tests/fixtures/java/BranchLoop.java
javap -classpath /tmp/wp02-w2-diagnostic -v -c LocalArithmetic BranchLoop
sha256sum /tmp/wp02-w2-diagnostic/*.class
```

This produced **52.0** classes, which are **not accepted by the G1 reader's
45.x–48.0 input range**. The loop class also contains a modern `StackMapTable`,
**not** a CLDC legacy `StackMap`. These 52.0 binaries are deliberately **not
committed** and do not enter `manifest.json`. Their diagnostic-only SHA-256
values for the stated `javac` command are:

- `LocalArithmetic.class`: `ed0ea44992b3f9c3495dbb7ca905e837012a6757c0b20ed248b40e8a90fb3980`
- `BranchLoop.class`: `a55cc8e30b558c2b3f794112c9f40eed6bb58fdb1dff06853312bd9236883444`

For a **48.0 candidate** using the compiler already documented for
`MinimalClass.java`, the intended command is **not yet executed**:

```sh
java -jar ecj-3.26.0.jar -source 1.4 -target 1.4 -g:none \
  -d tests/fixtures/compiled \
  tests/fixtures/java/LocalArithmetic.java \
  tests/fixtures/java/BranchLoop.java
```

That compiler is unavailable in this session. Before admitting any generated
48.0 binaries, separately inspect actual method code/class version, establish
binary distribution rights, record new SHA-256 values in the manifest, and
resolve the W1 CLDC preverification/`StackMap` acceptance boundary. **No new
fixture was executed by `j2me-vm` or verified for CLDC.**

### Verified Java 1.4 (48.0) compiler output for W2 sources

The coordinator independently built the *already committed, unchanged* W2
Java sources with **ECJ 3.26.0** from the private J2ME Dev Kit (on OpenJDK
21.0.12.1). Source files were checked byte-for-byte against their GitHub Git
blob SHA before compilation. Neither ECJ, the SDK bundle, nor any Java ME API
implementation is included in the repository.

```sh
java -jar ecj-3.26.0.jar -source 1.4 -target 1.4 -g:none \
  -d tests/fixtures/compiled \
  tests/fixtures/java/LocalArithmetic.java \
  tests/fixtures/java/BranchLoop.java
javap -classpath tests/fixtures/compiled -v -c LocalArithmetic BranchLoop
sha256sum tests/fixtures/compiled/{LocalArithmetic,BranchLoop}.class
```

| Binary | Version | Size | SHA-256 |
| --- | --- | ---: | --- |
| `compiled/LocalArithmetic.class` | 48.0 | 189 bytes | `01595ac71eb2b433c650548aef6e199b2416cf0961d876cb88847f1ceef78e30` |
| `compiled/BranchLoop.class` | 48.0 | 201 bytes | `75492454cf4457f8b700b4874ea9c63235da345dfeb9c79364ad89f9b9abe827` |

The binaries contain only classes generated from the project's GPLv3-only
sources; they are suitable as **G1 structural parser fixtures** and candidate
inputs for future explicitly bounded G2 experiments. **Not CLDC-preverified:**
neither method's control-flow safety nor legacy `StackMap` admission is proved.
The existing experimental `IntProgram` does not support these `evaluate(I)I`
methods and is **not** claimed to execute them.

The previous `javac --release 8` hashes above remain diagnostic-only and
must not be mistaken for these committed 48.0 binaries.
