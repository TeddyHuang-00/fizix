use core::{
    fmt::{self, Display, Formatter, Write},
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use typenum::{Integer, Z0};

const EMPTY: char = '\0';

/// A physical quantity with compile-time dimension checking.
///
/// Seven type parameters (after `V`) corresponding to the SI base dimensions
/// for
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
/// use siunit::Unit;
///
/// let d: Unit<f64, typenum::consts::P1> = Unit::new(5.0); // 5.0 kg
/// assert_eq!(d.value, 5.0);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Unit<
    V,
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
    _phantom: PhantomData<(M, L, T, I, K, N, J)>,
}

impl<V, M, L, T, I, K, N, J> Unit<V, M, L, T, I, K, N, J>
where
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
    pub fn apply<U>(self, func: fn(V) -> U) -> Unit<U, M, L, T, I, K, N, J> {
        Unit::new(func(self.value))
    }

    /// Format the unit name using ASCII symbols
    ///
    /// # Errors
    ///
    /// This function should return [`Err`] if, and only if, the provided
    /// [`Formatter`] returns [`Err`]. String formatting is considered an
    /// infallible operation; this function only returns a [`Result`]
    /// because writing to the underlying stream might fail and it must provide
    /// a way to propagate the fact that an error has occurred back up the
    /// stack.
    pub fn ascii_unit(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        Self::fmt_unit(
            f,
            '*',
            '^',
            '-',
            &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        )?;
        Ok(())
    }

    /// Format the unit name using pretty unicode symbols
    ///
    /// # Errors
    ///
    /// This function should return [`Err`] if, and only if, the provided
    /// [`Formatter`] returns [`Err`]. String formatting is considered an
    /// infallible operation; this function only returns a [`Result`]
    /// because writing to the underlying stream might fail and it must provide
    /// a way to propagate the fact that an error has occurred back up the
    /// stack.
    pub fn pretty_unit(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        Self::fmt_unit(
            f,
            '⋅',
            EMPTY,
            '⁻',
            &['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'],
        )?;
        Ok(())
    }

    /// Format the unit name using specific char set
    fn fmt_unit(
        f: &mut Formatter<'_>,
        dot: char,
        exp: char,
        minus: char,
        digits: &[char; 10],
    ) -> Result<(), fmt::Error> {
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
        let unit_name = positive.chain(negative).map(|&(id, exponent)| {
            let mut e = [EMPTY; 5];

            let mut idx = 4;
            let (mut exponent, exp, sign) = match exponent {
                // When exponent is 1, it is completely omitted, so no number or exp operator
                1 => (0, EMPTY, EMPTY),
                2.. => (exponent.unsigned_abs(), exp, EMPTY),
                ..=-1 => (exponent.unsigned_abs(), exp, minus),
                0 => unreachable!("exp cannot be 0"),
            };
            // Insert exp operator between unit name and exponent
            e[0] = exp;
            // Insert the minus sign if exponent is negative
            e[1] = sign;

            // Sets each and every digit in backward order
            while exponent > 0 {
                e[idx] = digits[(exponent % 10) as usize];
                exponent /= 10;
                idx -= 1;
                if idx <= 1 {
                    unreachable!("i8 should be no more than 3 digits in decimal")
                }
            }
            id.chars().chain(e.into_iter().filter(|&ch| ch != EMPTY))
        });

        // Since we need to support no_std, no str can directly constructed,
        // so we just print each char one by one in order
        for (i, u) in unit_name.enumerate() {
            // If the unit has more than one part,
            // we need a dot operator to concat them
            if i > 0 {
                f.write_char(dot)?;
            }
            for s in u {
                f.write_char(s)?;
            }
        }
        Ok(())
    }
}

