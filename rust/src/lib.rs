// This is free and unencumbered software released into the public domain.

//! This crate provides well-known languages.
//!
//! ```rust
//! use known_languages::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod language;
pub use language::*;

#[doc = include_str!("../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
