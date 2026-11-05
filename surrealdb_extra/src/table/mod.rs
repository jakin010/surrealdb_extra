//! The `Table` trait represents a database table
//!
//! Derive it on the struct that represents a table:
//! - `#[derive(Table)]` implements the `Table` trait.
//! - `#[table(name = "...")]` sets the table name. It is optional: without it, the name is the
//!   struct name in snake_case (`UserProfile` becomes `user_profile`).
//! - `#[table(crate = "...")]` sets the path to this crate. You only need it when the derive
//!   comes through another crate's re-export: a renamed dependency in `Cargo.toml` is found
//!   automatically.
//!
//! ```rust
//! use surrealdb_extra::table::Table;
//!
//! #[derive(Table)]
//! struct UserProfile;
//!
//! #[derive(Table)]
//! #[table(name = "people")]
//! struct Person;
//!
//! assert_eq!(UserProfile::TABLE_NAME, "user_profile");
//! assert_eq!(Person::TABLE_NAME, "people");
//! ```

pub use surrealdb_extra_derive::Table;

/// A database table. Derive it with `#[derive(Table)]`.
pub trait Table {
    /// The table's name in the database.
    const TABLE_NAME: &'static str;

    /// Creates a record ID in this table, e.g. `user:abc`.
    fn create_record_id(id: impl Into<crate::types::RecordIdKey>) -> crate::types::RecordId {
        crate::types::RecordId::new(Self::TABLE_NAME, id)
    }
}
