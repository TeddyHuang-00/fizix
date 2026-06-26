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
///     NameA | NameB => ("Other custom type with two different aliases", P1),
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
    ($(|)?$pre:vis $name:ident => ($doc:literal $(, $dim:ty)*)$(,)?) => {
        #[doc = $doc]
        $pre type $name<V> = $crate::Unit<V, $($dim),*>;
    };

    ($(|)?$pre:vis $name:ident|$($pres:vis $names:ident)|+ => ($doc:literal $(, $dim:ty)*)$(,)?) => {
        alias_types! { $pre $name => ($doc $(, $dim)*) }
        alias_types! { $($pres $names)|+ => ($doc $(, $dim)*) }
    };

    (
        $(|)?$($pre:vis $name:ident)|+ => ($doc:literal $(, $dim:ty)*),
        $($(|)?$($pres:vis $names:ident)|+ => ($docs:literal $(, $dims:ty)*)),+$(,)?
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
    ($(|)?$pre:vis $name:ident => ($doc:literal $(, $dim:ty)*)$(,)?) => {
        $crate::alias_types! { $pre $name => ($doc $(, $dim)*) }

        paste::paste! {
            #[doc = $doc]
            #[inline]
            $pre const fn [<$name:snake>]<V>(v: V) -> $name<V> {
                $name::new(v)
            }

            #[doc = $doc]
            $pre const [<$name:snake:upper>]: $name<f64> = $name::new(1.0);
        }
    };

    ($(|)?$pre:vis $name:ident|$($pres:vis $names:ident)|+ => ($doc:literal $(, $dim:ty)*)$(,)?) => {
        alias_units! { $pre $name => ($doc $(, $dim)*) }
        alias_units! { $($pres $names)|+ => ($doc $(, $dim)*) }
    };

    (
        $(|)?$($pre:vis $name:ident)|+ => ($doc:literal $(, $dim:ty)*),
        $($(|)?$($pres:vis $names:ident)|+ => ($docs:literal $(, $dims:ty)*)),+$(,)?
    ) => {
        alias_units! { $($pre $name)|+ => ($doc $(, $dim)*) }
        alias_units! { $($($pres $names)|+ => ($docs $(, $dims)*)),+ }
    }
}
