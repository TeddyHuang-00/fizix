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
/// let d: Dim<f64, 1, 0, 0> = Dim::new(5.0); // metres
/// assert_eq!(d.value, 5.0);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Dim<
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
    Dim<V, M, G, S, A, K, O, C>
{
    /// Create a new quantity with the given numeric value.
    #[inline]
    #[must_use]
    pub const fn new(value: V) -> Self {
        Self { value }
    }
}
