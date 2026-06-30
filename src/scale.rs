//! Runtime scale conversion for [`Unit`](crate::Unit).

/// Self × 10^exp. `exp` can be negative (implies division).
pub trait ScaleCast {
    /// Multiply (or divide if `exp < 0`) `self` by 10^|exp|.
    #[must_use]
    fn scale(self, exp: i8) -> Self;
}

// float implementations

macro_rules! impl_scale_cast_float {
    ($($t:ty),*) => {
        $(
            impl ScaleCast for $t {
                #[inline]
                fn scale(self, exp: i8) -> Self {
                    // Compute 10^|exp| iteratively (avoids dependency
                    // on `f64::powi` which isn't available in `core`).
                    let mut pow10 = 1.0_f64;
                    let n = exp.unsigned_abs();
                    for _ in 0..n {
                        pow10 *= 10.0;
                    }
                    #[allow(clippy::cast_possible_truncation)]
                    if exp >= 0 {
                        self * pow10 as Self
                    } else {
                        self / pow10 as Self
                    }
                }
            }
        )*
    };
}
impl_scale_cast_float!(f32, f64);

// integer implementations

macro_rules! impl_scale_cast_int {
    ($($t:ty),*) => {
        $(
            impl ScaleCast for $t {
                #[inline]
                fn scale(self, exp: i8) -> Self {
                    #[allow(
                        clippy::cast_possible_truncation,
                        clippy::cast_lossless,
                        clippy::cast_possible_wrap
                    )]
                    let pow10 = 10u64.pow(exp.unsigned_abs() as u32) as Self;
                    if exp >= 0 {
                        self * pow10
                    } else {
                        self / pow10
                    }
                }
            }
        )*
    };
}
impl_scale_cast_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
