# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2024-07-11

### Added

- Use Github Actions ([#7](https://github.com/gadomski/pos-rs/issues/7), [#12](https://github.com/gadomski/pos-rs/pull/12))

### Changed

- Use `edition = "2021"` ([#12](https://github.com/gadomski/pos-rs/pull/12))
- Use `thiserror` for error handling instead of `quick-error` ([#4](https://github.com/gadomski/pos-rs/issues/4), [#15](https://github.com/gadomski/pos-rs/pull/15))

### Removed

- `git-lfs` for test data files, using Github Releases instead ([#14](https://github.com/gadomski/pos-rs/pull/14))

## [0.1.1] - 2016-01-11

### Added

- Interpolator
- `Source` implementation for `pos`

### Changed

- Relicense to dual MIT/Apache-2.0

### Removed

- Travis CI

## [0.1.0] - 2015-12-03

Initial release

[Unreleased]: https://github.com/gadomski/pos-rs/compare/v0.2.0...main
[0.2.0]: https://github.com/gadomski/pos-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/gadomski/pos-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gadomski/pos-rs/releases/tag/v0.1.0

<!-- markdownlint-disable-file MD024 -->