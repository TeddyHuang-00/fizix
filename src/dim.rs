use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use typenum::Z0;

/// A physical quantity with compile-time dimension checking.
///
/// Seven type parameters corresponding to the SI base dimensions for
/// 1. length (Meter),
/// 2. mass (kiloGram),
/// 3. time (Second),
/// 4. electric current (Ampere),
/// 5. thermodynamic temperature (Kelvin),
/// 6. amount of substance (mOle), and
/// 7. luminous intensity (Candela)
///
/// Dimension arithmetic is performed using [`typenum`] type-level integers.
///
/// # Examples
///
/// ```
/// use siunit::Unit;
///
/// let d: Unit<f64, typenum::consts::P1> = Unit::new(5.0); // metres
/// assert_eq!(d.value, 5.0);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Unit<
    V,
    // Length exponent (Meter)
    M = Z0,
    // Mass exponent (kiloGram)
    G = Z0,
    // Time exponent (Second)
    S = Z0,
    // Current exponent (Ampere)
    A = Z0,
    // Temperature exponent (Kelvin)
    K = Z0,
    // Amount exponent (mOle)
    O = Z0,
    // Intensity exponent (Candela)
    C = Z0,
> {
    /// The numeric value of this quantity.
    pub value: V,
    _phantom: PhantomData<(M, G, S, A, K, O, C)>,
}

impl<V, M, G, S, A, K, O, C> Unit<V, M, G, S, A, K, O, C> {
    /// Create a new quantity with the given numeric value.
    #[inline]
    #[must_use]
    pub const fn new(value: V) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
}

// Raw values can only be directly cast into scalar
impl<V> From<V> for Unit<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}

