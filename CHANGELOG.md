# Changelog

## 0.20.0

* **Breaking Change**: EnumVariantNames now properly adjusts to the `to_string` and `serialize` attributes.

## 0.19.0

* Fixed a regression using `nth_back`. [#85](https://github.com/Peternator7/strum/pull/85)
* Added repository to Cargo.toml. [#90](https://github.com/Peternator7/strum/pull/90)
* Correctly handle fill align in `Display` impls. [#95](https://github.com/Peternator7/strum/pull/95)
* **Breaking Change**: Use Associated Constant for EnumCount instead of const fn and free constant. [#99](https://github.com/Peternator7/strum/pull/99)
  This behavior is consistent with the other derives.
* **Breaking Change**. `default` and `disabled` should now be written as markers instead of key value pairs.
  Here is the old way of adding these attributes to a variant.
  ```rust
  // OLD WAY
  enum Test {
    #[strum(disabled = "true", default = "true")]
    Variant(String)
  }
  ```

  Here is the new way. There is less ambiguity in the new syntax.

  ```rust
  enum Test {
    #[strum(disabled, default)]
    Variant(String)
  }
  ```
* **Breaking Change**. Most of the strum plugins will now error more aggresively on invalid options being
  used. Historically, the plugins have ignore invalid options, but most of these should error now. Silent
  errors are a rust anti-pattern.

## 0.18.0

* Only copy across `"doc", "cfg", "allow", "deny"` attributes from main enum variants to discriminant variants. [#73](https://github.com/Peternator7/strum/issues/73)
* The formatting of generated serialization variants returned by `get_serializations()` from an
  enum that derives `EnumMessage` is now affected by the `serialize_all` property on the enum.
  [#84](https://github.com/Peternator7/strum/pull/84)
* IntoEnumIterator now has the constraint `IntoEnumIterator::Iterator<Item=Self>` and `Self: Sized`. This makes
  it much easier to be generic over `IntoEnumIterator` and enum variants must be sized. [#80]

## 0.17.1

* Fixed an issue caused by combining [#60](https://github.com/Peternator7/strum/pull/60) and [#76](https://github.com/Peternator7/strum/pull/76)

## 0.17.0

* **Breaking Change**. Enum variant names now exports an associated constant `VARIANTS` in the
  `strum::VariantNames` trait instead of adding a `variants` method directly to the enum.
  The fix is to `use strum::VariantNames` in your module and replace occurrances of `variants()`
  with `VARIANTS`.
  * [#74](https://github.com/Peternator7/strum/pull/74)
  * [#75](https://github.com/Peternator7/strum/pull/75)
* 🐛 fix - stop incrementing iterator index after we reach the end. [#76](https://github.com/Peternator7/strum/pull/76)
* Strum iterators now implemented [DoubleEndedIterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html). [#60](https://github.com/Peternator7/strum/pull/60)

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
