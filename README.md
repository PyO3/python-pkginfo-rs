# python-pkginfo-rs

[![GitHub Actions](https://github.com/PyO3/python-pkginfo-rs/workflows/CI/badge.svg)](https://github.com/PyO3/python-pkginfo-rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/python-pkginfo.svg)](https://crates.io/crates/python-pkginfo)
[![docs.rs](https://docs.rs/python-pkginfo/badge.svg)](https://docs.rs/python-pkginfo/)

Parse Python package metadata from sdist and bdists and etc.
A Rust port of the [pkginfo](https://pypi.org/project/pkginfo/) Python library.

## Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
python-pkginfo = "0.6"
```

then you are good to go. If you are using Rust 2015 you have to add `extern crate python_pkginfo` to your crate root as well.

## Example

```rust
use python_pkginfo::Distribution;

fn main() {
    let dist = Distribution::new("path/to/package.whl").unwrap();
    println!("{:#?}", dist.metadata());
}
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
