// SPDX-License-Identifier: GPL-3.0-only
//! Bounded, in-memory access to the ZIP container of an application package.

use miniz_oxide::inflate::core::{
    decompress, inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF, DecompressorOxide,
};
use miniz_oxide::inflate::TINFLStatus;
use rawzip::{
    extra_fields::{ExtraFieldId, ExtraFields},
    CompressionMethod, ZipArchive,
};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::ops::Range;

const MAX_ARCHIVE_BYTES: usize = 16 * 1024 * 1024;
const MAX_ENTRY_BYTES: usize = 16 * 1024 * 1024;
const MAX_EXPANDED_BYTES: usize = 64 * 1024 * 1024;
const MAX_ENTRIES: usize = 4096;
const MAX_NAME_BYTES: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveError {
    Malformed,
    UnsupportedFormat,
    InvalidName,
    DuplicateName,
    LimitExceeded,
    AllocationFailed,
    CorruptEntry,
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Malformed => "malformed ZIP container",
            Self::UnsupportedFormat => "unsupported ZIP container feature",
            Self::InvalidName => "invalid ZIP entry name",
            Self::DuplicateName => "duplicate ZIP entry name",
            Self::LimitExceeded => "ZIP container exceeds the package limits",
            Self::AllocationFailed => "could not allocate an entry buffer",
            Self::CorruptEntry => "ZIP entry content does not match its metadata",
        };
        f.write_str(message)
    }
}

impl std::error::Error for ArchiveError {}

#[derive(Debug)]
struct Entry {
    data: Range<usize>,
    expanded_size: usize,
    method: CompressionMethod,
    crc: u32,
}

/// A borrowed classic ZIP package with case-sensitive UTF-8 names.
///
/// Stored and raw DEFLATE files are admitted. Limits apply to the input,
/// each entry, total declared expanded size, name length and entry count.
/// Structural validation happens during indexing. Content is checked on read.
#[derive(Debug)]
pub struct PackageArchive<'a> {
    bytes: &'a [u8],
    entries: BTreeMap<String, Entry>,
}

impl<'a> PackageArchive<'a> {
    pub fn open(bytes: &'a [u8]) -> Result<Self, ArchiveError> {
        if bytes.len() > MAX_ARCHIVE_BYTES {
            return Err(ArchiveError::LimitExceeded);
        }
        let zip = ZipArchive::from_slice(bytes).map_err(|_| ArchiveError::Malformed)?;
        let eocd = zip.eocd_offset() as usize;
        let (directory_start, count) = directory_bounds(bytes, eocd)?;
        if zip.directory_offset() != directory_start as u64 || zip.entries_hint() != count as u64 {
            return Err(ArchiveError::Malformed);
        }

        let mut entries = BTreeMap::new();
        let mut ranges = Vec::with_capacity(count);
        let mut expanded_total = 0usize;
        let mut central_end = directory_start;
        for header in zip.entries() {
            if ranges.len() == count {
                return Err(ArchiveError::Malformed);
            }
            let header = header.map_err(|_| ArchiveError::Malformed)?;
            let central_offset = header.central_directory_offset() as usize;
            central_end = [28, 30, 32]
                .into_iter()
                .try_fold(central_offset + 46, |end, field| {
                    read_u16(bytes, central_offset + field).map(|len| end + usize::from(len))
                })
                .ok_or(ArchiveError::Malformed)?;
            if read_u16(bytes, central_offset + 34) != Some(0) {
                return Err(ArchiveError::UnsupportedFormat);
            }
            validate_extra_fields(header.extra_fields())?;
            let flags = header.flags();
            let method = header.compression_method();
            if !matches!(
                method,
                CompressionMethod::STORE | CompressionMethod::DEFLATE
            ) {
                return Err(ArchiveError::UnsupportedFormat);
            }
            // Permit data descriptors, the UTF-8 flag and DEFLATE option bits.
            let allowed_flags = if method == CompressionMethod::DEFLATE {
                0x080e
            } else {
                0x0808
            };
            if flags.bits() & !allowed_flags != 0 {
                return Err(ArchiveError::UnsupportedFormat);
            }
            let path = header.file_path();
            let name = entry_name(path.as_bytes())?;
            let expanded_size = usize::try_from(header.uncompressed_size_hint())
                .map_err(|_| ArchiveError::LimitExceeded)?;
            expanded_total = expanded_total
                .checked_add(expanded_size)
                .filter(|total| *total <= MAX_EXPANDED_BYTES)
                .ok_or(ArchiveError::LimitExceeded)?;
            if expanded_size > MAX_ENTRY_BYTES {
                return Err(ArchiveError::LimitExceeded);
            }
            if name.ends_with('/') && expanded_size != 0 {
                return Err(ArchiveError::Malformed);
            }

            let slice = zip
                .get_entry(header.wayfinder())
                .map_err(|_| ArchiveError::Malformed)?;
            let (data_start, data_end) = slice.compressed_data_range();
            if data_end > directory_start as u64 {
                return Err(ArchiveError::Malformed);
            }
            let data = data_start as usize..data_end as usize;
            let crc = header.crc32();
            let local = slice.local_header();
            if local.file_path().as_bytes() != path.as_bytes()
                || local.flags() != flags
                || local.compression_method() != method
                || (method == CompressionMethod::STORE && data.len() != expanded_size)
            {
                return Err(ArchiveError::Malformed);
            }
            validate_extra_fields(local.extra_fields())?;
            let end = if flags.has_data_descriptor() {
                if (local.crc32() != 0 && local.crc32() != crc)
                    || (local.compressed_size_hint() != 0
                        && local.compressed_size_hint() != header.compressed_size_hint())
                    || (local.uncompressed_size_hint() != 0
                        && local.uncompressed_size_hint() != header.uncompressed_size_hint())
                {
                    return Err(ArchiveError::Malformed);
                }
                let expected = [crc, data.len() as u32, expanded_size as u32];
                descriptor_end(bytes, data.end, expected).ok_or(ArchiveError::Malformed)?
            } else {
                if local.crc32() != crc
                    || local.compressed_size_hint() != header.compressed_size_hint()
                    || local.uncompressed_size_hint() != header.uncompressed_size_hint()
                {
                    return Err(ArchiveError::Malformed);
                }
                data.end
            };
            ranges.push(header.local_header_offset() as usize..end);
            if entries
                .insert(
                    name.to_owned(),
                    Entry {
                        data,
                        expanded_size,
                        method,
                        crc,
                    },
                )
                .is_some()
            {
                return Err(ArchiveError::DuplicateName);
            }
        }

        ranges.sort_unstable_by_key(|range| range.start);
        let covered_end = ranges.iter().try_fold(0, |offset, range| {
            (range.start == offset).then_some(range.end)
        });
        if ranges.len() != count || central_end != eocd || covered_end != Some(directory_start) {
            return Err(ArchiveError::Malformed);
        }
        Ok(Self { bytes, entries })
    }

