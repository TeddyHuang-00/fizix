use typenum::{N1, N2, N3, N4, P1, P2, P3, P4, Z0};

use crate::alias_units;

// Help improve code readability, doesn't affect actual diagnosis
type __ = Z0;

// base units
alias_units! {
    // Name      => const (Doc string,                      kg,  m, s,  A,  K, mol, cd)
    pub Scalar   => const ("Dimensionless quantity",        __, __, __, __, __, __, __),
    pub Kilogram => const ("Mass (kg)",                     P1, __, __, __, __, __, __),
    pub Meter    => const ("Length (m)",                    __, P1, __, __, __, __, __),
    pub Second   => const ("Time (s)",                      __, __, P1, __, __, __, __),
    pub Ampere   => const ("Electric Current (A)",          __, __, __, P1, __, __, __),
    pub Kelvin   => const ("Thermodynamic temperature (K)", __, __, __, __, P1, __, __),
    pub Mole     => const ("Amount of substance (mol)",     __, __, __, __, __, P1, __),
    pub Candela  => const ("Luminous intensity (cd)",       __, __, __, __, __, __, P1),
}

// derived units with names
// <https://en.wikipedia.org/wiki/SI_derived_unit>
alias_units! {
    // Name       => const (Doc string,                              kg,  m, s,  A,  K, mol, cd)
    pub Radian    => const ("Plane angle (rad, 1)",                  __, __, __, __, __, __, __),
    pub Steradian => const ("Solid angle (sr, 1)",                   __, __, __, __, __, __, __),
    pub Hertz     => const ("Frequency (Hz, s⁻¹)",                   __, __, N1, __, __, __, __),
    pub Newton    => const ("Force (N, kg⋅m⋅s⁻²)",                   P1, P1, N2, __, __, __, __),
    pub Pascal    => const ("Pressure (Pa, kg⋅m⁻¹⋅s⁻²)",             P1, N1, N2, __, __, __, __),
    pub Joule     => const ("Energy (J, kg⋅m²⋅s⁻²)",                 P1, P2, N2, __, __, __, __),
    pub Watt      => const ("Power (W, kg⋅m²⋅s⁻³)",                  P1, P2, N3, __, __, __, __),
    pub Coulomb   => const ("Electric charge (C, s⋅A)",              __, __, P1, P1, __, __, __),
    pub Volt      => const ("Voltage (V, kg⋅m²⋅s⁻³⋅A⁻¹)",            P1, P2, N3, N1, __, __, __),
    pub Ohm       => const ("Resistance (Ω, kg⋅m²⋅s⁻³⋅A⁻²)",         P1, P2, N3, N2, __, __, __),
    pub Siemens   => const ("Conductance (S, kg⁻¹⋅m⁻²⋅s³⋅A²)",       N1, N2, P3, P2, __, __, __),
    pub Farad     => const ("Capacitance (F, kg⁻¹⋅m⁻²⋅s⁴⋅A²)",       N1, N2, P4, P2, __, __, __),
    pub Henry     => const ("Inductance (H, kg⋅m²⋅s⁻²⋅A⁻²)",         P1, P2, N2, N2, __, __, __),
    pub Tesla     => const ("Magnetic flux density (T, kg⋅s⁻²⋅A⁻¹)", P1, __, N2, N1, __, __, __),
    pub Weber     => const ("Magnetic flux (Wb, kg⋅m²⋅s⁻²⋅A⁻¹)",     P1, P2, N2, N1, __, __, __),
    pub Lumen     => const ("Luminous flux (lm, cd⋅sr)",             __, __, __, __, __, __, P1),
    pub Lux       => const ("Illuminance (lx, cd⋅sr⋅m⁻²)",           __, N2, __, __, __, __, P1),
    pub Becquerel => const ("Radioactivity (Bq, s⁻¹)",               __, __, N1, __, __, __, __),
    pub Gray      => const ("Absorbed dose (Gy, m²⋅s⁻²)",            __, P2, N2, __, __, __, __),
    pub Sievert   => const ("Equivalent dose (Sv, m²⋅s⁻²)",          __, P2, N2, __, __, __, __),
    pub Katal     => const ("Catalytic activity (kat, s⁻¹⋅mol)",     __, __, N1, __, __, P1, __),
}

