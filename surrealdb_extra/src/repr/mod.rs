//! Store fieldless enums as integers, like [`serde_repr`](https://docs.rs/serde_repr) but for
//! SurrealDB's `SurrealValue` trait.
//!
//! Enable with the `repr` feature.

/// Derives `surrealdb::types::SurrealValue` for a fieldless enum with an integer `#[repr]`.
///
/// Each variant is stored as its discriminant, as a plain SurrealDB integer. No serde is involved.
///
/// # Requirements
///
/// - The enum must have an integer repr: `u8`, `u16`, `u32`, `u64`, `usize`, `i8`, `i16`,
///   `i32`, `i64` or `isize`. 128-bit reprs are not supported, because SurrealDB has no
///   128-bit integer type.
/// - Every variant must be a unit variant (no fields), and the enum can't be generic.
/// - The `surrealdb` crate must be a dependency under that name. Use
///   `#[surreal_repr(crate = "...")]` to point at a different path.
///
/// Discriminants can be implicit, negative, or const expressions. The compiler works them out,
/// so they always match what `MyEnum::Variant as u8` gives.
///
/// # Attributes
///
/// | Attribute | Where | Meaning |
/// |---|---|---|
/// | `#[surreal_repr(other)]` | one variant | Returned for unknown values that still fit the repr type |
/// | `#[surreal_repr(kind = "literal")]` | enum | `kind_of()` is the exact values, e.g. `0 \| 1 \| 7` (default without `other`) |
/// | `#[surreal_repr(kind = "int")]` | enum | `kind_of()` is `int` (default with `other`) |
/// | `#[surreal_repr(kind = "number")]` | enum | `kind_of()` is `number` |
/// | `#[surreal_repr(crate = "path")]` | enum | Path to `surrealdb::types` (default `::surrealdb::types`) |
///
/// `other` can't be combined with `kind = "literal"`, since the schema would reject the values
/// `other` is meant to catch.
///
/// # Reading values
///
/// `from_value` accepts any integer width and whole decimals (`7`, `7.000`). It returns an error
/// for floats, non-numbers, values outside the repr's range, and unknown values (unless there is
/// an `other` variant).
///
/// For `u64`/`usize` enums, values above `i64::MAX` are stored as a whole decimal, the same as a
/// plain `u64` field.
///
/// # Examples
///
/// ```rust
/// use surrealdb::types::{Kind, KindLiteral, Number, SurrealValue, ToSql, Value};
/// use surrealdb_extra::repr::SurrealValueRepr;
///
/// #[derive(Debug, PartialEq, SurrealValueRepr)]
/// #[repr(u8)]
/// enum Status {
///     Inactive = 0,
///     Active = 1,
///     Banned = 7,
/// }
///
/// assert_eq!(Status::Banned.into_value(), Value::Number(Number::Int(7)));
/// assert_eq!(Status::from_value(Value::Number(Number::Int(1))).unwrap(), Status::Active);
/// assert!(Status::from_value(Value::Number(Number::Int(3))).is_err());
///
/// // The kind can be used in a schema: `DEFINE FIELD status ON user TYPE 0 | 1 | 7`
/// assert_eq!(Status::kind_of().to_sql(), "0 | 1 | 7");
/// ```
///
/// With a fallback variant:
///
/// ```rust
/// use surrealdb::types::{Kind, Number, SurrealValue, Value};
/// use surrealdb_extra::repr::SurrealValueRepr;
///
/// #[derive(Debug, PartialEq, SurrealValueRepr)]
/// #[repr(i16)]
/// enum Level {
///     Low = -1,
///     Mid, // 0
///     High = 10,
///     #[surreal_repr(other)]
///     Unknown = 99,
/// }
///
/// assert_eq!(Level::from_value(Value::Number(Number::Int(0))).unwrap(), Level::Mid);
/// assert_eq!(Level::from_value(Value::Number(Number::Int(42))).unwrap(), Level::Unknown);
/// assert_eq!(Level::kind_of(), Kind::Int);
/// ```
///
/// It also works as a field in a type that uses SurrealDB's own derive:
///
/// ```rust
/// use surrealdb::types::SurrealValue;
/// use surrealdb_extra::repr::SurrealValueRepr;
///
/// #[derive(SurrealValueRepr)]
/// #[repr(u8)]
/// enum Status { Inactive, Active }
///
/// #[derive(SurrealValue)]
/// struct User {
///     name: String,
///     status: Status,
/// }
/// ```
pub use surrealdb_extra_derive::SurrealValueRepr;
