// SPDX-License-Identifier: GPL-3.0-only
//! WP02: compiler-authored Java 1.4 class -> checked subset -> VM-owned frame.

use j2me_vm::classfile::parse_class;
use j2me_vm::execution::{IntMachine, IntProgram, PrepareErrorKind, Step};

const CLASS: &[u8] = include_bytes!("fixtures/compiled/MinimalClass.class");

#[test]
fn original_class_is_executed_only_after_subset_validation() {
    let class = parse_class(CLASS).unwrap();
    let program = IntProgram::prepare(&class, b"answer").unwrap();
    assert_eq!(program.code_len(), 3);
    let mut vm = IntMachine::new(program);
    assert_eq!(vm.pc(), 0);
    assert_eq!(vm.run(0), None);
    assert_eq!(vm.step(), Step::Advanced { next_pc: 2 });
    assert_eq!(vm.operand_stack(), &[42]);
    assert_eq!(vm.step(), Step::Returned(42));
    assert_eq!(vm.pc(), 3);
    assert_eq!(vm.step(), Step::AlreadyReturned(42));
}

#[test]
fn other_methods_are_not_assumed_executable() {
    let class = parse_class(CLASS).unwrap();
    assert_eq!(
        IntProgram::prepare(&class, b"<init>").unwrap_err().kind,
        PrepareErrorKind::UnsupportedMethod("signature must be ()I")
    );
    assert_eq!(
        IntProgram::prepare(&class, b"notPresent").unwrap_err().kind,
        PrepareErrorKind::MethodNotFound
    );
}
