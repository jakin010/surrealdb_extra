pub use ::surrealdb_orm_derive::Table;


pub trait Table {
    fn name() -> String;

    fn id(&self) -> ::surrealdb::sql::Thing;
}