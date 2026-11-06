// SPDX-License-Identifier: GPL-3.0-only
use std::collections::{btree_map::Entry, BTreeMap};

use super::ArchiveError;

pub(super) const MANIFEST_PATH: &str = "META-INF/MANIFEST.MF";
pub(super) const MAX_MANIFEST_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestError {
    Archive(ArchiveError),
    InvalidAttribute,
    DuplicateAttribute,
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Archive(error) => error.fmt(f),
            Self::InvalidAttribute => f.write_str("invalid manifest main attribute"),
            Self::DuplicateAttribute => f.write_str("duplicate manifest main attribute"),
        }
    }
}

impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Archive(error) => Some(error),
            _ => None,
        }
    }
}

/// UTF-8 attributes from the first section of a package manifest.
#[derive(Debug, PartialEq, Eq)]
pub struct ManifestMain {
    attributes: BTreeMap<String, String>,
}

impl ManifestMain {
    /// Look up an attribute by its ASCII case-insensitive name.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.attributes
            .get(&name.to_ascii_lowercase())
            .map(String::as_str)
    }

    pub(super) fn parse(mut bytes: &[u8]) -> Result<Self, ManifestError> {
        let mut headers = BTreeMap::new();
        let mut current: Option<&mut Vec<u8>> = None;
        while !bytes.is_empty() {
            let line = take_line(&mut bytes).ok_or(ManifestError::InvalidAttribute)?;
            if line.is_empty() {
                break;
            }
            if let Some(part) = line.strip_prefix(b" ") {
                current
                    .as_deref_mut()
                    .ok_or(ManifestError::InvalidAttribute)?
                    .extend_from_slice(part);
                continue;
            }
            let colon = line
                .iter()
                .position(|&byte| byte == b':')
                .ok_or(ManifestError::InvalidAttribute)?;
            let (name, suffix) = line.split_at(colon);
            if !name.first().is_some_and(u8::is_ascii_alphanumeric)
                || !name
                    .iter()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
            {
                return Err(ManifestError::InvalidAttribute);
            }
            let value = suffix
                .strip_prefix(b": ")
                .ok_or(ManifestError::InvalidAttribute)?;
            let name: String = name
                .iter()
                .map(|byte| char::from(byte.to_ascii_lowercase()))
                .collect();
            current = match headers.entry(name) {
                Entry::Vacant(entry) => Some(entry.insert(value.to_vec())),
                Entry::Occupied(_) => return Err(ManifestError::DuplicateAttribute),
            };
        }

        let attributes = headers
            .into_iter()
            .map(|(name, value)| {
                if value.contains(&0) {
                    return Err(ManifestError::InvalidAttribute);
                }
                let value =
                    String::from_utf8(value).map_err(|_| ManifestError::InvalidAttribute)?;
                Ok((name, value))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { attributes })
    }
}

fn take_line<'a>(bytes: &mut &'a [u8]) -> Option<&'a [u8]> {
    let end = bytes
        .iter()
        .position(|byte| matches!(byte, b'\r' | b'\n'))?;
    let (line, tail) = bytes.split_at(end);
    let terminator_len = if tail.starts_with(b"\r\n") { 2 } else { 1 };
    *bytes = &tail[terminator_len..];
    Some(line)
}
