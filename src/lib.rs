//! **siunit** — Compile-time checked SI units via const generics.
//!
//! Every physical quantity carries its dimension as const generic parameters.
//! Dimension mismatches are caught at compile time with zero runtime overhead.
//!
//! # Example
//!
//! ```rust
//! use siunit::*;
//!
//! let d = meters(100.0);
//! let t = seconds(10.0);
//! let _ = d + d;               // OK: same dimension
//! // let _ = d + t;            // ❌ compile error: dimension mismatch
//! ```
//!
//! Type-safe dimension arithmetic is achieved through `Mul`/`Div` — the
//! returned type is inferred and dimension mismatches are caught at
//! compile time. See the [tests](https://github.com/TeddyHuang-00/siunit)
//! for worked examples.
//!
//! # Nightly Required
//!
//! This crate uses `#![feature(generic_const_exprs)]` for const arithmetic
//! on dimension exponents. Stable support tracked at [rust#76560].

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![no_std]

mod dim;
mod ops;
mod units;

pub use dim::Dim;
pub use ops::{hertzs, joules, kilograms, meters, newtons, pascals, scalar, seconds, watts};
pub use units::*;
