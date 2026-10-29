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
