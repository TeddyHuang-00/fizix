use crate::Dim;

// ============================================================
// SI base units
// ============================================================

/// Length (metre) — m
pub type Meter<V> = Dim<V, 1, 0, 0>;
/// Mass (kilogram) — kg
pub type Kilogram<V> = Dim<V, 0, 1, 0>;
/// Time (second) — s
pub type Second<V> = Dim<V, 0, 0, 1>;

/// Dimensionless quantity
pub type Scalar<V> = Dim<V, 0, 0, 0>;

// ============================================================
// SI derived units (L, M, T only)
// ============================================================

/// Frequency (hertz) — s⁻¹
pub type Hertz<V> = Dim<V, 0, 0, -1>;
/// Force (newton) — kg·m·s⁻²
pub type Newton<V> = Dim<V, 1, 1, -2>;
/// Energy (joule) — N·m = kg·m²·s⁻²
pub type Joule<V> = Dim<V, 2, 1, -2>;
/// Power (watt) — J·s⁻¹ = kg·m²·s⁻³
pub type Watt<V> = Dim<V, 2, 1, -3>;
/// Pressure (pascal) — N·m⁻² = kg·m⁻¹·s⁻²
pub type Pascal<V> = Dim<V, -1, 1, -2>;

// ============================================================
// Convenience aliases
// ============================================================

/// Speed / velocity — m·s⁻¹
pub type Speed<V> = Dim<V, 1, 0, -1>;
/// Acceleration — m·s⁻²
pub type Acceleration<V> = Dim<V, 1, 0, -2>;
/// Area — m²
pub type Area<V> = Dim<V, 2, 0, 0>;
/// Volume — m³
pub type Volume<V> = Dim<V, 3, 0, 0>;
/// Momentum — kg·m·s⁻¹
pub type Momentum<V> = Dim<V, 1, 1, -1>;
/// Angular momentum — kg·m²·s⁻¹
pub type AngularMomentum<V> = Dim<V, 2, 1, -1>;
/// Torque — N·m = kg·m²·s⁻² (same dimensions as energy)
pub type Torque<V> = Joule<V>;
/// Density — kg·m⁻³
pub type Density<V> = Dim<V, -3, 1, 0>;
/// Dynamic viscosity — Pa·s = kg·m⁻¹·s⁻¹
pub type Viscosity<V> = Dim<V, -1, 1, -1>;
/// Kinematic viscosity — m²·s⁻¹
pub type KinematicViscosity<V> = Dim<V, 2, 0, -1>;
