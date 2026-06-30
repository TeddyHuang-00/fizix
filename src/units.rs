use typenum::{N1, N2, N3, N4, N6, N9, N10, N12, P1, P2, P3, P4, P5, P6, P9, P12, Z0};

use crate::alias_units;

// Help improve code readability, doesn't affect actual diagnosis
type __ = Z0;

// base units
alias_units! {
    // Name      => const (Doc,                          scale, kg,  m, s,  A,  K, mol, cd)
    pub Scalar   => const ("Dimensionless quantity",        __, __, __, __, __, __, __, __),
    pub Kilogram => const ("Mass (kg)",                     __, P1, __, __, __, __, __, __),
    pub Meter    => const ("Length (m)",                    __, __, P1, __, __, __, __, __),
    pub Second   => const ("Time (s)",                      __, __, __, P1, __, __, __, __),
    pub Ampere   => const ("Electric Current (A)",          __, __, __, __, P1, __, __, __),
    pub Kelvin   => const ("Thermodynamic temperature (K)", __, __, __, __, __, P1, __, __),
    pub Mole     => const ("Amount of substance (mol)",     __, __, __, __, __, __, P1, __),
    pub Candela  => const ("Luminous intensity (cd)",       __, __, __, __, __, __, __, P1),
}

// derived units with names
// <https://en.wikipedia.org/wiki/SI_derived_unit>
alias_units! {
    // Name       => const (Doc,                                  scale, kg,  m, s,  A,  K, mol, cd)
    pub Radian    => const ("Plane angle (rad, 1)",                  __, __, __, __, __, __, __, __),
    pub Steradian => const ("Solid angle (sr, 1)",                   __, __, __, __, __, __, __, __),
    pub Hertz     => const ("Frequency (Hz, s⁻¹)",                   __, __, __, N1, __, __, __, __),
    pub Newton    => const ("Force (N, kg⋅m⋅s⁻²)",                   __, P1, P1, N2, __, __, __, __),
    pub Pascal    => const ("Pressure (Pa, kg⋅m⁻¹⋅s⁻²)",             __, P1, N1, N2, __, __, __, __),
    pub Joule     => const ("Energy (J, kg⋅m²⋅s⁻²)",                 __, P1, P2, N2, __, __, __, __),
    pub Watt      => const ("Power (W, kg⋅m²⋅s⁻³)",                  __, P1, P2, N3, __, __, __, __),
    pub Coulomb   => const ("Electric charge (C, s⋅A)",              __, __, __, P1, P1, __, __, __),
    pub Volt      => const ("Voltage (V, kg⋅m²⋅s⁻³⋅A⁻¹)",            __, P1, P2, N3, N1, __, __, __),
    pub Ohm       => const ("Resistance (Ω, kg⋅m²⋅s⁻³⋅A⁻²)",         __, P1, P2, N3, N2, __, __, __),
    pub Siemens   => const ("Conductance (S, kg⁻¹⋅m⁻²⋅s³⋅A²)",       __, N1, N2, P3, P2, __, __, __),
    pub Farad     => const ("Capacitance (F, kg⁻¹⋅m⁻²⋅s⁴⋅A²)",       __, N1, N2, P4, P2, __, __, __),
    pub Henry     => const ("Inductance (H, kg⋅m²⋅s⁻²⋅A⁻²)",         __, P1, P2, N2, N2, __, __, __),
    pub Tesla     => const ("Magnetic flux density (T, kg⋅s⁻²⋅A⁻¹)", __, P1, __, N2, N1, __, __, __),
    pub Weber     => const ("Magnetic flux (Wb, kg⋅m²⋅s⁻²⋅A⁻¹)",     __, P1, P2, N2, N1, __, __, __),
    pub Lumen     => const ("Luminous flux (lm, cd⋅sr)",             __, __, __, __, __, __, __, P1),
    pub Lux       => const ("Illuminance (lx, cd⋅sr⋅m⁻²)",           __, __, N2, __, __, __, __, P1),
    pub Becquerel => const ("Radioactivity (Bq, s⁻¹)",               __, __, __, N1, __, __, __, __),
    pub Gray      => const ("Absorbed dose (Gy, m²⋅s⁻²)",            __, __, P2, N2, __, __, __, __),
    pub Sievert   => const ("Equivalent dose (Sv, m²⋅s⁻²)",          __, __, P2, N2, __, __, __, __),
    pub Katal     => const ("Catalytic activity (kat, s⁻¹⋅mol)",     __, __, __, N1, __, __, P1, __),
}

