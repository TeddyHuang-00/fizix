use fizix::alias_units;

alias_units! {
    pub Foo => ("non-typenum dimension", String),
}

fn main() {
    let _ = Foo::new(1.0f64);
}
