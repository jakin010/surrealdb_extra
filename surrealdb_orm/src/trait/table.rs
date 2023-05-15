pub use surrealdb_orm_derive::Table;

pub trait Table {
    fn name() -> String;
}