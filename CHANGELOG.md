# Changelog

## 0.25.3 (strum_macros)

Received a number of bug fix PR's.

* [#300](https://github.com/Peternator7/strum/pull/300): Fixes EnumIter for enums named `Option`.
* [#301](https://github.com/Peternator7/strum/pull/301): Improved doc comments
* [#304](https://github.com/Peternator7/strum/pull/304): Removed some unused Vec's
* [#305](https://github.com/Peternator7/strum/pull/305): Added generic support to `EnumIs`

## 0.25.2 (strum_macros)

* [#289](https://github.com/Peternator7/strum/pull/289): Enables a previously disabled rustdoc.
* [#287](https://github.com/Peternator7/strum/pull/287): Fixes a bug in EnumIter code gen so that we produce `::core` to
  avoid issues with shadowing modules.

## 0.25.1 (strum_macros)

* [#276](https://github.com/Peternator7/strum/pull/276). Fixes [#275](https://github.com/Peternator7/strum/issues/275) and 
  [#281](https://github.com/Peternator7/strum/issues/281). Not sure exactly why this broke, perhaps incompatibilities between
  syn 1 and 2. PR fixes the issue by only looking at attributes of the "list" type `[attr(item, item)]`. 

## 0.25.0

### Breaking Changes

* [#261](https://github.com/Peternator7/strum/pull/261) Upgrade syn dependency to version 2. This bumps the msrv to 
  1.56. It's impractical to maintain a package where a core dependency of the ecosystem has a different msrv than this one.

* [270](https://github.com/Peternator7/strum/pull/270) Change the `to_string` behavior when using `default`. Now, when
  using `default`, the `display` method will return the display version of the value contained in the enum rather than
  the name of the variant.

  ```rust
  #[derive(strum::Display)]
  enum Color {
    Red,
    Blue,
    Green,
    #[strum(default)]
    Other(String)
  }

  fn main() {
    // This used to print "Other", now it prints "Purple"
    assert_eq!(Color::Other("Purple".to_string()).to_string(), "Purple");
  }
  ```

  If you want the old behavior, you can use the `to_string` attribute to override this behavior. See the PR for an example.

* [268](https://github.com/Peternator7/strum/pull/268) Update the behavior of `EnumCount` to exclude variants that are
  `disabled`. This is a breaking change, but the behavior makes it more consistent with other methods.

### New Features

* [#257](https://github.com/Peternator7/strum/pull/257) This PR adds the `EnumIs` macro that automatically implements
  `is_{variant_name}` methods for each variant. 

  ```rust
  #[derive(EnumIs)]
  enum Color {
      Red,
      Blue,
      Green,
  }

  #[test]
  fn simple_test() {
      assert!(Color::Red.is_red());
  }
  ```

## 0.24.3 (strum_macros)

* [#231](https://github.com/Peternator7/strum/pull/231) Add ignore lints for EnumIter not implementing Copy or Debug
  on the generated iter struct. Copy should generally not be implemented on Iterators. Its an oversight that Debug isn't
  implemented, but it will be a breaking change to add that so it'll be added in a future version.

## 0.24.2 (strum_macros)

* [#220](https://github.com/Peternator7/strum/pull/220). Add support for PHF in `EnumString` (opt-in runtime
  performance improvements for large enums as `#[strum(use_phf)]`, requires `phf` feature and increases MSRV to `1.46`)
  * [#224](https://github.com/Peternator7/strum/pull/224) tweaked the algorithm.

* Reverted [#217](https://github.com/peternator7/strum/pull/217) because it was disruptive and non-trivial to work around
  if you hit it.

## ~~0.24.1~~ (Yanked becase #217 was more "breaking" than I wanted)

* [#220](https://github.com/Peternator7/strum/pull/220). Add support for PHF in `EnumString` (opt-in runtime
  performance improvements for large enums as `#[strum(use_phf)]`, requires `phf` feature and increases MSRV to `1.46`)
  * [#224](https://github.com/Peternator7/strum/pull/224) tweaked the algorithm.
* [#217](https://github.com/Peternator7/strum/pull/217): Automatically implement `TryFrom` in `FromRepr`. This is 
  technically a breaking change, but the fix is to just remove the manual implementation of TryFrom so it shouldn't 
  be more than a minor inconvenience.

## 0.24.0

* [#212](https://github.com/Peternator7/strum/pull/212). Fix some clippy lints

* [#209](https://github.com/Peternator7/strum/pull/209). Use `core` instead of `std` in a few places.

* [#206](https://github.com/Peternator7/strum/pull/206). Add `get_documentation()` to `EnumMessage`. This provides
  the ability to get the doc comment for a variant. Currently, very little formatting is done. That is subject to change.
  Please do not abuse this feature. Rust docs are meant for developer documentation, not long messages for users. However,
  this may be useful in some situations so we've added support for it.

* [#202](https://github.com/Peternator7/strum/pull/202). Add a missing doc comment

* [#201](https://github.com/Peternator7/strum/pull/201). Upgrade Heck version

## 0.23.1

* [#193](https://github.com/Peternator7/strum/pull/193) Fixes an ambiguity introduced by #186 when your enum has a variant called Error.

* [#192](https://github.com/Peternator7/strum/pull/192) The work done in #189 was lost in other PR's. This re-added the functionality to support no-std.

## 0.23.0

* [#185](https://github.com/Peternator7/strum/pull/185) Adds the `FromRepr` derive that adds a `from_repr(x: usize) -> Option<Self>`
  method to your enum. This lets you convert integer values to your enum. If you specify a #[repr(..)] attribute on your enum, or use
  an explicit discriminant, this will be incorporated into the derive.
  * `from_repr` will be `const` if you use a recent rust version.
  * This cannot be a trait method currently because only inherent methods support `const`.

* [#186](https://github.com/Peternator7/strum/pull/186) Automatically implement `TryFrom<str>` for enums that implement `EnumString`.
  This is only enabled for rustc >= 1.34 which is when `TryFrom was stabilized.
  * This is a small breaking change. If you had manually implemented `TryFrom<str>` for your enum, this will cause a conflict. You
    can probably remove your manual implementation.

* [#189](https://github.com/Peternator7/strum/pull/189) Use `core::result::Result` instead of `std::result::Result`. This should be
  more portable in no-std environments.

## 0.22.0

* [#180](https://github.com/Peternator7/strum/pull/180): Deprecates `ToString` derive. You should use `Display`
  instead.

* [#178](https://github.com/Peternator7/strum/pull/178): Deprecates AsStaticStr. This has been undocumented for a while.
  The recommended method is to derive `IntoStaticStr` instead.

* [#171](https://github.com/Peternator7/strum/pull/171): Improve `no-std` support. 

* [#170](https://github.com/Peternator7/strum/pull/170): enable changing the path to strum traits. This is necessary
  if you re-export strum as a submodule in another crate.

## 0.21.1

* [#164](https://github.com/Peternator7/strum/pull/164) Improve compatibility with older versions of `syn`.

## 0.21.0

* Replace `std::` with `core::` to support no-std projects. [#145](https://github.com/Peternator7/strum/pull/145)

* **Breaking Changes**
  * MSRV is updated to 1.32 because `heck` does not work in `1.31.1` anymore. Rust 1.32 came out Jan 2019 so hopefully
    teams have moved to an equal or newer version.
  * [#149](https://github.com/Peternator7/strum/pull/149) Remove the "rename" feature. In Rust 2018, macros should be
    imported using a qualified path to avoid collisions. `strum_macros::EnumString` rather than using the "Rename"
    feature to avoid collisions.
  * [#160](https://github.com/Peternator7/strum/pull/160) enum messages: Make the returned values all 'static
    * It is unlikely this will break anyone, but the new signature for functions generated by EnumMessage 
      is `fn get_message(&self) -> Option<&'static str>`. 

* Added support for ascii_case_insensitive string comparisons. [#157](https://github.com/Peternator7/strum/pull/157)
  This feature allows `FromString` impls to match strings that differ in case from the expected variant.

## 0.20.0

* Refactors to do better error handling. Thanks @jplatte for these changes
  * [#133](https://github.com/Peternator7/strum/pull/133)
  * [#134](https://github.com/Peternator7/strum/pull/134)
  * [#135](https://github.com/Peternator7/strum/pull/135)

* Adding `vis(scope)` to EnumDiscriminants. [#137](https://github.com/Peternator7/strum/pull/137)
  * This feature works best with versions of rust >= 1.34 due to a rustc parsing error in
    earlier versions.
  * Earlier versions can still use `vis(r#pub)`

* These changes should not be breaking, but the amount of code modified was significant.

* FYI about [#122](https://github.com/Peternator7/strum/issues/122). The macro renames feature
  will be removed in early 2021. This feature was only necessary in rust 2015 because macros
  didn't support qualified imports so macro names could collide between crates.

## 0.19.4 / 0.19.5

* Updated docs

## 0.19.3

* Properly error on malformed strum attributes. [#119](https://github.com/Peternator7/strum/pull/119)
  * These types of inputs have historically been ignore silently. This may break code that is already
    incorrect.
* Move docs back to rust docs. [#121](https://github.com/Peternator7/strum/pull/121)
* Updated the docs a second time to improve discoverability.

## 0.19.2

* Fixes [#104](https://github.com/Peternator7/strum/issues/104). PR [#105](https://github.com/Peternator7/strum/issues/105)

## 0.19.1

* **Breaking Change**: EnumVariantNames now properly adjusts to the `to_string` and `serialize` attributes.
* There's a regression in this release that may make strum imcompatible with other plugins if those
  plugins use non-rust syntax in their attributes. [#104](https://github.com/Peternator7/strum/issues/104)

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