// derived units (types-only)
// <https://en.wikipedia.org/wiki/International_System_of_Units#Coherent_and_non-coherent_SI_units>
// <https://en.wikipedia.org/wiki/SI_derived_unit#By_field_of_application>
alias_units! {
    // Name                     => (Doc string,      kg,  m, s,  A,  K, mol, cd)
    pub Speed |
    pub Velocity                => ("(m⋅s⁻¹)",       __, P1, N1, __, __, __, __),
    pub Acceleration            => ("(m⋅s⁻²)",       __, P1, N2, __, __, __, __),
    pub Jerk |
    pub Jolt                    => ("(m⋅s⁻³)",       __, P1, N3, __, __, __, __),
    pub Snap |
    pub Jounce                  => ("(m⋅s⁻⁴)",       __, P1, N4, __, __, __, __),
    pub Yank                    => ("(kg⋅m⋅s⁻³)",    P1, P1, N3, __, __, __, __),
    pub AngularVelocity         => ("(rad⋅s⁻¹)",     __, __, N1, __, __, __, __),
    pub AngularAcceleration     => ("(rad⋅s⁻²)",     __, __, N2, __, __, __, __),
    pub FrequencyDrift          => ("(Hz⋅s⁻¹)",      __, __, N2, __, __, __, __),
    pub VolumetricFlow          => ("(m³⋅s⁻¹)",      __, P3, N1, __, __, __, __),

    pub Area                    => ("(m²)",          __, P2, __, __, __, __, __),
    pub Volume                  => ("(m³)",          __, P3, __, __, __, __, __),
    pub Momentum |
    pub Impulse                 => ("(N⋅s)",         P1, P1, N1, __, __, __, __),
    pub AngularMomentum         => ("(N⋅m⋅s)",       P1, P2, N1, __, __, __, __),
    pub Torque |
    pub MomentOfForce           => ("(N⋅m)",         P1, P2, N2, __, __, __, __),
    pub WaveNumber |
    pub OpticalPower |
    pub Curvature |
    pub Vergence |
    pub SpatialFrequency        => ("(m⁻¹)",         __, N1, __, __, __, __, __),
    pub AreaDensity             => ("(kg⋅m⁻²)",      P1, N2, __, __, __, __, __),
    pub Density                 => ("(kg⋅m⁻³)",      P1, N3, __, __, __, __, __),
    pub SpecificVolume          => ("(m³⋅kg⁻¹)",     N1, P3, __, __, __, __, __),
    pub Action                  => ("(J⋅s)",         P1, P2, N1, __, __, __, __),
    pub SpecificEnergy          => ("(J⋅m⁻³)",       N1, P3, __, __, __, __, __),
    pub SurfaceTension |
    pub Stiffness               => ("(N⋅m⁻¹)",       P1, __, N2, __, __, __, __),
    pub HeatFluxDensity |
    pub Irradiance              => ("(W⋅m⁻²)",       P1, __, N3, __, __, __, __),
    pub KinematicViscosity |
    pub ThermalDiffusivity |
    pub DiffusionCoefficient    => ("(m²⋅s⁻¹)",      __, P2, N1, __, __, __, __),
    pub DynamicViscosity        => ("(Pa⋅s)",        P1, N1, N1, __, __, __, __),
    pub LinearMassDensity       => ("(kg⋅m⁻¹)",      P1, N1, __, __, __, __, __),
    pub MassFlowRate            => ("(kg⋅s⁻¹)",      P1, __, N1, __, __, __, __),
    pub Radiance                => ("(W⋅sr⁻¹⋅m⁻²)",  P1, __, N3, __, __, __, __),
    pub SpectralPower           => ("(W⋅m⁻¹)",       P1, P1, N3, __, __, __, __),
    pub AbsorbedDoseRate        => ("(Gy⋅s⁻¹)",      __, P2, N3, __, __, __, __),
    pub FuelEfficiency          => ("(m⋅m⁻³)",       __, N2, __, __, __, __, __),
    pub SpectralIrradiance |
    pub PowerDensity            => ("(Gy⋅s⁻¹)",      P1, N1, N3, __, __, __, __),
    pub EnergyFluxDensity       => ("(J⋅m⁻²⋅s⁻¹)",   P1, __, N3, __, __, __, __),
    pub Compressibility         => ("(Pa⁻¹)",        N1, P1, P2, __, __, __, __),
    pub RadiantExposure         => ("(J⋅m⁻²)",       P1, __, N2, __, __, __, __),
    pub MomentOfInertia         => ("(kg⋅m²)",       P1, P2, __, __, __, __, __),
    pub SpecificAngularMomentum => ("(N⋅m⋅s⋅kg⁻¹)",  __, P2, N1, __, __, __, __),
    pub RadiantIntensity        => ("(W⋅sr⁻¹)",      P1, P2, N3, __, __, __, __),
    pub SpectralIntensity       => ("(W⋅sr⁻¹⋅m⁻¹)",  P1, P1, N3, __, __, __, __),

    pub Molarity |
    pub Concentration           => ("(mol⋅m⁻³)",     __, N3, __, __, __, P1, __),
    pub MolarVolume             => ("(m³⋅mol⁻¹)",    __, P3, __, __, __, N1, __),
    pub MolarHeatCapacity |
    pub MolarEntropy            => ("(J⋅K⁻¹⋅mol⁻¹)", P1, P2, N2, __, N1, N1, __),
    pub MolarEnergy             => ("(J⋅mol⁻¹)",     P1, P2, N2, __, __, N1, __),
    pub MolarConductivity       => ("(S⋅m²⋅mol⁻¹)",  N1, __, P3, P2, __, N1, __),
    pub Molality                => ("(mol⋅kg⁻¹)",    N1, __, __, __, __, P1, __),

    pub SurfaceDensity          => ("(kg⋅m⁻²)",      P1, N2, __, __, __, __, __),
    pub CurrentDensity          => ("(A⋅m⁻²)",       __, N2, __, P1, __, __, __),
    pub MassConcentration       => ("(kg⋅m⁻³)",      P1, N3, __, __, __, __, __),
    pub MagneticFieldStrength   => ("(A⋅m⁻¹)",       __, N1, __, P1, __, __, __),
    pub Luminance               => ("(cd⋅m⁻²)",      __, N3, __, __, __, __, P1),
}

#[cfg(test)]
mod tests {
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
        let _: Unit<f64> = Scalar::new(1.0f64);
        let _: Unit<f64> = Scalar::new(1.0f64);
    }

    #[test]
    fn test_fn() {
        let _: Unit<f64> = scalar(1.0f64);
    }

    #[test]
    fn test_const() {
        let _: Unit<f64> = SCALAR;
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
