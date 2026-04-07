### renumber

[![crates.io][crates-badge]][crates-url]
[![coverage][cov-badge]][cov-url]  
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build macOs][build-badge-macos]
![build macOs arm64][build-badge-macos-arm64]  
[![mit-license][mit-badge]][mit-license-url]
[![apache-license][apache-badge]][apache-license-url]
[![cc][cc-badge]][cc-url]  
[![mbh][mbh-badge]][mbh-url]
[![es][es-badge]][es-url]

[crates-badge]: https://img.shields.io/crates/v/renumber.svg
[crates-url]: https://crates.io/crates/renumber
[cov-badge]: https://img.shields.io/badge/coverage-77%25%20%E2%94%82%2066%25%20%E2%94%82%2078%25-f4b01b.svg
[cov-url]: https://crates.io/crates/coverio
[build-badge-linux]: https://github.com/EngosSoftware/renumber/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/renumber/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/renumber/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/renumber/actions/workflows/build-macos-arm64.yml/badge.svg
[mit-badge]: https://img.shields.io/badge/License-MIT-4169E1.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/renumber/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-4169E1.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/renumber/blob/main/LICENSE
[apache-notice-url]: https://github.com/EngosSoftware/renumber/blob/main/NOTICE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4169E1.svg
[cc-url]: https://github.com/EngosSoftware/renumber/blob/main/CODE_OF_CONDUCT.md
[mbh-badge]: https://img.shields.io/badge/Made_by_a-HUMAN-DC143C.svg
[mbh-url]: https://github.com/DariuszDepta
[es-badge]: https://img.shields.io/badge/at-Engos_Software-32CD32.svg
[es-url]: https://engos.de
[repository-url]: https://github.com/EngosSoftware/renumber

# Renumber Rust tests or benchmarks

## Overview

**renumber** is a simple command-line utility that renumbers Rust tests and benchmarks.

It is particularly useful when a single test or benchmark file contains multiple functions
that don't require meaningful names, such as in test-driven development (TDD).

**renumber** renames these functions sequentially as `_0001`, `_0002`, `_0003`, and so on.

## Example

### Input file before renumbering

```shell
cat ./tests/test_doc.rs
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

> [!NOTE]
> Test names begin with underscore, otherwise **renumber** will skip it.

### Renumbering

```shell
renumber ./tests/test_doc.rs
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

## Hints & Tips

- **renumber** takes a single command-line argument, the input file to be renumbered.
- If renumbering is successful, the input file is **OVERWRITTEN** with the new, renumbered content.
- Only functions annotated with `#[test]` or `#[bench]` and whose names begin with an underscore (`_`) are renamed.

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [renumber][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
