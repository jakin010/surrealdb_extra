//! The `Table` trait represents a database table
//!
//! Derive it on the struct that represents a table:
//! - `#[derive(Table)]` implements the `Table` trait.
//! - `#[table(name = "...")]` sets the table name. It is optional: without it, the name is the
//!   struct name in snake_case (`UserProfile` becomes `user_profile`).
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

pub trait Table {
    const TABLE_NAME: &'static str;

    fn create_record_id(
        id: impl Into<surrealdb::types::RecordIdKey>,
    ) -> surrealdb::types::RecordId {
        surrealdb::types::RecordId::new(Self::TABLE_NAME, id)
    }
}
