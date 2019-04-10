//! # Fundamental Data Types

#![no_std]

#![feature(link_llvm_intrinsics)]
#![feature(raw)]

pub use num::f16::f16;

pub mod num;
pub mod murmur3;
pub mod reify;
