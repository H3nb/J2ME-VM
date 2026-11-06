// SPDX-License-Identifier: GPL-3.0-only
use j2me_vm::archive::{ArchiveError, PackageArchive};
use std::borrow::Cow;

const PACKAGE: &[u8] = include_bytes!("fixtures/package-archive.zip");
const CLASS: &[u8] = include_bytes!("fixtures/compiled/MinimalClass.class");

#[test]
fn reads_stored_class_and_deflated_resource_from_streamed_package() {
    let archive = PackageArchive::open(PACKAGE).unwrap();
    assert_eq!(
        archive.names().collect::<Vec<_>>(),
        ["MinimalClass.class", "data/readme.txt", "data/second.txt"]
    );
    assert!(matches!(
        archive.read("MinimalClass.class"),
        Ok(Some(Cow::Borrowed(bytes))) if bytes == CLASS
    ));
    let text = b"Original package resource.\n".repeat(12);
    assert_eq!(archive.read("data/readme.txt"), Ok(Some(Cow::Owned(text))));
    assert_eq!(archive.read("data/"), Ok(None));
    assert_eq!(archive.read("data/README.txt"), Ok(None));
}

#[test]
fn rejects_local_name_mismatch_duplicates_and_noncanonical_names() {
    let name = b"data/second.txt";
    let [local, central] = name_offsets(name);
    let mut changed = PACKAGE.to_vec();
    changed[local] = b'X';
    assert_eq!(
        PackageArchive::open(&changed).unwrap_err(),
        ArchiveError::Malformed
    );
    for (replacement, expected) in [
        (b"data/readme.txt", ArchiveError::DuplicateName),
        (b"../unsafe/entry", ArchiveError::InvalidName),
    ] {
        let mut changed = PACKAGE.to_vec();
        for offset in [local, central] {
            changed[offset..offset + name.len()].copy_from_slice(replacement);
        }
        assert_eq!(PackageArchive::open(&changed).unwrap_err(), expected);
    }
}

#[test]
fn rejects_conflicting_descriptor_and_bounds_output_before_inflate() {
    let [_, central] = name_offsets(b"data/readme.txt");
    let descriptor = PACKAGE
        .windows(4)
        .enumerate()
        .filter(|(_, window)| *window == b"PK\x07\x08")
        .nth(2)
        .unwrap()
        .0;
    let mut changed = PACKAGE.to_vec();
    // Central uncompressed length is 22 bytes before the variable filename.
    changed[central - 22..central - 18].copy_from_slice(&1u32.to_le_bytes());
    assert_eq!(
        PackageArchive::open(&changed).unwrap_err(),
        ArchiveError::Malformed
    );
    changed[descriptor + 12..descriptor + 16].copy_from_slice(&1u32.to_le_bytes());
    let archive = PackageArchive::open(&changed).unwrap();
    assert_eq!(
        archive.read("data/readme.txt"),
        Err(ArchiveError::CorruptEntry)
    );
    changed[central - 22..central - 18].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(
        PackageArchive::open(&changed).unwrap_err(),
        ArchiveError::LimitExceeded
    );
}

#[test]
fn rejects_bad_payload_crc_and_inexact_archive_boundary() {
    let [local, _] = name_offsets(b"MinimalClass.class");
    let mut changed = PACKAGE.to_vec();
    changed[local + b"MinimalClass.class".len()] ^= 1;
    let archive = PackageArchive::open(&changed).unwrap();
    assert_eq!(
        archive.read("MinimalClass.class"),
        Err(ArchiveError::CorruptEntry)
    );

    changed.push(0);
    assert_eq!(
        PackageArchive::open(&changed).unwrap_err(),
        ArchiveError::Malformed
    );
}

fn name_offsets(name: &[u8]) -> [usize; 2] {
    PACKAGE
        .windows(name.len())
        .enumerate()
        .filter_map(|(offset, bytes)| (bytes == name).then_some(offset))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
