use crate::Unit;

// Helper structs: let users write `Mole<f64>::new(1.0)` instead of
// `Unit<f64, 0, 0, 0, 0, 0, 1>::new(1.0)`
macro_rules! alias_units {
    ($(($name:ident, $doc:literal, $($val:expr),+)),+$(,)?) => {
        $(
            #[doc = $doc]
            pub type $name<V> = Unit<V$(, $val)+>;
        )+
    };
}

alias_units! {
    // (Name, Doc string,                     M, G, S, A, K, O, C)
    // base units
    (Scalar, "Dimensionless quantity",        0, 0, 0, 0, 0, 0, 0),
    (Meter, "Length (m)",                     1, 0, 0, 0, 0, 0, 0),
    (Kilogram, "Mass (kg)",                   0, 1, 0, 0, 0, 0, 0),
    (Second, "Time (s)",                      0, 0, 1, 0, 0, 0, 0),
    (Amphere, "Electric Current (A)",         0, 0, 0, 1, 0, 0, 0),
    (Kelvin, "Thermodynamic temperature (K)", 0, 0, 0, 0, 1, 0, 0),
    (Mole, "Amount of substance (mol)",       0, 0, 0, 0, 0, 1, 0),
    (Candela, "Luminous intensity (cd)",      0, 0, 0, 0, 0, 0, 1),
    // derived units
    (Newton, "Force in newton (N, kg·m·s⁻²)",   1, 1, -2, 0, 0, 0, 0),
    (Joule, "Energy in joule (J, N·m)",         2, 1, -2, 0, 0, 0, 0),
    (Watt, "Power in watt (W, J·s⁻¹)",          2, 1, -3, 0, 0, 0, 0),
    (Pascal, "Pressure in pascal (Pa, N·m⁻²)", -1, 1, -2, 0, 0, 0, 0),
    (Hertz, "Frequency in hertz (Hz, s⁻¹)",     0, 0, -1, 0, 0, 0, 0),
    // convenience aliases
    (Speed, "Speed in m·s⁻¹",                             1, 0, -1, 0, 0, 0, 0),
    (Acceleration, "Acceleration in m·s⁻²",               1, 0, -2, 0, 0, 0, 0),
    (Area, "Area in m²",                                  2, 0,  0, 0, 0, 0, 0),
    (Volume, "Volume in m³",                              3, 0,  0, 0, 0, 0, 0),
    (Momentum, "Momentum in kg·m·s⁻¹",                    1, 1, -1, 0, 0, 0, 0),
    (AngularMomentum, "Angular momentum in kg·m²·s⁻¹",    2, 1, -1, 0, 0, 0, 0),
    (Torque, "Torque in N·m",                             2, 1, -2, 0, 0, 0, 0),
    (Density, "Density in kg·m⁻³",                       -3, 1,  0, 0, 0, 0, 0),
    (Viscosity, "Dynamic viscosity in Pa·s",             -1, 1, -1, 0, 0, 0, 0),
    (KinematicViscosity, "Kinematic viscosity in m²·s⁻¹", 2, 0, -1, 0, 0, 0, 0),
}
