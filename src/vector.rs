//! Vector algebra traits for use as custom numeric types in
//! [`Unit`](crate::Unit).
//!
//! These traits decouple generic vector operations (dot product, cross product,
//! norm) from `Unit`'s dimension arithmetic, letting downstream types implement
//! whichever operations apply and get the same functionality when wrapped in
//! [`Unit`](crate::Unit) overloading.
//!
//! # Example
//!
//! ```
//! use fizix::vector::*;
//!
//! struct Vector3([f64; 3]);
//!
//! impl CrossProduct for Vector3 {
//!     type Output = Self;
//!
//!     fn cross(self, rhs: Self) -> Self::Output {
//!         Self([
//!             self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
//!             self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
//!             self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
//!         ])
//!     }
//! }
//!
//! impl DotProduct for Vector3 {
//!     type Output = f64;
//!
//!     fn dot(self, rhs: Self) -> Self::Output {
//!         self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2]
//!     }
//! }
//!
//! impl VectorNorm for Vector3 {
//!     type Output = f64;
//!
//!     fn norm(self) -> Self::Output {
//!         (self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2)).sqrt()
//!     }
//! }
//!
//! impl Vector for Vector3 {}
//! ```

/// A container that support dot product, cross product and norm
pub trait Vector<Rhs = Self>: DotProduct<Rhs> + CrossProduct<Rhs> + VectorNorm {}

/// The binary dot product operator `⋅`
pub trait DotProduct<Rhs = Self> {
    /// The resulting type after applying the `⋅` operator.
    type Output;

    /// Performs the `⋅` operation
    fn dot(self, rhs: Rhs) -> Self::Output;
}

/// The binary cross product operator `×`
pub trait CrossProduct<Rhs = Self> {
    /// The resulting type after applying the `×` operator.
    type Output;

    /// Performs the `×` operation
    fn cross(self, rhs: Rhs) -> Self::Output;
}

/// The unary norm operator `|⋅|`
pub trait VectorNorm {
    /// The resulting type after applying the `|⋅|` operator.
    type Output;

    /// Performs the `|⋅|` operation
    fn norm(self) -> Self::Output;
}
