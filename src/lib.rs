//! **fizix**: Compile-time checked SI units via type-level integers.
//!
//! Every physical quantity carries its dimension as [`typenum`] type
//! parameters. Dimension mismatches are caught at compile time with zero
//! runtime overhead.
//!
//! # Example
//!
//! ```rust
//! use fizix::*;
//!
//! let d = Meter::new(100.0);
//! let t = Second::new(10.0);
//! let _ = d + d;               // OK: same dimension
//! // let _ = d + t;            // ❌ compile error: dimension mismatch
//! ```
//!
//! Type-safe dimension arithmetic is achieved through `Mul`/`Div` with
//! [`typenum`] type-level arithmetic on dimension exponents.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![no_std]

mod dim;
pub mod macros;
mod units;
pub mod vector;

pub use dim::Unit;
// Re-export paste for macro_rules hygiene.
#[doc(hidden)]
pub use paste;
pub use units::*;
