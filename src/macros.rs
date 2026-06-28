/// Create `Unit` type aliases, optionally with const helpers.
///
/// For each entry, this generates:
/// - `Name<V>` — type alias
///
/// If the entry uses `const` (after `=>`), it also generates:
/// - `name<V>(v: V) -> Name<V>` — const helper function
/// - `NAME: Name<f64>` — const value of `1.0`
///
/// Each invocation handles a single entry or multiple entries separated by commas.
/// And each entry can include a single alias or multiple aliases separated by pipes `|`.
///
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/doc/macro_syntax.md"))]
///
/// # Examples
///
/// ```rust
/// use siunit::alias_units;
/// use typenum::P1;
///
/// // Type-only alias
/// alias_units! { pub PureValue => ("Scalar type") }
///
/// // With const helpers (const fn + const value)
/// alias_units! { pub Meter => const ("Length (m)", P1) }
///
/// // Multi-alias
/// alias_units! { NameA | NameB => const ("Same unit, two aliases", P1) }
///
/// // Multi-entry
/// alias_units! {
///     // A conceptual unit type, not an actual unit
///     Force => ("Force (N)", P1, P1),
///     // This is an actual named unit so it should be const
///     Newton => const ("Force alias", P1, P1),
/// }
///
/// let _ = PureValue::new(0usize);
/// let _ = meter(5.0f64);
/// let _ = METER;
/// let _ = name_a(0usize);
/// ```
#[macro_export]
macro_rules! alias_units {
    ($(|)?$pre:vis $name:ident => ($doc:literal $(, $dim:path)*)$(,)?) => {
        #[doc = $doc]
        $pre type $name<V> = $crate::Unit<V, $($dim),*>;
    };

    ($(|)?$pre:vis $name:ident => const ($doc:literal $(, $dim:path)*)$(,)?) => {
        $crate::alias_units! { $pre $name => ($doc $(, $dim)*) }

        $crate::paste::paste! {
            #[doc = $doc]
            #[inline]
            $pre const fn [<$name:snake>]<V>(v: V) -> $name<V> {
                $name::new(v)
            }

            #[doc = $doc]
            $pre const [<$name:snake:upper>]: $name<f64> = $name::new(1.0);
        }
    };

    ($(|)?$pre:vis $name:ident|$($pres:vis $names:ident)|+ => $($const:ident)? ($doc:literal $(, $dim:path)*)$(,)?) => {
        alias_units! { $pre $name => $($const)? ($doc $(, $dim)*) }
        alias_units! { $($pres $names)|+ => $($const)? ($doc $(, $dim)*) }
    };

    (
        $(|)?$($pre:vis $name:ident)|+ => $($const:ident)? ($doc:literal $(, $dim:path)*),
        $($(|)?$($pres:vis $names:ident)|+ => $($consts:ident)? ($docs:literal $(, $dims:path)*)),+$(,)?
    ) => {
        alias_units! { $($pre $name)|+ => $($const)? ($doc $(, $dim)*) }
        alias_units! { $($($pres $names)|+ => $($consts)? ($docs $(, $dims)*)),+ }
    }
}

#[cfg(test)]
mod tests {
    use core::any::TypeId;

    use typenum::{P1, Z0};

    use crate::Unit;

    macro_rules! assert_all_eq {
        ($left:expr, $right:expr) => {
            assert_eq!($left, $right);
        };
        ($left:expr, $right:expr, $($rest:expr),+$(,)?) => {
            assert_eq!($left, $right);
            assert_all_eq!($right, $($rest),+);
        };
    }

    macro_rules! assert_all_ne {
        ($left:expr, $right:expr) => {
            assert_ne!($left, $right);
        };
        ($left:expr, $right:expr, $($rest:expr),+$(,)?) => {
            assert_ne!($left, $right);
            assert_all_ne!($left, $($rest),+);
            assert_all_ne!($right, $($rest),+);
        };
    }

    #[test]
    fn test_alias_units_scalar() {
        // Only creating a single alias
        alias_units! { Single => const ("doc") }

        // Creates two aliases with same config
        alias_units! { Left | Right => const ("doc") }

        // Creates multiple aliases, multi-line, mixed of patterns
        alias_units! {
            First => const ("doc"),
            Second | Third => const ("doc")
        }

        // Should all generate same type
        assert_all_eq!(
            TypeId::of::<Single<f64>>(),
            TypeId::of::<Left<f64>>(),
            TypeId::of::<Right<f64>>(),
            TypeId::of::<First<f64>>(),
            TypeId::of::<Second<f64>>(),
            TypeId::of::<Third<f64>>(),
            TypeId::of::<Unit<f64>>()
        );

        assert_all_eq!(
            SINGLE,
            LEFT,
            RIGHT,
            FIRST,
            SECOND,
            THIRD,
            single(1.0f64),
            left(1.0f64),
            right(1.0f64),
            first(1.0f64),
            second(1.0f64),
            third(1.0f64),
        );
    }

