/// A physical quantity with compile-time dimension checking (3D subset).
///
/// Three const generic parameters correspond to the SI base dimensions for
/// length (L), mass (M), and time (T). This covers mechanics, acoustics,
/// and thermodynamics. Electromagnetism and photometry dimensions (I, Θ,
/// N, J) are reserved for a future version when the compiler's const-expr
/// support matures (tracked at [rust#76560]).
///
/// # Examples
///
/// ```
/// use siunit::Dim;
///
/// let d: Dim<1, 0, 0> = Dim::new(5.0);  // metres
/// assert_eq!(d.value, 5.0);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dim<
    // Value type
    V,
    // Length exponent (L)
    const L: i8 = 0,
    // Mass exponent (M)
    const M: i8 = 0,
    // Time exponent (T)
    const T: i8 = 0,
> {
    /// The numeric value of this quantity.
    pub value: V,
}

impl<const L: i8, const M: i8, const T: i8, V> Dim<V, L, M, T> {
    /// Create a new quantity with the given numeric value.
    #[inline]
    #[must_use]
    pub const fn new(value: V) -> Self {
        Self { value }
    }
}
