#![cfg_attr(docsrs, feature(doc_cfg))]

/// The SurrealDB value types, re-exported from
/// [`surrealdb-types`](https://docs.rs/surrealdb-types).
///
/// This is the same crate the SurrealDB SDK exposes as `surrealdb::types`, so the types are
/// interchangeable: `surrealdb_extra::types::RecordId` *is* `surrealdb::types::RecordId`.
/// You don't need to depend on `surrealdb` to use this crate.
///
/// # Using `#[derive(SurrealValue)]` through this re-export
///
/// SurrealDB's own derive generates paths to `::surrealdb::types` (when the `surrealdb` crate is
/// in your build) or `::surrealdb_types` (when it isn't). If you depend only on
/// `surrealdb_extra`, point it at this module:
///
/// ```rust
/// use surrealdb_extra::types::SurrealValue;
///
/// #[derive(SurrealValue)]
/// #[surreal(crate = "surrealdb_extra::types")]
/// struct User {
///     name: String,
/// }
/// ```
///
/// Two quirks of SurrealDB's derive (not of this crate):
///
/// - It refers to `SurrealValue` without a path, so keep
///   `use surrealdb_extra::types::SurrealValue;` in scope, even if you write
///   `#[derive(surrealdb_extra::types::SurrealValue)]`.
/// - For `#[surreal(untagged)]` enums whose variants use `#[surreal(value = ...)]`, it ignores
///   `crate = ...` and always refers to `surrealdb_types`. Add an alias in that module:
///
/// ```rust
/// use surrealdb_extra::types::SurrealValue;
/// use surrealdb_extra::types as surrealdb_types;
///
/// #[derive(SurrealValue)]
/// #[surreal(crate = "surrealdb_extra::types")]
/// #[surreal(untagged)]
/// enum Speed {
///     #[surreal(value = "fast")]
///     Fast,
///     #[surreal(value = "slow")]
///     Slow,
/// }
/// ```
pub use surrealdb_types as types;

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "repr")]
pub mod repr;

// Compile and run the README examples as doctests.
#[cfg(all(doctest, feature = "table", feature = "repr"))]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
