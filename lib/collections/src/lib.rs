//! # Collection Data Structures

#![no_std]

#![feature(arbitrary_self_types)]
#![feature(dropck_eyepatch)]
#![feature(exact_size_is_empty)]
#![feature(trusted_len)]
#![feature(untagged_unions)]

extern crate swim_core;
extern crate swim_mem;

pub mod hash_trie;
