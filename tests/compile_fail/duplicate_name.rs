use siunit::alias_units;

alias_units! { pub Foo => ("first definition") }
alias_units! { pub Foo => ("duplicate definition") }

fn main() {}
