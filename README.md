# Python Compiler

> ⚠️ **THIS IS NOT PRODUCTION READY AT ALL, DO NOT USE IT**. I'm also currently rewriting it, so this is not the most up-to-date version.

A Python compiler targeting JavaScript, implemented in Rust.

[![Crates.io](https://img.shields.io/crates/v/python-compiler.svg)](https://crates.io/crates/python-compiler)
[![Docs.rs](https://docs.rs/python-compiler/badge.svg)](https://docs.rs/python-compiler)
[![CI](https://github.com/gideongrinberg/python-compiler/workflows/Continuous%20Integration/badge.svg)](https://github.com/gideongrinberg/python-compiler/actions)
[![Coverage Status](https://coveralls.io/repos/github/gideongrinberg/python-compiler/badge.svg)](https://coveralls.io/github/gideongrinberg/python-compiler)

## Installation

TODO. Should either be a PyPi/Cargo package, or something like [rustup](https://rustup.rs).

## Usage
### As A Compiler

``` bash
./python [input-file]
```

### As A Library

TODO

## Contribution

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. Please make sure to update tests as appropriate.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).


## Roadmap
### Complete

- [x] ~~Python Parser~~
- [x] ~~"Scaffolding" for the rest of the project~~
- [x] ~~Support for built-in function calls~~
- [x] ~~Support for variable declarations~~

### Todos

- [ ] **100% of Python supported** (this is the big one)
- [ ] Better test coverage (and fuzzing)
- [ ] Better optimization, less "fragile" code
- [ ] Helpful error messages.
- [ ] JS Interop


## License

Licensed under either of Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.

