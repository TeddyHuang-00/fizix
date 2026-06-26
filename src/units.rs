use typenum::{N1, N2, N3, N4, P1, P2, P3, P4, Z0};

/// Create `Unit` type aliases only.
///
/// # Uses
///
/// Unlike `alias_units`, this macro WON'T create const and helper functions.
/// This is useful when creating aliases that are only meant for type annotation
/// and type checks rather than acting as a valid named unit.
///
/// # Examples
///
/// ```
/// use siunit::alias_types;
/// use typenum::P1;
///
/// alias_types! {
///     (pub PureValue, "Some custom scalar type"),
///     (NameA | NameB, "Other custom type with two different aliases", P1),
/// }
/// let _ = PureValue::new(0usize);
/// ```
///
/// but we don't have constant and helper functions:
///
/// ```compile_fail
/// # use siunit::alias_types;
/// # use typenum::P1;
///
/// # alias_types! {
/// #     (pub PureValue, "Some custom scalar type"),
/// #     (NameA | NameB, "Same unit with two different aliases", P1),
/// # }
/// let _ = pure_value(0usize);
/// let _ = PURE_VALUE;
/// ```
#[macro_export]
macro_rules! alias_types {
    (($(|)?$pre:vis $name:ident, $doc:literal $(, $dim:ty)*)$(,)?) => {
        paste::paste! {
            #[doc = $doc]
            $pre type $name<V> = $crate::Unit<V, $($dim),*>;
        }
    };

    (($(|)?$pre:vis $name:ident|$($pres:vis $names:ident)|+, $doc:literal $(, $dim:ty)*)$(,)?) => {
        alias_types! { ($pre $name, $doc $(, $dim)*) }
        alias_types! { ($($pres $names)|+, $doc $(, $dim)*) }
    };

    (
        ($(|)?$($pre:vis $name:ident)|+, $doc:literal $(, $dim:ty)*),
        $(($(|)?$($pres:vis $names:ident)|+, $docs:literal $(, $dims:ty)*)),+$(,)?
    ) => {
        alias_types! { ($($pre $name)|+, $doc $(, $dim)*) }
        alias_types! { $(($($pres $names)|+, $docs $(, $dims)*)),+ }
    }
}

/// Create `Unit` type aliases and populate const and helper functions.
///
/// # Uses
///
/// For desired `TypeAlias`, this creates:
/// - `TypeAlias`: a `type` with only one generic of data type
/// - `type_alias`: a `fn` that takes any value and wraps it in `TypeAlias`
/// - `TYPE_ALIAS`: a `const` of `TypeAlias` with value of `1.0f64`
///
/// # Examples
///
/// ```
/// use siunit::alias_units;
/// use typenum::P1;
///
/// alias_units! {
///     (pub PureValue, "Some custom scalar type"),
///     (NameA | NameB, "Same unit with two different aliases", P1),
/// }
///
/// let _ = PureValue::new(0usize);
/// let _ = name_a(0usize);
/// let _ = NAME_B; // constants use f64
/// ```
#[macro_export]
macro_rules! alias_units {
    (($(|)?$pre:vis $name:ident, $doc:literal $(, $dim:ty)*)$(,)?) => {
        paste::paste! {
            #[doc = $doc]
            $pre type $name<V> = $crate::Unit<V, $($dim),*>;

            #[doc = $doc]
            #[inline]
            $pre const fn [<$name:snake>]<V>(v: V) -> $name<V> {
                $name::new(v)
            }

            #[doc = $doc]
            $pre const [<$name:snake:upper>]: $name<f64> = $name::new(1.0);
        }
    };

    (($(|)?$pre:vis $name:ident|$($pres:vis $names:ident)|+, $doc:literal $(, $dim:ty)*)$(,)?) => {
        alias_units! { ($pre $name, $doc $(, $dim)*) }
        alias_units! { ($($pres $names)|+, $doc $(, $dim)*) }
    };

    (
        ($(|)?$($pre:vis $name:ident)|+, $doc:literal $(, $dim:ty)*),
        $(($(|)?$($pres:vis $names:ident)|+, $docs:literal $(, $dims:ty)*)),+$(,)?
    ) => {
        alias_units! { ($($pre $name)|+, $doc $(, $dim)*) }
        alias_units! { $(($($pres $names)|+, $docs $(, $dims)*)),+ }
    }
}

