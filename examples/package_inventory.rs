// SPDX-License-Identifier: GPL-3.0-only
//! Report structural package evidence without printing guest names or contents.

use j2me_vm::archive::{PackageArchive, MAX_ARCHIVE_BYTES};
use j2me_vm::classfile::parse_class;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::ExitCode;

fn inspect(path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|err| format!("input: {:?}", err.kind()))?;
    let mut bytes = Vec::new();
    file.take(MAX_ARCHIVE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|err| format!("input: {:?}", err.kind()))?;
    let archive = PackageArchive::open(&bytes).map_err(|err| format!("archive: {err}"))?;
    let has_manifest = archive
        .manifest_main()
        .map_err(|err| format!("manifest: {err}"))?
        .is_some();

    let mut class_versions = BTreeMap::new();
    let mut class_count = 0;
    let mut stack_map_methods = 0;
    for name in archive.names().filter(|name| name.ends_with(".class")) {
        class_count += 1;
        let class_bytes = archive
            .read(name)
            .map_err(|err| format!("class entry {class_count}: {err}"))?
            .ok_or_else(|| format!("class entry {class_count}: missing payload"))?;
        let class = parse_class(&class_bytes).map_err(|err| {
            format!(
                "class entry {class_count} at byte {}: {:?}",
                err.offset, err.kind
            )
        })?;
        *class_versions
            .entry((class.major_version, class.minor_version))
            .or_insert(0usize) += 1;
        stack_map_methods += class
            .methods
            .iter()
            .filter_map(|method| method.code.as_ref())
            .filter(|code| {
                code.attributes
                    .iter()
                    .any(|attr| class.utf8_bytes(attr.name_index) == Some(b"StackMap"))
            })
            .count();
    }

    println!("entries={}", archive.names().count());
    println!("manifest={has_manifest}");
    println!("classes={class_count}");
    for ((major, minor), count) in class_versions {
        println!("class_version_{major}_{minor}={count}");
    }
    println!("methods_with_stack_map={stack_map_methods}");
    Ok(())
}

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let path = args.next();
    if let (Some(path), None) = (path, args.next()) {
        return match inspect(Path::new(&path)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::FAILURE
            }
        };
    }
    eprintln!("usage: package_inventory <jar>");
    ExitCode::from(2)
}
