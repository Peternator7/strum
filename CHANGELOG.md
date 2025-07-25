# Changelog

## 0.27.2

* [#141](https://github.com/Peternator7/strum/pull/141): Adding support for doc comments on `EnumDiscriminants` generated type.
  * The doc comment will be copied from the variant on the type itself.
* [#435](https://github.com/Peternator7/strum/pull/435):allow discriminants on empty enum.
* [#443](https://github.com/Peternator7/strum/pull/443): Change enum table callbacks to FnMut.
* [#444](https://github.com/Peternator7/strum/pull/444): Add `#[automatically_derived]` to the `impl`s by @dandedotdev in https://github.com/Peternator7/strum/pull/444
  * This should make the linter less noisy with warnings in generated code.
* [#440](https://github.com/Peternator7/strum/pull/440): Implement a `suffix` attribute for serialization of enum variants.

  ```rust
  #[derive(strum::Display)]
  #[strum(suffix=".json")]
  #[strum(serialize_all="snake_case")]
  enum StorageConfiguration {
    PostgresProvider,
    S3StorageProvider,
    AzureStorageProvider,
  }

  fn main() {
    let response = SurveyResponse::Other("It was good".into());
    println!("Loading configuration from: {}", StorageConfiguration::PostgresProvider);
    // prints: Loaded Configuration from: postgres_provider.json
  }
  ```

* [#446](https://github.com/Peternator7/strum/pull/446): Drop needless `rustversion` dependency.

## 0.27.1

* [#414](https://github.com/Peternator7/strum/pull/414): Fix docrs build error.
* [#417](https://github.com/Peternator7/strum/pull/417): Mention `parse_error_ty` and `parse_error_fn` that had been
  left out of the docs accidentally.
* [#421](https://github.com/Peternator7/strum/pull/421)[#331](https://github.com/Peternator7/strum/pull/331): Implement
  `#[strum(transparent)]` attribute on `IntoStaticStr`, `Display` and `AsRefStr` that forwards the implmenentation to
  the inner value. Note that for static strings, the inner value must be convertible to an `&'static str`. 

  ```rust
  #[derive(strum::Display)]
  enum SurveyResponse {
    Yes,
    No,
    #[strum(transparent)]
    Other(String)
  }

  fn main() {
    let response = SurveyResponse::Other("It was good".into());
    println!("Question: Did you have fun?");
    println!("Answer: {}", response);
    // prints: Answer: It was good
  }
  ```

## 0.27.0

### Highlights

* [#407](https://github.com/Peternator7/strum/pull/407): `Display` is now correctly derived in `[no_std]` projects.
* [#402](https://github.com/Peternator7/strum/pull/402): `EnumIter` now implements `Send + Sync`
* [#400](https://github.com/Peternator7/strum/pull/400): `EnumTryAs` now handles attributes on variant fields correctly.
* [#398](https://github.com/Peternator7/strum/pull/398): `strum` is now on rust 2021
* [#391](https://github.com/Peternator7/strum/pull/391): `EnumProperties` correctly implements `get_bool` and `get_int`
  finally. 🎉
* [#380](https://github.com/Peternator7/strum/pull/380): `FromString` now supports 2 additional attributes, `parse_error_ty`
  and `parse_error_fn` that can be added to use a custom error type rather than the default strum error message.
  * [#410](https://github.com/Peternator7/strum/pull/410): These attributes accept a `Path` rather than a `String`
    to improve behavior with rust-analyzer.

### Breaking Changes

* [#384](https://github.com/Peternator7/strum/pull/384): MSRV is now 1.66.1
* [#391](https://github.com/Peternator7/strum/pull/391): `EnumProperties` doesn't provide default implementations anymore.
  This would have required you to manually implement this trait which should be very uncommon.

## 0.26.4 (strum_macros)

* [#360](https://github.com/Peternator7/strum/pull/360): Fixes bug introduced with new string interpolation feature where
  unit structs took an implicit unnecessary dependency on `::core::alloc`.

## 0.26.3 (strum_macros)

* [#344](https://github.com/Peternator7/strum/pull/344): Hide `EnumTable` because it's going to be deprecated in the next
  version.
* [#357](https://github.com/Peternator7/strum/pull/357): Fixes an incompatiblity with `itertools` by using the fully
  qualified name rather than the inherent method.
* [#345](https://github.com/Peternator7/strum/pull/345): Allows unnamed tuple like variants to use their variants in
  string interpolation. `#[strum(to_string = "Field 0: {0}, Field 1: {1})")]` will now work for tuple variants

## 0.26.2

* [#337](https://github.com/Peternator7/strum/pull/337): Fix missing generic impls for `EnumTryAs`
* [#334](https://github.com/Peternator7/strum/pull/334): Support prefix in `AsRefStr`. Technically a breaking change,
  but `prefix` was just added in `0.26.0` so it's a newer feature and it makes the feature more consisent in general.

## 0.26.1

* [#325](https://github.com/Peternator7/strum/pull/325): use `core` instead of `std` in VariantArray.

## 0.26.0

### Breaking Changes

* The `EnumVariantNames` macro has been renamed `VariantNames`. The deprecation warning should steer you in
  the right direction for fixing the warning.
* The Iterator struct generated by EnumIter now has new bounds on it. This shouldn't break code unless you manually
  added the implementation in your code.
* `Display` now supports format strings using named fields in the enum variant. This should be a no-op for most code.
  However, if you were outputting a string like `"Hello {field}"`, this will now be interpretted as a format string.
* EnumDiscriminant now inherits the repr and discriminant values from your main enum. This makes the discriminant type
  closer to a mirror of the original and that's always the goal.

### New features

* The `VariantArray` macro has been added. This macro adds an associated constant `VARIANTS` to your enum. The constant
  is a `&'static [Self]` slice so that you can access all the variants of your enum. This only works on enums that only
  have unit variants.

  ```rust
  use strum::VariantArray;

  #[derive(Debug, VariantArray)]
  enum Color {
    Red,
    Blue,
    Green,
  }

  fn main() {
    println!("{:?}", Color::VARIANTS); // prints: ["Red", "Blue", "Green"]
  }
  ```

* The `EnumTable` macro has been *experimentally* added. This macro adds a new type that stores an item for each variant
  of the enum. This is useful for storing a value for each variant of an enum. This is an experimental feature because
  I'm not convinced the current api surface area is correct.

  ```rust
  use strum::EnumTable;

  #[derive(Copy, Clone, Debug, EnumTable)]
  enum Color {
    Red,
    Blue,
    Green,
  }

  fn main() {
    let mut counts = ColorTable::filled(0);
    for color in &[Color::Red, Color::Red, Color::Green]] {
      counts[color] += 1;
    }

    assert_eq!(counts[Color::Red], 2);
    assert_eq!(counts[Color::Blue], 0);
    assert_eq!(counts[Color::Green], 1);
  }
  ```

* `Display` has 2 new features:
  * the `strum(prefix = "some_value")` attribute on an enum now allows you to prepend a string onto every
    variant when you serialize it.

  * Custom `to_string` and `serialize` attributes now support string interopolation on serialization.

### PR's Merged

* [#322](https://github.com/Peternator7/strum/pull/322): avoid collisions on `std::fmt::Debug`
* [#321](https://github.com/Peternator7/strum/pull/321): avoid conflicts with consecutive underscores.
* [#314](https://github.com/Peternator7/strum/pull/314): add additional bounds to EnumIterator
* [#311](https://github.com/Peternator7/strum/pull/311): add FusedIterator bounds to EnumIterator
* [#297](https://github.com/Peternator7/strum/pull/297): New macro, add `VariantArray`
* [#296](https://github.com/Peternator7/strum/pull/296): adds prefix attribute to To/From String macros.
* [#294](https://github.com/Peternator7/strum/pull/294): use named enum fields in to_string macro.
* [#288](https://github.com/Peternator7/strum/pull/288): discriminant enums now inherit the repr from the original enum.
* [#279](https://github.com/Peternator7/strum/pull/279): Add `EnumTable` macro to generate a mapping between fieldless variants and data.


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
