# siunit

Compile-time checked SI units via type-level integers.

Every physical quantity carries its dimension as `typenum` type parameters.
Dimension mismatches are caught at compile time with zero runtime overhead.

## Usage

```rust
use siunit::*;

let distance = Meter::new(100.0);
let time = Second::new(10.0);
let speed: Speed<f64> = distance / time;
assert_eq!(speed.value, 10.0);
```

## License

MIT OR Apache-2.0
