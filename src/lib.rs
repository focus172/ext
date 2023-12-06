//! The Rust User Standard Library
//!
//! This is a drop in replacement for the rust standard library. To
//! get started add it as a dependency:
//! ```toml
//! [dependencies]
//! ext = "*"
//! ```
//!
//! At which point you can start coding and get acsess to all the great user
//! crates of the rust ecosystem. If you are looking for something you will
//! find that the structure is very similar to the rust standard so try looking
//! in the same module path and you might find something interesting.

#![deny(unused_crate_dependencies)]

extern crate self as ext;

pub mod collections;
pub mod default;
pub mod log;
pub mod parse;
pub mod prelude;
pub mod sync;
pub extern crate either;
pub mod num;

pub extern crate error_stack as error;
pub extern crate glam;
pub extern crate rand;

pub use cfg_if::cfg_if;
