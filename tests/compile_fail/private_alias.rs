mod inner {
    use fizix::alias_units;

    alias_units! {
        NotPub => ("private alias"),
    }
}

use inner::NotPub;

fn main() {}
