use core::{
    fmt::{self, Display, Formatter, Write},
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use typenum::{Integer, Z0};

use crate::{
    scale::ScaleCast,
    vector::{CrossProduct, DotProduct, VectorNorm},
};

const EMPTY: char = '\0';

fn fmt_superscript_number(
    num: i8,
    f: &mut Formatter<'_>,
    exp: char,
    minus: char,
    digits: &[char; 10],
) -> Result<(), fmt::Error> {
    let mut e = [EMPTY; 5];

    let mut idx = 4;
    let (mut num, exp, sign) = match num {
        // When exponent is 1, it is completely omitted, so no number or exp operator
        1 => (0, EMPTY, EMPTY),
        2.. => (num.unsigned_abs(), exp, EMPTY),
        ..=-1 => (num.unsigned_abs(), exp, minus),
        0 => unreachable!("exp cannot be 0"),
    };
    // Insert exp operator before number
    e[0] = exp;
    // Insert the minus sign if number is negative
    e[1] = sign;

    // Sets each and every digit in backward order
    while num > 0 {
        e[idx] = digits[(num % 10) as usize];
        num /= 10;
        idx -= 1;
        if idx <= 1 {
            unreachable!("i8 should be no more than 3 digits in decimal")
        }
    }

    // Since we need to support no_std, no str can directly constructed,
    // so we just print each char one by one in order
    for ch in e.into_iter().filter(|&ch| ch != EMPTY) {
        f.write_char(ch)?;
    }
    Ok(())
}

/// A physical quantity with compile-time dimension and scale checking.
///
/// The type parameter `S` (after `V`) controls the base-10 scale exponent
/// (e.g. `S = typenum::P3` for ×10³).  The remaining seven type parameters
/// correspond to the SI base dimensions for
/// 1. mass (kilogram),
/// 2. length (meter),
/// 3. time (second),
/// 4. electric current (ampere),
/// 5. thermodynamic temperature (kelvin),
/// 6. amount of substance (mole), and
/// 7. luminous intensity (candela).
///
/// Dimension arithmetic is performed using [`typenum`] type-level integers.
///
/// # Examples
///
/// ```
/// use fizix::Unit;
/// use typenum::*;
///
/// let d: Unit<f64, Z0, P1> = Unit::new(5.0); // 5.0 kg
/// assert_eq!(d.value, 5.0);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Unit<
    V,
    // Scale exponent (base-10)
    S = Z0,
    // Mass exponent (kilogram)
    M = Z0,
    // Length exponent (meter)
    L = Z0,
    // Time exponent (second)
    T = Z0,
    // Current exponent (ampere)
    I = Z0,
    // Temperature exponent (kelvin)
    K = Z0,
    // Amount exponent (mole)
    N = Z0,
    // Intensity exponent (candela)
    J = Z0,
> {
    /// The numeric value of this quantity.
    pub value: V,
    _phantom: PhantomData<(S, M, L, T, I, K, N, J)>,
}

impl<V, S, M, L, T, I, K, N, J> Unit<V, S, M, L, T, I, K, N, J>
where
    S: Integer,
    M: Integer,
    L: Integer,
    T: Integer,
    I: Integer,
    K: Integer,
    N: Integer,
    J: Integer,
{
    /// Create a new quantity with the given numeric value.
    #[inline]
    #[must_use]
    pub const fn new(value: V) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    /// Applies a function to underlying value to access unary operator on it
    pub fn apply<U>(self, func: fn(V) -> U) -> Unit<U, S, M, L, T, I, K, N, J> {
        Unit::new(func(self.value))
    }

    /// Convert between scale exponents at runtime.
    ///
    /// The difference `S - R` is computed at the type level and applied as a
    /// power-of-10 factor via [`ScaleCast::scale`].
    pub fn convert<R>(self) -> Unit<V, R, M, L, T, I, K, N, J>
    where
        R: Integer,
        V: ScaleCast,
    {
        let diff = S::to_i8() - R::to_i8();
        Unit::new(self.value.scale(diff))
    }

    /// Convert to the base scale (×10⁰).
    pub fn to_base(self) -> Unit<V, Z0, M, L, T, I, K, N, J>
    where
        V: ScaleCast,
    {
        self.convert()
    }
}

impl<V, S, M, L, T, I, K, N, J> Display for Unit<V, S, M, L, T, I, K, N, J>
where
    V: Display,
    S: Integer,
    M: Integer,
    L: Integer,
    T: Integer,
    I: Integer,
    K: Integer,
    N: Integer,
    J: Integer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        self.value.fmt(f)?;

        let (times, digits, dot, exp, minus) = if f.alternate() {
            (
                '*',
                ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
                '*',
                '^',
                '-',
            )
        } else {
            (
                '×',
                ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'],
                '⋅',
                '\0',
                '⁻',
            )
        };

        let scale = S::to_i8();
        let dims = [
            M::to_i8(),
            L::to_i8(),
            T::to_i8(),
            I::to_i8(),
            K::to_i8(),
            N::to_i8(),
            J::to_i8(),
        ];
        let has_dims = dims.iter().any(|&exp| exp != 0);
        let show_scale = scale != 0;

        if show_scale {
            f.write_char(times)?;
            f.write_str("10")?;
            fmt_superscript_number(scale, f, exp, minus, &digits)?;
        }

        if has_dims {
            f.write_char(' ')?;
            let items = [
                ("kg", M::to_i8()),
                ("m", L::to_i8()),
                ("s", T::to_i8()),
                ("A", I::to_i8()),
                ("K", K::to_i8()),
                ("mol", N::to_i8()),
                ("cd", J::to_i8()),
            ];
            let positive = items.iter().filter(|&&(_, exponent)| exponent > 0);
            let negative = items.iter().filter(|&&(_, exponent)| exponent < 0);
            for (i, &(u, e)) in positive.chain(negative).enumerate() {
                // If the unit has more than one part,
                // we need a dot operator to concat them
                if i > 0 {
                    f.write_char(dot)?;
                }
                f.write_str(u)?;
                fmt_superscript_number(e, f, exp, minus, &digits)?;
            }
        }
        Ok(())
    }
}

