// SPDX-License-Identifier: GPL-3.0-only
//! Bounded class-file structural reader for the CLDC 1.1 research baseline.
//!
//! Format: JVMS Second Edition, chapter 4 (u1/u2/u4, constant pool,
//! fields, methods and attributes), with the CLDC 1.1 class-file range
//! (45.x, 46.0, 47.0, 48.0). This is **not** a bytecode verifier.

/// Maximum accepted class input size. This is an explicit host resource policy,
/// not a limit imposed by the Java class-file specification.
pub const MAX_CLASS_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub offset: usize,
    pub kind: ParseErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    InputTooLarge,
    UnexpectedEof,
    BadMagic,
    UnsupportedVersion { major: u16, minor: u16 },
    InvalidConstantPoolCount,
    UnknownConstantTag(u8),
    InvalidModifiedUtf8,
    InvalidReference { index: u16, expected: &'static str },
    InvalidCodeLength,
    InvalidExceptionRange,
    TrailingBytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant<'a> {
    /// Index zero, or the reserved second slot of a Long/Double.
    Unusable,
    /// Raw bytes in JVM modified UTF-8 (not necessarily valid Rust UTF-8).
    Utf8(&'a [u8]),
    Integer(i32),
    Float(u32),
    Long(i64),
    Double(u64),
    Class {
        name_index: u16,
    },
    String {
        string_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute<'a> {
    pub name_index: u16,
    /// Absolute byte offset at the beginning of the attribute contents.
    pub info_offset: usize,
    /// Retained for attributes whose semantics are handled at a later gate.
    pub info: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code<'a> {
    pub max_stack: u16,
    pub max_locals: u16,
    pub instructions: &'a [u8],
    pub exceptions: Vec<ExceptionHandler>,
    pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member<'a> {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute<'a>>,
    /// First Code attribute of a method, if present. Duplicate Code attributes
    /// are a verifier-level concern and are not accepted as executable yet.
    pub code: Option<Code<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassFile<'a> {
    pub minor_version: u16,
    pub major_version: u16,
    /// Indexed by constant-pool index, including index zero and reserved holes.
    pub constant_pool: Vec<Constant<'a>>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Member<'a>>,
    pub methods: Vec<Member<'a>>,
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> ClassFile<'a> {
    pub fn utf8_bytes(&self, index: u16) -> Option<&'a [u8]> {
        match self.constant_pool.get(index as usize)? {
            Constant::Utf8(bytes) => Some(bytes),
            _ => None,
        }
    }

    pub fn class_name_bytes(&self) -> Option<&'a [u8]> {
        match self.constant_pool.get(self.this_class as usize)? {
            Constant::Class { name_index } => self.utf8_bytes(*name_index),
            _ => None,
        }
    }
}

struct Reader<'a> {
    input: &'a [u8],
    pos: usize,
    base: usize,
}

impl<'a> Reader<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            pos: 0,
            base: 0,
        }
    }

    fn nested(input: &'a [u8], base: usize) -> Self {
        Self {
            input,
            pos: 0,
            base,
        }
    }

    fn offset(&self) -> usize {
        self.base + self.pos
    }

    fn error(&self, kind: ParseErrorKind) -> ParseError {
        ParseError {
            offset: self.offset(),
            kind,
        }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], ParseError> {
        let end = self
            .pos
            .checked_add(n)
            .ok_or_else(|| self.error(ParseErrorKind::UnexpectedEof))?;
        let chunk = self
            .input
            .get(self.pos..end)
            .ok_or_else(|| self.error(ParseErrorKind::UnexpectedEof))?;
        self.pos = end;
        Ok(chunk)
    }

    fn u1(&mut self) -> Result<u8, ParseError> {
        Ok(self.take(1)?[0])
    }

    fn u2(&mut self) -> Result<u16, ParseError> {
        Ok(u16::from_be_bytes(
            self.take(2)?.try_into().expect("exact width"),
        ))
    }

    fn u4(&mut self) -> Result<u32, ParseError> {
        Ok(u32::from_be_bytes(
            self.take(4)?.try_into().expect("exact width"),
        ))
    }

    fn finish(&self) -> Result<(), ParseError> {
        if self.pos == self.input.len() {
            Ok(())
        } else {
            Err(self.error(ParseErrorKind::TrailingBytes))
        }
    }
}

