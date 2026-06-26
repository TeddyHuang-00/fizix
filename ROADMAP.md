# siunit: SI Units at Compile Time

**Compile-time checked SI units via [`typenum`](https://crates.io/crates/typenum) type-level integers.**
Dimension mismatches caught at compile time, zero runtime overhead.

[![Crates.io][crates-badge]](https://crates.io/crates/siunit)

## Status

**v0.1.0**: 7 base dimensions (L, M, T, I, Θ, N, J).\
Works on **stable Rust** (edition 2024, ≥1.85).\
Cross-crate usage confirmed.

## Roadmap

### v0.1.x: Foundation ✅

- [x] `Unit<V, M, L, T, I, K, N, J>` struct with `typenum` type-level dimensions
- [x] `Add`, `Sub`, `Mul`, `Div`, `Neg` operators
- [x] SI base unit aliases (Meter, Kilogram, Second, Ampere, Kelvin, Mole, Candela)
- [x] SI derived unit aliases (Newton, Joule, Watt, Pascal, Hertz)
- [x] Convenience aliases (Speed, Acceleration, Area, Volume, Momentum, etc.)
- [x] Helper constructors (`meters(5.0)`, `seconds(10.0)`, etc.)
- [x] Compile-time dimension mismatch rejection
- [x] `no_std` support
- [x] Cross-crate operation support
- [x] `cargo +stable test` passes (24 tests)

### v0.2.x: Usability

- [ ] `Display` impl: pretty-print "5 m·s⁻²"
- [ ] `From<f64>` / `Into<f64>` for dimensionless quantities
- [ ] `Pow` trait for integer exponentiation
- [ ] `MulAssign`, `DivAssign`
- [ ] `PartialOrd`, `Ord` (same dimensions only)
- [ ] More unit aliases (Angstrom, LightYear, Parsec, etc. — via features)
- [ ] `serde` feature for serialization

### v1.0: Maturity

- [ ] Benchmark vs `uom` (compile time, binary size)
- [ ] `generic_const_exprs` stabilization → optional const-generic backend
- [ ] SI prefix macros (`kilo!(Meter)`, `milli!(Second)`)

## Earlier Attempts (Nightly)

Before migrating to typenum, we attempted a `generic_const_exprs`-based design:

```rust
// Original design (nightly-only, archived)
pub struct Unit<V, const M: i8 = 0, …>;
// type Output = Unit<V, { M1 + M2 }, …>;
```

This approach was abandoned because:
1. `generic_const_exprs` (rust-lang/rust#76560) is unstable and "very broken" per rustc team
2. Cross-crate usage causes E0275 trait solver overflow
3. No viable workaround found (packed i64, combined where-clauses, `-Znext-solver` all failed)
4. The successor feature `generic_const_args` (#151972) is also nightly-only and doesn't allow `{M1 + M2}` in type position

When either feature stabilizes on stable Rust, siunit can offer an optional const-generic backend via a feature flag — but for now, typenum provides a proven, stable foundation (used by `uom` for 8+ years).

## Comparison

| Crate         | Approach                | Stable | Lines |
| ------------- | ----------------------- | ------ | ----- |
| **siunit**    | typenum type-level ints | ✅     | ~300  |
| `uom`         | typenum type-level ints | ✅     | ~90K  |
| `dimensioned` | typenum type-level ints | ✅     | ~10K  |

## License

MIT OR Apache-2.0

[crates-badge]: https://img.shields.io/crates/v/siunit.svg