// base units
alias_units! {
    // (Name,  Doc string,                      kg,  m, s,  A,  K, mol, cd)
    (Scalar,   "Dimensionless quantity",        Z0, Z0, Z0, Z0, Z0, Z0, Z0),
    (Kilogram, "Mass (kg)",                     P1, Z0, Z0, Z0, Z0, Z0, Z0),
    (Meter,    "Length (m)",                    Z0, P1, Z0, Z0, Z0, Z0, Z0),
    (Second,   "Time (s)",                      Z0, Z0, P1, Z0, Z0, Z0, Z0),
    (Ampere,   "Electric Current (A)",          Z0, Z0, Z0, P1, Z0, Z0, Z0),
    (Kelvin,   "Thermodynamic temperature (K)", Z0, Z0, Z0, Z0, P1, Z0, Z0),
    (Mole,     "Amount of substance (mol)",     Z0, Z0, Z0, Z0, Z0, P1, Z0),
    (Candela,  "Luminous intensity (cd)",       Z0, Z0, Z0, Z0, Z0, Z0, P1),
}

// derived units with names <https://en.wikipedia.org/wiki/SI_derived_unit>
alias_units! {
    // (Name,   Doc string,                              kg,  m, s,  A,  K, mol, cd)
    (Radian,    "Plane angle (rad, 1)",                  Z0, Z0, Z0, Z0, Z0, Z0, Z0),
    (Steradian, "Solid angle (sr, 1)",                   Z0, Z0, Z0, Z0, Z0, Z0, Z0),
    (Hertz,     "Frequency (Hz, s⁻¹)",                   Z0, Z0, N1, Z0, Z0, Z0, Z0),
    (Newton,    "Force (N, kg⋅m⋅s⁻²)",                   P1, P1, N2, Z0, Z0, Z0, Z0),
    (Pascal,    "Pressure (Pa, kg⋅m⁻¹⋅s⁻²)",             P1, N1, N2, Z0, Z0, Z0, Z0),
    (Joule,     "Energy (J, kg⋅m²⋅s⁻²)",                 P1, P2, N2, Z0, Z0, Z0, Z0),
    (Watt,      "Power (W, kg⋅m²⋅s⁻³)",                  P1, P2, N3, Z0, Z0, Z0, Z0),
    (Coulomb,   "Electric charge (C, s⋅A)",              Z0, Z0, P1, P1, Z0, Z0, Z0),
    (Volt,      "Voltage (V, kg⋅m²⋅s⁻³⋅A⁻¹)",            P1, P2, N3, N1, Z0, Z0, Z0),
    (Ohm,       "Resistance (Ω, kg⋅m²⋅s⁻³⋅A⁻²)",         P1, P2, N3, N2, Z0, Z0, Z0),
    (Siemens,   "Conductance (S, kg⁻¹⋅m⁻²⋅s³⋅A²)",       N1, N2, P3, P2, Z0, Z0, Z0),
    (Farad,     "Capacitance (F, kg⁻¹⋅m⁻²⋅s⁴⋅A²)",       N1, N2, P4, P2, Z0, Z0, Z0),
    (Henry,     "Inductance (H, kg⋅m²⋅s⁻²⋅A⁻²)",         P1, P2, N2, N2, Z0, Z0, Z0),
    (Tesla,     "Magnetic flux density (T, kg⋅s⁻²⋅A⁻¹)", P1, Z0, N2, N1, Z0, Z0, Z0),
    (Weber,     "Magnetic flux (Wb, kg⋅m²⋅s⁻²⋅A⁻¹)",     P1, P2, N2, N1, Z0, Z0, Z0),
    (Lumen,     "Luminous flux (lm, cd⋅sr)",             Z0, Z0, Z0, Z0, Z0, Z0, P1),
    (Lux,       "Illuminance (lx, cd⋅sr⋅m⁻²)",           Z0, N2, Z0, Z0, Z0, Z0, P1),
    (Becquerel, "Radioactivity (Bq, s⁻¹)",               Z0, Z0, N1, Z0, Z0, Z0, Z0),
    (Gray,      "Absorbed dose (Gy, m²⋅s⁻²)",            Z0, P2, N2, Z0, Z0, Z0, Z0),
    (Sievert,   "Equivalent dose (Sv, m²⋅s⁻²)",          Z0, P2, N2, Z0, Z0, Z0, Z0),
    (Katal,     "Catalytic activity (kat, s⁻¹⋅mol)",     Z0, Z0, N1, Z0, Z0, P1, Z0),
}

