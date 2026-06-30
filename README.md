# fizix

[![Crates.io][crates-badge]](https://crates.io/crates/fizix)

**fizix** (pronounced like *physics*): compile-time SI dimensions for physics computation, via [`typenum`] type-level integers.

```toml
[dependencies]
fizix = "0.1.0"
```

Dimension mismatches caught at compile time, zero runtime overhead.
Includes type-level scale prefixes (Millimeter, Kilometer, etc.).

```rust
use fizix::*;

let d = Meter::new(100.0);
let t = Second::new(10.0);
let speed: Speed<f64> = d / t;       // 10 m⋅s⁻¹
let km = Kilometer::new(1.0);        // 1 × 10³ m
let m: Meter<f64> = km.to_base();    // 1000 m
```

## Status

- **7 SI base dimensions** (M, L, T, I, Θ, N, J)
- **50+ derived units** (Newton, Joule, Watt, Pascal, Hertz, …)
- **40+ SI prefix aliases** (mm, km, mg, MHz, GPa, …)
- **Type-level scale**: prefixes tracked at compile time
- **`no_std`**: works in embedded environments
- **Custom value types**: `Vec3`, etc. via `DotProduct`/`CrossProduct` traits
- **Stable Rust** (edition 2024, ≥1.85)

## License

[MIT](https://github.com/TeddyHuang-00/fizix/blob/main/LICENSE-MIT) OR [Apache-2.0](https://github.com/TeddyHuang-00/fizix/blob/main/LICENSE-Apache)

[`typenum`]: https://crates.io/crates/typenum
[crates-badge]: https://img.shields.io/crates/v/fizix.svg
