// SPDX-License-Identifier: GPL-3.0-only
//! Original G1 class fixtures and negative cases; JVMS Second Edition §4.

use j2me_vm::classfile::{parse_class, Constant, ParseErrorKind, MAX_CLASS_BYTES};

const COMPILED: &[u8] = include_bytes!("fixtures/compiled/MinimalClass.class");

fn put2(v: &mut Vec<u8>, n: u16) {
    v.extend(n.to_be_bytes());
}
fn put4(v: &mut Vec<u8>, n: u32) {
    v.extend(n.to_be_bytes());
}
fn utf8(v: &mut Vec<u8>, bytes: &[u8]) {
    v.push(1);
    put2(v, bytes.len() as u16);
    v.extend(bytes);
}

// Spec-derived, project-authored tiny class, independent of a VM implementation.
fn minimal(major: u16, minor: u16) -> Vec<u8> {
    let mut v = Vec::new();
    put4(&mut v, 0xcafe_babe);
    put2(&mut v, minor);
    put2(&mut v, major);
    put2(&mut v, 5); // Utf8, Class, Utf8, Class
    utf8(&mut v, b"Sample");
    v.push(7);
    put2(&mut v, 1);
    utf8(&mut v, b"java/lang/Object");
    v.push(7);
    put2(&mut v, 3);
    put2(&mut v, 0x21);
    put2(&mut v, 2);
    put2(&mut v, 4);
    for _ in 0..4 {
        put2(&mut v, 0);
    } // interfaces/fields/methods/attributes
    v
}

#[test]
fn original_compiler_fixture_contains_method_and_instructions() {
    let c = parse_class(COMPILED).unwrap();
    assert_eq!((c.major_version, c.minor_version), (48, 0));
    assert_eq!(c.class_name_bytes(), Some(&b"MinimalClass"[..]));
    assert_eq!(c.methods.len(), 2);
    let m = c
        .methods
        .iter()
        .find(|m| c.utf8_bytes(m.name_index) == Some(&b"answer"[..]))
        .unwrap();
    assert_eq!(m.code.as_ref().unwrap().instructions, &[0x10, 42, 0xac]);
}

#[test]
fn versions_are_explicit_and_reject_unsupported_binaries() {
    for (major, minor) in [(45, 0), (45, 3), (45, 65535), (46, 0), (47, 0), (48, 0)] {
        assert!(parse_class(&minimal(major, minor)).is_ok());
    }
    for (major, minor) in [(44, 0), (46, 1), (48, 1), (49, 0), (52, 0)] {
        assert!(matches!(
            parse_class(&minimal(major, minor)).unwrap_err().kind,
            ParseErrorKind::UnsupportedVersion { .. }
        ));
    }
}

#[test]
fn every_proper_prefix_of_compiled_fixture_is_invalid() {
    for end in 0..COMPILED.len() {
        assert!(
            parse_class(&COMPILED[..end]).is_err(),
            "accepted {end}-byte prefix"
        );
    }
}

#[test]
fn rejects_bad_magic_trailing_data_and_unknown_pool_tag() {
    let mut v = minimal(48, 0);
    v[0] = 0;
    assert_eq!(parse_class(&v).unwrap_err().kind, ParseErrorKind::BadMagic);
    let mut v = minimal(48, 0);
    v.push(0);
    assert_eq!(
        parse_class(&v).unwrap_err().kind,
        ParseErrorKind::TrailingBytes
    );
    let mut v = minimal(48, 0);
    v[10] = 15;
    assert_eq!(
        parse_class(&v).unwrap_err().kind,
        ParseErrorKind::UnknownConstantTag(15)
    );
}

#[test]
fn validates_constant_pool_types_and_modified_utf8() {
    let mut v = minimal(48, 0);
    v[13] = 0;
    assert_eq!(
        parse_class(&v).unwrap_err().kind,
        ParseErrorKind::InvalidModifiedUtf8
    );
    let mut v = minimal(48, 0);
    let header = v.len() - 14;
    v[header + 2..header + 4].copy_from_slice(&1u16.to_be_bytes());
    assert!(matches!(
        parse_class(&v).unwrap_err().kind,
        ParseErrorKind::InvalidReference { .. }
    ));
    let mut v = minimal(48, 0);
    let first_class_offset = 13 + b"Sample".len();
    v[first_class_offset + 1..first_class_offset + 3].copy_from_slice(&99u16.to_be_bytes());
    let error = parse_class(&v).unwrap_err();
    assert!(matches!(
        error.kind,
        ParseErrorKind::InvalidReference { index: 99, .. }
    ));
    assert_eq!(error.offset, first_class_offset + 1); // CP Class.name_index
}

