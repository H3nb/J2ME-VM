// SPDX-License-Identifier: GPL-3.0-only
//! Closed arithmetic bytecode proof with VM-owned state, NOT a CLDC verifier.
//! Rules: JVMS Second Edition §§3.6, 4.7.3, 4.9 and instruction descriptions.

use crate::classfile::{ClassFile, Code};

const MAX_CODE: usize = 4096;
const MAX_STACK: usize = 64;

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
    StackUnderflow,
    StackOverflow,
    InvalidReturnStack,
    MissingReturn,
    InstructionsAfterReturn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Push(i32),
    Add,
    Subtract,
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
}

fn error(pc: usize, kind: PrepareErrorKind) -> PrepareError {
    PrepareError { pc, kind }
}

impl IntProgram {
    /// Checks one uniquely named, static, unsynchronized method with the
    /// exact no-argument, int-return signature. No class linking is implied.
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
        if class.utf8_bytes(member.descriptor_index) != Some(b"()I") {
            return Err(error(
                0,
                PrepareErrorKind::UnsupportedMethod("signature must be ()I"),
            ));
        }
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
        if code.max_locals != 0 || !code.exceptions.is_empty() || !code.attributes.is_empty() {
            return Err(error(
                0,
                PrepareErrorKind::UnsupportedMethod(
                    "locals, handlers and nested attributes unsupported",
                ),
            ));
        }
        Self::verify(code)
    }

    fn verify(code: &Code<'_>) -> Result<Self, PrepareError> {
        let bytes = code.instructions;
        if bytes.len() > MAX_CODE || code.max_stack as usize > MAX_STACK {
            return Err(error(
                0,
                PrepareErrorKind::UnsupportedMethod("resource budget exceeded"),
            ));
        }
        let mut instructions = Vec::new();
        let mut pc = 0;
        let mut depth = 0usize;
        while pc < bytes.len() {
            let at = pc;
            let opcode = bytes[pc];
            pc += 1;
            let op = match opcode {
                0x02..=0x08 => Op::Push(opcode as i32 - 3),
                0x10 => {
                    let byte = *bytes
                        .get(pc)
                        .ok_or_else(|| error(at, PrepareErrorKind::TruncatedOperand))?;
                    pc += 1;
                    Op::Push(i8::from_be_bytes([byte]) as i32)
                }
                0x11 => {
                    let hi = *bytes
                        .get(pc)
                        .ok_or_else(|| error(at, PrepareErrorKind::TruncatedOperand))?;
                    let lo = *bytes
                        .get(pc + 1)
                        .ok_or_else(|| error(at, PrepareErrorKind::TruncatedOperand))?;
                    pc += 2;
                    Op::Push(i16::from_be_bytes([hi, lo]) as i32)
                }
                0x60 => Op::Add,
                0x64 => Op::Subtract,
                0xac => Op::Return,
                _ => return Err(error(at, PrepareErrorKind::UnsupportedOpcode(opcode))),
            };
            match op {
                Op::Push(_) => {
                    depth += 1;
                    if depth > code.max_stack as usize {
                        return Err(error(at, PrepareErrorKind::StackOverflow));
                    }
                }
                Op::Add | Op::Subtract => {
                    if depth < 2 {
                        return Err(error(at, PrepareErrorKind::StackUnderflow));
                    }
                    depth -= 1;
                }
                Op::Return => {
                    if depth != 1 {
                        return Err(error(at, PrepareErrorKind::InvalidReturnStack));
                    }
                    if pc != bytes.len() {
                        return Err(error(pc, PrepareErrorKind::InstructionsAfterReturn));
                    }
                }
            }
            instructions.push(Instruction { pc: at, op });
        }
        if !matches!(instructions.last().map(|item| &item.op), Some(Op::Return)) {
            return Err(error(pc, PrepareErrorKind::MissingReturn));
        }
        Ok(Self {
            instructions,
            code_len: bytes.len(),
            max_stack: code.max_stack as usize,
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
    result: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Advanced { next_pc: usize },
    Returned(i32),
    AlreadyReturned(i32),
}

impl IntMachine {
    pub fn new(program: IntProgram) -> Self {
        Self {
            program,
            position: 0,
            stack: Vec::new(),
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
            Op::Return => {
                let value = self.stack.pop().expect("validated return");
                self.result = Some(value);
                return Step::Returned(value);
            }
        }
        debug_assert!(self.stack.len() <= self.program.max_stack);
        Step::Advanced { next_pc: self.pc() }
    }

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
        IntProgram::verify(&code)
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
