use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::Dim;

// ============================================================
// Add / Sub — same dimensions only
// ============================================================

macro_rules! impl_add_sub {
    ($trait:ident, $fn:ident) => {
        impl<const L: i8, const M: i8, const T: i8, V> $trait for Dim<L, M, T, V>
        where
            V: $trait<Output = V>,
        {
            type Output = Self;

            #[inline]
            fn $fn(self, rhs: Self) -> Self::Output {
                Dim::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_add_sub!(Add, add);
impl_add_sub!(Sub, sub);

// ============================================================
// Mul — exponents add
// ============================================================

impl<const L1: i8, const M1: i8, const T1: i8, const L2: i8, const M2: i8, const T2: i8, V>
    Mul<Dim<L2, M2, T2, V>> for Dim<L1, M1, T1, V>
where
    [(); (L1 + L2).unsigned_abs() as usize]:,
    [(); (M1 + M2).unsigned_abs() as usize]:,
    [(); (T1 + T2).unsigned_abs() as usize]:,
    V: Mul<Output = V>,
{
    type Output = Dim<{ L1 + L2 }, { M1 + M2 }, { T1 + T2 }, V>;

    #[inline]
    fn mul(self, rhs: Dim<L2, M2, T2, V>) -> Self::Output {
        Dim::new(self.value * rhs.value)
    }
}

// ============================================================
// Div — exponents subtract
// ============================================================

impl<const L1: i8, const M1: i8, const T1: i8, const L2: i8, const M2: i8, const T2: i8, V>
    Div<Dim<L2, M2, T2, V>> for Dim<L1, M1, T1, V>
where
    [(); (L1 - L2).unsigned_abs() as usize]:,
    [(); (M1 - M2).unsigned_abs() as usize]:,
    [(); (T1 - T2).unsigned_abs() as usize]:,
    V: Div<Output = V>,
{
    type Output = Dim<{ L1 - L2 }, { M1 - M2 }, { T1 - T2 }, V>;

    #[inline]
    fn div(self, rhs: Dim<L2, M2, T2, V>) -> Self::Output {
        Dim::new(self.value / rhs.value)
    }
}

// ============================================================
// Neg — value negation, dimensions unchanged
// ============================================================

impl<const L: i8, const M: i8, const T: i8, V> Neg for Dim<L, M, T, V>
where
    V: Neg<Output = V>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

// ============================================================
// Helper constructors — lets users write `meters(5.0)` instead
// of `Dim::<1,0,0>::new(5.0)` or relying on type inference.
// ============================================================

macro_rules! impl_ctor {
    ($(($fn:ident, $l:expr, $m:expr, $t:expr, $doc:literal)),+) => {
        $(
            #[doc = $doc]
            #[inline]
            pub const fn $fn<V>(v: V) -> Dim<$l, $m, $t, V> { Dim::new(v) }
        )+
    };
}

impl_ctor! {
    (meters,   1, 0, 0, "Create a quantity in metres (m)"),
    (kilograms, 0, 1, 0, "Create a quantity in kilograms (kg)"),
    (seconds,   0, 0, 1, "Create a quantity in seconds (s)"),
    (newtons,   1, 1, -2, "Create a quantity in newtons (N, kg·m·s⁻²)"),
    (joules,    2, 1, -2, "Create a quantity in joules (J, N·m)"),
    (watts,     2, 1, -3, "Create a quantity in watts (W, J·s⁻¹)"),
    (pascals,  -1, 1, -2, "Create a quantity in pascals (Pa, N·m⁻²)"),
    (hertzs,    0, 0, -1, "Create a quantity in hertz (Hz, s⁻¹)"),
    (scalar,    0, 0, 0,  "Create a dimensionless quantity")
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use crate::*;

    const EPSILON: f64 = 1e-10;

    fn is_close(computed: f64, expected: f64) {
        assert!((computed - expected).abs() < EPSILON)
    }

    #[test]
    fn test_speed() {
        let d = meters(100.0);
        let t = seconds(10.0);
        let v: Speed<f64> = d / t;
        is_close(v.value, 10.0);
    }

    #[test]
    fn test_round_trip() {
        let d = meters(100.0);
        let t = seconds(10.0);
        let v = d / t;
        let d2 = v * t;
        is_close(d2.value, 100.0);
    }

    #[test]
    fn test_area() {
        let a = meters(5.0) * meters(3.0);
        is_close(a.value, 15.0);
    }

    #[test]
    fn test_add_same_unit() {
        let l1 = meters(10.0);
        let l2 = meters(20.0);
        let l = l1 + l2;
        is_close(l.value, 30.0);
    }

    #[test]
    fn test_sub_same_unit() {
        let t1 = seconds(20.0);
        let t2 = seconds(5.0);
        let t = t1 - t2;
        is_close(t.value, 15.0);
    }

    #[test]
    fn test_neg() {
        let m = kilograms(5.0);
        let m = -m;
        is_close(m.value, -5.0);
    }

    #[test]
    fn test_frequency() {
        let period = seconds(0.02);
        let freq: Hertz<f64> = scalar(1.0) / period;
        is_close(freq.value, 50.0);
    }

    #[test]
    fn test_newton() {
        let force: Newton<f64> = kilograms(2.0) * meters(9.8) / (seconds(1.0) * seconds(1.0));
        is_close(force.value, 19.6);
    }

    #[test]
    fn test_joule() {
        let e = newtons(10.0) * meters(5.0);
        is_close(e.value, 50.0);
    }

    // The following should NOT compile — uncomment to verify:
    // #[test]
    // fn test_unit_mismatch_add() {
    //     let _ = meters(10.0) + seconds(5.0);
    // }
}
