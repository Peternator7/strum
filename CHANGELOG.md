# Changelog

## 0.16.0

* Added serialize UPPERCASE
* Added "case-styles" to match the formatting convention they imply such as `kebab-case` and `camelCase`.
* Added Enum Variant Names to improve compatibility with `structopt` and `clap`. [#56](https://github.com/Peternator7/strum/pull/56)
* Added derive re-export to `strum` to allow re-exporting macros from main crate. [#57](https://github.com/Peternator7/strum/pull/57)
* Bumped syn and quote to `1.0`. This raises minimal compatible rust version to `1.31`.
* Did internal refactoring to improve organization of code. Shouldn't change user-facing api though.
* Added license file to subdirectories so they are included in crate distros.

## 0.15.0

### Added

* Added Feature flags to rename macros. This is to improve compatibility with older versions of rust. [Wiki](https://github.com/Peternator7/strum/wiki/Macro-Renames)

## 0.14.0

### Added

* Allow Missing Docs on EnumCount. [PR #43](https://github.com/Peternator7/strum/pull/43)
* Fix serialize_all in `AsRefStr`, `AsStaticStr` and `IntoStaticStr`. [PR #42](https://github.com/Peternator7/strum/pull/42)
  * This is a bug fix, but it may break code that was relying on the incorrect behavior.

## 0.13.0

### Added

* Added a derive to implement `From<YourEnum>` for `&'static str`. This deprecates `AsStaticStr` since
  the new solution doesn't require a `strum` specific trait to use.

## 0.12.0

### Added

* Serialization case can be controlled using `#[strum(serialize_all = "snake_case")]`. ([#21][#21])
* `#[derive(EnumDiscriminants)]` generates enum with variants without fields. ([#33][#33])

[#21]: https://github.com/Peternator7/strum/issues/21
[#33]: https://github.com/Peternator7/strum/issues/33

## 0.10.0

### Added

* Implemented `Clone` for `EnumIter`s. ([#18][#18])
* Added `AsStaticRef` derive to allow enums to `impl AsStaticRef<str>`. ([#23][#23])

### Fixed

* `#[allow(missing_docs)]` on generated `EnumIter`s. ([#19][#19])

[#18]: https://github.com/Peternator7/strum/pull/18
[#19]: https://github.com/Peternator7/strum/issues/19
[#23]: https://github.com/Peternator7/strum/issues/23
