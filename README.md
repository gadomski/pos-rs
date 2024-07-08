# pos-rs

[![Documentation](https://docs.rs/pos/badge.svg)](https://docs.rs/pos)
[![Build Status](https://travis-ci.org/gadomski/pos-rs.svg?branch=master)](https://travis-ci.org/gadomski/pos-rs)

Small rust library for reading GNSS/IMU position and accuracy data.

Currently supported formats:

- sbet
- pof/poq (Riegl)
- pos (ASCII)

## Developing

Some of the test files are large and require [git-lfs](https://git-lfs.com/):

```shell
git lfs fetch
git lfs checkout
```

Then, to run the tests:

```shell
cargo test
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
