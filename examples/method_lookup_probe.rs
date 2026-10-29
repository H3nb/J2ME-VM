// SPDX-License-Identifier: GPL-3.0-only
// Original raw class-file experiment for issue #28; JVMS Second Edition chapter 4.
// Generates only project-authored bytecode, without a Java compiler or API classes.
use j2me_vm::classfile::parse_class;
use j2me_vm::execution::IntProgram;
use std::time::Instant;
fn put_u16(o: &mut Vec<u8>, v: u16) {
    o.extend_from_slice(&v.to_be_bytes())
}
fn put_u32(o: &mut Vec<u8>, v: u32) {
    o.extend_from_slice(&v.to_be_bytes())
}
fn utf8_constant(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    let mut e = vec![1];
    put_u16(&mut e, b.len() as u16);
    e.extend_from_slice(b);
    e
}
fn class_constant(n: u16) -> Vec<u8> {
    let mut e = vec![7];
    put_u16(&mut e, n);
    e
}
fn name_and_type(n: u16, d: u16) -> Vec<u8> {
    let mut e = vec![12];
    put_u16(&mut e, n);
    put_u16(&mut e, d);
    e
}
fn method_reference(c: u16, n: u16) -> Vec<u8> {
    let mut e = vec![10];
    put_u16(&mut e, c);
    put_u16(&mut e, n);
    e
}
fn push_constant(cp: &mut Vec<Vec<u8>>, e: Vec<u8>) -> u16 {
    cp.push(e);
    cp.len() as u16
}
fn write_method(
    o: &mut Vec<u8>,
    name: u16,
    desc: u16,
    code_name: u16,
    code: &[u8],
    max_stack: u16,
    max_locals: u16,
) {
    put_u16(o, 9);
    put_u16(o, name);
    put_u16(o, desc);
    put_u16(o, 1);
    put_u16(o, code_name);
    put_u32(o, 12 + code.len() as u32);
    put_u16(o, max_stack);
    put_u16(o, max_locals);
    put_u32(o, code.len() as u32);
    o.extend_from_slice(code);
    put_u16(o, 0);
    put_u16(o, 0)
}
fn class_prefix(cp: &[Vec<u8>], count: usize) -> Vec<u8> {
    let mut o = vec![];
    put_u32(&mut o, 0xcafebabe);
    put_u16(&mut o, 0);
    put_u16(&mut o, 48);
    put_u16(&mut o, (cp.len() + 1) as u16);
    for e in cp {
        o.extend_from_slice(e)
    }
    put_u16(&mut o, 0x21);
    put_u16(&mut o, 2);
    put_u16(&mut o, 4);
    put_u16(&mut o, 0);
    put_u16(&mut o, 0);
    put_u16(&mut o, count as u16);
    o
}
fn finish_class(mut o: Vec<u8>) -> Vec<u8> {
    put_u16(&mut o, 0);
    o
}
fn single_call_fixture(count: usize) -> Vec<u8> {
    let mut cp = vec![];
    let own = push_constant(&mut cp, utf8_constant("AuditClass"));
    let owner = push_constant(&mut cp, class_constant(own));
    let sup = push_constant(&mut cp, utf8_constant("java/lang/Object"));
    push_constant(&mut cp, class_constant(sup));
    let desc = push_constant(&mut cp, utf8_constant("()I"));
    let target = push_constant(&mut cp, utf8_constant("target"));
    let n = push_constant(&mut cp, name_and_type(target, desc));
    let r = push_constant(&mut cp, method_reference(owner, n));
    let code = push_constant(&mut cp, utf8_constant("Code"));
    let entry = push_constant(&mut cp, utf8_constant("entry"));
    let fillers: Vec<u16> = (0..count - 2)
        .map(|i| push_constant(&mut cp, utf8_constant(&format!("f{i}"))))
        .collect();
    let mut o = class_prefix(&cp, count);
    write_method(&mut o, target, desc, code, &[0x04, 0xac], 1, 0);
    write_method(
        &mut o,
        entry,
        desc,
        code,
        &[0xb8, (r >> 8) as u8, r as u8, 0xac],
        1,
        0,
    );
    for name in fillers {
        write_method(&mut o, name, desc, code, &[0x04, 0xac], 1, 0)
    }
    finish_class(o)
}
fn many_calls_fixture(count: usize, reachable: usize, calls: usize) -> Vec<u8> {
    let mut cp = vec![];
    let own = push_constant(&mut cp, utf8_constant("AuditClass"));
    push_constant(&mut cp, class_constant(own));
    let sup = push_constant(&mut cp, utf8_constant("java/lang/Object"));
    push_constant(&mut cp, class_constant(sup));
    let desc = push_constant(&mut cp, utf8_constant("()I"));
    let code_name = push_constant(&mut cp, utf8_constant("Code"));
    let names: Vec<u16> = (0..count)
        .map(|i| push_constant(&mut cp, utf8_constant(&format!("m{i}"))))
        .collect();
    let refs: Vec<u16> = (0..reachable)
        .map(|i| {
            let n = push_constant(&mut cp, name_and_type(names[i], desc));
            push_constant(&mut cp, method_reference(2, n))
        })
        .collect();
    let mut o = class_prefix(&cp, count);
    for (i, name) in names.iter().take(reachable).enumerate() {
        let mut code = Vec::with_capacity(calls * 4 + 2);
        for call in 0..calls {
            let target = if call == 0 && i + 1 < reachable {
                i + 1
            } else {
                reachable - 1
            };
            let r = refs[target];
            code.extend_from_slice(&[0xb8, (r >> 8) as u8, r as u8, 0x3b])
        }
        code.extend_from_slice(&[0x04, 0xac]);
        write_method(&mut o, *name, desc, code_name, &code, 1, 1)
    }
    for name in names.into_iter().skip(reachable) {
        write_method(&mut o, name, desc, code_name, &[0x04, 0xac], 1, 0)
    }
    finish_class(o)
}
fn run_case(label: &str, bytes: Vec<u8>, entry: &[u8]) {
    if let Some(directory) = std::env::args_os().nth(1) {
        let path = std::path::PathBuf::from(directory).join(format!("{label}.class"));
        std::fs::write(path, &bytes).expect("write authored fixture to existing output directory");
    }
    let t = Instant::now();
    let c = parse_class(&bytes).expect("synthetic 48.0 class parses");
    let parse_ms = t.elapsed().as_millis();
    let t = Instant::now();
    IntProgram::prepare_with_calls(&c, entry, b"()I")
        .expect("authored fixture is admitted by the bounded execution proof");
    let prep_ms = t.elapsed().as_millis();
    println!(
        "{label}: methods={} bytes={} parse_ms={parse_ms} prepare_ms={prep_ms} outcome=accepted",
        c.methods.len(),
        bytes.len()
    )
}
fn main() {
    run_case(
        "single-invoke-control",
        single_call_fixture(40_000),
        b"entry",
    );
    run_case(
        "accepted-amplification",
        many_calls_fixture(40_000, 64, 1_023),
        b"m0",
    )
}
