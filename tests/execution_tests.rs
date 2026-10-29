// SPDX-License-Identifier: GPL-3.0-only
//! WP02: original Java 1.4 class -> checked integer subset -> VM-owned frame.
//! Not CLDC verification: fixtures have no legacy StackMap preverification.

use j2me_vm::classfile::{parse_class, Constant};
use j2me_vm::execution::{
    ExecutionError, IntMachine, IntProgram, PrepareError, PrepareErrorKind, Step,
};

const CLASS: &[u8] = include_bytes!("fixtures/compiled/MinimalClass.class");
const LOCALS: &[u8] = include_bytes!("fixtures/compiled/LocalArithmetic.class");
const LOOP: &[u8] = include_bytes!("fixtures/compiled/BranchLoop.class");
const NESTED: &[u8] = include_bytes!("fixtures/compiled/NestedStaticCalls.class");
const RECURSIVE: &[u8] = include_bytes!("fixtures/compiled/RecursiveStaticCalls.class");

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
        PrepareErrorKind::UnsupportedMethod("signature must be ()I or (I)I")
    );
    assert_eq!(
        IntProgram::prepare(&class, b"notPresent").unwrap_err().kind,
        PrepareErrorKind::MethodNotFound
    );
}

#[test]
fn original_local_arithmetic_vectors_and_independent_frames() {
    let class = parse_class(LOCALS).unwrap();
    let program = IntProgram::prepare(&class, b"evaluate").unwrap();
    assert_eq!(program.code_len(), 15);
    for (input, expected) in [(5, 12), (-3, -4), (i32::MAX, 0), (i32::MIN, 2)] {
        let mut vm = IntMachine::with_int_arg(program.clone(), input);
        assert_eq!(vm.locals(), &[Some(input), None, None]);
        assert_eq!(vm.run(100), Some(expected));
        assert_eq!(vm.result(), Some(expected));
        assert_eq!(vm.step(), Step::AlreadyReturned(expected));
    }

    let mut first = IntMachine::with_int_arg(program.clone(), 5);
    let mut second = IntMachine::with_int_arg(program, -3);
    assert_eq!(first.step(), Step::Advanced { next_pc: 1 });
    assert_eq!(first.operand_stack(), &[5]);
    assert_eq!(second.pc(), 0);
    assert_eq!(second.operand_stack(), &[]);
    assert_eq!(second.locals(), &[Some(-3), None, None]);
    assert_eq!(first.run(100), Some(12));
    assert_eq!(second.run(100), Some(-4));
}

#[test]
fn original_loop_vectors_budget_and_pc_boundaries() {
    let class = parse_class(LOOP).unwrap();
    let program = IntProgram::prepare(&class, b"evaluate").unwrap();
    assert_eq!(program.code_len(), 32);
    for (input, expected) in [(-2, 0), (0, 0), (2, 1), (3, 0), (5, 7)] {
        let mut vm = IntMachine::with_int_arg(program.clone(), input);
        assert_eq!(vm.run(1), None);
        assert_eq!(vm.run(0), None);
        assert_eq!(vm.run(1000), Some(expected));
    }

    let mut vm = IntMachine::with_int_arg(program.clone(), 3);
    assert_eq!(vm.run(4), None); // two initialized integer locals
    assert_eq!(vm.pc(), 4);
    assert_eq!(vm.locals(), &[Some(3), Some(0), Some(0)]);
    assert_eq!(vm.step(), Step::Advanced { next_pc: 25 }); // forward goto
    assert_eq!(vm.run(2), None);
    assert_eq!(vm.pc(), 27);
    assert_eq!(vm.operand_stack(), &[0, 3]);
    assert_eq!(vm.step(), Step::Advanced { next_pc: 7 }); // signed backward if_icmplt
    assert_eq!(vm.operand_stack(), &[]);

    // A single run budget is not a global lifetime cap. The guest can be
    // paused in a backward loop with its next PC and locals intact.
    let mut one = IntMachine::with_int_arg(program.clone(), 5);
    let mut other = IntMachine::with_int_arg(program, 2);
    for _ in 0..10 {
        assert_eq!(one.run(1), None);
    }
    assert_ne!(one.pc(), other.pc());
    assert_eq!(other.run(1000), Some(1));
    assert_eq!(one.run(1000), Some(7));
}

fn changed_loop_code(
    code_bytes: &[u8],
    max_stack: u16,
    max_locals: u16,
) -> Result<IntProgram, PrepareError> {
    let mut class = parse_class(LOOP).unwrap();
    let code = class.methods[1].code.as_mut().unwrap();
    code.instructions = code_bytes;
    code.max_stack = max_stack;
    code.max_locals = max_locals;
    IntProgram::prepare(&class, b"evaluate")
}

