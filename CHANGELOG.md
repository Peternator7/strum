# Changelog

## Unreleased

### Added

* Serialization case can be controlled using `#[strum(serialize_all = "snake_case")]`. ([#21][#21])

[#21]: https://github.com/Peternator7/strum/issues/21

## 0.10.0

### Added

* Implemented `Clone` for `EnumIter`s. ([#18][#18])
* Added `AsStaticRef` derive to allow enums to `impl AsStaticRef<str>`. ([#23][#23])

### Fixed

* `#[allow(missing_docs)]` on generated `EnumIter`s. ([#19][#19])

[#18]: https://github.com/Peternator7/strum/pull/18
[#19]: https://github.com/Peternator7/strum/issues/19
[#23]: https://github.com/Peternator7/strum/issues/23