impl<V, M, L, T, I, K, N, J> Display for Unit<V, M, L, T, I, K, N, J>
where
    V: Display,
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
        if [
            M::to_i8(),
            L::to_i8(),
            T::to_i8(),
            I::to_i8(),
            K::to_i8(),
            N::to_i8(),
            J::to_i8(),
        ]
        .into_iter()
        .any(|exp| exp != 0)
        {
            // If the not a scalar type, the unit is not empty,
            // so we should place a space between the value and it
            f.write_char(' ')?;
            #[cfg(feature = "pretty-display")]
            self.pretty_unit(f)?;
            #[cfg(not(feature = "pretty-display"))]
            self.ascii_unit(f)?;
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

// Add / Sub: same type
macro_rules! impl_add_sub {
    ($trait:ident, $fn:ident) => {
        impl<V1, V2, V, M, L, T, I, K, N, J> $trait<Unit<V2, M, L, T, I, K, N, J>>
            for Unit<V1, M, L, T, I, K, N, J>
        where
            V1: $trait<V2, Output = V>,
            M: Integer,
            L: Integer,
            T: Integer,
            I: Integer,
            K: Integer,
            N: Integer,
            J: Integer,
        {
            type Output = Unit<V, M, L, T, I, K, N, J>;

            #[inline]
            fn $fn(self, rhs: Unit<V2, M, L, T, I, K, N, J>) -> Self::Output {
                Unit::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_add_sub!(Add, add);
impl_add_sub!(Sub, sub);

// Mul / Div: dimension exponents addition/subtraction
macro_rules! impl_mul_div {
    ($trait:ident, $fn:ident, $op_trait:ident) => {
        impl<V1, V2, V, M1, L1, T1, I1, K1, N1, J1, M2, L2, T2, I2, K2, N2, J2, M, L, T, I, K, N, J>
            $trait<Unit<V2, M2, L2, T2, I2, K2, N2, J2>> for Unit<V1, M1, L1, T1, I1, K1, N1, J1>
        where
            V1: $trait<V2, Output = V>,
            M1: $op_trait<M2, Output = M>,
            L1: $op_trait<L2, Output = L>,
            T1: $op_trait<T2, Output = T>,
            I1: $op_trait<I2, Output = I>,
            K1: $op_trait<K2, Output = K>,
            N1: $op_trait<N2, Output = N>,
            J1: $op_trait<J2, Output = J>,
            M: Integer,
            L: Integer,
            T: Integer,
            I: Integer,
            K: Integer,
            N: Integer,
            J: Integer,
        {
            type Output = Unit<V, M, L, T, I, K, N, J>;

            #[inline]
            fn $fn(self, rhs: Unit<V2, M2, L2, T2, I2, K2, N2, J2>) -> Self::Output {
                Unit::new(self.value.$fn(rhs.value))
            }
        }
    };
}

impl_mul_div!(Mul, mul, Add);
impl_mul_div!(Div, div, Sub);

// Neg: value negation, dimensions unchanged
impl<U, V, M, L, T, I, K, N, J> Neg for Unit<U, M, L, T, I, K, N, J>
where
    U: Neg<Output = V>,
    M: Integer,
    L: Integer,
    T: Integer,
    I: Integer,
    K: Integer,
    N: Integer,
    J: Integer,
{
    type Output = Unit<V, M, L, T, I, K, N, J>;

    #[inline]
    fn neg(self) -> Self::Output {
        Unit::new(-self.value)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::format;

    use typenum::{N1, N2, N3, N6, N12, P1, P2, P3, P4, P5, P6, P7, P8, P10, P12, P14};

    use super::*;

    macro_rules! assert_display {
        ($x:ident, $pretty:literal, $ascii:literal) => {
            #[cfg(feature = "pretty-display")]
            assert_eq!(format!("{}", $x), $pretty);
            #[cfg(not(feature = "pretty-display"))]
            assert_eq!(format!("{}", $x), $ascii);
        };
    }

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
        let kg: Unit<_, P1> = Unit::new(1.0);
        let meter: Unit<_, Z0, P1> = Unit::new(1.0);
        let sec: Unit<_, Z0, Z0, P1> = Unit::new(1.0);

        let x: Unit<_> = Unit::new(1.0);
        let x = x * kg * meter / sec;
        let x = x * kg / meter / sec;
        let x = x * kg * meter / sec;
        let x = x * kg / meter / sec;
        let x = x * kg * meter / sec;
        let x = x * kg / meter / sec;

        let _: Unit<f64, P6, Z0, N6> = x;
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
        let m: Unit<f64, Z0, P1> = Unit::new(5.0);
        assert_display!(m, "5 m", "5 m");
    }

    #[test]
    fn test_display_single_negative_dim() {
        // frequency (s⁻¹ = Hz) - negative exponent -1
        let hz: Unit<f64, Z0, Z0, N1> = Unit::new(440.0);
        assert_display!(hz, "440 s⁻¹", "440 s^-1");
    }

    #[test]
    fn test_display_single_higher_positive_dim() {
        // Area (m²) - exponent 2
        let area: Unit<f64, Z0, P2> = Unit::new(25.0);
        assert_display!(area, "25 m²", "25 m^2");
    }

    #[test]
    fn test_display_multi_dim_pos_neg() {
        // Velocity (m⋅s⁻¹) = length^1, time^-1
        let v: Unit<f64, Z0, P1, N1> = Unit::new(10.0);
        assert_display!(v, "10 m⋅s⁻¹", "10 m*s^-1");
    }

    #[test]
    fn test_display_three_dim_pos_neg() {
        // Newton-like: kg⋅m⋅s⁻²  (P1, P1, N2)
        let n: Unit<f64, P1, P1, N2> = Unit::new(100.0);
        assert_display!(n, "100 kg⋅m⋅s⁻²", "100 kg*m*s^-2");
    }

    #[test]
    fn test_display_neg_first_then_pos() {
        // Siemens-like (conductance): kg⁻¹⋅m⁻²⋅s³⋅A²
        // dim: N1, N2, P3, P2
        let s: Unit<f64, N1, N2, P3, P2> = Unit::new(2.0);
        assert_display!(s, "2 s³⋅A²⋅kg⁻¹⋅m⁻²", "2 s^3*A^2*kg^-1*m^-2");
    }

    #[test]
    fn test_display_all_negative() {
        // All negative exponents
        let all_neg: Unit<f64, N1, N2, N3, N1, N1, N1, N1> = Unit::new(1.0);
        assert_display!(
            all_neg,
            "1 kg⁻¹⋅m⁻²⋅s⁻³⋅A⁻¹⋅K⁻¹⋅mol⁻¹⋅cd⁻¹",
            "1 kg^-1*m^-2*s^-3*A^-1*K^-1*mol^-1*cd^-1"
        );
    }

    #[test]
    fn test_display_int_value() {
        let m: Unit<i32, Z0, P1> = Unit::new(42);
        assert_display!(m, "42 m", "42 m");
    }

    #[test]
    fn test_display_neg_exponent_multi_digit() {
        // Exponent -12 (i8) - tests multi-digit superscript rendering
        // Use a dimension with N12
        let big_neg: Unit<f64, Z0, Z0, N12> = Unit::new(1.0);
        assert_display!(big_neg, "1 s⁻¹²", "1 s^-12");
    }

    #[test]
    fn test_display_pos_exponent_multi_digit() {
        // Exponent 10 - tests multi-digit positive exponent rendering
        let big_pos: Unit<f64, Z0, P10> = Unit::new(1.0);
        assert_display!(big_pos, "1 m¹⁰", "1 m^10");
    }
}