#[test]
fn corrupt_branch_offsets_are_rejected_before_machine_creation() {
    let class = parse_class(LOOP).unwrap();
    let code = class.methods[1].code.as_ref().unwrap().instructions;
    // The original goto at pc=4 is +21, targeting pc=25.
    let mut inside_goto_operand = code.to_vec();
    inside_goto_operand[6] = 1; // target pc=5, an operand byte
    assert_eq!(
        changed_loop_code(&inside_goto_operand, 2, 3).unwrap_err(),
        PrepareError {
            pc: 4,
            kind: PrepareErrorKind::InvalidBranchTarget
        }
    );

    // The if_icmpne at pc=9 now jumps inside iinc's operands at pc=13.
    let mut inside_iinc_operand = code.to_vec();
    inside_iinc_operand[11] = 4;
    assert_eq!(
        changed_loop_code(&inside_iinc_operand, 2, 3)
            .unwrap_err()
            .kind,
        PrepareErrorKind::InvalidBranchTarget
    );

    let mut out_of_range = code.to_vec();
    out_of_range[5] = 0x7f;
    out_of_range[6] = 0xff;
    assert_eq!(
        changed_loop_code(&out_of_range, 2, 3).unwrap_err().kind,
        PrepareErrorKind::InvalidBranchTarget
    );
}

#[test]
fn uninitialized_locals_and_incompatible_control_flow_are_rejected() {
    // Method is (I)I: argument occupies local 0, local 1 is unassigned.
    assert_eq!(
        changed_loop_code(&[0x1b, 0xac], 1, 2).unwrap_err().kind,
        PrepareErrorKind::UninitializedLocal
    );
    assert_eq!(
        changed_loop_code(&[0x84, 0x01, 0x01, 0x1a, 0xac], 1, 2)
            .unwrap_err()
            .kind,
        PrepareErrorKind::UninitializedLocal
    );
    assert_eq!(
        changed_loop_code(&[0x1b, 0xac], 1, 1).unwrap_err().kind,
        PrepareErrorKind::InvalidLocal
    );

    // At pc=9, the taken edge carries zero stack entries, but the
    // fallthrough edge carries one; neither may be executed.
    let inconsistent = [
        0x03, 0x03, 0x9f, 0x00, 0x07, 0x04, 0xa7, 0x00, 0x03, 0x03, 0xac,
    ];
    assert_eq!(
        changed_loop_code(&inconsistent, 2, 1).unwrap_err().kind,
        PrepareErrorKind::InconsistentStackJoin
    );

    // One predecessor initializes local 1; the other does not.
    let not_definitely_assigned = [
        0x1a, 0x03, 0x9f, 0x00, 0x08, 0x04, 0x3c, 0xa7, 0x00, 0x03, 0x1b, 0xac,
    ];
    assert_eq!(
        changed_loop_code(&not_definitely_assigned, 2, 2)
            .unwrap_err()
            .kind,
        PrepareErrorKind::UninitializedLocal
    );
    assert_eq!(
        changed_loop_code(&[0xa7, 0], 1, 1).unwrap_err().kind,
        PrepareErrorKind::TruncatedOperand
    );
    assert_eq!(
        changed_loop_code(&[0x03, 0x3c, 0x1b, 0xac], 0, 2)
            .unwrap_err()
            .kind,
        PrepareErrorKind::StackOverflow
    );
}

#[test]
fn iinc_wraps_java_signed_int_instead_of_panicking() {
    // iinc #0, +1; iload_0; ireturn
    let program = changed_loop_code(&[0x84, 0x00, 0x01, 0x1a, 0xac], 1, 1).unwrap();
    let mut vm = IntMachine::with_int_arg(program, i32::MAX);
    assert_eq!(vm.step(), Step::Advanced { next_pc: 3 });
    assert_eq!(vm.locals(), &[Some(i32::MIN)]);
    assert_eq!(vm.run(2), Some(i32::MIN));
}

