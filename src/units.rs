use typenum::{N1, N2, N3, N4, P1, P2, P3, P4, Z0};

use crate::{alias_types, alias_units};

// base units
alias_units! {
    // Name      => (Doc string,                      kg,  m, s,  A,  K, mol, cd)
    pub Scalar   => ("Dimensionless quantity",         _,  _,  _,  _,  _,  _,  _),
    pub Kilogram => ("Mass (kg)",                     P1,  _,  _,  _,  _,  _,  _),
    pub Meter    => ("Length (m)",                     _, P1,  _,  _,  _,  _,  _),
    pub Second   => ("Time (s)",                       _,  _, P1,  _,  _,  _,  _),
    pub Ampere   => ("Electric Current (A)",           _,  _,  _, P1,  _,  _,  _),
    pub Kelvin   => ("Thermodynamic temperature (K)",  _,  _,  _,  _, P1,  _,  _),
    pub Mole     => ("Amount of substance (mol)",      _,  _,  _,  _,  _, P1,  _),
    pub Candela  => ("Luminous intensity (cd)",        _,  _,  _,  _,  _,  _, P1),
}

// derived units with names
// <https://en.wikipedia.org/wiki/SI_derived_unit>
alias_units! {
    // Name       => (Doc string,                              kg,  m, s,  A,  K, mol, cd)
    pub Radian    => ("Plane angle (rad, 1)",                   _,  _,  _,  _,  _,  _,  _),
    pub Steradian => ("Solid angle (sr, 1)",                    _,  _,  _,  _,  _,  _,  _),
    pub Hertz     => ("Frequency (Hz, s⁻¹)",                    _,  _, N1,  _,  _,  _,  _),
    pub Newton    => ("Force (N, kg⋅m⋅s⁻²)",                   P1, P1, N2,  _,  _,  _,  _),
    pub Pascal    => ("Pressure (Pa, kg⋅m⁻¹⋅s⁻²)",             P1, N1, N2,  _,  _,  _,  _),
    pub Joule     => ("Energy (J, kg⋅m²⋅s⁻²)",                 P1, P2, N2,  _,  _,  _,  _),
    pub Watt      => ("Power (W, kg⋅m²⋅s⁻³)",                  P1, P2, N3,  _,  _,  _,  _),
    pub Coulomb   => ("Electric charge (C, s⋅A)",               _,  _, P1, P1,  _,  _,  _),
    pub Volt      => ("Voltage (V, kg⋅m²⋅s⁻³⋅A⁻¹)",            P1, P2, N3, N1,  _,  _,  _),
    pub Ohm       => ("Resistance (Ω, kg⋅m²⋅s⁻³⋅A⁻²)",         P1, P2, N3, N2,  _,  _,  _),
    pub Siemens   => ("Conductance (S, kg⁻¹⋅m⁻²⋅s³⋅A²)",       N1, N2, P3, P2,  _,  _,  _),
    pub Farad     => ("Capacitance (F, kg⁻¹⋅m⁻²⋅s⁴⋅A²)",       N1, N2, P4, P2,  _,  _,  _),
    pub Henry     => ("Inductance (H, kg⋅m²⋅s⁻²⋅A⁻²)",         P1, P2, N2, N2,  _,  _,  _),
    pub Tesla     => ("Magnetic flux density (T, kg⋅s⁻²⋅A⁻¹)", P1,  _, N2, N1,  _,  _,  _),
    pub Weber     => ("Magnetic flux (Wb, kg⋅m²⋅s⁻²⋅A⁻¹)",     P1, P2, N2, N1,  _,  _,  _),
    pub Lumen     => ("Luminous flux (lm, cd⋅sr)",              _,  _,  _,  _,  _,  _, P1),
    pub Lux       => ("Illuminance (lx, cd⋅sr⋅m⁻²)",            _, N2,  _,  _,  _,  _, P1),
    pub Becquerel => ("Radioactivity (Bq, s⁻¹)",                _,  _, N1,  _,  _,  _,  _),
    pub Gray      => ("Absorbed dose (Gy, m²⋅s⁻²)",             _, P2, N2,  _,  _,  _,  _),
    pub Sievert   => ("Equivalent dose (Sv, m²⋅s⁻²)",           _, P2, N2,  _,  _,  _,  _),
    pub Katal     => ("Catalytic activity (kat, s⁻¹⋅mol)",      _,  _, N1,  _,  _, P1,  _),
}