    /// File names in bytewise order. Directory markers have no readable payload.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.entries
            .keys()
            .map(String::as_str)
            .filter(|name| !name.ends_with('/'))
    }

    /// Reads exactly one file and checks its length and CRC before returning it.
    /// Stored entries borrow the input. DEFLATE entries own a bounded buffer.
    pub fn read(&self, name: &str) -> Result<Option<Cow<'a, [u8]>>, ArchiveError> {
        let Some(entry) = self.entries.get(name).filter(|_| !name.ends_with('/')) else {
            return Ok(None);
        };
        let input = &self.bytes[entry.data.clone()];
        let data = if entry.method == CompressionMethod::STORE {
            Cow::Borrowed(input)
        } else {
            Cow::Owned(inflate(input, entry.expanded_size)?)
        };
        if rawzip::crc32(&data) != entry.crc {
            return Err(ArchiveError::CorruptEntry);
        }
        Ok(Some(data))
    }
}

// The accepted package has a single classic directory at the end of the input.
fn directory_bounds(bytes: &[u8], eocd: usize) -> Result<(usize, usize), ArchiveError> {
    let malformed = ArchiveError::Malformed;
    let on_disk = read_u16(bytes, eocd + 8).ok_or(malformed)?;
    let total = read_u16(bytes, eocd + 10).ok_or(malformed)?;
    let size = read_u32(bytes, eocd + 12).ok_or(malformed)?;
    let offset = read_u32(bytes, eocd + 16).ok_or(malformed)?;
    let comment_size = read_u16(bytes, eocd + 20).ok_or(malformed)?;
    if read_u32(bytes, eocd + 4) != Some(0)
        || on_disk != total
        || total == u16::MAX
        || size == u32::MAX
        || offset == u32::MAX
    {
        return Err(ArchiveError::UnsupportedFormat);
    }
    if offset.checked_add(size).map(|end| end as usize) != Some(eocd)
        || eocd + 22 + usize::from(comment_size) != bytes.len()
    {
        return Err(malformed);
    }
    let count = usize::from(total);
    if count > MAX_ENTRIES {
        return Err(ArchiveError::LimitExceeded);
    }
    Ok((offset as usize, count))
}

fn entry_name(path: &[u8]) -> Result<&str, ArchiveError> {
    if path.len() > MAX_NAME_BYTES {
        return Err(ArchiveError::LimitExceeded);
    }
    let name = std::str::from_utf8(path).map_err(|_| ArchiveError::InvalidName)?;
    let components = name.strip_suffix('/').unwrap_or(name).split('/');
    if name.contains(['\\', '\0', ':'])
        || components
            .into_iter()
            .any(|part| matches!(part, "" | "." | ".."))
    {
        return Err(ArchiveError::InvalidName);
    }
    Ok(name)
}

fn validate_extra_fields(mut fields: ExtraFields<'_>) -> Result<(), ArchiveError> {
    if fields.any(|(id, _)| id == ExtraFieldId::ZIP64) {
        return Err(ArchiveError::UnsupportedFormat);
    }
    if !fields.remaining_bytes().is_empty() {
        return Err(ArchiveError::Malformed);
    }
    Ok(())
}

fn descriptor_end(bytes: &[u8], start: usize, expected: [u32; 3]) -> Option<usize> {
    let matches = |body| {
        expected
            .iter()
            .enumerate()
            .all(|(index, word)| read_u32(bytes, body + index * 4) == Some(*word))
    };
    if read_u32(bytes, start) == Some(0x08074b50) && matches(start + 4) {
        Some(start + 16)
    } else {
        matches(start).then_some(start + 12)
    }
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_le_bytes(
        bytes.get(offset..offset + 2)?.try_into().ok()?,
    ))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

fn inflate(input: &[u8], size: usize) -> Result<Vec<u8>, ArchiveError> {
    let mut output = Vec::new();
    // A spare byte lets the inflater finish an end marker after exact-size output.
    output
        .try_reserve_exact(size + 1)
        .map_err(|_| ArchiveError::AllocationFailed)?;
    output.resize(size + 1, 0);
    let (status, consumed, written) = decompress(
        &mut DecompressorOxide::new(),
        input,
        &mut output,
        0,
        TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
    );
    if status != TINFLStatus::Done || consumed != input.len() || written != size {
        return Err(ArchiveError::CorruptEntry);
    }
    output.truncate(size);
    Ok(output)
}
