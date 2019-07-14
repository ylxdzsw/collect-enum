collect_enum
============

[![Build Status](https://travis-ci.org/ylxdzsw/collect_enum.svg?branch=master)](https://travis-ci.org/ylxdzsw/collect_enum)

A procedual macro that defines `enum`s with variants collected from thier usages in a scope. Mainly designed for generating
ad-hoc error types and small state machines.

```rust
#[macro_use]
extern crate collect_enum;

#[collect_enum(FooErrorKind)]
fn foo() -> Result<f64, (FooErrorKind, &'static str)> {
    let data = read_a_file().map_err(|e| (FooErrorKind::UnexpectedEof, "read file failed"))?;
    data.parse().map_err(|e| (FooErrorKind::ParseError, "invalid number string"))
}
```

Currently it defines the enums as `pub` and derives `Debug` and `Eq` for them. This is perfectly enough for my usages but
I'm open to add more features if you have different use cases.

### TODO
- [ ] custom `derive`s
- [ ] allow fields. A possible syntax could be `FooErrorKind::StringError<String>("xxx".into())`. i.e. specifying the fields
types at call sites.