fn check_ref<'a>(
    pool: &[Constant<'a>],
    index: u16,
    expected: &'static str,
    predicate: impl FnOnce(&Constant<'a>) -> bool,
    offset: usize,
) -> Result<(), ParseError> {
    if index == 0 || !pool.get(index as usize).is_some_and(predicate) {
        return Err(ParseError {
            offset,
            kind: ParseErrorKind::InvalidReference { index, expected },
        });
    }
    Ok(())
}

fn is_class(c: &Constant<'_>) -> bool {
    matches!(c, Constant::Class { .. })
}
fn is_utf8(c: &Constant<'_>) -> bool {
    matches!(c, Constant::Utf8(_))
}
fn is_name_and_type(c: &Constant<'_>) -> bool {
    matches!(c, Constant::NameAndType { .. })
}

// JVM modified UTF-8 represents UTF-16 code units; lone surrogates can occur
// at the encoding layer. Never normalize guest identifiers lossy.
fn valid_modified_utf8(bytes: &[u8]) -> bool {
    let mut pos = 0;
    while pos < bytes.len() {
        let b = bytes[pos];
        let width = match b {
            0x01..=0x7f => 1,
            0xc0..=0xdf => 2,
            0xe0..=0xef => 3,
            _ => return false,
        };
        if pos + width > bytes.len() {
            return false;
        }
        if width > 1 && (1..width).any(|i| bytes[pos + i] & 0xc0 != 0x80) {
            return false;
        }
        if width == 2 {
            let value = ((b as u16 & 0x1f) << 6) | (bytes[pos + 1] as u16 & 0x3f);
            if value != 0 && value < 0x80 {
                return false;
            }
        }
        if width == 3 {
            let value = ((b as u16 & 0x0f) << 12)
                | ((bytes[pos + 1] as u16 & 0x3f) << 6)
                | (bytes[pos + 2] as u16 & 0x3f);
            if value < 0x800 {
                return false;
            }
        }
        pos += width;
    }
    true
}

fn parse_pool<'a>(r: &mut Reader<'a>) -> Result<Vec<Constant<'a>>, ParseError> {
    let count = r.u2()? as usize;
    if count == 0 {
        return Err(r.error(ParseErrorKind::InvalidConstantPoolCount));
    }
    let mut pool = Vec::new();
    pool.push(Constant::Unusable);
    while pool.len() < count {
        let at = r.offset();
        let tag = r.u1()?;
        let constant = match tag {
            1 => {
                let len = r.u2()? as usize;
                let data = r.take(len)?;
                if !valid_modified_utf8(data) {
                    return Err(ParseError {
                        offset: at,
                        kind: ParseErrorKind::InvalidModifiedUtf8,
                    });
                }
                Constant::Utf8(data)
            }
            3 => Constant::Integer(r.u4()? as i32),
            4 => Constant::Float(r.u4()?),
            5 => {
                let high = r.u4()? as u64;
                let low = r.u4()? as u64;
                let value = Constant::Long(((high << 32) | low) as i64);
                if pool.len() + 1 >= count {
                    return Err(ParseError {
                        offset: at,
                        kind: ParseErrorKind::InvalidConstantPoolCount,
                    });
                }
                pool.push(value);
                pool.push(Constant::Unusable);
                continue;
            }
            6 => {
                let high = r.u4()? as u64;
                let low = r.u4()? as u64;
                let value = Constant::Double((high << 32) | low);
                if pool.len() + 1 >= count {
                    return Err(ParseError {
                        offset: at,
                        kind: ParseErrorKind::InvalidConstantPoolCount,
                    });
                }
                pool.push(value);
                pool.push(Constant::Unusable);
                continue;
            }
            7 => Constant::Class {
                name_index: r.u2()?,
            },
            8 => Constant::String {
                string_index: r.u2()?,
            },
            9 => Constant::Fieldref {
                class_index: r.u2()?,
                name_and_type_index: r.u2()?,
            },
            10 => Constant::Methodref {
                class_index: r.u2()?,
                name_and_type_index: r.u2()?,
            },
            11 => Constant::InterfaceMethodref {
                class_index: r.u2()?,
                name_and_type_index: r.u2()?,
            },
            12 => Constant::NameAndType {
                name_index: r.u2()?,
                descriptor_index: r.u2()?,
            },
            _ => {
                return Err(ParseError {
                    offset: at,
                    kind: ParseErrorKind::UnknownConstantTag(tag),
                })
            }
        };
        pool.push(constant);
    }
    for c in &pool {
        let at = r.offset();
        match c {
            Constant::Class { name_index } => check_ref(&pool, *name_index, "Utf8", is_utf8, at)?,
            Constant::String { string_index } => {
                check_ref(&pool, *string_index, "Utf8", is_utf8, at)?
            }
            Constant::Fieldref {
                class_index,
                name_and_type_index,
            }
            | Constant::Methodref {
                class_index,
                name_and_type_index,
            }
            | Constant::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => {
                check_ref(&pool, *class_index, "Class", is_class, at)?;
                check_ref(
                    &pool,
                    *name_and_type_index,
                    "NameAndType",
                    is_name_and_type,
                    at,
                )?;
            }
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => {
                check_ref(&pool, *name_index, "Utf8", is_utf8, at)?;
                check_ref(&pool, *descriptor_index, "Utf8", is_utf8, at)?;
            }
            _ => {}
        }
    }
    Ok(pool)
}

