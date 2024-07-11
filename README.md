# pos-rs

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/gadomski/pos-rs/ci.yml?style=for-the-badge)](https://github.com/gadomski/pos-rs/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/pos?style=for-the-badge)](https://docs.rs/pos/latest/pos/)

Small Rust library for reading GNSS/IMU position and accuracy data.

Currently supported formats:

- sbet
- pof/poq (Riegl)
- pos (ASCII)

## Developing

The test files are not contained within this repo — use `scripts/download-test-files` to download them:

```shell
scripts/download-test-files
```

Then, to run the tests:

```shell
cargo test
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
