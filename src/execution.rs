// SPDX-License-Identifier: GPL-3.0-only
//! Bounded integer-only execution proof, not a CLDC verifier or class loader.
//! JVMS Second Edition §§3.6, 4.7.3, 4.9 and chapter 6 instruction contracts.

use std::collections::VecDeque;

use crate::classfile::{ClassFile, Code};

const MAX_CODE: usize = 4096;
const MAX_STACK: usize = 64;
const MAX_LOCALS: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrepareError {
    pub pc: usize,
    pub kind: PrepareErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrepareErrorKind {
    MethodNotFound,
    AmbiguousMethod,
    UnsupportedMethod(&'static str),
    UnsupportedOpcode(u8),
    TruncatedOperand,
    InvalidLocal,
    UninitializedLocal,
    InvalidBranchTarget,
    InconsistentStackJoin,
    UnreachableInstruction,
    StackUnderflow,
    StackOverflow,
    InvalidReturnStack,
    MissingReturn,
    InstructionsAfterReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    Equal,
    NotEqual,
    Less,
    GreaterOrEqual,
    Greater,
    LessOrEqual,
}

impl Comparison {
    fn test(self, lhs: i32, rhs: i32) -> bool {
        match self {
            Self::Equal => lhs == rhs,
            Self::NotEqual => lhs != rhs,
            Self::Less => lhs < rhs,
            Self::GreaterOrEqual => lhs >= rhs,
            Self::Greater => lhs > rhs,
            Self::LessOrEqual => lhs <= rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Push(i32),
    Load(usize),
    Store(usize),
    Increment(usize, i32),
    Add,
    Subtract,
    CompareBranch(Comparison, usize),
    Goto(usize),
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    pc: usize,
    op: Op,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntProgram {
    instructions: Vec<Instruction>,
    code_len: usize,
    max_stack: usize,
    max_locals: usize,
    arguments: usize,
}

#[derive(Clone, PartialEq, Eq)]
struct FrameShape {
    depth: usize,
    initialized: Vec<bool>,
}

fn error(pc: usize, kind: PrepareErrorKind) -> PrepareError {
    PrepareError { pc, kind }
}

fn operand(bytes: &[u8], at: usize, offset: usize) -> Result<u8, PrepareError> {
    bytes
        .get(offset)
        .copied()
        .ok_or_else(|| error(at, PrepareErrorKind::TruncatedOperand))
}

fn branch_target(bytes: &[u8], at: usize) -> Result<usize, PrepareError> {
    let hi = operand(bytes, at, at + 1)?;
    let lo = operand(bytes, at, at + 2)?;
    let offset = i16::from_be_bytes([hi, lo]) as i32;
    let target = at as i32 + offset;
    if target < 0 || target >= bytes.len() as i32 {
        return Err(error(at, PrepareErrorKind::InvalidBranchTarget));
    }
    Ok(target as usize)
}

impl IntProgram {
    /// Prepare exactly one static int-return method with either zero or one
    /// int parameter. This checks only the explicitly supported opcode subset.
    pub fn prepare(class: &ClassFile<'_>, name: &[u8]) -> Result<Self, PrepareError> {
        let mut selected = None;
        for member in &class.methods {
            if class.utf8_bytes(member.name_index) == Some(name)
                && selected.replace(member).is_some()
            {
                return Err(error(0, PrepareErrorKind::AmbiguousMethod));
            }
        }
        let member = selected.ok_or_else(|| error(0, PrepareErrorKind::MethodNotFound))?;
        let arguments = match class.utf8_bytes(member.descriptor_index) {
            Some(signature) if signature == b"()I" => 0,
            Some(signature) if signature == b"(I)I" => 1,
            _ => {
                return Err(error(
                    0,
                    PrepareErrorKind::UnsupportedMethod("signature must be ()I or (I)I"),
                ));
            }
        };
        if member.access_flags & 0x0008 == 0
            || member.access_flags & (0x0020 | 0x0100 | 0x0400) != 0
        {
            return Err(error(
                0,
                PrepareErrorKind::UnsupportedMethod(
                    "requires static unsynchronized concrete method",
                ),
            ));
        }
        let code = member
            .code
            .as_ref()
            .ok_or_else(|| error(0, PrepareErrorKind::UnsupportedMethod("missing Code")))?;
        if !code.exceptions.is_empty() || !code.attributes.is_empty() {
            return Err(error(
                0,
                PrepareErrorKind::UnsupportedMethod("handlers and nested attributes unsupported"),
            ));
        }
        Self::verify(code, arguments)
    }

    fn verify(code: &Code<'_>, arguments: usize) -> Result<Self, PrepareError> {
        let bytes = code.instructions;
        let max_stack = code.max_stack as usize;
        let max_locals = code.max_locals as usize;
        if bytes.len() > MAX_CODE || max_stack > MAX_STACK || max_locals > MAX_LOCALS {
            return Err(error(
                0,
                PrepareErrorKind::UnsupportedMethod("resource budget exceeded"),
            ));
        }
        if max_locals < arguments {
            return Err(error(0, PrepareErrorKind::InvalidLocal));
        }

        // Decode every byte, including unreachable regions, and record only
        // opcode boundaries. Branch operands are PC-relative to their opcodes.
        let mut instructions = Vec::new();
        let mut pc_to_index = vec![None; bytes.len()];
        let mut pc = 0;
        while pc < bytes.len() {
            let at = pc;
            pc_to_index[at] = Some(instructions.len());
            let opcode = bytes[pc];
            pc += 1;
            let op = match opcode {
                0x02..=0x08 => Op::Push(opcode as i32 - 3),
                0x10 => {
                    let value = operand(bytes, at, pc)? as i8 as i32;
                    pc += 1;
                    Op::Push(value)
                }
                0x11 => {
                    let hi = operand(bytes, at, pc)?;
                    let lo = operand(bytes, at, pc + 1)?;
                    pc += 2;
                    Op::Push(i16::from_be_bytes([hi, lo]) as i32)
                }
                0x15 | 0x36 => {
                    let index = operand(bytes, at, pc)? as usize;
                    pc += 1;
                    if opcode == 0x15 {
                        Op::Load(index)
                    } else {
                        Op::Store(index)
                    }
                }
                0x1a..=0x1d => Op::Load((opcode - 0x1a) as usize),
                0x3b..=0x3e => Op::Store((opcode - 0x3b) as usize),
                0x60 => Op::Add,
                0x64 => Op::Subtract,
                0x84 => {
                    let index = operand(bytes, at, pc)? as usize;
                    let change = operand(bytes, at, pc + 1)? as i8 as i32;
                    pc += 2;
                    Op::Increment(index, change)
                }
                0x9f..=0xa4 => {
                    let condition = match opcode {
                        0x9f => Comparison::Equal,
                        0xa0 => Comparison::NotEqual,
                        0xa1 => Comparison::Less,
                        0xa2 => Comparison::GreaterOrEqual,
                        0xa3 => Comparison::Greater,
                        _ => Comparison::LessOrEqual,
                    };
                    pc += 2;
                    Op::CompareBranch(condition, branch_target(bytes, at)?)
                }
                0xa7 => {
                    pc += 2;
                    Op::Goto(branch_target(bytes, at)?)
                }
                0xac => Op::Return,
                _ => return Err(error(at, PrepareErrorKind::UnsupportedOpcode(opcode))),
            };
            if matches!(op, Op::Load(i) | Op::Store(i) | Op::Increment(i, _) if i >= max_locals) {
                return Err(error(at, PrepareErrorKind::InvalidLocal));
            }
            instructions.push(Instruction { pc: at, op });
        }

        // Replace bytecode targets with indices only after all boundaries are known.
        for instruction in &mut instructions {
            let target = match &mut instruction.op {
                Op::CompareBranch(_, target) | Op::Goto(target) => target,
                _ => continue,
            };
            *target = pc_to_index[*target]
                .ok_or_else(|| error(instruction.pc, PrepareErrorKind::InvalidBranchTarget))?;
        }

        if instructions.is_empty() {
            return Err(error(0, PrepareErrorKind::MissingReturn));
        }

        // Forward fixed point: every incoming edge must agree on operand
        // stack height; a local is definitely assigned only if assigned on
        // every predecessor path. Back-edges are revisited when facts shrink.
        let mut entry = FrameShape {
            depth: 0,
            initialized: vec![false; max_locals],
        };
        for initialized in &mut entry.initialized[..arguments] {
            *initialized = true;
        }
        let mut states = vec![None::<FrameShape>; instructions.len()];
        states[0] = Some(entry);
        let mut pending = VecDeque::from([0]);
        let mut has_return = false;
        while let Some(index) = pending.pop_front() {
            let instruction = &instructions[index];
            let mut out = states[index].as_ref().expect("queued state").clone();
            let successors = match instruction.op {
                Op::Push(_) => {
                    out.depth += 1;
                    vec![index + 1]
                }
                Op::Load(local) => {
                    if !out.initialized[local] {
                        return Err(error(instruction.pc, PrepareErrorKind::UninitializedLocal));
                    }
                    out.depth += 1;
                    vec![index + 1]
                }
                Op::Store(local) => {
                    if out.depth == 0 {
                        return Err(error(instruction.pc, PrepareErrorKind::StackUnderflow));
                    }
                    out.depth -= 1;
                    out.initialized[local] = true;
                    vec![index + 1]
                }
                Op::Increment(local, _) => {
                    if !out.initialized[local] {
                        return Err(error(instruction.pc, PrepareErrorKind::UninitializedLocal));
                    }
                    vec![index + 1]
                }
                Op::Add | Op::Subtract => {
                    if out.depth < 2 {
                        return Err(error(instruction.pc, PrepareErrorKind::StackUnderflow));
                    }
                    out.depth -= 1;
                    vec![index + 1]
                }
                Op::CompareBranch(_, target) => {
                    if out.depth < 2 {
                        return Err(error(instruction.pc, PrepareErrorKind::StackUnderflow));
                    }
                    out.depth -= 2;
                    vec![target, index + 1]
                }
                Op::Goto(target) => vec![target],
                Op::Return => {
                    if out.depth != 1 {
                        return Err(error(instruction.pc, PrepareErrorKind::InvalidReturnStack));
                    }
                    has_return = true;
                    Vec::new()
                }
            };
            if out.depth > max_stack {
                return Err(error(instruction.pc, PrepareErrorKind::StackOverflow));
            }
            for successor in successors {
                if successor >= instructions.len() {
                    return Err(error(instruction.pc, PrepareErrorKind::MissingReturn));
                }
                match &mut states[successor] {
                    None => {
                        states[successor] = Some(out.clone());
                        pending.push_back(successor);
                    }
                    Some(existing) => {
                        if existing.depth != out.depth {
                            return Err(error(
                                instructions[successor].pc,
                                PrepareErrorKind::InconsistentStackJoin,
                            ));
                        }
                        let mut changed = false;
                        for (current, incoming) in
                            existing.initialized.iter_mut().zip(&out.initialized)
                        {
                            if *current && !incoming {
                                *current = false;
                                changed = true;
                            }
                        }
                        if changed {
                            pending.push_back(successor);
                        }
                    }
                }
            }
        }
        if !has_return {
            return Err(error(bytes.len(), PrepareErrorKind::MissingReturn));
        }
        for (index, state) in states.iter().enumerate() {
            if state.is_none() {
                let kind = if index > 0 && matches!(instructions[index - 1].op, Op::Return) {
                    PrepareErrorKind::InstructionsAfterReturn
                } else {
                    PrepareErrorKind::UnreachableInstruction
                };
                return Err(error(instructions[index].pc, kind));
            }
        }

        Ok(Self {
            instructions,
            code_len: bytes.len(),
            max_stack,
            max_locals,
            arguments,
        })
    }

    pub fn code_len(&self) -> usize {
        self.code_len
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntMachine {
    program: IntProgram,
    position: usize,
    stack: Vec<i32>,
    locals: Vec<Option<i32>>,
    result: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Advanced { next_pc: usize },
    Returned(i32),
    AlreadyReturned(i32),
}

impl IntMachine {
    /// Construct the original no-argument proof. Supplying the wrong invocation
    /// shape is a host programming error, not guest-bytecode validation.
    pub fn new(program: IntProgram) -> Self {
        assert_eq!(program.arguments, 0, "use with_int_arg for (I)I");
        Self::create(program, None)
    }

    pub fn with_int_arg(program: IntProgram, value: i32) -> Self {
        assert_eq!(program.arguments, 1, "use new for ()I");
        Self::create(program, Some(value))
    }

    fn create(program: IntProgram, arg: Option<i32>) -> Self {
        let mut locals = vec![None; program.max_locals];
        if let Some(value) = arg {
            locals[0] = Some(value);
        }
        Self {
            program,
            position: 0,
            stack: Vec::new(),
            locals,
            result: None,
        }
    }

    pub fn pc(&self) -> usize {
        self.program
            .instructions
            .get(self.position)
            .map_or(self.program.code_len, |instruction| instruction.pc)
    }

    pub fn operand_stack(&self) -> &[i32] {
        &self.stack
    }

    pub fn locals(&self) -> &[Option<i32>] {
        &self.locals
    }

    pub fn result(&self) -> Option<i32> {
        self.result
    }

    pub fn step(&mut self) -> Step {
        if let Some(value) = self.result {
            return Step::AlreadyReturned(value);
        }
        let instruction = &self.program.instructions[self.position];
        self.position += 1;
        match instruction.op {
            Op::Push(value) => self.stack.push(value),
            Op::Load(local) => self.stack.push(self.locals[local].expect("validated local read")),
            Op::Store(local) => {
                self.locals[local] = Some(self.stack.pop().expect("validated store"));
            }
            Op::Increment(local, amount) => {
                let value = self.locals[local].expect("validated increment");
                self.locals[local] = Some(value.wrapping_add(amount));
            }
            Op::Add => {
                let rhs = self.stack.pop().expect("validated RHS");
                let lhs = self.stack.pop().expect("validated LHS");
                self.stack.push(lhs.wrapping_add(rhs));
            }
            Op::Subtract => {
                let rhs = self.stack.pop().expect("validated RHS");
                let lhs = self.stack.pop().expect("validated LHS");
                self.stack.push(lhs.wrapping_sub(rhs));
            }
            Op::CompareBranch(condition, target) => {
                let rhs = self.stack.pop().expect("validated RHS");
                let lhs = self.stack.pop().expect("validated LHS");
                if condition.test(lhs, rhs) {
                    self.position = target;
                }
            }
            Op::Goto(target) => self.position = target,
            Op::Return => {
                let value = self.stack.pop().expect("validated return");
                self.result = Some(value);
                return Step::Returned(value);
            }
        }
        debug_assert!(self.stack.len() <= self.program.max_stack);
        Step::Advanced { next_pc: self.pc() }
    }

    /// Each call consumes at most `budget` guest instructions; exhausted
    /// budget preserves the entire frame for a subsequent call or inspection.
    pub fn run(&mut self, budget: usize) -> Option<i32> {
        if self.result.is_some() {
            return self.result;
        }
        for _ in 0..budget {
            if let Step::Returned(value) = self.step() {
                return Some(value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verified(bytes: &[u8], stack: u16) -> Result<IntProgram, PrepareError> {
        let code = Code {
            max_stack: stack,
            max_locals: 0,
            instructions: bytes,
            exceptions: Vec::new(),
            attributes: Vec::new(),
        };
        IntProgram::verify(&code, 0)
    }

    #[test]
    fn rejects_invalid_shapes_before_execution() {
        for (bytes, stack, expected) in [
            (&[0x60, 0xac][..], 2, PrepareErrorKind::StackUnderflow),
            (&[0x08, 0xac][..], 0, PrepareErrorKind::StackOverflow),
            (&[0x10][..], 1, PrepareErrorKind::TruncatedOperand),
            (&[0x11, 0x01][..], 1, PrepareErrorKind::TruncatedOperand),
            (&[0x03][..], 1, PrepareErrorKind::MissingReturn),
            (&[0xac][..], 1, PrepareErrorKind::InvalidReturnStack),
            (
                &[0x03, 0xac, 0x04][..],
                1,
                PrepareErrorKind::InstructionsAfterReturn,
            ),
            (&[0xb8][..], 1, PrepareErrorKind::UnsupportedOpcode(0xb8)),
        ] {
            assert_eq!(verified(bytes, stack).unwrap_err().kind, expected);
        }
    }

    #[test]
    fn signed_operands_arithmetic_and_budget() {
        // sipush -32768; bipush -1; isub; ireturn
        let mut vm =
            IntMachine::new(verified(&[0x11, 0x80, 0, 0x10, 0xff, 0x64, 0xac], 2).unwrap());
        assert_eq!(vm.run(2), None);
        assert_eq!(vm.operand_stack(), &[-32768, -1]);
        assert_eq!(vm.run(2), Some(-32767));
        assert_eq!(vm.step(), Step::AlreadyReturned(-32767));
    }

    #[test]
    fn independently_owned_invocation_state() {
        let program = verified(&[0x05, 0x06, 0x60, 0xac], 2).unwrap();
        let mut first = IntMachine::new(program.clone());
        let second = IntMachine::new(program);
        assert_eq!(first.step(), Step::Advanced { next_pc: 1 });
        assert_eq!(first.operand_stack(), &[2]);
        assert_eq!(second.pc(), 0);
        assert_eq!(second.operand_stack(), &[]);
        assert_eq!(first.run(3), Some(5));
    }
}