    #[test]
    fn test_alias_units_non_scalar() {
        alias_units! {
            None => const ("doc"),
            First => const ("doc", P1),
            Second => const ("doc", Z0, P1),
            Third => const ("doc", Z0, Z0, P1),
            Fourth => const ("doc", Z0, Z0, Z0, P1),
            Fifth => const ("doc", Z0, Z0, Z0, Z0, P1),
            Sixth => const ("doc", Z0, Z0, Z0, Z0, Z0, P1),
            Seventh => const ("doc", Z0, Z0, Z0, Z0, Z0, Z0, P1),
        }

        assert_eq!(TypeId::of::<None<f64>>(), TypeId::of::<Unit<f64>>());
        assert_eq!(TypeId::of::<First<f64>>(), TypeId::of::<Unit<f64, P1>>());
        assert_eq!(
            TypeId::of::<Second<f64>>(),
            TypeId::of::<Unit<f64, Z0, P1>>()
        );
        assert_eq!(
            TypeId::of::<Third<f64>>(),
            TypeId::of::<Unit<f64, Z0, Z0, P1>>()
        );
        assert_eq!(
            TypeId::of::<Fourth<f64>>(),
            TypeId::of::<Unit<f64, Z0, Z0, Z0, P1>>()
        );
        assert_eq!(
            TypeId::of::<Fifth<f64>>(),
            TypeId::of::<Unit<f64, Z0, Z0, Z0, Z0, P1>>()
        );
        assert_eq!(
            TypeId::of::<Sixth<f64>>(),
            TypeId::of::<Unit<f64, Z0, Z0, Z0, Z0, Z0, P1>>()
        );
        assert_eq!(
            TypeId::of::<Seventh<f64>>(),
            TypeId::of::<Unit<f64, Z0, Z0, Z0, Z0, Z0, Z0, P1>>()
        );

        assert_all_ne!(
            TypeId::of::<None<f64>>(),
            TypeId::of::<First<f64>>(),
            TypeId::of::<Second<f64>>(),
            TypeId::of::<Third<f64>>(),
            TypeId::of::<Fourth<f64>>(),
            TypeId::of::<Fifth<f64>>(),
            TypeId::of::<Sixth<f64>>(),
            TypeId::of::<Seventh<f64>>(),
        );

        assert_eq!(NONE, none(1.0f64));
        assert_eq!(FIRST, first(1.0f64));
        assert_eq!(SECOND, second(1.0f64));
        assert_eq!(THIRD, third(1.0f64));
        assert_eq!(FOURTH, fourth(1.0f64));
        assert_eq!(FIFTH, fifth(1.0f64));
        assert_eq!(SIXTH, sixth(1.0f64));
        assert_eq!(SEVENTH, seventh(1.0f64));
    }

    #[test]
    fn test_alias_units_delims() {
        // Simple one-liner
        alias_units! { SinglePlain => const ("doc") }

        // One-liner with all extra delim
        alias_units! { | SingleExtra => const ("doc"), }

        // Multi-line
        alias_units! {
            MultiPlain => const ("doc"),
            | MultiExtra => const ("doc"),
        }

        assert_all_eq!(
            TypeId::of::<SinglePlain<f64>>(),
            TypeId::of::<SingleExtra<f64>>(),
            TypeId::of::<MultiPlain<f64>>(),
            TypeId::of::<MultiExtra<f64>>(),
            TypeId::of::<Unit<f64>>()
        );

        assert_all_eq!(
            SINGLE_PLAIN,
            SINGLE_EXTRA,
            MULTI_PLAIN,
            MULTI_EXTRA,
            single_plain(1.0f64),
            single_extra(1.0f64),
            multi_plain(1.0f64),
            multi_extra(1.0f64),
        );
    }

    #[test]
    fn test_alias_units_visibility() {
        alias_units! {
            Private => const ("doc"),
            pub PubAll => const ("doc"),
            pub(crate) PubCrate => const ("doc"),
            pub(self) PubSelf => const ("doc"),
            pub(super) PubSuper => const ("doc"),
        }

        assert_all_eq!(
            TypeId::of::<Private<f64>>(),
            TypeId::of::<PubAll<f64>>(),
            TypeId::of::<PubCrate<f64>>(),
            TypeId::of::<PubSelf<f64>>(),
            TypeId::of::<PubSuper<f64>>(),
            TypeId::of::<Unit<f64>>()
        );

        assert_all_eq!(
            PRIVATE,
            PUB_ALL,
            PUB_CRATE,
            PUB_SELF,
            PUB_SUPER,
            private(1.0f64),
            pub_all(1.0f64),
            pub_crate(1.0f64),
            pub_self(1.0f64),
            pub_super(1.0f64),
        );
    }

    #[test]
    #[allow(unused_qualifications)]
    fn test_alias_units_path() {
        type MyP1 = typenum::P1;

        alias_units! {
            InScope => const ("doc", P1),
            ByPath => const ("doc", typenum::P1),
            ByAlias => const ("doc", MyP1),
            ByDeepPath => const ("doc", typenum::consts::P1),
        }

        assert_all_eq!(
            TypeId::of::<InScope<f64>>(),
            TypeId::of::<ByPath<f64>>(),
            TypeId::of::<ByAlias<f64>>(),
            TypeId::of::<ByDeepPath<f64>>(),
            TypeId::of::<Unit<f64, P1>>()
        );

        assert_all_eq!(
            IN_SCOPE,
            BY_PATH,
            BY_ALIAS,
            BY_DEEP_PATH,
            in_scope(1.0f64),
            by_path(1.0f64),
            by_alias(1.0f64),
            by_deep_path(1.0f64),
        );
    }
}
