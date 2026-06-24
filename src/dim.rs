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

    #[derive(Debug, Default, Eq, PartialEq)]
    struct X;

    type FloatScalar = Unit<f64>;
    type IntScalar = Unit<i64>;
    type ArbitraryScalar = Unit<X>;

    const TYPE_FLOAT_SCALAR: TypeId = TypeId::of::<FloatScalar>();
    const TYPE_INT_SCALAR: TypeId = TypeId::of::<IntScalar>();
    const TYPE_ARBITRARY_SCALAR: TypeId = TypeId::of::<ArbitraryScalar>();

    type FloatUnit = Unit<f64, 1, 2, 3, 4, 5, 6, 7>;
    type FloatUnitSquared = Unit<f64, 2, 4, 6, 8, 10, 12, 14>;
    type IntUnit = Unit<i64, 1, 2, 3, 4, 5, 6, 7>;
    type IntUnitSquared = Unit<i64, 2, 4, 6, 8, 10, 12, 14>;
    type ArbitraryUnit = Unit<X, 1, 2, 3, 4, 5, 6, 7>;

    const TYPE_FLOAT_UNIT: TypeId = TypeId::of::<FloatUnit>();
    const TYPE_FLOAT_UNIT_SQUARED: TypeId = TypeId::of::<FloatUnitSquared>();
    const TYPE_INT_UNIT: TypeId = TypeId::of::<IntUnit>();
    const TYPE_INT_UNIT_SQUARED: TypeId = TypeId::of::<IntUnitSquared>();
    const TYPE_ARBITRARY_UNIT: TypeId = TypeId::of::<ArbitraryUnit>();

    // Test for scalar creation

    #[test]
    fn test_scalar_initialization() {
        let float = FloatScalar::new(1.0);
        let int = IntScalar::new(1);
        let arbitrary = ArbitraryScalar::new(X);

        assert_eq!(float.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int.type_id(), TYPE_INT_SCALAR);
        assert_eq!(arbitrary.type_id(), TYPE_ARBITRARY_SCALAR);

        assert_eq!(float.value, 1.0);
        assert_eq!(int.value, 1);
        assert_eq!(arbitrary.value, X);
    }

    #[test]
    fn test_scalar_from_value() {
        let float: FloatScalar = 1.0.into();
        let int: IntScalar = 1.into();
        let arbitrary: ArbitraryScalar = X.into();

        assert_eq!(float.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int.type_id(), TYPE_INT_SCALAR);
        assert_eq!(arbitrary.type_id(), TYPE_ARBITRARY_SCALAR);

        assert_eq!(float.value, 1.0);
        assert_eq!(int.value, 1);
        assert_eq!(arbitrary.value, X);
    }

    // Test for scalar arithmic

    #[test]
    fn test_scalar_neg() {
        let float: FloatScalar = 10.0.into();
        let int: IntScalar = 10.into();

        let float_neg = -float;
        let int_neg = -int;

        // Calculation should NOT change unit type
        assert_eq!(float_neg.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_neg.type_id(), TYPE_INT_SCALAR);

        // The values should reflect the result
        assert_eq!(float_neg.value, -10.0);
        assert_eq!(int_neg.value, -10);
    }

    #[test]
    fn test_scalar_add_sub() {
        let float: FloatScalar = 10.0.into();
        let int: IntScalar = 10.into();

        let float_add = float + float;
        let float_sub = float - float;
        let int_add = int + int;
        let int_sub = int - int;

        // Calculation should NOT change unit type
        assert_eq!(float_add.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(float_sub.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_add.type_id(), TYPE_INT_SCALAR);
        assert_eq!(int_sub.type_id(), TYPE_INT_SCALAR);

        // The values should reflect the result
        assert_eq!(float_add.value, 20.0);
        assert_eq!(float_sub.value, 0.0);
        assert_eq!(int_add.value, 20);
        assert_eq!(int_sub.value, 0);
    }

    #[test]
    fn test_scalar_mul_div() {
        let float: FloatScalar = 10.0.into();
        let int: IntScalar = 10.into();

        let float_mul = float * float;
        let float_div = float / float;
        let int_mul = int * int;
        let int_div = int / int;

        // Calculation should NOT change unit type
        assert_eq!(float_mul.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(float_div.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_mul.type_id(), TYPE_INT_SCALAR);
        assert_eq!(int_div.type_id(), TYPE_INT_SCALAR);

        // The values should reflect the result
        assert_eq!(float_mul.value, 100.0);
        assert_eq!(float_div.value, 1.0);
        assert_eq!(int_mul.value, 100);
        assert_eq!(int_div.value, 1);
    }

    #[test]
    fn test_scalar_mul_div_raw() {
        let float: FloatScalar = 10.0.into();
        let int: IntScalar = 10.into();

        let float_mul_lhs = float * 10.0;
        let float_div_lhs = float / 10.0;
        let int_mul_lhs = int * 10;
        let int_div_lhs = int / 10;

        // Calculation should NOT change unit type
        assert_eq!(float_mul_lhs.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(float_div_lhs.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_mul_lhs.type_id(), TYPE_INT_SCALAR);
        assert_eq!(int_div_lhs.type_id(), TYPE_INT_SCALAR);

        // The values should reflect the result
        assert_eq!(float_mul_lhs.value, 100.0);
        assert_eq!(float_div_lhs.value, 1.0);
        assert_eq!(int_mul_lhs.value, 100);
        assert_eq!(int_div_lhs.value, 1);
    }

    // Test for scalar derives

    #[test]
    fn test_scalar_default() {
        let float = FloatScalar::default();
        let int = IntScalar::default();
        let arbitrary = ArbitraryScalar::default();

        assert_eq!(float.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int.type_id(), TYPE_INT_SCALAR);
        assert_eq!(arbitrary.type_id(), TYPE_ARBITRARY_SCALAR);

        assert_eq!(float.value, f64::default());
        assert_eq!(int.value, i64::default());
        assert_eq!(arbitrary.value, X);
    }

    #[test]
    fn test_scalar_cmp() {
        let float = FloatScalar::new(10.0);
        let int = IntScalar::new(10);

        assert!(float < float * 2.0);
        assert!(int < int * 2);

        assert!(float == float);
        assert!(int == int);
    }

    // Test for non-scalar creation

    #[test]
    fn test_non_scalar_initialization() {
        let float = FloatUnit::new(1.0);
        let int = IntUnit::new(1);
        let arbitrary = ArbitraryUnit::new(X);

        assert_eq!(float.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(int.type_id(), TYPE_INT_UNIT);
        assert_eq!(arbitrary.type_id(), TYPE_ARBITRARY_UNIT);

        assert_eq!(float.value, 1.0);
        assert_eq!(int.value, 1);
        assert_eq!(arbitrary.value, X);
    }

    // Test for non-scalar arithmic

    #[test]
    fn test_non_scalar_neg() {
        let float = FloatUnit::new(10.0);
        let int = IntUnit::new(10);

        let float_neg = -float;
        let int_neg = -int;

        // Calculation should NOT change unit type
        assert_eq!(float_neg.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(int_neg.type_id(), TYPE_INT_UNIT);

        // The values should reflect the result
        assert_eq!(float_neg.value, -10.0);
        assert_eq!(int_neg.value, -10);
    }

    #[test]
    fn test_non_scalar_add_sub() {
        let float = FloatUnit::new(10.0);
        let int = IntUnit::new(10);

        let float_add = float + float;
        let float_sub = float - float;
        let int_add = int + int;
        let int_sub = int - int;

        // Calculation should NOT change unit type
        assert_eq!(float_add.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(float_sub.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(int_add.type_id(), TYPE_INT_UNIT);
        assert_eq!(int_sub.type_id(), TYPE_INT_UNIT);

        // The values should reflect the result
        assert_eq!(float_add.value, 20.0);
        assert_eq!(float_sub.value, 0.0);
        assert_eq!(int_add.value, 20);
        assert_eq!(int_sub.value, 0);
    }

    #[test]
    fn test_non_scalar_mul_div() {
        let float = FloatUnit::new(10.0);
        let int = IntUnit::new(10);

        let float_mul = float * float;
        let float_div = float / float;
        let int_mul = int * int;
        let int_div = int / int;

        // Calculation SHOULD change unit type
        assert_eq!(float_mul.type_id(), TYPE_FLOAT_UNIT_SQUARED);
        assert_eq!(float_div.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_mul.type_id(), TYPE_INT_UNIT_SQUARED);
        assert_eq!(int_div.type_id(), TYPE_INT_SCALAR);

        // The values should reflect the result
        assert_eq!(float_mul.value, 100.0);
        assert_eq!(float_div.value, 1.0);
        assert_eq!(int_mul.value, 100);
        assert_eq!(int_div.value, 1);
    }

    #[test]
    fn test_non_scalar_mul_div_raw() {
        let float = FloatUnit::new(10.0);
        let int = IntUnit::new(10);

        let float_mul_lhs = float * 10.0;
        let float_div_lhs = float / 10.0;
        let int_mul_lhs = int * 10;
        let int_div_lhs = int / 10;

        // Calculation should NOT change unit type
        assert_eq!(float_mul_lhs.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(float_div_lhs.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(int_mul_lhs.type_id(), TYPE_INT_UNIT);
        assert_eq!(int_div_lhs.type_id(), TYPE_INT_UNIT);

        // The values should reflect the result
        assert_eq!(float_mul_lhs.value, 100.0);
        assert_eq!(float_div_lhs.value, 1.0);
        assert_eq!(int_mul_lhs.value, 100);
        assert_eq!(int_div_lhs.value, 1);
    }

    // Test for non-scalar creation

    #[test]
    fn test_non_scalar_default() {
        let float = FloatUnit::default();
        let int = IntUnit::default();
        let arbitrary = ArbitraryUnit::default();

        assert_eq!(float.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(int.type_id(), TYPE_INT_UNIT);
        assert_eq!(arbitrary.type_id(), TYPE_ARBITRARY_UNIT);

        assert_eq!(float.value, f64::default());
        assert_eq!(int.value, i64::default());
        assert_eq!(arbitrary.value, X);
    }

    #[test]
    fn test_non_scalar_cmp() {
        let float = FloatUnit::new(10.0);
        let int = IntUnit::new(10);

        assert!(float < float * 2.0);
        assert!(int < int * 2);

        assert!(float == float);
        assert!(int == int);
    }
}
