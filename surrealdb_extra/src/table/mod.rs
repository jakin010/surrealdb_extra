//! The `Table` trait represents a database table
//!
//! To use this trait, implement it for your struct representing the table. The struct must have the following attributes:
//! - `#[derive(Table)]` to automatically derive the implementation of the `Table` trait.
//! - `#[table(name = "...")]` to specify the name of the table in the database.


pub use surrealdb_extra_derive::Table;

pub trait Table {
    const TABLE_NAME: &'static str;

    fn create_record_id(id: impl Into<surrealdb::types::RecordIdKey>) -> surrealdb::types::RecordId {
        surrealdb::types::RecordId::new(Self::TABLE_NAME, id)
    }
}
