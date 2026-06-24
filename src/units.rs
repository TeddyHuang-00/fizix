use crate::Dim;

// ============================================================
// SI base units
// ============================================================

/// Length (metre) — m
pub type Meter<V> = Dim<1, 0, 0, V>;
/// Mass (kilogram) — kg
pub type Kilogram<V> = Dim<0, 1, 0, V>;
/// Time (second) — s
pub type Second<V> = Dim<0, 0, 1, V>;

/// Dimensionless quantity
pub type Scalar<V> = Dim<0, 0, 0, V>;

// ============================================================
// SI derived units (L, M, T only)
// ============================================================

/// Frequency (hertz) — s⁻¹
pub type Hertz<V> = Dim<0, 0, -1, V>;
/// Force (newton) — kg·m·s⁻²
pub type Newton<V> = Dim<1, 1, -2, V>;
/// Energy (joule) — N·m = kg·m²·s⁻²
pub type Joule<V> = Dim<2, 1, -2, V>;
/// Power (watt) — J·s⁻¹ = kg·m²·s⁻³
pub type Watt<V> = Dim<2, 1, -3, V>;
/// Pressure (pascal) — N·m⁻² = kg·m⁻¹·s⁻²
pub type Pascal<V> = Dim<-1, 1, -2, V>;

// ============================================================
// Convenience aliases
// ============================================================

/// Speed / velocity — m·s⁻¹
pub type Speed<V> = Dim<1, 0, -1, V>;
/// Acceleration — m·s⁻²
pub type Acceleration<V> = Dim<1, 0, -2, V>;
/// Area — m²
pub type Area<V> = Dim<2, 0, 0, V>;
/// Volume — m³
pub type Volume<V> = Dim<3, 0, 0, V>;
/// Momentum — kg·m·s⁻¹
pub type Momentum<V> = Dim<1, 1, -1, V>;
/// Angular momentum — kg·m²·s⁻¹
pub type AngularMomentum<V> = Dim<2, 1, -1, V>;
/// Torque — N·m = kg·m²·s⁻² (same dimensions as energy)
pub type Torque<V> = Joule<V>;
/// Density — kg·m⁻³
pub type Density<V> = Dim<-3, 1, 0, V>;
/// Dynamic viscosity — Pa·s = kg·m⁻¹·s⁻¹
pub type Viscosity<V> = Dim<-1, 1, -1, V>;
/// Kinematic viscosity — m²·s⁻¹
pub type KinematicViscosity<V> = Dim<2, 0, -1, V>;
