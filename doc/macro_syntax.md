# Syntax

For each entry, it must follow the following syntax:

```ignore
[vis] Alias [| [vis] Alias [| [vis] Alias [..]]] => [const] ("doc" [, dim [, dim [..]]])
```

where:

- `vis` is the optional visibility like `pub`, `pub(crate)`. Defaults to private when omitted.
- `Alias` is the PascalCase type alias to create, can be multiple aliases separated by pipe `|`.
- `const` is the optional marker for populating const function and const value.
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
alias! {
    NameA => ("doc", P1),
    NameB => const ("doc", N1),
}
```
