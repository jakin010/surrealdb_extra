/// Derives the `SurrealValue` trait for a struct or enum.
///
/// This macro simplifies the process of integrating custom Rust types with `surrealdb_extra`'s
/// JSON value system. It automatically implements the `kind_of()`, `into_value()`, and
/// `from_value()` methods.
///
/// # Requirements
///
/// This macro relies on `serde` for the underlying data transformation. You **must** ensure that
/// `serde::Serialize` and `serde::Deserialize` (or their `_repr` variants) are derived or implemented
/// on the target type.
///
/// # Configuration
///
/// You can optionally configure the specific SurrealDB type "Kind" using the helper attribute:
/// `#[surreal_value_json(kind = "...")]`.
///
/// *   The value provided to `kind` is parsed as a Rust expression mapping to the `Kind` enum variants.
/// *   **Default Behavior:** If the `kind` attribute is omitted, the type defaults to `Kind::Any`.
///
/// # Examples
///
/// ### 1. Explicitly defining an Object
/// Use this when your struct maps directly to a SurrealDB object.
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use surrealdb_extra::surreal_value_json::SurrealValueJson;
///
/// #[derive(Serialize, Deserialize, SurrealValueJson)]
/// #[surreal_value_json(kind = "Object")]
/// struct User {
///     name: String,
/// }
/// ```
///
/// ### 2. Enums and Primitives
/// You can map Rust enums to specific SurrealDB primitives, such as Integers.
///
/// ```rust
/// use serde_repr::{Serialize_repr, Deserialize_repr};
/// use surrealdb_extra::surreal_value_json::SurrealValueJson;
///
/// #[derive(Serialize_repr, Deserialize_repr, SurrealValueJson)]
/// #[surreal_value_json(kind = "Int")]
/// #[repr(u8)]
/// enum Status {
///     Active = 1,
///     Inactive = 0,
/// }
/// ```
///
/// ### 3. Default Behavior (Kind::Any)
/// If no kind is specified, it defaults to `Any`.
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use surrealdb_extra::surreal_value_json::SurrealValueJson;
///
/// #[derive(Debug, Serialize, Deserialize, SurrealValueJson)]
/// struct GenericData {
///     name: String,
/// }
///
/// // GenericData::kind_of() will return Kind::Any
/// ```

pub use surrealdb_extra_derive::SurrealValueJson;
pub use surrealdb::types::{anyhow, SurrealValue, Kind, Value};
pub use serde_json::{json, from_value};