// derived units (types-only)
// <https://en.wikipedia.org/wiki/International_System_of_Units#Coherent_and_non-coherent_SI_units>
// <https://en.wikipedia.org/wiki/SI_derived_unit#By_field_of_application>
alias_units! {
    // Kinematics
    // Name                 => (Doc,       scale, kg,  m, s,  A,  K, mol, cd)
    pub Speed |
    pub Velocity            => ("(m⋅s⁻¹)",    __, __, P1, N1, __, __, __, __),
    pub Acceleration        => ("(m⋅s⁻²)",    __, __, P1, N2, __, __, __, __),
    pub Jerk |
    pub Jolt                => ("(m⋅s⁻³)",    __, __, P1, N3, __, __, __, __),
    pub Snap |
    pub Jounce              => ("(m⋅s⁻⁴)",    __, __, P1, N4, __, __, __, __),
    pub Yank                => ("(kg⋅m⋅s⁻³)", __, P1, P1, N3, __, __, __, __),
    pub AngularVelocity     => ("(rad⋅s⁻¹)",  __, __, __, N1, __, __, __, __),
    pub AngularAcceleration => ("(rad⋅s⁻²)",  __, __, __, N2, __, __, __, __),
    pub FrequencyDrift      => ("(Hz⋅s⁻¹)",   __, __, __, N2, __, __, __, __),
    pub VolumetricFlow      => ("(m³⋅s⁻¹)",   __, __, P3, N1, __, __, __, __),

    // Mechanics
    // Name                     => (Doc,         scale, kg,  m, s,  A,  K, mol, cd)
    pub Area                    => ("(m²)",         __, __, P2, __, __, __, __, __),
    pub Volume                  => ("(m³)",         __, __, P3, __, __, __, __, __),
    pub Momentum |
    pub Impulse                 => ("(N⋅s)",        __, P1, P1, N1, __, __, __, __),
    pub AngularMomentum         => ("(N⋅m⋅s)",      __, P1, P2, N1, __, __, __, __),
    pub Torque |
    pub MomentOfForce           => ("(N⋅m)",        __, P1, P2, N2, __, __, __, __),
    pub WaveNumber |
    pub OpticalPower |
    pub Curvature |
    pub Vergence |
    pub SpatialFrequency        => ("(m⁻¹)",        __, __, N1, __, __, __, __, __),
    pub AreaDensity             => ("(kg⋅m⁻²)",     __, P1, N2, __, __, __, __, __),
    pub Density |
    pub MassDensity             => ("(kg⋅m⁻³)",     __, P1, N3, __, __, __, __, __),
    pub SpecificVolume          => ("(m³⋅kg⁻¹)",    __, N1, P3, __, __, __, __, __),
    pub Action                  => ("(J⋅s)",        __, P1, P2, N1, __, __, __, __),
    pub SpecificEnergy          => ("(J⋅kg⁻¹)",     __, __, P2, N2, __, __, __, __),
    pub EnergyDensity           => ("(J⋅m⁻³)",      __, P1, N1, N2, __, __, __, __),
    pub SurfaceTension |
    pub Stiffness               => ("(N⋅m⁻¹)",      __, P1, __, N2, __, __, __, __),
    pub HeatFluxDensity |
    pub Irradiance              => ("(W⋅m⁻²)",      __, P1, __, N3, __, __, __, __),
    pub KinematicViscosity |
    pub ThermalDiffusivity |
    pub DiffusionCoefficient    => ("(m²⋅s⁻¹)",     __, __, P2, N1, __, __, __, __),
    pub DynamicViscosity        => ("(Pa⋅s)",       __, P1, N1, N1, __, __, __, __),
    pub LinearMassDensity       => ("(kg⋅m⁻¹)",     __, P1, N1, __, __, __, __, __),
    pub MassFlowRate            => ("(kg⋅s⁻¹)",     __, P1, __, N1, __, __, __, __),
    pub Radiance                => ("(W⋅sr⁻¹⋅m⁻²)", __, P1, __, N3, __, __, __, __),
    pub SpectralPower           => ("(W⋅m⁻¹)",      __, P1, P1, N3, __, __, __, __),
    pub AbsorbedDoseRate        => ("(Gy⋅s⁻¹)",     __, __, P2, N3, __, __, __, __),
    pub FuelEfficiency          => ("(m⋅m⁻³)",      __, __, N2, __, __, __, __, __),
    pub SpectralIrradiance |
    pub PowerDensity            => ("(Gy⋅s⁻¹)",     __, P1, N1, N3, __, __, __, __),
    pub EnergyFluxDensity       => ("(J⋅m⁻²⋅s⁻¹)",  __, P1, __, N3, __, __, __, __),
    pub Compressibility         => ("(Pa⁻¹)",       __, N1, P1, P2, __, __, __, __),
    pub RadiantExposure         => ("(J⋅m⁻²)",      __, P1, __, N2, __, __, __, __),
    pub MomentOfInertia         => ("(kg⋅m²)",      __, P1, P2, __, __, __, __, __),
    pub SpecificAngularMomentum => ("(N⋅m⋅s⋅kg⁻¹)", __, __, P2, N1, __, __, __, __),
    pub RadiantIntensity        => ("(W⋅sr⁻¹)",     __, P1, P2, N3, __, __, __, __),
    pub SpectralIntensity       => ("(W⋅sr⁻¹⋅m⁻¹)", __, P1, P1, N3, __, __, __, __),

    // Chemistry
    // Name                 => (Doc,           scale, kg,  m, s,  A,  K, mol, cd)
    pub Molarity |
    pub Concentration       => ("(mol⋅m⁻³)",      __, __, N3, __, __, __, P1, __),
    pub MolarVolume         => ("(m³⋅mol⁻¹)",     __, __, P3, __, __, __, N1, __),
    pub MolarHeatCapacity |
    pub MolarEntropy        => ("(J⋅K⁻¹⋅mol⁻¹)",  __, P1, P2, N2, __, N1, N1, __),
    pub MolarEnergy         => ("(J⋅mol⁻¹)",      __, P1, P2, N2, __, __, N1, __),
    pub MolarConductivity   => ("(S⋅m²⋅mol⁻¹)",   __, N1, __, P3, P2, __, N1, __),
    pub Molality            => ("(mol⋅kg⁻¹)",     __, N1, __, __, __, __, P1, __),
    pub MolarMass           => ("(kg⋅mol⁻¹)",     __, P1, __, __, __, __, N1, __),
    pub CatalyticEfficiency => ("(m³⋅mol⁻¹⋅s⁻¹)", __, __, P3, N1, __, __, N1, __),

    // Electromagnetics
    // Name                     => (Doc,         scale, kg,  m, s,  A,  K, mol, cd)
    pub LinearChargeDensity     => ("(C⋅m⁻¹)",      __, __, N1, P1, P1, __, __, __),
    pub SurfaceChargeDensity |
    pub PolarizationDensity |
    pub ElectricFluxDensity     => ("(C⋅m⁻²)",      __, __, N2, P1, P1, __, __, __),
    pub VolumeChargeDensity     => ("(C⋅m⁻³)",      __, __, N3, P1, P1, __, __, __),
    pub Magnetization |
    pub MagneticFieldStrength   => ("(A⋅m⁻¹)",      __, __, N1, __, P1, __, __, __),
    pub CurrentDensity          => ("(A⋅m⁻²)",      __, __, N2, __, P1, __, __, __),
    pub ElectricField           => ("(V⋅m⁻¹)",      __, P1, P1, N3, N1, __, __, __),
    pub ElectricalConductivity  => ("(S⋅m⁻¹)",      __, N1, N3, P3, P2, __, __, __),
    pub Permittivity            => ("(F⋅m⁻¹)",      __, N1, N3, P4, P2, __, __, __),
    pub Permeability            => ("(H⋅m⁻¹)",      __, P1, P1, N2, N2, __, __, __),
    pub MagneticVectorPotential => ("(Wb⋅m⁻¹)",     __, P1, P1, N2, N1, __, __, __),
    pub ElectricDipoleMoment    => ("(C⋅m)",        __, __, P1, P1, P1, __, __, __),
    pub MagneticMoment          => ("(A⋅m²)",       __, __, P2, __, P1, __, __, __),
    pub ElectricFlux            => ("(V⋅m)",        __, P1, P3, N3, N1, __, __, __),
    pub ElectricalResistivity   => ("(Ω⋅m)",        __, P1, P3, N3, N2, __, __, __),
    pub MagneticRigidity        => ("(T⋅m)",        __, P1, P1, N2, N1, __, __, __),
    pub MagneticReluctance      => ("(H⁻¹)",        __, N1, N2, P2, P2, __, __, __),
    pub ComplexPower |
    pub ApparentPower           => ("(V⋅A)",        __, P1, P2, N3, __, __, __, __),
    pub ElectronMobility        => ("(m²⋅V⁻¹⋅s⁻¹)", __, N1, __, P2, P1, __, __, __),
    pub Exposure                => ("(C⋅kg⁻¹)",     __, N1, __, P1, P1, __, __, __),

    // Photometry
    // Name              => (Doc,     scale, kg,  m, s,  A,  K, mol, cd)
    pub LuminousEnergy   => ("(lm⋅s)",   __, __, __, P1, __, __, __, P1),
    pub LuminousExposure => ("(lx⋅s)",   __, __, N2, P1, __, __, __, P1),
    pub LuminousEfficacy => ("(lm⋅W⁻¹)", __, N1, N2, P3, __, __, __, P1),
    pub Luminance        => ("(cd⋅m⁻²)", __, __, N2, __, __, __, __, P1),

    // Thermodynamics
    // Name                         => (Doc,         scale, kg,  m, s,  A,  K, mol, cd)
    pub HeatCapacity |
    pub Entropy                     => ("(J⋅K⁻¹)",      __, P1, P2, N2, __, N1, __, __),
    pub SpecificHeatCapacity |
    pub SpecificEntropy             => ("(J⋅K⁻¹⋅kg⁻¹)", __, __, P2, N2, __, N1, __, __),
    pub ThermalConductivity         => ("(W⋅m⁻¹⋅K⁻¹)",  __, P1, P1, N3, __, N1, __, __),
    pub ThermalResistance           => ("(K⋅W⁻¹)",      __, N1, N2, P3, __, P1, __, __),
    pub ThermalExpansionCoefficient => ("(K⁻¹)",        __, __, __, __, __, N1, __, __),
    pub TemperatureGradient         => ("(K⋅m⁻¹)",      __, __, N1, __, __, P1, __, __),
}

