#![warn(rust_2018_idioms)]
#![allow(dead_code)]

//! `algae` is a collection of abstract algebraic structures implemented in
//! Rust. It begins by defining sets and eventually builds up to vector spaces
//! and Lie groups.

pub mod algaeset;
pub mod magma;
pub mod mapping;
