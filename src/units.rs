use paste::paste;

use crate::Unit;

// Helper structs/functions/constants: let users write:
// - `Mole<f64>::new(1.0)`, or
// - `mole(1.0f64)`, or
// - `MOLE`
// instead of `Unit<f64, 0, 0, 0, 0, 0, 1>::new(1.0)`
macro_rules! alias_units {
    ($(($name:ident, $doc:literal, $($val:expr),+)),+$(,)?) => {
        $(
            paste! {
                #[doc = $doc]
                pub type $name<V> = Unit<V$(, $val)+>;

                #[doc = $doc]
                pub const fn [<$name:snake>]<V>(v: V) -> $name::<V> {
                    $name::<V>::new(v)
                }

                #[doc = $doc]
                pub const [<$name:snake:upper>]: $name<f64> =
                    [<$name:snake>](1.0);
            }
        )+
    };
}

alias_units! {
    // (Name, Doc string,                     M, G, S, A, K, O, C)
    // base units
    (Scalar,   "Dimensionless quantity",        0, 0, 0, 0, 0, 0, 0),
    (Meter,    "Length (m)",                    1, 0, 0, 0, 0, 0, 0),
    (Kilogram, "Mass (kg)",                     0, 1, 0, 0, 0, 0, 0),
    (Second,   "Time (s)",                      0, 0, 1, 0, 0, 0, 0),
    (Amphere,  "Electric Current (A)",          0, 0, 0, 1, 0, 0, 0),
    (Kelvin,   "Thermodynamic temperature (K)", 0, 0, 0, 0, 1, 0, 0),
    (Mole,     "Amount of substance (mol)",     0, 0, 0, 0, 0, 1, 0),
    (Candela,  "Luminous intensity (cd)",       0, 0, 0, 0, 0, 0, 1),
    // derived units
    (Newton, "Force in newton (N, kg·m·s⁻²)",   1, 1, -2, 0, 0, 0, 0),
    (Joule,  "Energy in joule (J, N·m)",        2, 1, -2, 0, 0, 0, 0),
    (Watt,   "Power in watt (W, J·s⁻¹)",        2, 1, -3, 0, 0, 0, 0),
    (Pascal, "Pressure in pascal (Pa, N·m⁻²)", -1, 1, -2, 0, 0, 0, 0),
    (Hertz,  "Frequency in hertz (Hz, s⁻¹)",    0, 0, -1, 0, 0, 0, 0),
    // convenience aliases
    (Speed,              "Speed in m·s⁻¹",                1, 0, -1, 0, 0, 0, 0),
    (Acceleration,       "Acceleration in m·s⁻²",         1, 0, -2, 0, 0, 0, 0),
    (Area,               "Area in m²",                    2, 0,  0, 0, 0, 0, 0),
    (Volume,             "Volume in m³",                  3, 0,  0, 0, 0, 0, 0),
    (Momentum,           "Momentum in kg·m·s⁻¹",          1, 1, -1, 0, 0, 0, 0),
    (AngularMomentum,    "Angular momentum in kg·m²·s⁻¹", 2, 1, -1, 0, 0, 0, 0),
    (Torque,             "Torque in N·m",                 2, 1, -2, 0, 0, 0, 0),
    (Density,            "Density in kg·m⁻³",            -3, 1,  0, 0, 0, 0, 0),
    (DynamicViscosity,   "Dynamic viscosity in Pa·s",    -1, 1, -1, 0, 0, 0, 0),
    (KinematicViscosity, "Kinematic viscosity in m²·s⁻¹", 2, 0, -1, 0, 0, 0, 0),
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

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
}
