#[macro_use]
extern crate collect_enum;

#[collect_enum(Foo)]
fn foo() -> Foo {
    Foo::Fuck
}

#[test]
fn it_sucks() {
    println!("{:?}", Foo::Fuck)
}