mod inner {
    use siunit::alias_types;

    alias_types! {
        NotPub => ("private alias"),
    }
}

use inner::NotPub;

fn main() {}
