// SPDX-License-Identifier: GPL-3.0-only
//! Bounded integer-only execution proof, not a CLDC verifier or class loader.
//! JVMS Second Edition §§3.6, 4.7.3, 4.9 and chapter 6 instruction contracts.

use std::collections::VecDeque;

use crate::classfile::{ClassFile, Code, Constant, Member};

const MAX_CODE: usize = 4096;
const MAX_STACK: usize = 64;
const MAX_LOCALS: usize = 64;
// Host policy: one run call cannot monopolize the caller indefinitely.
const MAX_RUN_STEPS: usize = 65_536;
const MAX_FRAMES: usize = 64;
const MAX_METHODS: usize = 64;

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
    InvalidMethodReference,
    UnresolvedMethod,
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
    Invoke(u16),
    Call(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    pc: usize,
    op: Op,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MethodProgram {
    instructions: Vec<Instruction>,
    code_len: usize,
    max_stack: usize,
    max_locals: usize,
    arguments: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntProgram {
    methods: Vec<MethodProgram>,
    entry: usize,
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

impl MethodProgram {
    fn decode(code: &Code<'_>, arguments: usize, allow_calls: bool) -> Result<Self, PrepareError> {
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
                0xb8 if allow_calls => {
                    let hi = operand(bytes, at, pc)?;
                    let lo = operand(bytes, at, pc + 1)?;
                    pc += 2;
                    Op::Invoke(u16::from_be_bytes([hi, lo]))
                },
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


        Ok(Self {
            instructions,
            code_len: bytes.len(),
            max_stack,
            max_locals,
            arguments,
        })
    }

    fn validate(&self, methods: &[MethodProgram]) -> Result<(), PrepareError> {
        let instructions = &self.instructions;
        let max_stack = self.max_stack;
        let max_locals = self.max_locals;
        let arguments = self.arguments;
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
                Op::Call(target) => {
                    let arguments = methods[target].arguments;
                    if out.depth < arguments {
                        return Err(error(instruction.pc, PrepareErrorKind::StackUnderflow));
                    }
                    out.depth = out.depth - arguments + 1;
                    vec![index + 1]
                }
                Op::Invoke(_) => unreachable!("unresolved invoke cannot be verified"),
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
            return Err(error(self.code_len, PrepareErrorKind::MissingReturn));
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


        Ok(())
    }
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


    // Retain the original single-method admission path, including rejection of
    // invokestatic, so its callers and tests do not silently gain new semantics.
    fn verify(code: &Code<'_>, arguments: usize) -> Result<Self, PrepareError> {
        let method = MethodProgram::decode(code, arguments, false)?;
        method.validate(std::slice::from_ref(&method))?;
        Ok(Self { methods: vec![method], entry: 0 })
    }

    /// Closed experimental proof: a class without <clinit>, with same-class
    /// static invocations and integer-only ()I or (I)I method descriptors.
    /// This is not general class loading, initialization, or CLDC verification.
    pub fn prepare_with_calls(
        class: &ClassFile<'_>,
        name: &[u8],
        descriptor: &[u8],
    ) -> Result<Self, PrepareError> {
        if class.methods.iter().any(|member| class.utf8_bytes(member.name_index) == Some(b"<clinit>".as_slice())) {
            return Err(error(0, PrepareErrorKind::UnsupportedMethod("class initialization unsupported")));
        }
        let mut entry_member = None;
        for (index, member) in class.methods.iter().enumerate() {
            if class.utf8_bytes(member.name_index) == Some(name)
                && class.utf8_bytes(member.descriptor_index) == Some(descriptor)
            {
                if entry_member.replace(index).is_some() {
                    return Err(error(0, PrepareErrorKind::AmbiguousMethod));
                }
            }
        }
        let entry_member = entry_member.ok_or_else(|| error(0, PrepareErrorKind::MethodNotFound))?;
        let mut member_to_id = vec![None; class.methods.len()];
        member_to_id[entry_member] = Some(0usize);
        let mut pending = VecDeque::from([entry_member]);
        let mut compiled: Vec<Option<MethodProgram>> = vec![None];

        while let Some(member_index) = pending.pop_front() {
            let method_id = member_to_id[member_index].expect("queued method id");
            let member = &class.methods[member_index];
            let arguments = supported_method_arguments(class, member)?;
            let code = member.code.as_ref().expect("validated Code presence");
            let mut method = MethodProgram::decode(code, arguments, true)?;
            for instruction in &mut method.instructions {
                let Op::Invoke(cp_index) = instruction.op else { continue };
                let at = instruction.pc;
                let (class_index, name_and_type_index) =
                    match class.constant_pool.get(cp_index as usize) {
                        Some(Constant::Methodref { class_index, name_and_type_index }) =>
                            (*class_index, *name_and_type_index),
                        _ => return Err(error(at, PrepareErrorKind::InvalidMethodReference)),
                    };
                let Some(Constant::Class { name_index }) =
                    class.constant_pool.get(class_index as usize) else {
                    return Err(error(at, PrepareErrorKind::InvalidMethodReference));
                };
                if class.utf8_bytes(*name_index) != class.class_name_bytes() {
                    return Err(error(at, PrepareErrorKind::UnsupportedMethod("cross-class invocation unsupported")));
                }
                let Some(Constant::NameAndType { name_index, descriptor_index }) =
                    class.constant_pool.get(name_and_type_index as usize) else {
                    return Err(error(at, PrepareErrorKind::InvalidMethodReference));
                };
                let method_name = class.utf8_bytes(*name_index)
                    .ok_or_else(|| error(at, PrepareErrorKind::InvalidMethodReference))?;
                let signature = class.utf8_bytes(*descriptor_index)
                    .ok_or_else(|| error(at, PrepareErrorKind::InvalidMethodReference))?;
                if method_name == b"<init>" || method_name == b"<clinit>" {
                    return Err(error(at, PrepareErrorKind::UnsupportedMethod("special method invocation unsupported")));
                }
                let mut resolved = None;
                for (index, candidate) in class.methods.iter().enumerate() {
                    if class.utf8_bytes(candidate.name_index) == Some(method_name)
                        && class.utf8_bytes(candidate.descriptor_index) == Some(signature)
                    {
                        if resolved.replace(index).is_some() {
                            return Err(error(at, PrepareErrorKind::AmbiguousMethod));
                        }
                    }
                }
                let resolved = resolved.ok_or_else(|| error(at, PrepareErrorKind::UnresolvedMethod))?;
                // Callee eligibility and its Code are checked when dequeued.
                let next_id = if let Some(existing) = member_to_id[resolved] {
                    existing
                } else {
                    if compiled.len() >= MAX_METHODS {
                        return Err(error(at, PrepareErrorKind::UnsupportedMethod("method count limit")));
                    }
                    let id = compiled.len();
                    member_to_id[resolved] = Some(id);
                    compiled.push(None);
                    pending.push_back(resolved);
                    id
                };
                instruction.op = Op::Call(next_id);
            }
            compiled[method_id] = Some(method);
        }

        let methods: Vec<MethodProgram> = compiled.into_iter()
            .map(|method| method.expect("queued methods decoded")).collect();
        for method in &methods {
            method.validate(&methods)?;
        }
        Ok(Self { methods, entry: 0 })
    }

    pub fn code_len(&self) -> usize {
        self.methods[self.entry].code_len
    }
}

fn supported_method_arguments(class: &ClassFile<'_>, member: &Member<'_>) -> Result<usize, PrepareError> {
    let arguments = match class.utf8_bytes(member.descriptor_index) {
        Some(signature) if signature == b"()I" => 0,
        Some(signature) if signature == b"(I)I" => 1,
        _ => return Err(error(0, PrepareErrorKind::UnsupportedMethod("signature must be ()I or (I)I"))),
    };
    if member.access_flags & 0x0008 == 0
        || member.access_flags & (0x0020 | 0x0100 | 0x0400) != 0
    {
        return Err(error(0, PrepareErrorKind::UnsupportedMethod("requires static unsynchronized concrete method")));
    }
    let code = member.code.as_ref()
        .ok_or_else(|| error(0, PrepareErrorKind::UnsupportedMethod("missing Code")))?;
    if !code.exceptions.is_empty() || !code.attributes.is_empty() {
        return Err(error(0, PrepareErrorKind::UnsupportedMethod("handlers and nested attributes unsupported")));
    }
    Ok(arguments)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntFrame {
    method: usize,
    position: usize,
    stack: Vec<i32>,
    locals: Vec<Option<i32>>,
}

impl IntFrame {
    fn new(method: usize, program: &MethodProgram, argument: Option<i32>) -> Self {
        let mut locals = vec![None; program.max_locals];
        if let Some(value) = argument {
            locals[0] = Some(value);
        }
        Self { method, position: 0, stack: Vec::new(), locals }
    }

    pub fn method_id(&self) -> usize { self.method }
    pub fn operand_stack(&self) -> &[i32] { &self.stack }
    pub fn locals(&self) -> &[Option<i32>] { &self.locals }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionError { CallDepthExceeded }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntMachine {
    program: IntProgram,
    frames: Vec<IntFrame>,
    result: Option<i32>,
    failure: Option<ExecutionError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Advanced { next_pc: usize },
    Returned(i32),
    AlreadyReturned(i32),
    Failed(ExecutionError),
    AlreadyFailed(ExecutionError),
}

impl IntMachine {
    pub fn new(program: IntProgram) -> Self {
        assert_eq!(program.methods[program.entry].arguments, 0, "use with_int_arg for (I)I");
        Self::create(program, None)
    }

    pub fn with_int_arg(program: IntProgram, value: i32) -> Self {
        assert_eq!(program.methods[program.entry].arguments, 1, "use new for ()I");
        Self::create(program, Some(value))
    }

    fn create(program: IntProgram, argument: Option<i32>) -> Self {
        let frame = IntFrame::new(program.entry, &program.methods[program.entry], argument);
        Self { program, frames: vec![frame], result: None, failure: None }
    }

    pub fn frames(&self) -> &[IntFrame] { &self.frames }
    pub fn frame_pc(&self, depth: usize) -> Option<usize> {
        let frame = self.frames.get(depth)?;
        let method = &self.program.methods[frame.method];
        Some(method.instructions.get(frame.position)
            .map_or(method.code_len, |instruction| instruction.pc))
    }
    pub fn pc(&self) -> usize {
        self.frame_pc(self.frames.len().saturating_sub(1))
            .unwrap_or_else(|| self.program.methods[self.program.entry].code_len)
    }
    pub fn operand_stack(&self) -> &[i32] {
        self.frames.last().map_or(&[], |frame| frame.operand_stack())
    }
    pub fn locals(&self) -> &[Option<i32>] {
        self.frames.last().map_or(&[], |frame| frame.locals())
    }
    pub fn result(&self) -> Option<i32> { self.result }
    pub fn failure(&self) -> Option<ExecutionError> { self.failure }

    /// Exactly one guest opcode, including call or return, is one step. No
    /// native Rust recursion is used for nested Java calls.
    pub fn step(&mut self) -> Step {
        if let Some(value) = self.result { return Step::AlreadyReturned(value); }
        if let Some(err) = self.failure { return Step::AlreadyFailed(err); }
        let current = self.frames.len() - 1;
        let method_id = self.frames[current].method;
        let instruction = self.program.methods[method_id].instructions[self.frames[current].position].clone();
        if let Op::Call(target) = instruction.op {
            if self.frames.len() >= MAX_FRAMES {
                self.failure = Some(ExecutionError::CallDepthExceeded);
                return Step::Failed(ExecutionError::CallDepthExceeded);
            }
            let callee = &self.program.methods[target];
            let arg = if callee.arguments == 1 {
                Some(*self.frames[current].stack.last().expect("validated invocation argument"))
            } else {
                None
            };
            // State transition is committed only after the depth preflight.
            self.frames[current].position += 1;
            if callee.arguments == 1 { self.frames[current].stack.pop(); }
            self.frames.push(IntFrame::new(target, callee, arg));
            return Step::Advanced { next_pc: self.pc() };
        }
        if instruction.op == Op::Return {
            let value = self.frames[current].stack.pop().expect("validated return");
            self.frames.pop();
            if let Some(caller) = self.frames.last_mut() {
                caller.stack.push(value);
                return Step::Advanced { next_pc: self.pc() };
            }
            self.result = Some(value);
            return Step::Returned(value);
        }
        let frame = &mut self.frames[current];
        frame.position += 1;
        match instruction.op {
            Op::Push(value) => frame.stack.push(value),
            Op::Load(local) => frame.stack.push(frame.locals[local].expect("validated local read")),
            Op::Store(local) => frame.locals[local] = Some(frame.stack.pop().expect("validated store")),
            Op::Increment(local, amount) => {
                let value = frame.locals[local].expect("validated increment");
                frame.locals[local] = Some(value.wrapping_add(amount));
            }
            Op::Add => {
                let rhs = frame.stack.pop().expect("validated RHS");
                let lhs = frame.stack.pop().expect("validated LHS");
                frame.stack.push(lhs.wrapping_add(rhs));
            }
            Op::Subtract => {
                let rhs = frame.stack.pop().expect("validated RHS");
                let lhs = frame.stack.pop().expect("validated LHS");
                frame.stack.push(lhs.wrapping_sub(rhs));
            }
            Op::CompareBranch(condition, target) => {
                let rhs = frame.stack.pop().expect("validated RHS");
                let lhs = frame.stack.pop().expect("validated LHS");
                if condition.test(lhs, rhs) { frame.position = target; }
            }
            Op::Goto(target) => frame.position = target,
            Op::Return | Op::Call(_) | Op::Invoke(_) => unreachable!("handled or rejected before execution"),
        }
        debug_assert!(frame.stack.len() <= self.program.methods[frame.method].max_stack);
        Step::Advanced { next_pc: self.pc() }
    }

    /// Guest instruction budget spans every nested call. Failure is terminal
    /// and separately inspectable; budget exhaustion leaves frames resumable.
    pub fn run(&mut self, budget: usize) -> Option<i32> {
        if self.result.is_some() { return self.result; }
        if self.failure.is_some() { return None; }
        for _ in 0..budget.min(MAX_RUN_STEPS) {
            match self.step() {
                Step::Returned(value) | Step::AlreadyReturned(value) => return Some(value),
                Step::Failed(_) | Step::AlreadyFailed(_) => return None,
                Step::Advanced { .. } => {}
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
