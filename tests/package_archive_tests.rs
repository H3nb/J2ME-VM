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

#[test]
fn rejects_unindexed_bytes_claimed_as_central_directory() {
    let mut changed = PACKAGE.to_vec();
    let eocd = changed.len() - 22;
    let signature_record = b"PK\x05\x05\0\0";
    assert_eq!(&changed[eocd..eocd + 4], b"PK\x05\x06");
    let size_offset = eocd + 12;
    let directory_size =
        u32::from_le_bytes(changed[size_offset..size_offset + 4].try_into().unwrap());
    changed[size_offset..size_offset + 4]
        .copy_from_slice(&(directory_size + signature_record.len() as u32).to_le_bytes());
    changed.splice(eocd..eocd, signature_record.iter().copied());
    assert_eq!(
        PackageArchive::open(&changed).unwrap_err(),
        ArchiveError::Malformed
    );
}

#[test]
fn rejects_truncated_local_and_central_extra_fields() {
    let [local, central] = name_offsets(b"\x0f\xf0\x02\x00ab");
    for offset in [local, central] {
        let mut changed = PACKAGE.to_vec();
        changed[offset + 2] = 3;
        assert_eq!(
            PackageArchive::open(&changed).unwrap_err(),
            ArchiveError::Malformed
        );
    }
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

#[test]
fn reads_folded_utf8_main_attributes_without_named_section_overrides() {
    let manifest = b"Manifest-Version: 1.0\r\nApp-Label: Caf\xc3\r\n \xa9\nEmpty: \r\n\r\nName: assets/logo.png\r\nApp-Label: Other\r\n";
    let jar = package_with_manifest(manifest);
    let archive = PackageArchive::open(&jar).unwrap();
    let attrs = archive.manifest_main().unwrap().unwrap();
    assert_eq!(attrs.get("APP-label"), Some("Caf\u{e9}"));
    assert_eq!(attrs.get("empty"), Some(""));
    assert_eq!(attrs.get("Name"), None);
    assert!(PackageArchive::open(PACKAGE)
        .unwrap()
        .manifest_main()
        .unwrap()
        .is_none());
}

#[test]
fn rejects_ambiguous_and_malformed_main_attributes() {
    use j2me_vm::archive::ManifestError;

    for (input, error) in [
        (
            b"App-Name: First\rapp-name: Second\r".as_slice(),
            ManifestError::DuplicateAttribute,
        ),
        (b" continuation\n", ManifestError::InvalidAttribute),
        (b"App-Name:No space\n", ManifestError::InvalidAttribute),
        (b"App-Name: \xff\n", ManifestError::InvalidAttribute),
        (b"App-Name: A\0B\n", ManifestError::InvalidAttribute),
        (b"App-Name: unterminated", ManifestError::InvalidAttribute),
    ] {
        let jar = package_with_manifest(input);
        let archive = PackageArchive::open(&jar).unwrap();
        assert_eq!(archive.manifest_main(), Err(error));
    }
}

#[test]
fn applies_the_manifest_limit_and_checks_the_whole_entry_crc() {
    use j2me_vm::archive::ManifestError;

    let mut jar = package_with_manifest(&vec![b'x'; 256 * 1024 + 1]);
    let archive = PackageArchive::open(&jar).unwrap();
    assert_eq!(
        archive.manifest_main(),
        Err(ManifestError::Archive(ArchiveError::LimitExceeded))
    );

    let bytes = b"Manifest-Version: 1.0\n\nName: assets/logo.png\n";
    jar = package_with_manifest(bytes);
    let start = jar
        .windows(bytes.len())
        .position(|part| part == bytes)
        .unwrap();
    jar[start + bytes.len() - 2] ^= 1;
    let archive = PackageArchive::open(&jar).unwrap();
    assert_eq!(
        archive.manifest_main(),
        Err(ManifestError::Archive(ArchiveError::CorruptEntry))
    );
}

fn package_with_manifest(bytes: &[u8]) -> Vec<u8> {
    use std::io::Write;

    let mut archive = rawzip::ZipArchiveWriter::new(Vec::new());
    let (entry, config) = archive.new_file("META-INF/MANIFEST.MF").start().unwrap();
    let mut writer = config.wrap(entry);
    writer.write_all(bytes).unwrap();
    let (entry, descriptor) = writer.finish().unwrap();
    entry.finish(descriptor).unwrap();
    archive.finish().unwrap()
}

#[test]
fn accepts_only_zero_alignment_padding_in_stored_local_extras() {
    let manifest = b"Manifest-Version: 1.0\n\n";
    let padded = manifest_with_local_extra_suffix(manifest, &[0, 0]);
    let archive = PackageArchive::open(&padded).unwrap();
    assert_eq!(
        archive.read("META-INF/MANIFEST.MF"),
        Ok(Some(Cow::Borrowed(manifest.as_slice())))
    );

    for suffix in [&[0, 1][..], &[0], &[0, 0, 0]] {
        let invalid = manifest_with_local_extra_suffix(manifest, suffix);
        assert_eq!(
            PackageArchive::open(&invalid).unwrap_err(),
            ArchiveError::Malformed
        );
    }

    let mut central_padded = package_with_manifest(manifest);
    let zip = rawzip::ZipArchive::from_slice(&central_padded).unwrap();
    let eocd = zip.eocd_offset() as usize;
    let directory = zip.directory_offset() as usize;
    let directory_size = eocd - directory;
    central_padded[directory + 30..directory + 32].copy_from_slice(&2u16.to_le_bytes());
    central_padded[eocd + 12..eocd + 16]
        .copy_from_slice(&((directory_size + 2) as u32).to_le_bytes());
    central_padded.splice(eocd..eocd, [0, 0]);
    assert_eq!(
        PackageArchive::open(&central_padded).unwrap_err(),
        ArchiveError::Malformed
    );
}

fn manifest_with_local_extra_suffix(manifest: &[u8], suffix: &[u8]) -> Vec<u8> {
    let mut bytes = package_with_manifest(manifest);
    let zip = rawzip::ZipArchive::from_slice(&bytes).unwrap();
    let eocd_offset = zip.eocd_offset() as usize;
    let directory_offset = zip.directory_offset() as usize;
    let data_offset = 30 + b"META-INF/MANIFEST.MF".len();
    assert_eq!(&bytes[28..30], &[0, 0]);
    assert_eq!(&bytes[data_offset..data_offset + manifest.len()], manifest);
    assert_eq!(data_offset % 4, 2);
    bytes[28..30].copy_from_slice(&(suffix.len() as u16).to_le_bytes());
    bytes[eocd_offset + 16..eocd_offset + 20]
        .copy_from_slice(&((directory_offset + suffix.len()) as u32).to_le_bytes());
    bytes.splice(data_offset..data_offset, suffix.iter().copied());
    bytes
}
