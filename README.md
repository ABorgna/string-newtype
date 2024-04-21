# string-newtype: New Type idiom helper for string-like types

[![build_status][]](https://github.com/aborgna/string-newtype/actions)
![msrv][]
[![codecov][]](https://codecov.io/gh/aborgna/string-newtype)

  [build_status]: https://github.com/ABorgna/string-newtype/actions/workflows/ci-rs.yml/badge.svg?branch=main
  [msrv]: https://img.shields.io/badge/rust-1.75.0%2B-blue.svg
  [codecov]: https://img.shields.io/codecov/c/gh/aborgna/string-newtype?logo=codecov

`string-newtype` is a helper library for using the [newtype idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) with string-like types, including newtyped string slices.

# Usage

Define an empty enum as a marker for your type, and add aliases based on it:

```rust
// A marker type, can be anything.
enum S {}

// The newtype definitions.
// `StringBuf` acts as a `String`, while `StringRef` acts as a `str`.
type SBuf = StringBuf<S>;
type SRef = StringRef<S>;

// Define functions that only accept the newtype.
fn my_func(owned: SBuf, reference: &SRef) {
    // ...
}

// Only the newtype can be passed to the function.
let s: SBuf = "hello".into();
my_func(s.clone(), &s);
```

The crate also defines `SmolStrBuf` and `SmolStrRef` types, which are newtypes
for `smol_str::SmolStr`.

## Features

- `smol_str`
  Enables newtypes for `smol_str::SmolStr`s.

- `serde`
  Enables serialization and deserialization implementations.

## Developing string-newtype

See [DEVELOPMENT.md](DEVELOPMENT.md) for instructions on setting up the development environment.

## License

This project is licensed under Apache License, Version 2.0 ([LICENSE][] or http://www.apache.org/licenses/LICENSE-2.0).

  [LICENSE]: LICENCE
