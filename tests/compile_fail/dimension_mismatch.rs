use siunit::*;

fn main() {
    let d = Meter::new(100.0);
    let t = Second::new(10.0);
    let _ = d + t; // can't add meter and second: dimension mismatch
}