#[test]
fn all_signed_int_comparisons_take_only_their_defined_edge() {
    // iload_0; iconst_0; if_icmp<cond> +5; iconst_0; ireturn;
    // iconst_1; ireturn. Both paths have valid terminal states.
    for (opcode, truthy, falsy) in [
        (0x9f, 0, -1), // eq
        (0xa0, -1, 0), // ne
        (0xa1, -1, 0), // lt
        (0xa2, 0, -1), // ge
        (0xa3, 1, 0),  // gt
        (0xa4, -1, 1), // le
    ] {
        let bytes = [0x1a, 0x03, opcode, 0x00, 0x05, 0x03, 0xac, 0x04, 0xac];
        let program = changed_loop_code(&bytes, 2, 1).unwrap();
        let mut taken = IntMachine::with_int_arg(program.clone(), truthy);
        assert_eq!(taken.run(2), None);
        assert_eq!(taken.step(), Step::Advanced { next_pc: 7 });
        assert_eq!(taken.run(2), Some(1));

        let mut not_taken = IntMachine::with_int_arg(program, falsy);
        assert_eq!(not_taken.run(2), None);
        assert_eq!(not_taken.step(), Step::Advanced { next_pc: 5 });
        assert_eq!(not_taken.run(2), Some(0));
    }
}

#[test]
fn a_branch_fallthrough_cannot_escape_the_method() {
    // The taken edge loops to pc=0; the other edge would fall beyond code.
    assert_eq!(
        changed_loop_code(&[0x03, 0x03, 0x9f, 0xff, 0xfe], 2, 1)
            .unwrap_err()
            .kind,
        PrepareErrorKind::MissingReturn
    );
}

#[test]
fn large_host_budget_does_not_spin_forever_on_guest_back_edge() {
    // If input == 0, return 1; otherwise execute goto pc=5 forever.
    // The verifier accepts the reachable return while run enforces a cap.
    let bytes = [0x1a, 0x03, 0x9f, 0x00, 0x06, 0xa7, 0x00, 0x00, 0x04, 0xac];
    let program = changed_loop_code(&bytes, 2, 1).unwrap();
    let mut looping = IntMachine::with_int_arg(program.clone(), 1);
    assert_eq!(looping.run(usize::MAX), None);
    assert_eq!(looping.pc(), 5);
    assert_eq!(looping.operand_stack(), &[]);
    assert_eq!(looping.run(1), None);
    assert_eq!(looping.pc(), 5);

    let mut terminating = IntMachine::with_int_arg(program, 0);
    assert_eq!(terminating.run(usize::MAX), Some(1));
}

#[test]
fn original_nested_invocations_select_descriptors_and_preserve_callers() {
    let class = parse_class(NESTED).unwrap();
    let program = IntProgram::prepare_with_calls(&class, b"evaluate", b"(I)I").unwrap();
    for (input, expected) in [
        (i32::MIN, 7),
        (-3, 1),
        (-1, 5),
        (0, 7),
        (4, 15),
        (i32::MAX, 5),
    ] {
        let mut vm = IntMachine::with_int_arg(program.clone(), input);
        assert_eq!(vm.run(200), Some(expected));
        assert_eq!(vm.frames().len(), 0);
        assert_eq!(vm.failure(), None);
    }

    let overload = IntProgram::prepare_with_calls(&class, b"adjust", b"()I").unwrap();
    assert_eq!(IntMachine::new(overload).run(10), Some(4));

    let mut vm = IntMachine::with_int_arg(program, 4);
    assert_eq!(vm.run(0), None);
    assert_eq!(vm.step(), Step::Advanced { next_pc: 1 }); // iload_0
    assert_eq!(vm.frames().len(), 1);
    assert_eq!(vm.step(), Step::Advanced { next_pc: 0 }); // invokestatic adjust(I)I
    assert_eq!(vm.frames().len(), 2);
    assert_eq!(vm.frames()[0].operand_stack(), &[]); // caller argument transferred
    assert_eq!(vm.frames()[1].locals(), &[Some(4)]);
    assert_eq!(vm.frame_pc(0), Some(4)); // caller resumes after the 3-byte invoke
    assert_eq!(vm.run(1), None);
    assert_eq!(vm.frames().len(), 2);
    assert_eq!(vm.run(100), Some(15));
    assert_eq!(vm.step(), Step::AlreadyReturned(15));
}

