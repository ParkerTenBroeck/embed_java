#![no_std]
#![feature(concat_idents)]
#![feature(lang_items)]
#![feature(asm_experimental_arch)]
#![feature(strict_provenance)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(allow_internal_unstable)]
#![feature(linkage)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(negative_impls)]
#![feature(box_syntax)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(tuple_trait)]

#[cfg(not(target_arch = "mips"))]
compile_error!("ONLY MIPS ARCHITECTURE SUPPORTED");
#[cfg(not(target_endian = "big"))]
compile_error!("NOT BIG ENDIAN");

// these are for fun :)
pub mod brock;
pub mod nji;

pub mod arch;
pub mod io;
pub mod rt;

pub mod process;
pub mod sync;
pub mod thread;

pub use core::*;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub use alloc::*;

pub mod macros;
pub use macros::*;