// Same types of different common scales
alias_units! {
    // Name        => const (Doc,       scale, kg,  m, s,  A,  K, mol, cd)
    pub Tonne      => const ("Mass (t)",   P3, P1, __, __, __, __, __, __),
    pub Gram       => const ("Mass (g)",   N3, P1, __, __, __, __, __, __),
    pub Milligram  => const ("Mass (mg)",  N6, P1, __, __, __, __, __, __),
    pub Microgram  => const ("Mass (µg)",  N9, P1, __, __, __, __, __, __),
    pub Nanogram   => const ("Mass (ng)", N12, P1, __, __, __, __, __, __),

    // Name        => const (Doc,        scale, kg,  m, s,  A,  K, mol, cd)
    pub Kilometer  => const ("Length (km)", P3, __, P1, __, __, __, __, __),
    pub Decimeter  => const ("Length (dm)", N1, __, P1, __, __, __, __, __),
    pub Centimeter => const ("Length (cm)", N2, __, P1, __, __, __, __, __),
    pub Millimeter => const ("Length (mm)", N3, __, P1, __, __, __, __, __),
    pub Micrometer => const ("Length (µm)", N6, __, P1, __, __, __, __, __),
    pub Nanometer  => const ("Length (nm)", N9, __, P1, __, __, __, __, __),
    pub Angstrom   => const ("Length (Å)", N10, P1, __, __, __, __, __, __),

    // Name         => const (Doc,      scale, kg,  m, s,  A,  K, mol, cd)
    pub Millisecond => const ("Time (ms)", N3, __, __, P1, __, __, __, __),
    pub Microsecond => const ("Time (µs)", N6, __, __, P1, __, __, __, __),
    pub Nanosecond  => const ("Time (ns)", N9, __, __, P1, __, __, __, __),

    // Name         => const (Doc,                  scale, kg,  m, s,  A,  K, mol, cd)
    pub Kiloampere  => const ("Electric Current (kA)", P3, __, __, __, P1, __, __, __),
    pub Milliampere => const ("Electric Current (mA)", N3, __, __, __, P1, __, __, __),
    pub Microampere => const ("Electric Current (µA)", N6, __, __, __, P1, __, __, __),

    // Name        => const (Doc,        scale, kg,  m, s,  A,  K, mol, cd)
    pub Litre      => const ("Volume (L)",  N3, __, P3, __, __, __, __, __),
    pub Millilitre => const ("Volume (mL)", N6, __, P3, __, __, __, __, __),
    pub Microlitre => const ("Volume (µL)", N6, __, P3, __, __, __, __, __),

    // Name       => const (Doc,             scale, kg,  m, s,  A,  K, mol, cd)
    pub Terahertz => const ("Frequency (THz)", P12, __, __, N1, __, __, __, __),
    pub Gigahertz => const ("Frequency (GHz)",  P9, __, __, N1, __, __, __, __),
    pub Megahertz => const ("Frequency (MHz)",  P6, __, __, N1, __, __, __, __),
    pub Kilohertz => const ("Frequency (kHz)",  P3, __, __, N1, __, __, __, __),

    // Name        => const (Doc,       scale, kg,  m, s,  A,  K, mol, cd)
    pub Kilonewton => const ("Force (kN)", P3, P1, P1, N2, __, __, __, __),

    // Name        => const (Doc,           scale, kg,  m, s,  A,  K, mol, cd)
    pub Gigapascal => const ("Pressure (GPa)", P9, P1, N1, N2, __, __, __, __),
    pub Megapascal => const ("Pressure (MPa)", P6, P1, N1, N2, __, __, __, __),
    pub Bar        => const ("Pressure (bar)", P5, P1, N1, N2, __, __, __, __),
    pub Kilopascal => const ("Pressure (kPa)", P3, P1, N1, N2, __, __, __, __),

    // Name       => const (Doc,        scale, kg,  m, s,  A,  K, mol, cd)
    pub Kilojoule => const ("Energy (kJ)", P3, P1, P2, N2, __, __, __, __),

    // Name      => const (Doc,       scale, kg,  m, s,  A,  K, mol, cd)
    pub Gigawatt => const ("Power (GW)", P9, P1, P2, N3, __, __, __, __),
    pub Megawatt => const ("Power (MW)", P6, P1, P2, N3, __, __, __, __),
    pub Kilowatt => const ("Power (kW)", P3, P1, P2, N3, __, __, __, __),

    // Name       => const (Doc,         scale, kg,  m, s,  A,  K, mol, cd)
    pub Megavolt  => const ("Voltage (MV)", P6, P1, P2, N3, N1, __, __, __),
    pub Kilovolt  => const ("Voltage (kV)", P3, P1, P2, N3, N1, __, __, __),
    pub Millivolt => const ("Voltage (mV)", N3, P1, P2, N3, N1, __, __, __),
    pub Microvolt => const ("Voltage (µV)", N6, P1, P2, N3, N1, __, __, __),

    // Name     => const (Doc,            scale, kg,  m, s,  A,  K, mol, cd)
    pub Megaohm => const ("Resistance (MΩ)", P6, P1, P2, N3, N2, __, __, __),
    pub Kiloohm => const ("Resistance (kΩ)", P3, P1, P2, N3, N2, __, __, __),

    // Name        => const (Doc,             scale, kg,  m, s,  A,  K, mol, cd)
    pub Millifarad => const ("Capacitance (mF)", N3, N1, N2, P4, P2, __, __, __),
    pub Microfarad => const ("Capacitance (µF)", N6, N1, N2, P4, P2, __, __, __),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Unit;

    /// Helper macro to turn type-only aliases into concrete values.
    ///
    /// Accepts expressions with `*` and `/` operators and `^ N` powers.
    /// Each ident is expanded to `ident::new(1.0f64)`.
    /// Operators are preserved as-is (left-associative, no parenthesis).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// eval!(Speed)              → Speed::new(1.0f64)
    /// eval!(Meter / Second)     → Meter::new(1.0f64) / Second::new(1.0f64)
    /// eval!(Meter / Second ^ 2) → Meter::new(1.0f64) / (Second::new(1.0f64) * Second::new(1.0f64))
    /// ```
    macro_rules! eval {
        // term expansion
        (@term $id:ident $(^ 1)?) => { $id::new(1.0f64) };
        (@term $id:ident ^ 2) => { $id::new(1.0f64) * eval!(@term $id) };
        (@term $id:ident ^ 3) => { $id::new(1.0f64) * eval!(@term $id ^ 2) };
        (@term $id:ident ^ 4) => { $id::new(1.0f64) * eval!(@term $id ^ 3) };

        // entry: single term, no operators
        ($id:ident) => { eval!(@term $id) };
        ($id:ident ^ $fe:tt) => { eval!(@term $id ^ $fe) };

        // entry: first term + ^ N + rest
        ($id:ident ^ $fe:tt $($rest:tt)+) => {
            eval!(@munch (eval!(@term $id ^ $fe)) $($rest)+)
        };
        // entry: first term + rest (no power)
        ($id:ident $($rest:tt)+) => {
            eval!(@munch (eval!(@term $id)) $($rest)+)
        };

        // muncher: op + ident + power + rest
        (@munch ($($acc:tt)*) $op:tt $next:ident ^ $e:tt $($rest:tt)+) => {
            eval!(@munch (($($acc)*) $op (eval!(@term $next ^ $e))) $($rest)+)
        };
        // muncher: op + ident + power, end
        (@munch ($($acc:tt)*) $op:tt $next:ident ^ $e:tt) => {
            ($($acc)*) $op (eval!(@term $next ^ $e))
        };
        // muncher: op + ident + rest (no power)
        (@munch ($($acc:tt)*) $op:tt $next:ident $($rest:tt)+) => {
            eval!(@munch (($($acc)*) $op (eval!(@term $next))) $($rest)+)
        };
        // muncher: op + ident, end (no power)
        (@munch ($($acc:tt)*) $op:tt $next:ident) => {
            ($($acc)*) $op (eval!(@term $next))
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
    fn test_derived_kinematics_type() {
        assert_eq!(eval!(Speed), eval!(Meter / Second));
        assert_eq!(eval!(Acceleration), eval!(Meter / Second ^ 2));
        assert_eq!(eval!(Jerk), eval!(Meter / Second ^ 3));
        assert_eq!(eval!(Snap), eval!(Meter / Second ^ 4));
        assert_eq!(eval!(Yank), eval!(Kilogram * Meter / Second ^ 3));
        assert_eq!(eval!(AngularVelocity), eval!(Radian / Second));
        assert_eq!(eval!(AngularAcceleration), eval!(Radian / Second ^ 2));
        assert_eq!(eval!(FrequencyDrift), eval!(Hertz / Second));
        assert_eq!(eval!(VolumetricFlow), eval!(Meter ^ 3 / Second));
    }

    #[test]
    fn test_derived_mechanics_type() {
        assert_eq!(eval!(Area), eval!(Meter ^ 2));
        assert_eq!(eval!(Volume), eval!(Meter ^ 3));
        assert_eq!(eval!(Momentum), eval!(Newton * Second));
        assert_eq!(eval!(AngularMomentum), eval!(Newton * Meter * Second));
        assert_eq!(eval!(Torque), eval!(Joule / Radian));
        assert_eq!(eval!(MomentOfForce), eval!(Newton * Meter));
        assert_eq!(eval!(WaveNumber), eval!(Scalar / Meter));
        assert_eq!(eval!(AreaDensity), eval!(Kilogram / Meter ^ 2));
        assert_eq!(eval!(Density), eval!(Kilogram / Meter ^ 3));
        assert_eq!(eval!(SpecificVolume), eval!(Meter ^ 3 / Kilogram));
        assert_eq!(eval!(Action), eval!(Joule * Second));
        assert_eq!(eval!(SpecificEnergy), eval!(Joule / Kilogram));
        assert_eq!(eval!(SurfaceTension), eval!(Joule / Meter ^ 2));
        assert_eq!(eval!(Stiffness), eval!(Newton / Meter));
        assert_eq!(eval!(HeatFluxDensity), eval!(Watt / Meter ^ 2));
        assert_eq!(eval!(KinematicViscosity), eval!(Meter ^ 2 / Second));
        assert_eq!(eval!(DynamicViscosity), eval!(Pascal * Second));
        assert_eq!(eval!(LinearMassDensity), eval!(Kilogram / Meter));
        assert_eq!(eval!(MassFlowRate), eval!(Kilogram / Second));
        assert_eq!(eval!(Radiance), eval!(Watt / Steradian / Meter ^ 2));
        assert_eq!(eval!(SpectralPower), eval!(Watt / Meter));
        assert_eq!(eval!(AbsorbedDoseRate), eval!(Gray / Second));
        assert_eq!(eval!(FuelEfficiency), eval!(Meter / Meter ^ 3));
        assert_eq!(eval!(SpectralIrradiance), eval!(Watt / Meter ^ 3));
        assert_eq!(eval!(EnergyFluxDensity), eval!(Joule / Meter ^ 2 / Second));
        assert_eq!(eval!(Compressibility), eval!(Scalar / Pascal));
        assert_eq!(eval!(RadiantExposure), eval!(Joule / Meter ^ 2));
        assert_eq!(eval!(MomentOfInertia), eval!(Kilogram * Meter ^ 2));
        assert_eq!(
            eval!(SpecificAngularMomentum),
            eval!(Newton * Meter * Second / Kilogram)
        );
        assert_eq!(eval!(RadiantIntensity), eval!(Watt / Steradian));
        assert_eq!(eval!(SpectralIntensity), eval!(Watt / Steradian / Meter));
    }

    #[test]
    fn test_derived_chemistry_type() {
        assert_eq!(eval!(Molarity), eval!(Mole / Meter ^ 3));
        assert_eq!(eval!(MolarVolume), eval!(Meter ^ 3 / Mole));
        assert_eq!(eval!(MolarHeatCapacity), eval!(Joule / Kelvin / Mole));
        assert_eq!(eval!(MolarEnergy), eval!(Joule / Mole));
        assert_eq!(eval!(MolarConductivity), eval!(Siemens * Meter ^ 2 / Mole));
        assert_eq!(eval!(Molality), eval!(Mole / Kilogram));
        assert_eq!(eval!(MolarMass), eval!(Kilogram / Mole));
        assert_eq!(eval!(CatalyticEfficiency), eval!(Meter ^ 3 / Mole / Second));
    }

    #[test]
    fn test_derived_electromagnetics_type() {
        assert_eq!(eval!(LinearChargeDensity), eval!(Coulomb / Meter));
        assert_eq!(eval!(SurfaceChargeDensity), eval!(Coulomb / Meter ^ 2));
        assert_eq!(eval!(VolumeChargeDensity), eval!(Coulomb / Meter ^ 3));
        assert_eq!(eval!(Magnetization), eval!(Ampere / Meter));
        assert_eq!(eval!(CurrentDensity), eval!(Ampere / Meter ^ 2));
        assert_eq!(eval!(ElectricField), eval!(Volt / Meter));
        assert_eq!(eval!(ElectricalConductivity), eval!(Siemens / Meter));
        assert_eq!(eval!(Permittivity), eval!(Farad / Meter));
        assert_eq!(eval!(Permeability), eval!(Henry / Meter));
        assert_eq!(eval!(MagneticVectorPotential), eval!(Weber / Meter));
        assert_eq!(eval!(ElectricDipoleMoment), eval!(Coulomb * Meter));
        assert_eq!(eval!(MagneticMoment), eval!(Ampere * Meter ^ 2));
        assert_eq!(eval!(ElectricFlux), eval!(Volt * Meter));
        assert_eq!(eval!(ElectricalResistivity), eval!(Ohm * Meter));
        assert_eq!(eval!(MagneticRigidity), eval!(Tesla * Meter));
        assert_eq!(eval!(MagneticReluctance), eval!(Scalar / Henry));
        assert_eq!(eval!(ComplexPower), eval!(Volt * Ampere));
        assert_eq!(eval!(ElectronMobility), eval!(Meter ^ 2 / Volt / Second));
        assert_eq!(eval!(Exposure), eval!(Coulomb / Kilogram));
    }

    #[test]
    fn test_derived_photometry_type() {
        assert_eq!(eval!(LuminousEnergy), eval!(Lumen * Second));
        assert_eq!(eval!(LuminousExposure), eval!(Lux * Second));
        assert_eq!(eval!(Luminance), eval!(Candela / Meter ^ 2));
        assert_eq!(eval!(LuminousEfficacy), eval!(Lumen / Watt));
    }

    #[test]
    fn test_derived_thermodynamics_type() {
        assert_eq!(eval!(Entropy), eval!(Joule / Kelvin));
        assert_eq!(
            eval!(SpecificHeatCapacity),
            eval!(Joule / Kelvin / Kilogram)
        );
        assert_eq!(eval!(ThermalConductivity), eval!(Watt / Meter / Kelvin));
        assert_eq!(eval!(ThermalResistance), eval!(Kelvin / Watt));
        assert_eq!(eval!(ThermalExpansionCoefficient), eval!(Scalar / Kelvin));
        assert_eq!(eval!(TemperatureGradient), eval!(Kelvin / Meter));
    }
}
