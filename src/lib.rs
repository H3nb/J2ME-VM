// SPDX-License-Identifier: GPL-3.0-only
//! Experimental, independent Java ME VM research core.
//!
//! G1 provides a bounded class-file structural reader only. No bytecode
//! interpreter, verifier, CLDC runtime, or snapshot implementation exists yet.

#![forbid(unsafe_code)]

pub mod classfile;
