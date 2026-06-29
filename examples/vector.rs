//! Using siunit with a custom numeric type: 3D vector arithmetic.
//!
//! Demonstrates type-safe physics with a custom `Vector` container including
//! Lorentz force `F = q(E + v × B)`. Note that cross product is mapped to `Mul`
//! (dimensionally multiplicative) while dot product is a named `.dot()` method.
//!
//! Run with: `cargo run --example vector`

use core::{
    fmt::{self, Display, Formatter},
    ops::{Add, Div, Mul, Neg, Sub},
};

use siunit::{vector::*, *};

/// A custom 3D vector container for directional calculation
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
struct Vector3([f64; 3]);

impl Vector3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0[0], self.0[1], self.0[2])
    }
}

// Add/Sub is only allowed between `Vector`s
impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}
// Mul/Div with scalar: component-wise scaling
impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}
impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3([rhs.0[0] * self, rhs.0[1] * self, rhs.0[2] * self])
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector3([-self.0[0], -self.0[1], -self.0[2]])
    }
}

// Vector arithmetic
impl CrossProduct for Vector3 {
    type Output = Self;

    fn cross(self, rhs: Self) -> Self::Output {
        Vector3([
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
        ])
    }
}

impl DotProduct for Vector3 {
    type Output = f64;

    fn dot(self, rhs: Self) -> Self::Output {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2]
    }
}

impl VectorNorm for Vector3 {
    type Output = f64;

    fn norm(self) -> Self::Output {
        (self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2)).sqrt()
    }
}

impl Vector for Vector3 {}

fn main() {
    // An object of 1 kg weight
    let object_mass = kilogram(1.0);
    // Three forces in different directions is applied to the object
    let force_a = newton(Vector3::new(1.0, 0.0, 0.0));
    let force_b = newton(Vector3::new(0.0, 2.0, 0.0));
    let force_c = newton(Vector3::new(0.0, 0.0, 2.0));
    // Now we can have the total acceleration
    let acceleration: Acceleration<_> = (force_a + force_b + force_c) / object_mass;
    println!(
        "Acceleration is {} ({:.3})",
        acceleration,
        acceleration.norm()
    );

    // Now a particle with 1 C charge (I know, I know, this is purely fictional so
    // please hold yourself, alright?)
    let charge = coulomb(1.0);
    // with some initial velocity
    let velocity = meter(Vector3::new(0.0, 1.0, 0.0)) / second(1.0);
    // And create electric and magnetic fields
    let electric_field = volt(Vector3::new(0.0, 1.0, 0.0)) / meter(1.0);
    let magnetic_field = tesla(Vector3::new(0.0, 0.0, 10.0));
    // Now we have the total force on the particle (F = q (E + v × B))
    let force: Newton<_> = charge * (electric_field + velocity.cross(magnetic_field));
    println!("Total force in EM field is {} ({:.3})", force, force.norm());
}