// Add / Sub — same type
macro_rules! impl_add_sub {
    ($trait:ident, $fn:ident) => {
        impl<U, V, M, G, S, A, K, O, C> $trait for Unit<U, M, G, S, A, K, O, C>
        where
            U: $trait<Output = V>,
        {
            type Output = Unit<V, M, G, S, A, K, O, C>;

            #[inline]
            fn $fn(self, rhs: Self) -> Self::Output {
                Unit::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_add_sub!(Add, add);
impl_add_sub!(Sub, sub);

// Mul / Div — dimension exponents addition/subtraction
macro_rules! impl_mul_div {
    ($trait:ident, $fn:ident, $op_trait:ident) => {
        impl<U, V, M1, G1, S1, A1, K1, O1, C1, M2, G2, S2, A2, K2, O2, C2>
            $trait<Unit<U, M2, G2, S2, A2, K2, O2, C2>> for Unit<U, M1, G1, S1, A1, K1, O1, C1>
        where
            U: $trait<Output = V>,
            M1: $op_trait<M2>,
            G1: $op_trait<G2>,
            S1: $op_trait<S2>,
            A1: $op_trait<A2>,
            K1: $op_trait<K2>,
            O1: $op_trait<O2>,
            C1: $op_trait<C2>,
        {
            type Output = Unit<
                V,
                <M1 as $op_trait<M2>>::Output,
                <G1 as $op_trait<G2>>::Output,
                <S1 as $op_trait<S2>>::Output,
                <A1 as $op_trait<A2>>::Output,
                <K1 as $op_trait<K2>>::Output,
                <O1 as $op_trait<O2>>::Output,
                <C1 as $op_trait<C2>>::Output,
            >;

            #[inline]
            fn $fn(self, rhs: Unit<U, M2, G2, S2, A2, K2, O2, C2>) -> Self::Output {
                Unit::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_mul_div!(Mul, mul, Add);
impl_mul_div!(Div, div, Sub);

// Neg — value negation, dimensions unchanged
impl<U, V, M, G, S, A, K, O, C> Neg for Unit<U, M, G, S, A, K, O, C>
where
    U: Neg<Output = V>,
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

    use typenum::{P1, P2, P3, P4, P5, P6, P7, P8, P10, P12, P14};

    use super::*;

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct X;

    type FloatScalar = Unit<f64>;
    type IntScalar = Unit<i64>;
    type ArbitraryScalar = Unit<X>;

    // Non-scalar types with dims (1,2,3,4,5,6,7) and squared (2,4,6,8,10,12,14)
    type FloatUnit = Unit<f64, P1, P2, P3, P4, P5, P6, P7>;
    type FloatUnitSquared = Unit<f64, P2, P4, P6, P8, P10, P12, P14>;
    type IntUnit = Unit<i64, P1, P2, P3, P4, P5, P6, P7>;
    type IntUnitSquared = Unit<i64, P2, P4, P6, P8, P10, P12, P14>;
    type ArbitraryUnit = Unit<X, P1, P2, P3, P4, P5, P6, P7>;

    const TYPE_FLOAT_SCALAR: TypeId = TypeId::of::<FloatScalar>();
    const TYPE_INT_SCALAR: TypeId = TypeId::of::<IntScalar>();
    const TYPE_ARBITRARY_SCALAR: TypeId = TypeId::of::<ArbitraryScalar>();

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

    // Test for scalar arithmetic

    #[test]
    fn test_scalar_neg() {
        let float: FloatScalar = 10.0.into();
        let int: IntScalar = 10.into();

        let float_neg = -float;
        let int_neg = -int;

        assert_eq!(float_neg.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_neg.type_id(), TYPE_INT_SCALAR);

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

        assert_eq!(float_add.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(float_sub.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_add.type_id(), TYPE_INT_SCALAR);
        assert_eq!(int_sub.type_id(), TYPE_INT_SCALAR);

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

        assert_eq!(float_mul.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(float_div.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_mul.type_id(), TYPE_INT_SCALAR);
        assert_eq!(int_div.type_id(), TYPE_INT_SCALAR);

        assert_eq!(float_mul.value, 100.0);
        assert_eq!(float_div.value, 1.0);
        assert_eq!(int_mul.value, 100);
        assert_eq!(int_div.value, 1);
    }

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

        assert!(float < float * 2.0.into());
        assert!(int < int * 2.into());
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

    // Test for non-scalar arithmetic

    #[test]
    fn test_non_scalar_neg() {
        let float = FloatUnit::new(10.0);
        let int = IntUnit::new(10);

        let float_neg = -float;
        let int_neg = -int;

        assert_eq!(float_neg.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(int_neg.type_id(), TYPE_INT_UNIT);

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

        assert_eq!(float_add.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(float_sub.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(int_add.type_id(), TYPE_INT_UNIT);
        assert_eq!(int_sub.type_id(), TYPE_INT_UNIT);

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

        assert_eq!(float_mul.type_id(), TYPE_FLOAT_UNIT_SQUARED);
        assert_eq!(float_div.type_id(), TYPE_FLOAT_SCALAR);
        assert_eq!(int_mul.type_id(), TYPE_INT_UNIT_SQUARED);
        assert_eq!(int_div.type_id(), TYPE_INT_SCALAR);

        assert_eq!(float_mul.value, 100.0);
        assert_eq!(float_div.value, 1.0);
        assert_eq!(int_mul.value, 100);
        assert_eq!(int_div.value, 1);
    }

    #[test]
    fn test_non_scalar_default() {
        let float = FloatUnit::default();
        let int = IntUnit::default();
        let arbitrary = ArbitraryUnit::default();

        assert_eq!(float.type_id(), TYPE_FLOAT_UNIT);
        assert_eq!(int.type_id(), TYPE_INT_UNIT);
        assert_eq!(arbitrary.type_id(), TYPE_ARBITRARY_UNIT);
    }

    #[test]
    fn test_non_scalar_cmp() {
        let float = FloatUnit::new(10.0);
        let int = IntUnit::new(10);

        assert!(float < float * 2.0.into());
        assert!(int < int * 2.into());
        assert!(float == float);
        assert!(int == int);
    }
}