// derived units (types-only)
// <https://en.wikipedia.org/wiki/International_System_of_Units#Coherent_and_non-coherent_SI_units>
// <https://en.wikipedia.org/wiki/SI_derived_unit#By_field_of_application>
alias_types! {
    // Name                     => (Doc string,      kg,  m, s,  A,  K, mol, cd)
    pub Speed |
    pub Velocity                => ("(m⋅s⁻¹)",        _, P1, N1,  _,  _,  _,  _),
    pub Acceleration            => ("(m⋅s⁻²)",        _, P1, N2,  _,  _,  _,  _),
    pub Jerk |
    pub Jolt                    => ("(m⋅s⁻³)",        _, P1, N3,  _,  _,  _,  _),
    pub Snap |
    pub Jounce                  => ("(m⋅s⁻⁴)",        _, P1, N4,  _,  _,  _,  _),
    pub Yank                    => ("(kg⋅m⋅s⁻³)",    P1, P1, N3,  _,  _,  _,  _),
    pub AngularVelocity         => ("(rad⋅s⁻¹)",      _,  _, N1,  _,  _,  _,  _),
    pub AngularAcceleration     => ("(rad⋅s⁻²)",      _,  _, N2,  _,  _,  _,  _),
    pub FrequencyDrift          => ("(Hz⋅s⁻¹)",       _,  _, N2,  _,  _,  _,  _),
    pub VolumetricFlow          => ("(m³⋅s⁻¹)",       _, P3, N1,  _,  _,  _,  _),

    pub Area                    => ("(m²)",           _, P2,  _,  _,  _,  _,  _),
    pub Volume                  => ("(m³)",           _, P3,  _,  _,  _,  _,  _),
    pub Momentum |
    pub Impulse                 => ("(N⋅s)",         P1, P1, N1,  _,  _,  _,  _),
    pub AngularMomentum         => ("(N⋅m⋅s)",       P1, P2, N1,  _,  _,  _,  _),
    pub Torque |
    pub MomentOfForce           => ("(N⋅m)",         P1, P2, N2,  _,  _,  _,  _),
    pub WaveNumber |
    pub OpticalPower |
    pub Curvature |
    pub Vergence |
    pub SpatialFrequency        => ("(m⁻¹)",          _, N1,  _,  _,  _,  _,  _),
    pub AreaDensity             => ("(kg⋅m⁻²)",      P1, N2,  _,  _,  _,  _,  _),
    pub Density                 => ("(kg⋅m⁻³)",      P1, N3,  _,  _,  _,  _,  _),
    pub SpecificVolume          => ("(m³⋅kg⁻¹)",     N1, P3,  _,  _,  _,  _,  _),
    pub Action                  => ("(J⋅s)",         P1, P2, N1,  _,  _,  _,  _),
    pub SpecificEnergy          => ("(J⋅m⁻³)",       N1, P3,  _,  _,  _,  _,  _),
    pub SurfaceTension |
    pub Stiffness               => ("(N⋅m⁻¹)",       P1,  _, N2,  _,  _,  _,  _),
    pub HeatFluxDensity |
    pub Irradiance              => ("(W⋅m⁻²)",       P1,  _, N3,  _,  _,  _,  _),
    pub KinematicViscosity |
    pub ThermalDiffusivity |
    pub DiffusionCoefficient    => ("(m²⋅s⁻¹)",       _, P2, N1,  _,  _,  _,  _),
    pub DynamicViscosity        => ("(Pa⋅s)",        P1, N1, N1,  _,  _,  _,  _),
    pub LinearMassDensity       => ("(kg⋅m⁻¹)",      P1, N1,  _,  _,  _,  _,  _),
    pub MassFlowRate            => ("(kg⋅s⁻¹)",      P1,  _, N1,  _,  _,  _,  _),
    pub Radiance                => ("(W⋅sr⁻¹⋅m⁻²)",  P1,  _, N3,  _,  _,  _,  _),
    pub SpectralPower           => ("(W⋅m⁻¹)",       P1, P1, N3,  _,  _,  _,  _),
    pub AbsorbedDoseRate        => ("(Gy⋅s⁻¹)",       _, P2, N3,  _,  _,  _,  _),
    pub FuelEfficiency          => ("(m⋅m⁻³)",        _, N2,  _,  _,  _,  _,  _),
    pub SpectralIrradiance |
    pub PowerDensity            => ("(Gy⋅s⁻¹)",      P1, N1, N3,  _,  _,  _,  _),
    pub EnergyFluxDensity       => ("(J⋅m⁻²⋅s⁻¹)",   P1,  _, N3,  _,  _,  _,  _),
    pub Compressibility         => ("(Pa⁻¹)",        N1, P1, P2,  _,  _,  _,  _),
    pub RadiantExposure         => ("(J⋅m⁻²)",       P1,  _, N2,  _,  _,  _,  _),
    pub MomentOfInertia         => ("(kg⋅m²)",       P1, P2,  _,  _,  _,  _,  _),
    pub SpecificAngularMomentum => ("(N⋅m⋅s⋅kg⁻¹)",   _, P2, N1,  _,  _,  _,  _),
    pub RadiantIntensity        => ("(W⋅sr⁻¹)",      P1, P2, N3,  _,  _,  _,  _),
    pub SpectralIntensity       => ("(W⋅sr⁻¹⋅m⁻¹)",  P1, P1, N3,  _,  _,  _,  _),

    pub Molarity |
    pub Concentration           => ("(mol⋅m⁻³)",      _, N3,  _,  _,  _, P1,  _),
    pub MolarVolume             => ("(m³⋅mol⁻¹)",     _, P3,  _,  _,  _, N1,  _),
    pub MolarHeatCapacity |
    pub MolarEntropy            => ("(J⋅K⁻¹⋅mol⁻¹)", P1, P2, N2,  _, N1, N1,  _),
    pub MolarEnergy             => ("(J⋅mol⁻¹)",     P1, P2, N2,  _,  _, N1,  _),
    pub MolarConductivity       => ("(S⋅m²⋅mol⁻¹)",  N1,  _, P3, P2,  _, N1,  _),
    pub Molality                => ("(mol⋅kg⁻¹)",    N1,  _,  _,  _,  _, P1,  _),

    pub SurfaceDensity          => ("(kg⋅m⁻²)",      P1, N2,  _,  _,  _,  _,  _),
    pub CurrentDensity          => ("(A⋅m⁻²)",        _, N2,  _, P1,  _,  _,  _),
    pub MassConcentration       => ("(kg⋅m⁻³)",      P1, N3,  _,  _,  _,  _,  _),
    pub MagneticFieldStrength   => ("(A⋅m⁻¹)",        _, N1,  _, P1,  _,  _,  _),
    pub Luminance               => ("(cd⋅m⁻²)",       _, N3,  _,  _,  _,  _, P1),
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
