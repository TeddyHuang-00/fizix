//! **siunit**: Compile-time checked SI units via type-level integers.
//!
//! Every physical quantity carries its dimension as [`typenum`] type
//! parameters. Dimension mismatches are caught at compile time with zero
//! runtime overhead.
//!
//! # Example
//!
//! ```rust
//! use siunit::*;
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
#![cfg_attr(not(feature = "std"), no_std)]

mod dim;
mod macros;
mod units;

/// Hidden re-exports so [`alias_types!`] and [`alias_units!`] can resolve
/// typenum type-level integers without users needing `use typenum::*;`.
#[doc(hidden)]
pub mod __typ {
    #[allow(clippy::wildcard_imports)]
    pub use typenum::*;
}

/// Commonly used typenum type-level integers, re-exported at the crate root
/// so error messages show clean paths like `siunit::Z0` instead of
/// `siunit::__typ::Z0`.
#[doc(inline)]
pub use __typ::{Integer, N1, N2, N3, N4, N5, N6, N7, N8, P1, P2, P3, P4, P5, P6, P7, P8, Z0};
pub use dim::Unit;
pub use units::*;
