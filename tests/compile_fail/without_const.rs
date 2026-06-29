use fizix::alias_units;

alias_units! {
    pub Foo => ("type-only, no const helpers"),
}

fn main() {
    let _ = FOO; // <- cannot find value
}
