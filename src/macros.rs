/// Create `Unit` type aliases only.
///
/// # Uses
///
/// Unlike `alias_units`, this macro will NOT create const and helper functions.
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
///     pub PureValue => ("Some custom scalar type"),
///     NameA | NameB => ("Same unit with two different aliases", P1),
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
/// #     pub PureValue => ("Some custom scalar type"),
/// #     NameA | NameB => ("Same unit with two different aliases", P1),
/// # }
/// let _ = pure_value(0usize);
/// let _ = PURE_VALUE;
/// ```
#[macro_export]
macro_rules! alias_types {
    // Single entry
    ($(|)?$pre:vis $name:ident => ($doc:literal $(, $dim:path)*)$(,)?) => {
        #[doc = $doc]
        $pre type $name<V> = $crate::Unit<V, $($dim),*>;
    };

    // Alternative names (aliases)
    ($(|)?$pre:vis $name:ident|$($pres:vis $names:ident)|+ => ($doc:literal $(, $dim:path)*)$(,)?) => {
        alias_types! { $pre $name => ($doc $(, $dim)*) }
        alias_types! { $($pres $names)|+ => ($doc $(, $dim)*) }
    };

    // Multiple entries
    (
        $(|)?$($pre:vis $name:ident)|+ => ($doc:literal $(, $dim:path)*),
        $($(|)?$($pres:vis $names:ident)|+ => ($docs:literal $(, $dims:path)*)),+$(,)?
    ) => {
        alias_types! { $($pre $name)|+ => ($doc $(, $dim)*) }
        alias_types! { $($($pres $names)|+ => ($docs $(, $dims)*)),+ }
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
///     pub PureValue => ("Some custom scalar type"),
///     NameA | NameB => ("Same unit with two different aliases", P1),
/// }
///
/// let _ = PureValue::new(0usize);
/// let _ = name_a(0usize);
/// let _ = NAME_B; // constants use f64
/// ```
#[macro_export]
macro_rules! alias_units {
    ($(|)?$pre:vis $name:ident => ($doc:literal $(, $dim:path)*)$(,)?) => {
        $crate::alias_types! { $pre $name => ($doc $(, $dim)*) }

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

    ($(|)?$pre:vis $name:ident|$($pres:vis $names:ident)|+ => ($doc:literal $(, $dim:path)*)$(,)?) => {
        alias_units! { $pre $name => ($doc $(, $dim)*) }
        alias_units! { $($pres $names)|+ => ($doc $(, $dim)*) }
    };

    (
        $(|)?$($pre:vis $name:ident)|+ => ($doc:literal $(, $dim:path)*),
        $($(|)?$($pres:vis $names:ident)|+ => ($docs:literal $(, $dims:path)*)),+$(,)?
    ) => {
        alias_units! { $($pre $name)|+ => ($doc $(, $dim)*) }
        alias_units! { $($($pres $names)|+ => ($docs $(, $dims)*)),+ }
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

    macro_rules! test_unit {
        ($macro:ident) => {
            paste::paste! {
                // Only creating a single alias
                $macro! { [<$macro:camel Single>] => ("doc") }

                // Creates two aliases with same config
                $macro! { [<$macro:camel Left>] | [<$macro:camel Right>] => ("doc") }

                // Creates multiple aliases, multi-line, mixed of patterns
                $macro! {
                    [<$macro:camel First>] => ("doc"),
                    [<$macro:camel Second>] | [<$macro:camel Third>] => ("doc")
                }

                // Should all generate same type
                assert_all_eq!(
                    TypeId::of::<[<$macro:camel Single>]<f64>>(),
                    TypeId::of::<[<$macro:camel Left>]<f64>>(),
                    TypeId::of::<[<$macro:camel Right>]<f64>>(),
                    TypeId::of::<[<$macro:camel First>]<f64>>(),
                    TypeId::of::<[<$macro:camel Second>]<f64>>(),
                    TypeId::of::<[<$macro:camel Third>]<f64>>(),
                    TypeId::of::<Unit<f64>>()
                );
            }
        };

        (const $macro:ident) => {
            test_unit!($macro);

            paste::paste! {
                assert_all_eq!(
                    [<$macro:upper _SINGLE>],
                    [<$macro:upper _LEFT>],
                    [<$macro:upper _RIGHT>],
                    [<$macro:upper _FIRST>],
                    [<$macro:upper _SECOND>],
                    [<$macro:upper _THIRD>],
                    [<$macro _single>](1.0f64),
                    [<$macro _left>](1.0f64),
                    [<$macro _right>](1.0f64),
                    [<$macro _first>](1.0f64),
                    [<$macro _second>](1.0f64),
                    [<$macro _third>](1.0f64),
                );
            }
        };
    }

    macro_rules! test_positional {
        ($macro:ident) => {
            paste::paste! {
                $macro! {
                    [<$macro:camel None>] => ("doc"),
                    [<$macro:camel First>] => ("doc", P1),
                    [<$macro:camel Second>] => ("doc", Z0, P1),
                    [<$macro:camel Third>] => ("doc", Z0, Z0, P1),
                    [<$macro:camel Fourth>] => ("doc", Z0, Z0, Z0, P1),
                    [<$macro:camel Fifth>] => ("doc", Z0, Z0, Z0, Z0, P1),
                    [<$macro:camel Sixth>] => ("doc", Z0, Z0, Z0, Z0, Z0, P1),
                    [<$macro:camel Seventh>] => ("doc", Z0, Z0, Z0, Z0, Z0, Z0, P1),
                }
            }

            paste::paste! {
                assert_eq!(
                    TypeId::of::<[<$macro:camel None>]<f64>>(),
                    TypeId::of::<Unit<f64>>()
                );
                assert_eq!(
                    TypeId::of::<[<$macro:camel First>]<f64>>(),
                    TypeId::of::<Unit<f64, P1>>()
                );
                assert_eq!(
                    TypeId::of::<[<$macro:camel Second>]<f64>>(),
                    TypeId::of::<Unit<f64, Z0, P1>>()
                );
                assert_eq!(
                    TypeId::of::<[<$macro:camel Third>]<f64>>(),
                    TypeId::of::<Unit<f64, Z0, Z0, P1>>()
                );
                assert_eq!(
                    TypeId::of::<[<$macro:camel Fourth>]<f64>>(),
                    TypeId::of::<Unit<f64, Z0, Z0, Z0, P1>>()
                );
                assert_eq!(
                    TypeId::of::<[<$macro:camel Fifth>]<f64>>(),
                    TypeId::of::<Unit<f64, Z0, Z0, Z0, Z0, P1>>()
                );
                assert_eq!(
                    TypeId::of::<[<$macro:camel Sixth>]<f64>>(),
                    TypeId::of::<Unit<f64, Z0, Z0, Z0, Z0, Z0, P1>>()
                );
                assert_eq!(
                    TypeId::of::<[<$macro:camel Seventh>]<f64>>(),
                    TypeId::of::<Unit<f64, Z0, Z0, Z0, Z0, Z0, Z0, P1>>()
                );
            }

            paste::paste! {
                assert_all_ne!(
                    TypeId::of::<[<$macro:camel None>]<f64>>(),
                    TypeId::of::<[<$macro:camel First>]<f64>>(),
                    TypeId::of::<[<$macro:camel Second>]<f64>>(),
                    TypeId::of::<[<$macro:camel Third>]<f64>>(),
                    TypeId::of::<[<$macro:camel Fourth>]<f64>>(),
                    TypeId::of::<[<$macro:camel Fifth>]<f64>>(),
                    TypeId::of::<[<$macro:camel Sixth>]<f64>>(),
                    TypeId::of::<[<$macro:camel Seventh>]<f64>>(),
                )
            }
        };

        (const $macro:ident) => {
            test_positional!($macro);

            paste::paste! {
                assert_eq!([<$macro:upper _NONE>], [<$macro _none>](1.0f64));
                assert_eq!([<$macro:upper _FIRST>], [<$macro _first>](1.0f64));
                assert_eq!([<$macro:upper _SECOND>], [<$macro _second>](1.0f64));
                assert_eq!([<$macro:upper _THIRD>], [<$macro _third>](1.0f64));
                assert_eq!([<$macro:upper _FOURTH>], [<$macro _fourth>](1.0f64));
                assert_eq!([<$macro:upper _FIFTH>], [<$macro _fifth>](1.0f64));
                assert_eq!([<$macro:upper _SIXTH>], [<$macro _sixth>](1.0f64));
                assert_eq!([<$macro:upper _SEVENTH>], [<$macro _seventh>](1.0f64));
            }
        };
    }

    macro_rules! test_extra_delim {
        ($macro:ident) => {
            paste::paste! {
                // Simple one-liner
                $macro! { [<$macro:camel SinglePlain>] => ("doc") }

                // One-liner with all extra delim
                $macro! { | [<$macro:camel SingleExtra>] => ("doc"), }

                // Multi-line
                $macro! {
                    [<$macro:camel MultiPlain>] => ("doc"),
                    | [<$macro:camel MultiExtra>] => ("doc"),
                }

                assert_all_eq!(
                    TypeId::of::<[<$macro:camel SinglePlain>]<f64>>(),
                    TypeId::of::<[<$macro:camel SingleExtra>]<f64>>(),
                    TypeId::of::<[<$macro:camel MultiPlain>]<f64>>(),
                    TypeId::of::<[<$macro:camel MultiExtra>]<f64>>(),
                    TypeId::of::<Unit<f64>>()
                )
            }
        };

        (const $macro:ident) => {
            test_extra_delim!($macro);

            paste::paste! {
                assert_all_eq!(
                    [<$macro:upper _SINGLE_PLAIN>],
                    [<$macro:upper _SINGLE_EXTRA>],
                    [<$macro:upper _MULTI_PLAIN>],
                    [<$macro:upper _MULTI_EXTRA>],
                    [<$macro _single_plain>](1.0f64),
                    [<$macro _single_extra>](1.0f64),
                    [<$macro _multi_plain>](1.0f64),
                    [<$macro _multi_extra>](1.0f64),
                );
            }
        };
    }

    macro_rules! test_visibility {
        ($macro:ident) => {
            paste::paste! {
                $macro! {
                    [<$macro:camel Private>] => ("doc"),
                    pub [<$macro:camel Pub>] => ("doc"),
                    pub(crate) [<$macro:camel PubCrate>] => ("doc"),
                    pub(self) [<$macro:camel PubSelf>] => ("doc"),
                }

                assert_all_eq!(
                    TypeId::of::<[<$macro:camel Private>]<f64>>(),
                    TypeId::of::<[<$macro:camel Pub>]<f64>>(),
                    TypeId::of::<[<$macro:camel PubCrate>]<f64>>(),
                    TypeId::of::<[<$macro:camel PubSelf>]<f64>>(),
                    TypeId::of::<Unit<f64>>()
                )
            }
        };

        (const $macro:ident) => {
            test_visibility!($macro);

            paste::paste! {
                assert_all_eq!(
                    [<$macro:upper _PRIVATE>],
                    [<$macro:upper _PUB>],
                    [<$macro:upper _PUB_CRATE>],
                    [<$macro:upper _PUB_SELF>],
                    [<$macro _private>](1.0f64),
                    [<$macro _pub>](1.0f64),
                    [<$macro _pub_crate>](1.0f64),
                    [<$macro _pub_self>](1.0f64),
                );
            }
        };
    }

    macro_rules! test_path {
        ($macro:ident) => {
            type MyP1 = typenum::P1;

            paste::paste! {
                $macro! {
                    [<$macro:camel InScope>] => ("doc", P1),
                    [<$macro:camel ByPath>] => ("doc", typenum::P1),
                    [<$macro:camel ByAlias>] => ("doc", MyP1),
                    [<$macro:camel ByDeepPath>] => ("doc", typenum::consts::P1),
                }

                assert_all_eq!(
                    TypeId::of::<[<$macro:camel InScope>]<f64>>(),
                    TypeId::of::<[<$macro:camel ByPath>]<f64>>(),
                    TypeId::of::<[<$macro:camel ByAlias>]<f64>>(),
                    TypeId::of::<[<$macro:camel ByDeepPath>]<f64>>(),
                    TypeId::of::<Unit<f64, P1>>()
                )
            }
        };

        (const $macro:ident) => {
            test_path!($macro);

            paste::paste! {
                assert_all_eq!(
                    [<$macro:upper _IN_SCOPE>],
                    [<$macro:upper _BY_PATH>],
                    [<$macro:upper _BY_ALIAS>],
                    [<$macro:upper _BY_DEEP_PATH>],
                    [<$macro _in_scope>](1.0f64),
                    [<$macro _by_path>](1.0f64),
                    [<$macro _by_alias>](1.0f64),
                    [<$macro _by_deep_path>](1.0f64),
                );
            }
        };
    }

    #[test]
    fn test_alias_types_scalar() {
        test_unit!(alias_types);
    }

    #[test]
    fn test_alias_units_scalar() {
        test_unit!(const alias_units);
    }

    #[test]
    fn test_alias_types_non_scalar() {
        test_positional!(alias_types);
    }

    #[test]
    fn test_alias_units_non_scalar() {
        test_positional!(const alias_units);
    }

    #[test]
    fn test_alias_types_delims() {
        test_extra_delim!(alias_types);
    }

    #[test]
    fn test_alias_units_delims() {
        test_extra_delim!(const alias_units);
    }

    #[test]
    fn test_alias_types_visibility() {
        test_visibility!(alias_types);
    }

    #[test]
    fn test_alias_units_visibility() {
        test_visibility!(const alias_units);
    }

    #[test]
    fn test_alias_types_path() {
        test_path!(alias_types);
    }

    #[test]
    fn test_alias_units_path() {
        test_path!(const alias_units);
    }
}
