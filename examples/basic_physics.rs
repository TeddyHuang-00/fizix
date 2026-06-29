//! Basic physics calculations with fizix type-safe dimensions.
//!
//! Run with: `cargo run --example basic_physics`

use fizix::*;

// Note that the type annotations are all optional, as they can be inferred by
// the compiler. We just add them here to see that it can be used to provide
// nice, easy-to-read annotations.
fn main() {
    // F = ma: a 2 kg mass accelerated at 3 m/s²
    let mass = kilogram(2.0);
    let acceleration: Acceleration<_> = meter(3.0) / second(1.0) / second(1.0);
    let force: Newton<_> = mass * acceleration;
    println!("Force: {}", force); // "6 kg⋅m⋅s⁻²"

    // Work: 10 N force over 5 m
    let work: Joule<_> = newton(10.0) * meter(5.0);
    println!("Work: {}", work); // "50 kg⋅m²⋅s⁻²"

    // Power: 50 J over 2 s
    let power: Watt<_> = work / second(2.0);
    println!("Power: {}", power); // "25 kg⋅m²⋅s⁻³"

    // Type safety: uncommenting this would fail to compile:
    // let _ = meter(1.0) + second(1.0);
}