#[test]
fn recursive_frames_budget_and_call_depth_are_vm_owned() {
    let class = parse_class(RECURSIVE).unwrap();
    let program = IntProgram::prepare_with_calls(&class, b"evaluate", b"(I)I").unwrap();
    for (input, expected) in [(-3, 0), (0, 0), (1, 1), (4, 4), (8, 8)] {
        let mut vm = IntMachine::with_int_arg(program.clone(), input);
        assert_eq!(vm.run(1), None);
        assert_eq!(vm.run(0), None);
        assert_eq!(vm.run(500), Some(expected));
    }
    let mut exact = IntMachine::with_int_arg(program.clone(), 63);
    assert_eq!(exact.run(1000), Some(63)); // maximum 64 simultaneous frames
    let mut too_deep = IntMachine::with_int_arg(program, 64);
    assert_eq!(too_deep.run(1000), None);
    assert_eq!(too_deep.failure(), Some(ExecutionError::CallDepthExceeded));
    assert_eq!(too_deep.frames().len(), 64);
    let pcs: Vec<_> = (0..too_deep.frames().len())
        .map(|i| too_deep.frame_pc(i))
        .collect();
    assert_eq!(
        too_deep.step(),
        Step::AlreadyFailed(ExecutionError::CallDepthExceeded)
    );
    assert_eq!(
        (0..too_deep.frames().len())
            .map(|i| too_deep.frame_pc(i))
            .collect::<Vec<_>>(),
        pcs
    );
}