#[test]
fn long_has_two_constant_pool_slots() {
    let mut v = minimal(45, 3);
    let cp_end = v.len() - 14;
    let mut constant = vec![5];
    constant.extend_from_slice(&0x0102_0304_0506_0708u64.to_be_bytes());
    v.splice(cp_end..cp_end, constant);
    v[8..10].copy_from_slice(&7u16.to_be_bytes());
    let c = parse_class(&v).unwrap();
    assert!(matches!(
        c.constant_pool[5],
        Constant::Long(0x0102_0304_0506_0708)
    ));
    assert_eq!(c.constant_pool[6], Constant::Unusable);
    v[8..10].copy_from_slice(&6u16.to_be_bytes());
    assert_eq!(
        parse_class(&v).unwrap_err().kind,
        ParseErrorKind::InvalidConstantPoolCount
    );
}

#[test]
fn rejects_oversize_and_corrupt_code_length() {
    assert_eq!(
        parse_class(&vec![0; MAX_CLASS_BYTES + 1]).unwrap_err().kind,
        ParseErrorKind::InputTooLarge
    );
    let class = parse_class(COMPILED).unwrap();
    let attr = class
        .methods
        .iter()
        .flat_map(|m| &m.attributes)
        .find(|a| class.utf8_bytes(a.name_index) == Some(&b"Code"[..]))
        .unwrap();
    let mut v = COMPILED.to_vec();
    v[attr.info_offset + 4..attr.info_offset + 8].copy_from_slice(&u32::MAX.to_be_bytes());
    assert_eq!(
        parse_class(&v).unwrap_err().kind,
        ParseErrorKind::InvalidCodeLength
    );
}

#[test]
fn code_attribute_cardinality_and_method_flags_follow_jvms() {
    let class = parse_class(COMPILED).unwrap();
    let answer = class
        .methods
        .iter()
        .find(|m| class.utf8_bytes(m.name_index) == Some(&b"answer"[..]))
        .unwrap();
    assert_eq!(answer.attributes.len(), 1);
    let attr = &answer.attributes[0];
    let start = attr.info_offset - 6; // attribute header: name (u2), length (u4)
    let end = attr.info_offset + attr.info.len();
    let count_at = start - 2; // method attributes_count directly precedes first attribute
    let flags_at = count_at - 6; // access_flags, name_index, descriptor_index

    // A second well-formed Code body must not silently replace or shadow the first.
    let mut duplicate = COMPILED.to_vec();
    duplicate.splice(end..end, COMPILED[start..end].iter().copied());
    duplicate[count_at..count_at + 2].copy_from_slice(&2u16.to_be_bytes());
    assert_eq!(
        parse_class(&duplicate).unwrap_err().kind,
        ParseErrorKind::InvalidCodeAttribute("duplicate Code attribute")
    );

    // A concrete method without Code has an invalid method_info structure.
    let mut missing = COMPILED.to_vec();
    missing.drain(start..end);
    missing[count_at..count_at + 2].copy_from_slice(&0u16.to_be_bytes());
    assert_eq!(
        parse_class(&missing).unwrap_err().kind,
        ParseErrorKind::InvalidCodeAttribute("concrete method missing Code")
    );

    for forbidden_flag in [0x0100u16, 0x0400u16] {
        let mut flagged = COMPILED.to_vec();
        let mut flags = u16::from_be_bytes([flagged[flags_at], flagged[flags_at + 1]]);
        flags |= forbidden_flag;
        flagged[flags_at..flags_at + 2].copy_from_slice(&flags.to_be_bytes());
        assert_eq!(
            parse_class(&flagged).unwrap_err().kind,
            ParseErrorKind::InvalidCodeAttribute("Code attribute on native/abstract method")
        );
    }
}

#[test]
fn original_wp02_source_fixtures_parse_as_java_1_4_class_files() {
    for (bytes, name) in [
        (
            &include_bytes!("fixtures/compiled/LocalArithmetic.class")[..],
            &b"LocalArithmetic"[..],
        ),
        (
            &include_bytes!("fixtures/compiled/BranchLoop.class")[..],
            &b"BranchLoop"[..],
        ),
    ] {
        let class = parse_class(bytes).expect("original ECJ-produced Java 1.4 class");
        assert_eq!((class.major_version, class.minor_version), (48, 0));
        assert_eq!(class.class_name_bytes(), Some(name));
        let method = class
            .methods
            .iter()
            .find(|m| class.utf8_bytes(m.name_index) == Some(&b"evaluate"[..]))
            .expect("authored Java method exists");
        assert_eq!(class.utf8_bytes(method.descriptor_index), Some(&b"(I)I"[..]));
        assert!(method.code.as_ref().is_some_and(|code| !code.instructions.is_empty()));
    }
}