// Raw values can only be directly cast into scalar
impl<V> From<V> for Unit<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}

// Add / Sub: same type (including scale)
macro_rules! impl_add_sub {
    ($trait:ident, $fn:ident) => {
        impl<V1, V2, V, S, M, L, T, I, K, N, J> $trait<Unit<V2, S, M, L, T, I, K, N, J>>
            for Unit<V1, S, M, L, T, I, K, N, J>
        where
            V1: $trait<V2, Output = V>,
            S: Integer,
            M: Integer,
            L: Integer,
            T: Integer,
            I: Integer,
            K: Integer,
            N: Integer,
            J: Integer,
        {
            type Output = Unit<V, S, M, L, T, I, K, N, J>;

            #[inline]
            fn $fn(self, rhs: Unit<V2, S, M, L, T, I, K, N, J>) -> Self::Output {
                Unit::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_add_sub!(Add, add);
impl_add_sub!(Sub, sub);

// Mul / Div: dimension and scale exponents addition/subtraction
macro_rules! impl_mul_div {
    ($trait:ident, $fn:ident, $op_trait:ident) => {
        impl<
            // Lhs
            V1,
            S1,
            M1,
            L1,
            T1,
            I1,
            K1,
            N1,
            J1,
            // Rhs
            V2,
            S2,
            M2,
            L2,
            T2,
            I2,
            K2,
            N2,
            J2,
            // Out
            V,
            S,
            M,
            L,
            T,
            I,
            K,
            N,
            J,
        > $trait<Unit<V2, S2, M2, L2, T2, I2, K2, N2, J2>>
            for Unit<V1, S1, M1, L1, T1, I1, K1, N1, J1>
        where
            V1: $trait<V2, Output = V>,
            S1: $op_trait<S2, Output = S>,
            M1: $op_trait<M2, Output = M>,
            L1: $op_trait<L2, Output = L>,
            T1: $op_trait<T2, Output = T>,
            I1: $op_trait<I2, Output = I>,
            K1: $op_trait<K2, Output = K>,
            N1: $op_trait<N2, Output = N>,
            J1: $op_trait<J2, Output = J>,
            S: Integer,
            M: Integer,
            L: Integer,
            T: Integer,
            I: Integer,
            K: Integer,
            N: Integer,
            J: Integer,
        {
            type Output = Unit<V, S, M, L, T, I, K, N, J>;

            #[inline]
            fn $fn(self, rhs: Unit<V2, S2, M2, L2, T2, I2, K2, N2, J2>) -> Self::Output {
                Unit::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_mul_div!(Mul, mul, Add);
impl_mul_div!(Div, div, Sub);

// Neg: value negation, dimensions and scale unchanged
impl<U, V, S, M, L, T, I, K, N, J> Neg for Unit<U, S, M, L, T, I, K, N, J>
where
    U: Neg<Output = V>,
    S: Integer,
    M: Integer,
    L: Integer,
    T: Integer,
    I: Integer,
    K: Integer,
    N: Integer,
    J: Integer,
{
    type Output = Unit<V, S, M, L, T, I, K, N, J>;

    #[inline]
    fn neg(self) -> Self::Output {
        Unit::new(-self.value)
    }
}

// Vector arithmetic: dot, cross, norm
impl<
    // Lhs
    V1,
    S1,
    M1,
    L1,
    T1,
    I1,
    K1,
    N1,
    J1,
    // Rhs
    V2,
    S2,
    M2,
    L2,
    T2,
    I2,
    K2,
    N2,
    J2,
    // Out
    V,
    S,
    M,
    L,
    T,
    I,
    K,
    N,
    J,
> DotProduct<Unit<V2, S2, M2, L2, T2, I2, K2, N2, J2>> for Unit<V1, S1, M1, L1, T1, I1, K1, N1, J1>
where
    V1: DotProduct<V2, Output = V>,
    S1: Add<S2, Output = S>,
    M1: Add<M2, Output = M>,
    L1: Add<L2, Output = L>,
    T1: Add<T2, Output = T>,
    I1: Add<I2, Output = I>,
    K1: Add<K2, Output = K>,
    N1: Add<N2, Output = N>,
    J1: Add<J2, Output = J>,
    S: Integer,
    M: Integer,
    L: Integer,
    T: Integer,
    I: Integer,
    K: Integer,
    N: Integer,
    J: Integer,
{
    type Output = Unit<V, S, M, L, T, I, K, N, J>;

    #[inline]
    fn dot(self, rhs: Unit<V2, S2, M2, L2, T2, I2, K2, N2, J2>) -> Self::Output {
        Unit::new(self.value.dot(rhs.value))
    }
}

impl<
    // Lhs
    V1,
    S1,
    M1,
    L1,
    T1,
    I1,
    K1,
    N1,
    J1,
    // Rhs
    V2,
    S2,
    M2,
    L2,
    T2,
    I2,
    K2,
    N2,
    J2,
    // Out
    V,
    S,
    M,
    L,
    T,
    I,
    K,
    N,
    J,
> CrossProduct<Unit<V2, S2, M2, L2, T2, I2, K2, N2, J2>>
    for Unit<V1, S1, M1, L1, T1, I1, K1, N1, J1>
where
    V1: CrossProduct<V2, Output = V>,
    S1: Add<S2, Output = S>,
    M1: Add<M2, Output = M>,
    L1: Add<L2, Output = L>,
    T1: Add<T2, Output = T>,
    I1: Add<I2, Output = I>,
    K1: Add<K2, Output = K>,
    N1: Add<N2, Output = N>,
    J1: Add<J2, Output = J>,
    S: Integer,
    M: Integer,
    L: Integer,
    T: Integer,
    I: Integer,
    K: Integer,
    N: Integer,
    J: Integer,
{
    type Output = Unit<V, S, M, L, T, I, K, N, J>;

    #[inline]
    fn cross(self, rhs: Unit<V2, S2, M2, L2, T2, I2, K2, N2, J2>) -> Self::Output {
        Unit::new(self.value.cross(rhs.value))
    }
}

impl<U, V, S, M, L, T, I, K, N, J> VectorNorm for Unit<V, S, M, L, T, I, K, N, J>
where
    V: VectorNorm<Output = U>,
    S: Integer,
    M: Integer,
    L: Integer,
    T: Integer,
    I: Integer,
    K: Integer,
    N: Integer,
    J: Integer,
{
    type Output = Unit<U, S, M, L, T, I, K, N, J>;

    #[inline]
    fn norm(self) -> Self::Output {
        Unit::new(self.value.norm())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::format;

    use typenum::{N1, N2, N3, N6, N12, P1, P2, P3, P4, P5, P6, P7, P8, P10, P12, P14};

    use super::*;

    // Helper: assert Display with both feature modes
    macro_rules! assert_display {
        ($x:expr, $pretty:literal, $ascii:literal $(,)?) => {
            assert_eq!(format!("{}", $x), $pretty);
            assert_eq!(format!("{:#}", $x), $ascii);
        };
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct X;

    type FloatScalar = Unit<f64>;
    type IntScalar = Unit<i64>;
    type ArbitraryScalar = Unit<X>;

    // Non-scalar types with dims (1,2,3,4,5,6,7) and squared (2,4,6,8,10,12,14)
    type FloatUnit = Unit<f64, Z0, P1, P2, P3, P4, P5, P6, P7>;
    type FloatUnitSquared = Unit<f64, Z0, P2, P4, P6, P8, P10, P12, P14>;
    type IntUnit = Unit<i64, Z0, P1, P2, P3, P4, P5, P6, P7>;
    type IntUnitSquared = Unit<i64, Z0, P2, P4, P6, P8, P10, P12, P14>;
    type ArbitraryUnit = Unit<X, Z0, P1, P2, P3, P4, P5, P6, P7>;

    // Test for scalar creation

    #[test]
    fn test_scalar_initialization() {
        let float: FloatScalar = FloatScalar::new(1.0);
        let int: IntScalar = IntScalar::new(1);
        let arbitrary: ArbitraryScalar = ArbitraryScalar::new(X);

        assert_eq!(float.value, 1.0);
        assert_eq!(int.value, 1);
        assert_eq!(arbitrary.value, X);
    }

    #[test]
    fn test_scalar_from_value() {
        let float: FloatScalar = 1.0.into();
        let int: IntScalar = 1.into();
        let arbitrary: ArbitraryScalar = X.into();

        assert_eq!(float.value, 1.0);
        assert_eq!(int.value, 1);
        assert_eq!(arbitrary.value, X);
    }

    #[test]
    fn test_scalar_neg() {
        let float: FloatScalar = 10.0.into();
        let int: IntScalar = 10.into();

        let float_neg: FloatScalar = -float;
        let int_neg: IntScalar = -int;

        assert_eq!(float_neg.value, -10.0);
        assert_eq!(int_neg.value, -10);
    }

    #[test]
    fn test_scalar_add_sub() {
        let float: FloatScalar = 10.0.into();
        let int: IntScalar = 10.into();

        let float_add: FloatScalar = float + float;
        let float_sub: FloatScalar = float - float;
        let int_add: IntScalar = int + int;
        let int_sub: IntScalar = int - int;

        assert_eq!(float_add.value, 20.0);
        assert_eq!(float_sub.value, 0.0);
        assert_eq!(int_add.value, 20);
        assert_eq!(int_sub.value, 0);
    }

    #[test]
    fn test_scalar_mul_div() {
        let float: FloatScalar = 10.0.into();
        let int: IntScalar = 10.into();

        let float_mul: FloatScalar = float * float;
        let float_div: FloatScalar = float / float;
        let int_mul: IntScalar = int * int;
        let int_div: IntScalar = int / int;

        assert_eq!(float_mul.value, 100.0);
        assert_eq!(float_div.value, 1.0);
        assert_eq!(int_mul.value, 100);
        assert_eq!(int_div.value, 1);
    }

    #[test]
    fn test_scalar_default() {
        let float: FloatScalar = FloatScalar::default();
        let int: IntScalar = IntScalar::default();
        let arbitrary: ArbitraryScalar = ArbitraryScalar::default();

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
        let float: FloatUnit = FloatUnit::new(1.0);
        let int: IntUnit = IntUnit::new(1);
        let arbitrary: ArbitraryUnit = ArbitraryUnit::new(X);

        assert_eq!(float.value, 1.0);
        assert_eq!(int.value, 1);
        assert_eq!(arbitrary.value, X);
    }

    // Test for non-scalar arithmetic

    #[test]
    fn test_non_scalar_neg() {
        let float: FloatUnit = FloatUnit::new(10.0);
        let int: IntUnit = IntUnit::new(10);

        let float_neg: FloatUnit = -float;
        let int_neg: IntUnit = -int;

        assert_eq!(float_neg.value, -10.0);
        assert_eq!(int_neg.value, -10);
    }

    #[test]
    fn test_non_scalar_add_sub() {
        let float: FloatUnit = FloatUnit::new(10.0);
        let int: IntUnit = IntUnit::new(10);

        let float_add: FloatUnit = float + float;
        let float_sub: FloatUnit = float - float;
        let int_add: IntUnit = int + int;
        let int_sub: IntUnit = int - int;

        assert_eq!(float_add.value, 20.0);
        assert_eq!(float_sub.value, 0.0);
        assert_eq!(int_add.value, 20);
        assert_eq!(int_sub.value, 0);
    }

    #[test]
    fn test_non_scalar_mul_div() {
        let float: FloatUnit = FloatUnit::new(10.0);
        let int: IntUnit = IntUnit::new(10);

        let float_mul: FloatUnitSquared = float * float;
        let float_div: FloatScalar = float / float;
        let int_mul: IntUnitSquared = int * int;
        let int_div: IntScalar = int / int;

        assert_eq!(float_mul.value, 100.0);
        assert_eq!(float_div.value, 1.0);
        assert_eq!(int_mul.value, 100);
        assert_eq!(int_div.value, 1);
    }

    #[test]
    fn test_non_scalar_default() {
        let _: FloatUnit = FloatUnit::default();
        let _: IntUnit = IntUnit::default();
        let _: ArbitraryUnit = ArbitraryUnit::default();
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

    #[test]
    fn test_chained() {
        let kg: Unit<_, Z0, P1> = Unit::new(1.0);
        let meter: Unit<_, Z0, Z0, P1> = Unit::new(1.0);
        let sec: Unit<_, Z0, Z0, Z0, P1> = Unit::new(1.0);

        let x: Unit<_> = Unit::new(1.0);
        let x = x * kg * meter / sec;
        let x = x * kg / meter / sec;
        let x = x * kg * meter / sec;
        let x = x * kg / meter / sec;
        let x = x * kg * meter / sec;
        let x = x * kg / meter / sec;

        let _: Unit<f64, Z0, P6, Z0, N6> = x;
    }

    #[test]
    fn test_scalar_display() {
        let s: Unit<f64> = Unit::new(42.0);
        let out = format!("{}", s);
        assert_eq!(out, "42");
    }

    #[test]
    fn test_display_single_positive_dim() {
        // length (m) - exponent 1, no exponent marker
        let m: Unit<f64, Z0, Z0, P1> = Unit::new(5.0);
        assert_display!(m, "5 m", "5 m");
    }

    #[test]
    fn test_display_single_negative_dim() {
        // frequency (s⁻¹ = Hz) - negative exponent -1
        let hz: Unit<f64, Z0, Z0, Z0, N1> = Unit::new(440.0);
        assert_display!(hz, "440 s⁻¹", "440 s^-1");
    }

    #[test]
    fn test_display_single_higher_positive_dim() {
        // Area (m²) - exponent 2
        let area: Unit<f64, Z0, Z0, P2> = Unit::new(25.0);
        assert_display!(area, "25 m²", "25 m^2");
    }

    #[test]
    fn test_display_multi_dim_pos_neg() {
        // Velocity (m⋅s⁻¹) = length^1, time^-1
        let v: Unit<f64, Z0, Z0, P1, N1> = Unit::new(10.0);
        assert_display!(v, "10 m⋅s⁻¹", "10 m*s^-1");
    }

    #[test]
    fn test_display_three_dim_pos_neg() {
        // Newton-like: kg⋅m⋅s⁻²  (P1, P1, N2)
        let n: Unit<f64, Z0, P1, P1, N2> = Unit::new(100.0);
        assert_display!(n, "100 kg⋅m⋅s⁻²", "100 kg*m*s^-2");
    }

    #[test]
    fn test_display_neg_first_then_pos() {
        // Siemens-like (conductance): kg⁻¹⋅m⁻²⋅s³⋅A²
        // dim: N1, N2, P3, P2
        let s: Unit<f64, Z0, N1, N2, P3, P2> = Unit::new(2.0);
        assert_display!(s, "2 s³⋅A²⋅kg⁻¹⋅m⁻²", "2 s^3*A^2*kg^-1*m^-2");
    }

    #[test]
    fn test_display_all_negative() {
        // All negative exponents
        let all_neg: Unit<f64, Z0, N1, N2, N3, N1, N1, N1, N1> = Unit::new(1.0);
        assert_display!(
            all_neg,
            "1 kg⁻¹⋅m⁻²⋅s⁻³⋅A⁻¹⋅K⁻¹⋅mol⁻¹⋅cd⁻¹",
            "1 kg^-1*m^-2*s^-3*A^-1*K^-1*mol^-1*cd^-1",
        );
    }

    #[test]
    fn test_display_int_value() {
        let m: Unit<i32, Z0, Z0, P1> = Unit::new(42);
        assert_display!(m, "42 m", "42 m");
    }

    #[test]
    fn test_display_neg_exponent_multi_digit() {
        // Exponent -12 (i8) - tests multi-digit superscript rendering
        // Use a dimension with N12
        let big_neg: Unit<f64, Z0, Z0, Z0, N12> = Unit::new(1.0);
        assert_display!(big_neg, "1 s⁻¹²", "1 s^-12");
    }

    #[test]
    fn test_display_pos_exponent_multi_digit() {
        // Exponent 10 - tests multi-digit positive exponent rendering
        let big_pos: Unit<f64, Z0, Z0, P10> = Unit::new(1.0);
        assert_display!(big_pos, "1 m¹⁰", "1 m^10");
    }

    // Vector algebra tests

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    struct Vec3(f64, f64, f64);

    impl CrossProduct for Vec3 {
        type Output = Self;
        fn cross(self, rhs: Self) -> Self {
            Vec3(
                self.1 * rhs.2 - self.2 * rhs.1,
                self.2 * rhs.0 - self.0 * rhs.2,
                self.0 * rhs.1 - self.1 * rhs.0,
            )
        }
    }
    impl DotProduct for Vec3 {
        type Output = f64;
        fn dot(self, rhs: Self) -> f64 {
            self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
        }
    }
    impl VectorNorm for Vec3 {
        type Output = f64;
        fn norm(self) -> f64 {
            (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
        }
    }

    type Vec3Scalar = Unit<Vec3, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
    type Vec3Unit = Unit<Vec3, Z0, P1, P2, P3, P4, P5, P6, P7>;
    type Vec3UnitSquared = Unit<Vec3, Z0, P2, P4, P6, P8, P10, P12, P14>;

    #[test]
    fn unit_vec3_cross() {
        let x: Vec3Scalar = Unit::new(Vec3(1.0, 0.0, 0.0));
        let y: Vec3Scalar = Unit::new(Vec3(0.0, 1.0, 0.0));
        let z: Vec3Scalar = x.cross(y);
        assert!((z.value.2 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn unit_vec3_cross_with_dim() {
        let x: Vec3Unit = Unit::new(Vec3(1.0, 0.0, 0.0));
        let y: Vec3Unit = Unit::new(Vec3(0.0, 1.0, 0.0));
        let z: Vec3UnitSquared = x.cross(y);
        assert!((z.value.2 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn unit_vec3_dot() {
        let a: Vec3Scalar = Unit::new(Vec3(3.0, 4.0, 0.0));
        let b: Vec3Scalar = Unit::new(Vec3(4.0, 3.0, 0.0));
        let d: FloatScalar = a.dot(b);
        assert!((d.value - 24.0).abs() < 1e-10);
    }

    #[test]
    fn unit_vec3_dot_with_dim() {
        let a: Vec3Unit = Unit::new(Vec3(3.0, 4.0, 0.0));
        let b: Vec3Unit = Unit::new(Vec3(4.0, 3.0, 0.0));
        let d: FloatUnitSquared = a.dot(b);
        assert!((d.value - 24.0).abs() < 1e-10);
    }

    #[test]
    fn unit_vec3_norm() {
        let a: Vec3Scalar = Unit::new(Vec3(3.0, 4.0, 0.0));
        let n: FloatScalar = a.norm();
        assert!((n.value - 5.0).abs() < 1e-10);
    }

    #[test]
    fn unit_vec3_norm_with_dim() {
        let a: Vec3Unit = Unit::new(Vec3(3.0, 4.0, 0.0));
        let n: FloatUnit = a.norm();
        assert!((n.value - 5.0).abs() < 1e-10);
    }
}
