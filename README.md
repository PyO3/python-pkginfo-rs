# python-pkginfo-rs

[![GitHub Actions](https://github.com/messense/python-pkginfo-rs/workflows/CI/badge.svg)](https://github.com/messense/python-pkginfo-rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/python-pkginfo.svg)](https://crates.io/crates/python-pkginfo)
[![docs.rs](https://docs.rs/python-pkginfo/badge.svg)](https://docs.rs/python-pkginfo/)

Query Python package metadata from sdist and bdists and etc.

## Installation

Add it to your ``Cargo.toml``:

```toml
[dependencies]
python-pkginfo = "0.1"
```

then you are good to go. If you are using Rust 2015 you have to add ``extern crate python_pkginfo`` to your crate root as well.

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.