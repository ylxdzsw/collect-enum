#[macro_use]
extern crate collect_enum;

#[collect_enum(Foo)]
fn foo() -> Foo {
    Foo::Fuck
}

fn main() {
    println!("{:?}", Foo::Fuck)
}