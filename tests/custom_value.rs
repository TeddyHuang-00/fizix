//! Integration test: verify that `Unit` works correctly with an arbitrary
//! custom value type implementing all traits required by `Unit`.
//!
//! This tests the full trait bounds: `Clone`, `Copy`, `Debug`, `Default`,
//! `Eq`, `Ord`, `PartialEq`, `PartialOrd`, `Display`, `Add`, `Sub`, `Mul`,
//! `Div`, `Neg`, and `From<i32>`.

use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

use siunit::*;
use typenum::*;

/// A custom value type representing a measurement in meters, stored as mm
/// (i64). Implements all traits Unit requires.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct MyValue(i64);

impl fmt::Display for MyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for MyValue {
    type Output = MyValue;

    fn add(self, rhs: MyValue) -> MyValue {
        MyValue(self.0 + rhs.0)
    }
}
impl Sub for MyValue {
    type Output = MyValue;

    fn sub(self, rhs: MyValue) -> MyValue {
        MyValue(self.0 - rhs.0)
    }
}
impl Mul for MyValue {
    type Output = MyValue;

    fn mul(self, rhs: MyValue) -> MyValue {
        MyValue(self.0 * rhs.0)
    }
}
impl Div for MyValue {
    type Output = MyValue;

    fn div(self, rhs: MyValue) -> MyValue {
        MyValue(self.0 / rhs.0)
    }
}
impl Neg for MyValue {
    type Output = MyValue;

    fn neg(self) -> MyValue {
        MyValue(-self.0)
    }
}
impl From<i32> for MyValue {
    fn from(v: i32) -> MyValue {
        MyValue(v as i64)
    }
}

// Helper: assert Display with both feature modes
macro_rules! assert_display {
    ($x:expr, $pretty:literal, $ascii:literal) => {
        #[cfg(feature = "pretty-display")]
        assert_eq!(format!("{}", $x), $pretty);
        #[cfg(not(feature = "pretty-display"))]
        assert_eq!(format!("{}", $x), $ascii);
    };
}

// Test: Creation (Unit::new, Unit::from, .value access)

#[test]
fn test_custom_creation() {
    let m = Meter::new(MyValue(500));
    assert_eq!(m.value, MyValue(500));

    // From<i32>, then From<MyValue>
    let s: Unit<MyValue> = Unit::from(MyValue::from(42i32));
    assert_eq!(s.value, MyValue(42));
}

// Test: Add / Sub (same dimension)

#[test]
fn test_custom_add_sub_same_dim() {
    let a = Meter::new(MyValue(300));
    let b = Meter::new(MyValue(200));

    let sum = a + b;
    assert_eq!(sum.value, MyValue(500));

    let diff = a - b;
    assert_eq!(diff.value, MyValue(100));
}

// Test: Mul / Div (cross dimension)

#[test]
fn test_custom_mul_div_cross_dim() {
    // Meter / Second → Speed (m⋅s⁻¹)
    let d = Meter::new(MyValue(500));
    let t = Second::new(MyValue(20));
    let speed: Speed<_> = d / t;
    assert_eq!(speed.value, MyValue(25));

    // Meter * Kilogram → Unit<Meters, P1, P1> (force-like: kg⋅m)
    let m = Meter::new(MyValue(10));
    let kg = Kilogram::new(MyValue(3));
    let force_like: Unit<_, P1, P1> = m * kg;
    assert_eq!(force_like.value, MyValue(30));
}

// Test: Negation

#[test]
fn test_custom_neg() {
    let m = Meter::new(MyValue(100));
    let neg = -m;
    assert_eq!(neg.value, MyValue(-100));
}

// Test: Default

#[test]
fn test_custom_default() {
    let m: Meter<MyValue> = Default::default();
    assert_eq!(m.value, MyValue(0));
}

// Test: Clone / Copy (no double-free issues)

#[test]
fn test_custom_clone_copy() {
    let m = Meter::new(MyValue(42));

    // Clone
    let cloned = m.clone();
    assert_eq!(cloned.value, MyValue(42));

    // Copy - both original and copy are independently valid
    let copied = m;
    assert_eq!(copied.value, MyValue(42));
    assert_eq!(m.value, MyValue(42)); // original still accessible after copy
}

// Test: Comparison (Eq, Ord)

#[test]
fn test_custom_comparison() {
    let a = Meter::new(MyValue(100));
    let b = Meter::new(MyValue(200));
    let c = Meter::new(MyValue(100));

    // Eq / PartialEq
    assert_eq!(a, c);
    assert_ne!(a, b);

    // Ord / PartialOrd
    assert!(a < b);
    assert!(b > a);
    assert!(a <= c);
    assert!(b >= a);
}

// Test: Display

#[test]
fn test_custom_display() {
    // Single unit: exponent 1 → no exponent marker in either mode
    let m = Meter::new(MyValue(500));
    assert_display!(m, "500 m", "500 m");

    // Composite: Speed = m⋅s⁻¹
    let d = Meter::new(MyValue(500));
    let t = Second::new(MyValue(20));
    let speed = d / t;
    assert_display!(speed, "25 m⋅s⁻¹", "25 m*s^-1");

    // Derived: Newton = kg⋅m⋅s⁻²
    let n = Newton::new(MyValue(100));
    assert_display!(n, "100 kg⋅m⋅s⁻²", "100 kg*m*s^-2");
}

// Test: Scalar from i32 via chained From impl

#[test]
fn test_custom_scalar_from() {
    // From<i32> for Meters
    let m: MyValue = 42i32.into();
    // From<Meters> for Unit<Meters>
    let s: Unit<_> = m.into();
    assert_eq!(s.value, MyValue(42));
}
