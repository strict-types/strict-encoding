# Strict encoding derivation macros

![Build](https://github.com/strict-types/strict-encoding/workflows/Build/badge.svg)
![Tests](https://github.com/strict-types/strict-encoding/workflows/Tests/badge.svg)
![Lints](https://github.com/strict-types/strict-encoding/workflows/Lints/badge.svg)
[![codecov](https://codecov.io/gh/strict-types/strict-encoding/branch/master/graph/badge.svg)](https://codecov.io/gh/strict-types/strict-encoding)

[![crates.io](https://meritbadge.herokuapp.com/strict_encoding_derive)](https://crates.io/crates/strict_encoding_derive)
[![Docs](https://docs.rs/strict_encoding_derive/badge.svg)](https://docs.rs/strict_encoding_derive)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Apache-2 licensed](https://img.shields.io/crates/l/strict_encoding_derive)](../../LICENSE)

Derivation macros for strict encoding. To learn more about the strict encoding
please check [`strict_encoding`] crate.

The development of the library is supported by
[UBIDECO Institute](https://ubideco.org).


## Documentation

Detailed developer & API documentation for the library can be accessed
at <https://docs.rs/strict_encoding_derive/>


## Usage

To use the library, you need to reference the latest version of the 
[`strict_encoding`] crate in`[dependencies]` section of your project 
`Cargo.toml`. This crate includes derivation macros from the present library by 
default.

```toml
strict_encoding = "2.0"
```

Library exports derivation macros `StrictType`, `StrictDumb`, `StrictEncode`, 
`StrictDecode`.


## Contributing

Contribution guidelines can be found in [CONTRIBUTING](../../CONTRIBUTING.md)


## Licensing

The libraries are distributed on the terms of Apache 2.0 opensource license.
See [LICENCE](LICENSE) file for the license details.

[`strict_encoding`]: https://crates.io/crates/strict_encoding
