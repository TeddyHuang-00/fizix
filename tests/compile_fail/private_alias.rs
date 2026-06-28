mod inner {
    use siunit::alias_units;

    alias_units! {
        NotPub => ("private alias"),
    }
}

use inner::NotPub;

fn main() {}