// derived units without unit names
// <https://en.wikipedia.org/wiki/International_System_of_Units#Coherent_and_non-coherent_SI_units>
// <https://en.wikipedia.org/wiki/SI_derived_unit#By_field_of_application>
alias_units! {
    // (Name,               Doc string,     kg,  m, s,  A,  K, mol, cd)
    (Speed
    |Velocity,              "(m⋅s⁻¹)",      Z0, P1, N1, Z0, Z0, Z0, Z0),
    (Acceleration,          "(m⋅s⁻²)",      Z0, P1, N2, Z0, Z0, Z0, Z0),
    (Jerk
    |Jolt,                  "(m⋅s⁻³)",      Z0, P1, N3, Z0, Z0, Z0, Z0),
    (Snap
    |Jounce,                "(m⋅s⁻⁴)",      Z0, P1, N4, Z0, Z0, Z0, Z0),
    (Yank,                  "(kg⋅m⋅s⁻³)",   P1, P1, N3, Z0, Z0, Z0, Z0),
    (AngularVelocity,       "(rad⋅s⁻¹)",    Z0, Z0, N1, Z0, Z0, Z0, Z0),
    (AngularAcceleration,   "(rad⋅s⁻²)",    Z0, Z0, N2, Z0, Z0, Z0, Z0),
    (FrequencyDrift,        "(Hz⋅s⁻¹)",     Z0, Z0, N2, Z0, Z0, Z0, Z0),
    (VolumetricFlow,        "(m³⋅s⁻¹)",     Z0, P3, N1, Z0, Z0, Z0, Z0),

    (Area,                  "(m²)",         Z0, P2, Z0, Z0, Z0, Z0, Z0),
    (Volume,                "(m³)",         Z0, P3, Z0, Z0, Z0, Z0, Z0),
    (Momentum
    |Impulse,               "(N⋅s)",        P1, P1, N1, Z0, Z0, Z0, Z0),
    (AngularMomentum,       "(N⋅m⋅s)",      P1, P2, N1, Z0, Z0, Z0, Z0),
    (Torque
    |MomentOfForce,         "(N⋅m)",        P1, P2, N2, Z0, Z0, Z0, Z0),
    (WaveNumber
    |OpticalPower
    |Curvature
    |Vergence
    |SpatialFrequency,      "(m⁻¹)",        Z0, N1, Z0, Z0, Z0, Z0, Z0),
    (AreaDensity,           "(kg⋅m⁻²)",     P1, N2, Z0, Z0, Z0, Z0, Z0),
    (Density,               "(kg⋅m⁻³)",     P1, N3, Z0, Z0, Z0, Z0, Z0),
    (SpecificVolume,        "(m³⋅kg⁻¹)",    N1, P3, Z0, Z0, Z0, Z0, Z0),
    (Action,                "(J⋅s)",        P1, P2, N1, Z0, Z0, Z0, Z0),
    (SpecificEnergy,        "(J⋅m⁻³)",      N1, P3, Z0, Z0, Z0, Z0, Z0),
    (SurfaceTension
    |Stiffness,             "(N⋅m⁻¹)",      P1, Z0, N2, Z0, Z0, Z0, Z0),
    (HeatFluxDensity
    |Irradiance,            "(W⋅m⁻²)",      P1, Z0, N3, Z0, Z0, Z0, Z0),
    (KinematicViscosity
    |ThermalDiffusivity
    |DiffusionCoefficient,  "(m²⋅s⁻¹)",     Z0, P2, N1, Z0, Z0, Z0, Z0),
    (DynamicViscosity,      "(Pa⋅s)",       P1, N1, N1, Z0, Z0, Z0, Z0),
    (LinearMassDensity,     "(kg⋅m⁻¹)",     P1, N1, Z0, Z0, Z0, Z0, Z0),
    (MassFlowRate,          "(kg⋅s⁻¹)",     P1, Z0, N1, Z0, Z0, Z0, Z0),
    (Radiance,              "(W⋅sr⁻¹⋅m⁻²)", P1, Z0, N3, Z0, Z0, Z0, Z0),
    (SpectralPower,         "(W⋅m⁻¹)",      P1, P1, N3, Z0, Z0, Z0, Z0),
    (AbsorbedDoseRate,      "(Gy⋅s⁻¹)",     Z0, P2, N3, Z0, Z0, Z0, Z0),
    (FuelEfficiency,        "(m⋅m⁻³)",      Z0, N2, Z0, Z0, Z0, Z0, Z0),
    (SpectralIrradiance
    |PowerDensity,          "(Gy⋅s⁻¹)",     P1, N1, N3, Z0, Z0, Z0, Z0),
    (EnergyFluxDensity,     "(J⋅m⁻²⋅s⁻¹)",  P1, Z0, N3, Z0, Z0, Z0, Z0),

    (SurfaceDensity,        "(kg⋅m⁻²)",     P1, N2, Z0, Z0, Z0, Z0, Z0),
    (CurrentDensity,        "(A⋅m⁻²)",      Z0, N2, Z0, P1, Z0, Z0, Z0),
    (Concentration,         "(mol⋅m⁻³)",    Z0, N3, Z0, Z0, Z0, P1, Z0),
    (MassConcentration,     "(kg⋅m⁻³)",     P1, N3, Z0, Z0, Z0, Z0, Z0),
    (MagneticFieldStrength, "(A⋅m⁻¹)",      Z0, N1, Z0, P1, Z0, Z0, Z0),
    (Luminance,             "(cd⋅m⁻²)",     Z0, N3, Z0, Z0, Z0, Z0, P1),
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;
    use crate::Unit;

    #[test]
    fn test_struct() {
        assert_eq!(TypeId::of::<Scalar<f64>>(), TypeId::of::<Unit<f64>>());
        assert_eq!(Scalar::new(1.0f64).type_id(), TypeId::of::<Unit<f64>>());
    }

    #[test]
    fn test_fn() {
        assert_eq!(scalar(1.0f64).type_id(), TypeId::of::<Unit<f64>>());
    }

    #[test]
    fn test_const() {
        assert_eq!(SCALAR.type_id(), TypeId::of::<Unit<f64>>());
    }

    #[test]
    fn test_derived() {
        assert_eq!(RADIAN, METER / METER);
        assert_eq!(STERADIAN, (METER * METER) / (METER * METER));
        assert_eq!(HERTZ, SCALAR / SECOND);
        assert_eq!(NEWTON, KILOGRAM * METER / SECOND / SECOND);
        assert_eq!(PASCAL, NEWTON / (METER * METER));
        assert_eq!(JOULE, NEWTON * METER);
        assert_eq!(WATT, JOULE / SECOND);
        assert_eq!(COULOMB, AMPERE * SECOND);
        assert_eq!(VOLT, WATT / AMPERE);
        assert_eq!(OHM, VOLT / AMPERE);
        assert_eq!(SIEMENS, AMPERE / VOLT);
        assert_eq!(FARAD, COULOMB / VOLT);
        assert_eq!(HENRY, VOLT * SECOND / AMPERE);
        assert_eq!(TESLA, VOLT * SECOND / (METER * METER));
        assert_eq!(WEBER, VOLT * SECOND);
        assert_eq!(LUMEN, CANDELA * STERADIAN);
        assert_eq!(LUX, LUMEN / (METER * METER));
        assert_eq!(BECQUEREL, SCALAR / SECOND);
        assert_eq!(GRAY, JOULE / KILOGRAM);
        assert_eq!(SIEVERT, JOULE / KILOGRAM);
        assert_eq!(KATAL, MOLE / SECOND);
    }

    #[test]
    fn test_convenience() {
        assert_eq!(VELOCITY, METER / SECOND);
        assert_eq!(ACCELERATION, VELOCITY / SECOND);
        assert_eq!(AREA, METER * METER);
        assert_eq!(VOLUME, AREA * METER);
        assert_eq!(MOMENTUM, KILOGRAM * VELOCITY);
        assert_eq!(ANGULAR_MOMENTUM, METER * MOMENTUM);
        assert_eq!(TORQUE, METER * NEWTON);
        assert_eq!(DENSITY, KILOGRAM / VOLUME);
        assert_eq!(DYNAMIC_VISCOSITY, PASCAL * SECOND);
        assert_eq!(KINEMATIC_VISCOSITY, DYNAMIC_VISCOSITY / DENSITY);
    }

    #[test]
    fn test_other_derived() {
        assert_eq!(GRAY, (METER * METER) / (SECOND * SECOND));
        assert_eq!(BECQUEREL, HERTZ);
        assert_eq!(LUX, CANDELA / (METER * METER));
    }

    #[test]
    fn test_chain() {
        let r = OHM;
        let v = VOLT;
        let t = SECOND * 2.0.into();
        let i = v / r;
        assert_eq!(i, AMPERE);
        let p = v * i;
        assert_eq!(p, WATT);
        let e = p * t;
        assert_eq!(e, JOULE * 2.0.into());
    }
}
