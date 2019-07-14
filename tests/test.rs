#[macro_use]
extern crate collect_enum;

#[collect_enum(Foo)]
fn foo() -> Foo {
    Foo::A;
    Foo::B
}

#[test]
fn it_works() {
    assert!(foo() == Foo::B)
}