#[test]
fn executable_class_admission_rejects_invalid_kind_and_class_flags() {
    let mut invalid_class = parse_class(NESTED).unwrap();
    // ACC_FINAL | ACC_ABSTRACT is not a legal ordinary class combination.
    invalid_class.access_flags = 0x0431;
    assert_eq!(
        IntProgram::prepare_with_calls(&invalid_class, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::InvalidClassAccessFlags
    );
    assert_eq!(
        IntProgram::prepare(&invalid_class, b"evaluate")
            .unwrap_err()
            .kind,
        PrepareErrorKind::InvalidClassAccessFlags
    );

    // Even a structurally parseable interface cannot enter this class-only
    // static-invocation proof (class-file 48.0).
    let mut interface = parse_class(NESTED).unwrap();
    interface.access_flags = 0x0601; // ACC_PUBLIC | ACC_INTERFACE | ACC_ABSTRACT
    assert_eq!(
        IntProgram::prepare_with_calls(&interface, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::UnsupportedMethod("interface class unsupported")
    );
}

#[test]
fn method_visibility_exclusion_applies_to_entry_and_reachable_callees() {
    // Reproduce an invalid public|private|static entry from the original
    // W6 48.0 fixture, without copying or committing a mutated class binary.
    let mut entry = parse_class(NESTED).unwrap();
    let method = entry
        .methods
        .iter_mut()
        .find(|m| m.name_index == 10)
        .expect("original evaluate(I)I");
    method.access_flags = 0x000b;
    assert_eq!(
        IntProgram::prepare_with_calls(&entry, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::InvalidMethodAccessFlags
    );
    assert_eq!(
        IntProgram::prepare(&entry, b"evaluate").unwrap_err().kind,
        PrepareErrorKind::InvalidMethodAccessFlags
    );

    // The invalid method need not be the entry: the reachable private static
    // adjust(I)I method with ACC_PROTECTED added must also fail admission.
    let mut callee = parse_class(NESTED).unwrap();
    let method = callee
        .methods
        .iter_mut()
        .find(|m| m.name_index == 14 && m.descriptor_index == 11)
        .expect("original adjust(I)I");
    method.access_flags |= 0x0004;
    assert_eq!(
        IntProgram::prepare_with_calls(&callee, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::InvalidMethodAccessFlags
    );

    // Private static, final, strictfp and reserved flags must not be
    // blanket-rejected by the narrow admission checks.
    let mut allowed = parse_class(NESTED).unwrap();
    allowed.access_flags |= 0x8000;
    let adjust = allowed
        .methods
        .iter_mut()
        .find(|m| m.name_index == 14 && m.descriptor_index == 11)
        .expect("original private static adjust(I)I");
    adjust.access_flags |= 0x8000 | 0x0010 | 0x0800;
    let program = IntProgram::prepare_with_calls(&allowed, b"evaluate", b"(I)I").unwrap();
    assert_eq!(IntMachine::with_int_arg(program, 4).run(200), Some(15));
}

#[test]
fn special_method_entry_is_rejected_before_building_a_machine() {
    // G1 accepts structural class data; it does not perform the complete
    // JVMS class/member-name verification. Do not execute a malformed
    // static int-return method masquerading as a constructor.
    for special_name in [b"<init>".as_slice(), b"<invalid>".as_slice()] {
        let mut class = parse_class(NESTED).unwrap();
        class.constant_pool[10] = Constant::Utf8(special_name);
        assert_eq!(
            IntProgram::prepare_with_calls(&class, special_name, b"(I)I")
                .unwrap_err()
                .kind,
            PrepareErrorKind::UnsupportedMethod("special method entry unsupported")
        );
    }

    // The transitive Methodref target must also be an ordinary method.
    // CP #20 is both increment(I)I's method name and its Methodref name.
    let mut invalid_callee = parse_class(NESTED).unwrap();
    invalid_callee.constant_pool[20] = Constant::Utf8(b"<invalid>");
    assert_eq!(
        IntProgram::prepare_with_calls(&invalid_callee, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::UnsupportedMethod("special method invocation unsupported")
    );

    // A user-supplied special name must never select a normal method.
    let class = parse_class(NESTED).unwrap();
    assert_eq!(
        IntProgram::prepare_with_calls(&class, b"<init>", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::UnsupportedMethod("special method entry unsupported")
    );
}

#[test]
fn malformed_reachable_callee_cfg_fails_before_execution() {
    let mut class = parse_class(NESTED).unwrap();
    // The entry calls adjust(I)I, which calls increment(I)I. A goto target
    // inside its own operand is invalid even though the entry Code is valid.
    let invalid_code = [0xa7, 0x00, 0x01, 0x03, 0xac];
    let callee = class
        .methods
        .iter_mut()
        .find(|method| method.name_index == 20 && method.descriptor_index == 11)
        .expect("increment(I)I in original W6 fixture");
    callee.code.as_mut().expect("Code").instructions = &invalid_code;
    assert_eq!(
        IntProgram::prepare_with_calls(&class, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::InvalidBranchTarget
    );
}

#[test]
fn invalid_method_references_and_callee_stack_are_rejected_during_preparation() {
    let mut invalid_zero = parse_class(NESTED).unwrap();
    let invalid_code = [0x1a, 0xb8, 0x00, 0x00, 0xac];
    let member = invalid_zero
        .methods
        .iter_mut()
        .find(|m| m.name_index == 10)
        .expect("entry method");
    member.code.as_mut().expect("Code").instructions = &invalid_code;
    assert_eq!(
        IntProgram::prepare_with_calls(&invalid_zero, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::InvalidMethodReference
    );

    let mut invalid_kind = parse_class(NESTED).unwrap();
    invalid_kind.constant_pool[12] = Constant::Integer(0);
    assert_eq!(
        IntProgram::prepare_with_calls(&invalid_kind, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::InvalidMethodReference
    );

    let mut external = parse_class(NESTED).unwrap();
    external.constant_pool[12] = Constant::Methodref {
        class_index: 3,
        name_and_type_index: 13,
    };
    assert_eq!(
        IntProgram::prepare_with_calls(&external, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::UnsupportedMethod("cross-class invocation unsupported")
    );

    let mut unresolved = parse_class(NESTED).unwrap();
    unresolved.constant_pool[13] = Constant::NameAndType {
        name_index: 2,
        descriptor_index: 11,
    };
    assert_eq!(
        IntProgram::prepare_with_calls(&unresolved, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::UnresolvedMethod
    );

    // A statically typed callee needs one int argument, but none is on stack.
    let mut missing_arg = parse_class(NESTED).unwrap();
    let no_argument = [0xb8, 0x00, 0x12, 0xac]; // adjust(I)I -> increment(I)I
    let callee = missing_arg
        .methods
        .iter_mut()
        .find(|m| m.name_index == 14 && m.descriptor_index == 11)
        .unwrap();
    callee.code.as_mut().unwrap().instructions = &no_argument;
    assert_eq!(
        IntProgram::prepare_with_calls(&missing_arg, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::StackUnderflow
    );

    // A non-static target cannot be admitted for invokestatic.
    let mut initializer = parse_class(NESTED).unwrap();
    initializer.constant_pool[14] = Constant::Utf8(b"<clinit>");
    assert_eq!(
        IntProgram::prepare_with_calls(&initializer, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::UnsupportedMethod("class initialization unsupported")
    );

    let mut nonstatic = parse_class(NESTED).unwrap();
    let target = nonstatic
        .methods
        .iter_mut()
        .find(|m| m.name_index == 14 && m.descriptor_index == 11)
        .unwrap();
    target.access_flags &= !0x0008;
    assert_eq!(
        IntProgram::prepare_with_calls(&nonstatic, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::UnsupportedMethod("requires static unsynchronized concrete method")
    );

    let mut truncated = parse_class(NESTED).unwrap();
    let code = truncated
        .methods
        .iter_mut()
        .find(|m| m.code.is_some() && m.name_index == 10)
        .unwrap()
        .code
        .as_mut()
        .unwrap();
    code.instructions = &[0xb8, 0x00];
    assert_eq!(
        IntProgram::prepare_with_calls(&truncated, b"evaluate", b"(I)I")
            .unwrap_err()
            .kind,
        PrepareErrorKind::TruncatedOperand
    );
}
