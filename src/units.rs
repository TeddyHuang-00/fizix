use paste::paste;
use typenum::{N1, N2, N3, P1, P2, P3, P4, Z0};

use crate::Unit;

macro_rules! alias_units {
    ($(($name:ident, $doc:literal, $($dim:ty),+)),+$(,)?) => {
        $(
            paste! {
                #[doc = $doc]
                pub type $name<V> = Unit<V, $($dim),+>;

                #[doc = $doc]
                #[inline]
                pub const fn [<$name:snake>]<V>(v: V) -> $name<V> {
                    $name::new(v)
                }

                #[doc = $doc]
                pub const [<$name:snake:upper>]: $name<f64> = $name::new(1.0);
            }
        )+
    };
}

alias_units! {
    // (Name, Doc string,                       M,  G,  S,  A,  K,  O,  C )
    // base units
    (Scalar,   "Dimensionless quantity",        Z0, Z0, Z0, Z0, Z0, Z0, Z0),
    (Meter,    "Length (m)",                    P1, Z0, Z0, Z0, Z0, Z0, Z0),
    (Kilogram, "Mass (kg)",                     Z0, P1, Z0, Z0, Z0, Z0, Z0),
    (Second,   "Time (s)",                      Z0, Z0, P1, Z0, Z0, Z0, Z0),
    (Ampere,   "Electric Current (A)",          Z0, Z0, Z0, P1, Z0, Z0, Z0),
    (Kelvin,   "Thermodynamic temperature (K)", Z0, Z0, Z0, Z0, P1, Z0, Z0),
    (Mole,     "Amount of substance (mol)",     Z0, Z0, Z0, Z0, Z0, P1, Z0),
    (Candela,  "Luminous intensity (cd)",       Z0, Z0, Z0, Z0, Z0, Z0, P1),
    // derived units
    (Newton,   "Force (N, kg·m·s⁻²)",                   P1, P1, N2, Z0, Z0, Z0, Z0),
    (Joule,    "Energy (J, N·m)",                       P2, P1, N2, Z0, Z0, Z0, Z0),
    (Watt,     "Power (W, J·s⁻¹)",                      P2, P1, N3, Z0, Z0, Z0, Z0),
    (Pascal,   "Pressure (Pa, N·m⁻²)",                  N1, P1, N2, Z0, Z0, Z0, Z0),
    (Hertz,    "Frequency (Hz, s⁻¹)",                   Z0, Z0, N1, Z0, Z0, Z0, Z0),
    (Coulomb,  "Electric charge (C, s·A)",              Z0, Z0, P1, P1, Z0, Z0, Z0),
    (Volt,     "Voltage (V, kg·m²·s⁻³·A⁻¹)",            P2, P1, N3, N1, Z0, Z0, Z0),
    (Ohm,      "Resistance (Ohm, kg·m²·s⁻³·A⁻²)",       P2, P1, N3, N2, Z0, Z0, Z0),
    (Siemens,  "Conductance (S, kg⁻¹·m⁻²·s³·A²)",       N2, N1, P3, P2, Z0, Z0, Z0),
    (Farad,    "Capacitance (F, kg⁻¹·m⁻²·s⁴·A²)",       N2, N1, P4, P2, Z0, Z0, Z0),
    (Tesla,    "Magnetic flux density (T, kg·s⁻²·A⁻¹)", Z0, P1, N2, N1, Z0, Z0, Z0),
    (Weber,    "Magnetic flux (Wb, kg·m²·s⁻²·A⁻¹)",     P2, P1, N2, N1, Z0, Z0, Z0),
    (Henry,    "Inductance (H, kg·m²·s⁻²·A⁻²)",         P2, P1, N2, N2, Z0, Z0, Z0),
    (Gray,     "Absorbed dose (Gy, m²·s⁻²)",            P2, Z0, N2, Z0, Z0, Z0, Z0),
    (Becquerel,"Radioactivity (Bq, s⁻¹)",               Z0, Z0, N1, Z0, Z0, Z0, Z0),
    (Lux,      "Illuminance (lx, cd·m⁻²)",              N2, Z0, Z0, Z0, Z0, Z0, P1),
    // convenience
    (Speed,              "(m·s⁻¹)",     P1, Z0, N1, Z0, Z0, Z0, Z0),
    (Acceleration,       "(m·s⁻²)",     P1, Z0, N2, Z0, Z0, Z0, Z0),
    (Area,               "(m²)",        P2, Z0, Z0, Z0, Z0, Z0, Z0),
    (Volume,             "(m³)",        P3, Z0, Z0, Z0, Z0, Z0, Z0),
    (Momentum,           "(kg·m·s⁻¹)",  P1, P1, N1, Z0, Z0, Z0, Z0),
    (AngularMomentum,    "(kg·m²·s⁻¹)", P2, P1, N1, Z0, Z0, Z0, Z0),
    (Torque,             "(N·m)",       P2, P1, N2, Z0, Z0, Z0, Z0),
    (Density,            "(kg·m⁻³)",    N3, P1, Z0, Z0, Z0, Z0, Z0),
    (DynamicViscosity,   "(Pa·s)",      N1, P1, N1, Z0, Z0, Z0, Z0),
    (KinematicViscosity, "(m²·s⁻¹)",    P2, Z0, N1, Z0, Z0, Z0, Z0),
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

    #[test]
    fn test_derived() {
        assert_eq!(NEWTON, KILOGRAM * METER / SECOND / SECOND);
        assert_eq!(JOULE, NEWTON * METER);
        assert_eq!(WATT, JOULE / SECOND);
        assert_eq!(PASCAL, NEWTON / (METER * METER));
        assert_eq!(HERTZ, SCALAR / SECOND);
    }

    #[test]
    fn test_convenience() {
        assert_eq!(SPEED, METER / SECOND);
        assert_eq!(ACCELERATION, SPEED / SECOND);
        assert_eq!(AREA, METER * METER);
        assert_eq!(VOLUME, AREA * METER);
        assert_eq!(MOMENTUM, KILOGRAM * SPEED);
        assert_eq!(ANGULAR_MOMENTUM, METER * MOMENTUM);
        assert_eq!(TORQUE, METER * NEWTON);
        assert_eq!(DENSITY, KILOGRAM / VOLUME);
        assert_eq!(DYNAMIC_VISCOSITY, PASCAL * SECOND);
        assert_eq!(KINEMATIC_VISCOSITY, DYNAMIC_VISCOSITY / DENSITY);
    }

    #[test]
    fn test_em_derived() {
        assert_eq!(COULOMB, SECOND * AMPERE);
        assert_eq!(VOLT, WATT / AMPERE);
        assert_eq!(OHM, VOLT / AMPERE);
        assert_eq!(SIEMENS, SCALAR / OHM);
        assert_eq!(FARAD, COULOMB / VOLT);
        assert_eq!(TESLA, WEBER / (METER * METER));
        assert_eq!(HENRY, WEBER / AMPERE);
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
