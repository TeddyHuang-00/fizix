use typenum::{N1, N2, N3, N4, P1, P2, P3, P4, Z0};

use crate::{alias_types, alias_units};

// base units
alias_units! {
    // Name      => (Doc string,                      kg,  m, s,  A,  K, mol, cd)
    pub Scalar   => ("Dimensionless quantity",        Z0, Z0, Z0, Z0, Z0, Z0, Z0),
    pub Kilogram => ("Mass (kg)",                     P1, Z0, Z0, Z0, Z0, Z0, Z0),
    pub Meter    => ("Length (m)",                    Z0, P1, Z0, Z0, Z0, Z0, Z0),
    pub Second   => ("Time (s)",                      Z0, Z0, P1, Z0, Z0, Z0, Z0),
    pub Ampere   => ("Electric Current (A)",          Z0, Z0, Z0, P1, Z0, Z0, Z0),
    pub Kelvin   => ("Thermodynamic temperature (K)", Z0, Z0, Z0, Z0, P1, Z0, Z0),
    pub Mole     => ("Amount of substance (mol)",     Z0, Z0, Z0, Z0, Z0, P1, Z0),
    pub Candela  => ("Luminous intensity (cd)",       Z0, Z0, Z0, Z0, Z0, Z0, P1),
}

// derived units with names
// <https://en.wikipedia.org/wiki/SI_derived_unit>
alias_units! {
    // Name       => (Doc string,                              kg,  m, s,  A,  K, mol, cd)
    pub Radian    => ("Plane angle (rad, 1)",                  Z0, Z0, Z0, Z0, Z0, Z0, Z0),
    pub Steradian => ("Solid angle (sr, 1)",                   Z0, Z0, Z0, Z0, Z0, Z0, Z0),
    pub Hertz     => ("Frequency (Hz, s⁻¹)",                   Z0, Z0, N1, Z0, Z0, Z0, Z0),
    pub Newton    => ("Force (N, kg⋅m⋅s⁻²)",                   P1, P1, N2, Z0, Z0, Z0, Z0),
    pub Pascal    => ("Pressure (Pa, kg⋅m⁻¹⋅s⁻²)",             P1, N1, N2, Z0, Z0, Z0, Z0),
    pub Joule     => ("Energy (J, kg⋅m²⋅s⁻²)",                 P1, P2, N2, Z0, Z0, Z0, Z0),
    pub Watt      => ("Power (W, kg⋅m²⋅s⁻³)",                  P1, P2, N3, Z0, Z0, Z0, Z0),
    pub Coulomb   => ("Electric charge (C, s⋅A)",              Z0, Z0, P1, P1, Z0, Z0, Z0),
    pub Volt      => ("Voltage (V, kg⋅m²⋅s⁻³⋅A⁻¹)",            P1, P2, N3, N1, Z0, Z0, Z0),
    pub Ohm       => ("Resistance (Ω, kg⋅m²⋅s⁻³⋅A⁻²)",         P1, P2, N3, N2, Z0, Z0, Z0),
    pub Siemens   => ("Conductance (S, kg⁻¹⋅m⁻²⋅s³⋅A²)",       N1, N2, P3, P2, Z0, Z0, Z0),
    pub Farad     => ("Capacitance (F, kg⁻¹⋅m⁻²⋅s⁴⋅A²)",       N1, N2, P4, P2, Z0, Z0, Z0),
    pub Henry     => ("Inductance (H, kg⋅m²⋅s⁻²⋅A⁻²)",         P1, P2, N2, N2, Z0, Z0, Z0),
    pub Tesla     => ("Magnetic flux density (T, kg⋅s⁻²⋅A⁻¹)", P1, Z0, N2, N1, Z0, Z0, Z0),
    pub Weber     => ("Magnetic flux (Wb, kg⋅m²⋅s⁻²⋅A⁻¹)",     P1, P2, N2, N1, Z0, Z0, Z0),
    pub Lumen     => ("Luminous flux (lm, cd⋅sr)",             Z0, Z0, Z0, Z0, Z0, Z0, P1),
    pub Lux       => ("Illuminance (lx, cd⋅sr⋅m⁻²)",           Z0, N2, Z0, Z0, Z0, Z0, P1),
    pub Becquerel => ("Radioactivity (Bq, s⁻¹)",               Z0, Z0, N1, Z0, Z0, Z0, Z0),
    pub Gray      => ("Absorbed dose (Gy, m²⋅s⁻²)",            Z0, P2, N2, Z0, Z0, Z0, Z0),
    pub Sievert   => ("Equivalent dose (Sv, m²⋅s⁻²)",          Z0, P2, N2, Z0, Z0, Z0, Z0),
    pub Katal     => ("Catalytic activity (kat, s⁻¹⋅mol)",     Z0, Z0, N1, Z0, Z0, P1, Z0),
}

