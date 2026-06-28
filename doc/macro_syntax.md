# Syntax

For each entry, it must follow the following syntax:

```ignore
[vis] Alias [| [vis] Alias [| [vis] Alias [..]]] => [const] ("doc" [, dim [, dim [..]]])
```

where:

- `vis` is the optional visibility like `pub`, `pub(crate)`. Defaults to private when omitted.
- `Alias` is the PascalCase type alias to create, can be multiple aliases separated by pipe `|`.
- `const` is the optional marker for generating const helper function and const value.
  Defaults to only creating type aliases when omitted.
- `"doc"` is the doc string to be attached to every generated item.
- `dim`s are the exponent numbers for each of the 7 SI base dimensions.
  They must be type-level integers using [`typenum`], and omitting trailing dimensions defaults them to `Z0`.
  They are in this positional order separated by comma:
  - **mass** (kg)
  - **length** (m)
  - **time** (s)
  - **current** (A)
  - **temperature** (K)
  - **amount** (mol)
  - **intensity** (cd).

## Generated items

For a single entry without `const`:

```ignore
alias_units! {
    pub Force => ("Force (N)", P1, P1, N2),
}
// generates:
pub type Force<V> = Unit<V, P1, P1, N2>;
```

With `const`, it additionally generates:

```ignore
alias_units! {
    pub Newton => const ("Force (N)", P1, P1, N2),
}
// generates:
pub type Newton<V> = Unit<V, P1, P1, N2>;
pub const fn newton<V>(v: V) -> Newton<V>;
pub const NEWTON: Newton<f64> = Newton::new(1.0);
```

The snake_case function (`newton`) and UPPER_SNAKE_CASE constant (`NEWTON`) are
derived from the PascalCase alias name (`Newton`). The constant always has value
`1.0` and type `Alias<f64>`.

## Pipe aliases (shared definition)

Multiple aliases with the same dimension can share one definition via `|`:

```ignore
alias_units! {
    pub Velocity | Speed => ("(m/s)", Z0, P1, N1),
}
// generates type aliases for both Velocity and Speed.
// If tagged `const`, both get their own helper function and constant.
// The visibility marker are still per-item and not shared.
```

## Formatting conveniences

- Optional leading `|` before the first entry (for visual alignment in tables).
- Trailing commas are always optional (single-entry, last entry, multi-entry).

## Dimension path resolution

Any type implementing `typenum::Integer` works:

```ignore
use typenum::P1;
P1                   // after `use`
typenum::P1          // module path
typenum::consts::P1  // deep path
```

## Multi-entry

Separate entries with commas. Each entry independently controls its own `const`:

```ignore
alias_units! {
    NameA => ("doc", P1),
    NameB => const ("doc", N1),
}
```
