// SPDX-License-Identifier: GPL-3.0-only
//! Experimental independent Java ME VM research core.
//!
//! G1 includes a structural parser. WP02 adds a narrow integer-only execution
//! proof; it is not a CLDC verifier, class loader, or general VM.

#![forbid(unsafe_code)]

pub mod classfile;
pub mod execution;