fn parse_attributes<'a>(
    r: &mut Reader<'a>,
    pool: &[Constant<'a>],
) -> Result<Vec<Attribute<'a>>, ParseError> {
    let count = r.u2()? as usize;
    let mut items = Vec::new();
    for _ in 0..count {
        let name_at = r.offset();
        let name_index = r.u2()?;
        check_ref(pool, name_index, "Utf8 attribute name", is_utf8, name_at)?;
        let len = r.u4()? as usize;
        let info_offset = r.offset();
        let info = r.take(len)?;
        items.push(Attribute {
            name_index,
            info_offset,
            info,
        });
    }
    Ok(items)
}

fn parse_code<'a>(attr: &Attribute<'a>, pool: &[Constant<'a>]) -> Result<Code<'a>, ParseError> {
    let mut r = Reader::nested(attr.info, attr.info_offset);
    let max_stack = r.u2()?;
    let max_locals = r.u2()?;
    let code_len = r.u4()? as usize;
    if !(1..=65535).contains(&code_len) {
        return Err(r.error(ParseErrorKind::InvalidCodeLength));
    }
    let instructions = r.take(code_len)?;
    let exception_count = r.u2()?;
    let mut exceptions = Vec::new();
    for _ in 0..exception_count {
        let start_pc = r.u2()?;
        let end_pc = r.u2()?;
        let handler_pc = r.u2()?;
        let catch_type = r.u2()?;
        if start_pc >= end_pc || end_pc as usize > code_len || handler_pc as usize >= code_len {
            return Err(r.error(ParseErrorKind::InvalidExceptionRange));
        }
        if catch_type != 0 {
            check_ref(pool, catch_type, "Class catch type", is_class, r.offset())?;
        }
        exceptions.push(ExceptionHandler {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        });
    }
    let attributes = parse_attributes(&mut r, pool)?;
    r.finish()?;
    Ok(Code {
        max_stack,
        max_locals,
        instructions,
        exceptions,
        attributes,
    })
}

fn parse_members<'a>(
    r: &mut Reader<'a>,
    pool: &[Constant<'a>],
    methods: bool,
) -> Result<Vec<Member<'a>>, ParseError> {
    let count = r.u2()? as usize;
    let mut items = Vec::new();
    for _ in 0..count {
        let access_flags = r.u2()?;
        let name_at = r.offset();
        let name_index = r.u2()?;
        check_ref(pool, name_index, "Utf8 member name", is_utf8, name_at)?;
        let descriptor_at = r.offset();
        let descriptor_index = r.u2()?;
        check_ref(
            pool,
            descriptor_index,
            "Utf8 descriptor",
            is_utf8,
            descriptor_at,
        )?;
        let attributes = parse_attributes(r, pool)?;
        let mut code = None;
        if methods {
            for attr in &attributes {
                let name = match pool.get(attr.name_index as usize) {
                    Some(Constant::Utf8(s)) => *s,
                    _ => unreachable!("attribute name verified"),
                };
                if name == b"Code" {
                    let parsed = parse_code(attr, pool)?;
                    if code.is_none() {
                        code = Some(parsed);
                    }
                }
            }
        }
        items.push(Member {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
            code,
        });
    }
    Ok(items)
}

/// Reads a class-file's structure and selected Code layout from untrusted
/// input. Parsing never loads or executes guest code and is not a substitute
/// for format/semantic verification or CLDC StackMap type checking.
pub fn parse_class(input: &[u8]) -> Result<ClassFile<'_>, ParseError> {
    if input.len() > MAX_CLASS_BYTES {
        return Err(ParseError {
            offset: 0,
            kind: ParseErrorKind::InputTooLarge,
        });
    }
    let mut r = Reader::new(input);
    let magic = r.u4()?;
    if magic != 0xcafe_babe {
        return Err(ParseError {
            offset: 0,
            kind: ParseErrorKind::BadMagic,
        });
    }
    let minor_version = r.u2()?;
    let major_version = r.u2()?;
    if !(major_version == 45 || ((46..=48).contains(&major_version) && minor_version == 0)) {
        return Err(r.error(ParseErrorKind::UnsupportedVersion {
            major: major_version,
            minor: minor_version,
        }));
    }
    let constant_pool = parse_pool(&mut r)?;
    let access_flags = r.u2()?;
    let this_at = r.offset();
    let this_class = r.u2()?;
    check_ref(
        &constant_pool,
        this_class,
        "Class (this_class)",
        is_class,
        this_at,
    )?;
    let super_at = r.offset();
    let super_class = r.u2()?;
    if super_class != 0 {
        check_ref(
            &constant_pool,
            super_class,
            "Class (super_class)",
            is_class,
            super_at,
        )?;
    }
    let interfaces_count = r.u2()? as usize;
    let mut interfaces = Vec::new();
    for _ in 0..interfaces_count {
        let at = r.offset();
        let index = r.u2()?;
        check_ref(&constant_pool, index, "Class (interface)", is_class, at)?;
        interfaces.push(index);
    }
    let fields = parse_members(&mut r, &constant_pool, false)?;
    let methods = parse_members(&mut r, &constant_pool, true)?;
    let attributes = parse_attributes(&mut r, &constant_pool)?;
    r.finish()?;
    Ok(ClassFile {
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes,
    })
}