// derived units (types-only)
// <https://en.wikipedia.org/wiki/International_System_of_Units#Coherent_and_non-coherent_SI_units>
// <https://en.wikipedia.org/wiki/SI_derived_unit#By_field_of_application>
alias_types! {
    // Name                     => (Doc string,      kg,  m, s,  A,  K, mol, cd)
    pub Speed |
    pub Velocity                => ("(m⋅s⁻¹)",       Z0, P1, N1, Z0, Z0, Z0, Z0),
    pub Acceleration            => ("(m⋅s⁻²)",       Z0, P1, N2, Z0, Z0, Z0, Z0),
    pub Jerk |
    pub Jolt                    => ("(m⋅s⁻³)",       Z0, P1, N3, Z0, Z0, Z0, Z0),
    pub Snap |
    pub Jounce                  => ("(m⋅s⁻⁴)",       Z0, P1, N4, Z0, Z0, Z0, Z0),
    pub Yank                    => ("(kg⋅m⋅s⁻³)",    P1, P1, N3, Z0, Z0, Z0, Z0),
    pub AngularVelocity         => ("(rad⋅s⁻¹)",     Z0, Z0, N1, Z0, Z0, Z0, Z0),
    pub AngularAcceleration     => ("(rad⋅s⁻²)",     Z0, Z0, N2, Z0, Z0, Z0, Z0),
    pub FrequencyDrift          => ("(Hz⋅s⁻¹)",      Z0, Z0, N2, Z0, Z0, Z0, Z0),
    pub VolumetricFlow          => ("(m³⋅s⁻¹)",      Z0, P3, N1, Z0, Z0, Z0, Z0),

    pub Area                    => ("(m²)",          Z0, P2, Z0, Z0, Z0, Z0, Z0),
    pub Volume                  => ("(m³)",          Z0, P3, Z0, Z0, Z0, Z0, Z0),
    pub Momentum |
    pub Impulse                 => ("(N⋅s)",         P1, P1, N1, Z0, Z0, Z0, Z0),
    pub AngularMomentum         => ("(N⋅m⋅s)",       P1, P2, N1, Z0, Z0, Z0, Z0),
    pub Torque |
    pub MomentOfForce           => ("(N⋅m)",         P1, P2, N2, Z0, Z0, Z0, Z0),
    pub WaveNumber |
    pub OpticalPower |
    pub Curvature |
    pub Vergence |
    pub SpatialFrequency        => ("(m⁻¹)",         Z0, N1, Z0, Z0, Z0, Z0, Z0),
    pub AreaDensity             => ("(kg⋅m⁻²)",      P1, N2, Z0, Z0, Z0, Z0, Z0),
    pub Density                 => ("(kg⋅m⁻³)",      P1, N3, Z0, Z0, Z0, Z0, Z0),
    pub SpecificVolume          => ("(m³⋅kg⁻¹)",     N1, P3, Z0, Z0, Z0, Z0, Z0),
    pub Action                  => ("(J⋅s)",         P1, P2, N1, Z0, Z0, Z0, Z0),
    pub SpecificEnergy          => ("(J⋅m⁻³)",       N1, P3, Z0, Z0, Z0, Z0, Z0),
    pub SurfaceTension |
    pub Stiffness               => ("(N⋅m⁻¹)",       P1, Z0, N2, Z0, Z0, Z0, Z0),
    pub HeatFluxDensity |
    pub Irradiance              => ("(W⋅m⁻²)",       P1, Z0, N3, Z0, Z0, Z0, Z0),
    pub KinematicViscosity |
    pub ThermalDiffusivity |
    pub DiffusionCoefficient    => ("(m²⋅s⁻¹)",      Z0, P2, N1, Z0, Z0, Z0, Z0),
    pub DynamicViscosity        => ("(Pa⋅s)",        P1, N1, N1, Z0, Z0, Z0, Z0),
    pub LinearMassDensity       => ("(kg⋅m⁻¹)",      P1, N1, Z0, Z0, Z0, Z0, Z0),
    pub MassFlowRate            => ("(kg⋅s⁻¹)",      P1, Z0, N1, Z0, Z0, Z0, Z0),
    pub Radiance                => ("(W⋅sr⁻¹⋅m⁻²)",  P1, Z0, N3, Z0, Z0, Z0, Z0),
    pub SpectralPower           => ("(W⋅m⁻¹)",       P1, P1, N3, Z0, Z0, Z0, Z0),
    pub AbsorbedDoseRate        => ("(Gy⋅s⁻¹)",      Z0, P2, N3, Z0, Z0, Z0, Z0),
    pub FuelEfficiency          => ("(m⋅m⁻³)",       Z0, N2, Z0, Z0, Z0, Z0, Z0),
    pub SpectralIrradiance |
    pub PowerDensity            => ("(Gy⋅s⁻¹)",      P1, N1, N3, Z0, Z0, Z0, Z0),
    pub EnergyFluxDensity       => ("(J⋅m⁻²⋅s⁻¹)",   P1, Z0, N3, Z0, Z0, Z0, Z0),
    pub Compressibility         => ("(Pa⁻¹)",        N1, P1, P2, Z0, Z0, Z0, Z0),
    pub RadiantExposure         => ("(J⋅m⁻²)",       P1, Z0, N2, Z0, Z0, Z0, Z0),
    pub MomentOfInertia         => ("(kg⋅m²)",       P1, P2, Z0, Z0, Z0, Z0, Z0),
    pub SpecificAngularMomentum => ("(N⋅m⋅s⋅kg⁻¹)",  Z0, P2, N1, Z0, Z0, Z0, Z0),
    pub RadiantIntensity        => ("(W⋅sr⁻¹)",      P1, P2, N3, Z0, Z0, Z0, Z0),
    pub SpectralIntensity       => ("(W⋅sr⁻¹⋅m⁻¹)",  P1, P1, N3, Z0, Z0, Z0, Z0),

    pub Molarity |
    pub Concentration           => ("(mol⋅m⁻³)",     Z0, N3, Z0, Z0, Z0, P1, Z0),
    pub MolarVolume             => ("(m³⋅mol⁻¹)",    Z0, P3, Z0, Z0, Z0, N1, Z0),
    pub MolarHeatCapacity |
    pub MolarEntropy            => ("(J⋅K⁻¹⋅mol⁻¹)", P1, P2, N2, Z0, N1, N1, Z0),
    pub MolarEnergy             => ("(J⋅mol⁻¹)",     P1, P2, N2, Z0, Z0, N1, Z0),

    pub SurfaceDensity          => ("(kg⋅m⁻²)",      P1, N2, Z0, Z0, Z0, Z0, Z0),
    pub CurrentDensity          => ("(A⋅m⁻²)",       Z0, N2, Z0, P1, Z0, Z0, Z0),
    pub MassConcentration       => ("(kg⋅m⁻³)",      P1, N3, Z0, Z0, Z0, Z0, Z0),
    pub MagneticFieldStrength   => ("(A⋅m⁻¹)",       Z0, N1, Z0, P1, Z0, Z0, Z0),
    pub Luminance               => ("(cd⋅m⁻²)",      Z0, N3, Z0, Z0, Z0, Z0, P1),
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;
    use crate::Unit;

    /// Helper macro to turn type-only aliases into concrete values
    macro_rules! eval {
        ($id:ident) => {
            $id::new(1.0f64)
        };
        ($left:ident * $right:tt) => {
            eval!($left) * eval!($right)
        };
        ($left:tt / $right:ident) => {
            eval!($left) / eval!($right)
        };
    }

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
    fn test_derived_unit() {
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
    fn test_derived_type() {
        assert_eq!(eval!(Speed), eval!(Meter / Second));
    }
}
