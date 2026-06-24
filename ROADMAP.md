# siunit — SI Units at Compile Time

**Compile-time checked SI units via Rust const generics.**
Dimension mismatches caught at compile time, zero runtime overhead.

[![Nightly][nightly-badge]](https://rust-lang.github.io/rfcs/2000-const-generics.html)

## Status

**v0.1.0** — 3 base dimensions (L, M, T).\
Covers mechanics, acoustics, and thermodynamics.\
Electromagnetism and photometry (I, Θ, N, J) blocked on compiler improvements.

## Roadmap

### v0.1.x — Foundation (3D)

- [x] `Unit<L, M, T>` struct with `f64` value
- [x] `Add`, `Sub`, `Mul`, `Div`, `Neg` operators
- [x] Helper constructors (`meters(5.0)`, `seconds(10.0)`, etc.)
- [x] SI base unit aliases (Meter, Kilogram, Second)
- [x] SI derived unit aliases (Newton, Joule, Watt, Pascal, Hertz, etc.)
- [x] Compile-time dimension mismatch rejection (via `Add`/`Sub` on different types)
- [ ] README with usage examples and known limitations
- [ ] `no_std` support (inherent from design)
- [ ] `serde` feature for serialization

### v0.2.x — Usability

- [ ] `Display` impl: pretty-print "5 m·s⁻²"
- [ ] `From<f64>` / `Into<f64>` for dimensionless quantities
- [ ] `Pow` trait for integer exponentiation
- [ ] `MulAssign`, `DivAssign`
- [ ] `PartialOrd`, `Ord` (same dimensions only)
- [ ] More unit aliases (Angstrom, LightYear, Parsec, etc. — via features)

### v0.3.x — Extended Dimensions

- [ ] 7D upgrade: L, M, T, I, Θ, N, J
  - *Blocked on:* rustc const-expr normalization ([rust#76560])
- [ ] EM derived units (Volt, Ohm, Farad, Tesla, Henry, etc.)
- [ ] `sqrt` trait (fractional exponents via rational const generics)

### v0.4.x — Maturity

- [ ] Benchmark vs `uom` (compile time, binary size)
- [ ] `generic_const_exprs` stabilization → stable Rust support
- [ ] SI prefix macros (`kilo!(Meter)`, `milli!(Second)`)
- [ ] Named derived unit constructor functions

## Known Limitations

- **Nightly only:** uses `#![feature(generic_const_exprs)]`
- **Explicit type annotations on computed quantities** fail due to const-expr
  non-normalization. Use type inference for result types.
- **3 dimensions only:** I, Θ, N, J reserved for future compiler improvements.

## Related

| Crate         | Approach                | Lines | Stable     |
| ------------- | ----------------------- | ----- | ---------- |
| **siunit**    | const generics          | ~100  | ❌ nightly |
| `uom`         | typenum type-level ints | ~40K  | ✅         |
| `dimensioned` | typenum type-level ints | ~10K  | ✅         |

[nightly-badge]: https://img.shields.io/badge/nightly-required-red
