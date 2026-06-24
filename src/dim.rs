use core::ops::{Add, Div, Mul, Neg, Sub};

/// A physical quantity with compile-time dimension checking.
///
/// Seven const generic parameters correspond to the SI base dimensions for
/// 1. length (Meter),
/// 2. mass (kiloGram),
/// 3. time (Second),
/// 4. electric current (Amphere),
/// 5. thermodynamic temperature (Kelvin),
/// 6. amount of substance (mOle), and
/// 7. luminous intensity (Candela)
///
/// # Examples
///
/// ```
/// use siunit::Unit;
///
/// let d: Unit<f64, 1> = Unit::new(5.0); // metres
/// assert_eq!(d.value, 5.0);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Unit<
    // Value type
    V,
    // Length exponent (Meter)
    const M: i8 = 0,
    // Mass exponent (kiloGram)
    const G: i8 = 0,
    // Time exponent (Second)
    const S: i8 = 0,
    // Current exponent (Amphere)
    const A: i8 = 0,
    // Temperature exponent (Kelvin)
    const K: i8 = 0,
    // Amount exponent (mOle)
    const O: i8 = 0,
    // Intensity exponent (Candela)
    const C: i8 = 0,
> {
    /// The numeric value of this quantity.
    pub value: V,
}

impl<const M: i8, const G: i8, const S: i8, const A: i8, const K: i8, const O: i8, const C: i8, V>
    Unit<V, M, G, S, A, K, O, C>
{
    /// Create a new quantity with the given numeric value.
    #[inline]
    #[must_use]
    pub const fn new(value: V) -> Self {
        Self { value }
    }
}

// Raw values can only be directly casted into scalar
impl<V> From<V> for Unit<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}

// Add / Sub — same dimensions only
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
        > $trait for Unit<V, M, G, S, A, K, O, C>
        where
            V: $trait<Output = V>,
        {
            type Output = Self;

            #[inline]
            fn $fn(self, rhs: Self) -> Self::Output {
                Unit::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_add_sub!(Add, add);
impl_add_sub!(Sub, sub);

// Mul / Div — dimension exponents calculation
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
        > $trait<Unit<Vi, M2, G2, S2, A2, K2, O2, C2>> for Unit<Vi, M1, G1, S1, A1, K1, O1, C1>
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
            type Output = Unit<
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
            fn $fn(self, rhs: Unit<Vi, M2, G2, S2, A2, K2, O2, C2>) -> Self::Output {
                Unit::new(self.value.$fn(rhs.value))
            }
        }

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
        > $trait<Vi> for Unit<Vi, M, G, S, A, K, O, C>
        where
            Vi: $trait<Output = V>,
        {
            type Output = Unit<V, M, G, S, A, K, O, C>;

            #[inline]
            fn $fn(self, rhs: Vi) -> Self::Output {
                Unit::new(self.value.$fn(rhs))
            }
        }
    };
}

impl_mul_div!(Mul, mul, +);
impl_mul_div!(Div, div, -);

// Neg — value negation, dimensions unchanged
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
> Neg for Unit<Vi, M, G, S, A, K, O, C>
where
    Vi: Neg<Output = V>,
{
    type Output = Unit<V, M, G, S, A, K, O, C>;

    #[inline]
    fn neg(self) -> Self::Output {
        Unit::new(-self.value)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn test_initialization() {
        struct U;
        let f64 = Unit::<f64>::new(1.0);
        let f32 = Unit::<f32>::new(1.0);
        let u64 = Unit::<u64>::new(1);
        let u32 = Unit::<u32>::new(1);
        let u16 = Unit::<u16>::new(1);
        let u8 = Unit::<u8>::new(1);
        let bool = Unit::<bool>::new(true);
        let aribtrary = Unit::<U>::new(U);

        assert_eq!(f64.type_id(), TypeId::of::<Unit<f64>>());
        assert_eq!(f32.type_id(), TypeId::of::<Unit<f32>>());
        assert_eq!(u64.type_id(), TypeId::of::<Unit<u64>>());
        assert_eq!(u32.type_id(), TypeId::of::<Unit<u32>>());
        assert_eq!(u16.type_id(), TypeId::of::<Unit<u16>>());
        assert_eq!(u8.type_id(), TypeId::of::<Unit<u8>>());
        assert_eq!(bool.type_id(), TypeId::of::<Unit<bool>>());
        assert_eq!(aribtrary.type_id(), TypeId::of::<Unit<U>>());
    }
}
