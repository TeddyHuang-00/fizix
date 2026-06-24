use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::Dim;

// ============================================================
// Add / Sub — same dimensions only
// ============================================================

macro_rules! impl_add_sub {
    ($trait:ident, $fn:ident) => {
        impl<
            const M: i8,
            const G: i8,
            const S: i8,
            const A: i8,
            const K: i8,
            const O: i8,
            const C: i8,
            V,
        > $trait for Dim<V, M, G, S, A, K, O, C>
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
// Mul / Div — dimension exponents calculation
// ============================================================

macro_rules! impl_mul_div {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl<
            const M1: i8,
            const G1: i8,
            const S1: i8,
            const A1: i8,
            const K1: i8,
            const O1: i8,
            const C1: i8,
            const M2: i8,
            const G2: i8,
            const S2: i8,
            const A2: i8,
            const K2: i8,
            const O2: i8,
            const C2: i8,
            Vi,
            V,
        > $trait<Dim<Vi, M2, G2, S2, A2, K2, O2, C2>> for Dim<Vi, M1, G1, S1, A1, K1, O1, C1>
        where
            [(); (M1 $op M2).unsigned_abs() as usize]:,
            [(); (G1 $op G2).unsigned_abs() as usize]:,
            [(); (S1 $op S2).unsigned_abs() as usize]:,
            [(); (A1 $op A2).unsigned_abs() as usize]:,
            [(); (K1 $op K2).unsigned_abs() as usize]:,
            [(); (O1 $op O2).unsigned_abs() as usize]:,
            [(); (C1 $op C2).unsigned_abs() as usize]:,
            Vi: $trait<Output = V>,
        {
            type Output = Dim<
                V,
                { M1 $op M2 },
                { G1 $op G2 },
                { S1 $op S2 },
                { A1 $op A2 },
                { K1 $op K2 },
                { O1 $op O2 },
                { C1 $op C2 }
            >;

            #[inline]
            fn $fn(self, rhs: Dim<Vi, M2, G2, S2, A2, K2, O2, C2>) -> Self::Output {
                Dim::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_mul_div!(Mul, mul, +);
impl_mul_div!(Div, div, -);

// ============================================================
// Neg — value negation, dimensions unchanged
// ============================================================

impl<
    const M: i8,
    const G: i8,
    const S: i8,
    const A: i8,
    const K: i8,
    const O: i8,
    const C: i8,
    Vi,
    V,
> Neg for Dim<Vi, M, G, S, A, K, O, C>
where
    Vi: Neg<Output = V>,
{
    type Output = Dim<V, M, G, S, A, K, O, C>;

    #[inline]
    fn neg(self) -> Self::Output {
        Dim::new(-self.value)
    }
}

// ============================================================
// Helper constructors — lets users write `meters(5.0)` instead
// of `Dim::<1,0,0>::new(5.0)` or relying on type inference.
// ============================================================

macro_rules! impl_ctor {
    ($(($fn:ident, $doc:literal, $($val:expr),+)),+) => {
        $(
            #[doc = $doc]
            #[inline]
            pub const fn $fn<V>(v: V) -> Dim<V$(, $val)+> { Dim::new(v) }
        )+
    };
}

impl_ctor! {
    (meters, "Create a quantity in metres (m)",             1, 0,  0, 0, 0, 0, 0),
    (kilograms, "Create a quantity in kilograms (kg)",      0, 1,  0, 0, 0, 0, 0),
    (seconds, "Create a quantity in seconds (s)",           0, 0,  1, 0, 0, 0, 0),
    (newtons, "Create a quantity in newtons (N, kg·m·s⁻²)", 1, 1, -2, 0, 0, 0, 0),
    (joules, "Create a quantity in joules (J, N·m)",        2, 1, -2, 0, 0, 0, 0),
    (watts, "Create a quantity in watts (W, J·s⁻¹)",        2, 1, -3, 0, 0, 0, 0),
    (pascals, "Create a quantity in pascals (Pa, N·m⁻²)",  -1, 1, -2, 0, 0, 0, 0),
    (hertzs, "Create a quantity in hertz (Hz, s⁻¹)",        0, 0, -1, 0, 0, 0, 0),
    (scalar, "Create a dimensionless quantity",             0, 0,  0, 0, 0, 0, 0)
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
        let v = d / t;
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
        let freq = scalar(1.0) / period;
        is_close(freq.value, 50.0);
    }

    #[test]
    fn test_newton() {
        let force = kilograms(2.0) * meters(9.8) / (seconds(1.0) * seconds(1.0));
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
