# Renumber Rust tests or benchmarks

[![Crates.io][crates-badge]][crates-url]
![Code coverage][coverage-badge]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]
![build MacOs arm64][build-badge-macos-arm64]
[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][coc-badge]][coc-url]

[crates-badge]: https://img.shields.io/crates/v/renumber.svg
[crates-url]: https://crates.io/crates/renumber
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: LICENSE
[build-badge-linux]: https://github.com/EngosSoftware/renumber/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/renumber/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/renumber/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/renumber/actions/workflows/build-macos-m1.yml/badge.svg
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[coc-url]: CODE_OF_CONDUCT.md
[repository-url]: https://github.com/EngosSoftware/renumber

## Overview

**renumber** is a simple command-line utility that renumbers Rust tests and benchmarks.

It is particularly useful when a single test or benchmark file contains multiple functions
that don't require meaningful names, such as in test-driven development (TDD).
**renumber** renames these functions sequentially as `_0001`, `_0002`, `_0003`, and so on.

## Example

### Input file

```shell
$ cat ./tests/test_doc.rs
```

Output:

```rust
#[test]
fn _one_should_be_equal_to_one() {
  assert_eq!(1, 1);
}

#[test]
fn _one_should_not_be_equal_to_two() {
  assert_ne!(1, 2);
}

#[test]
fn _first_name_should_be_shorter_than_the_second() {
  let first_name = "John";
  let second_name = "Alexander";
  assert!(first_name.cmp(&second_name).is_gt());
}
```

> Note that test names begin with underscore, otherwise **renumber** will skip such test.

### Renumbering

```shell
$ renumber ./tests/test_doc.rs
```

### Input file after renumbering

```rust
#[test]
fn _0001() {
  assert_eq!(1, 1);
}

#[test]
fn _0002() {
  assert_ne!(1, 2);
}

#[test]
fn _0003() {
  let first_name = "John";
  let second_name = "Alexander";
  assert!(first_name.cmp(&second_name).is_gt());
}
```

## Hints & tips

- **renumber** takes a single command-line argument, the input file to be renumbered.
- If renumbering is successful, the input file is **OVERWRITTEN** with the new, renumbered content.
- Only functions annotated with `#[test]` or `#[bench]` and whose names begin with an underscore (`_`) are renamed.
- Numbering starts from **1**.
- The new name follows this pattern: `format!("_{:04}", index)`.

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE](LICENSE))
  at your option.

## Contribution

Any contributions to [**renumber**][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
