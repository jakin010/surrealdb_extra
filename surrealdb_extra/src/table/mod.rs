//! The `Table` trait represents a database table and provides common operations for interacting with the table.
//!
//! To use this trait, implement it for your struct representing the table. The struct must have the following attributes:
//! - `#[derive(Table)]` to automatically derive the implementation of the `Table` trait.
//! - `#[table(name = "...")]` to specify the name of the table in the database.
//! - `id: Option<Thing>` needs to be one of the fields
//!
//! # Example
//!
//!
//! ``` rust
//!  use serde::{Serialize, Deserialize};
//!  use surrealdb_extra::table::Table;
//!  use surrealdb::sql::Thing;
//!  use surrealdb::engine::any::connect;
//!  use surrealdb::{Surreal, Result};
//!  use tokio::main;
//!  use surrealdb::kvs::Datastore;
//!
//! // Serialize and Deserialize are must have derives
//! #[derive(Table, Serialize, Deserialize, Clone)]
//! #[table(name = "my_table")]
//! struct MyStruct {
//!  // id is the only field that is a must id must be of type Option<::surrealdb::sql::Thing>
//!     id: Option<Thing>,
//!     // other fields...
//!     pub name: String
//! }
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     Datastore::new("memory").await.unwrap();
//!     let db = connect("mem://").await.unwrap();
//!     db.use_ns("ns").use_db("db").await.unwrap();
//!
//!     let my_struct = MyStruct {
//!         id: None,
//!         name: "my name is".into()
//!     };
//!
//!     let created_struct = my_struct.create(&db).await.unwrap();
//!
//!     let mut updated_struct = created_struct.first().unwrap().clone();
//!     updated_struct.name = "test".to_string();
//!
//!     let updated_struct: Option<MyStruct> = updated_struct.update(&db).await.unwrap();
//!
//!     let deleted_struct: Option<MyStruct> = MyStruct::delete(updated_struct.unwrap().id.unwrap().id.to_raw(), &db).await.unwrap();
//!
//!     let get_all: Vec<MyStruct> = MyStruct::get_all(&db).await.unwrap();
//!
//!     let get_by_id: Option<MyStruct> = MyStruct::get_by_id("id", &db).await.unwrap();
//!
//!     Ok(())
//! }
//! ```

pub mod err;

#[cfg(feature = "derive")]
pub use ::surrealdb_extra_derive::Table;

use ::async_trait::async_trait;
use ::serde::de::DeserializeOwned;
use ::serde::Serialize;
use ::surrealdb::{Connection, Surreal};
use crate::query_builder::Query;
use crate::query_builder::states::{NoFieldsQuery, NoFilterQuery, TableQuery};
pub use crate::table::err::TableError;

#[async_trait]
pub trait Table: Serialize + DeserializeOwned + Send + Sync + Sized
{
    fn table_name() -> String;

    fn get_id(&self) -> &Option<::surrealdb::sql::Thing>;

    fn set_id(&mut self, id: impl Into<::surrealdb::sql::Thing>);

    async fn create<C: Connection>(self, db: &Surreal<C>) -> Result<Vec<Self>, TableError> {
        let s: Vec<Self> = db.create(Self::table_name()).content(self).await.map_err(TableError::Db)?;

        Ok(s)
    }

    async fn delete<C: Connection>(id: impl Into<String> + std::marker::Send, db: &Surreal<C>) -> Result<Option<Self>, TableError> {
        let s: Option<Self> = db.delete((Self::table_name(), id.into())).await.map_err(TableError::Db)?;

        Ok(s)
    }

    async fn get_all<C: Connection>(db: &Surreal<C>) -> Result<Vec<Self>, TableError> {
        let vec_s: Vec<Self> = db.select(Self::table_name()).await.map_err(TableError::Db)?;

        Ok(vec_s)
    }

    async fn get_by_id<C: Connection>(id: impl Into<String> + std::marker::Send, db: &Surreal<C>) -> Result<Option<Self>, TableError> {
        let s: Option<Self> = db.select((Self::table_name(), id.into())).await.map_err(TableError::Db)?;

        Ok(s)
    }

    async fn update<C: Connection>(self, db: &Surreal<C>) -> Result<Option<Self>, TableError> {
        let s: Option<Self> = db
            .update(
                (
                    Self::table_name(),
                    self.get_id().clone().ok_or(TableError::IdEmpty)?.id.clone().to_raw()
                )
            )
            .merge(self)
            .await.map_err(TableError::Db)?;

        Ok(s)
    }

    #[cfg(feature = "query_builder")]
    fn select(id: Option<String>) -> Query<TableQuery, NoFieldsQuery, NoFilterQuery> {
        Query::new().from(Self::table_name(), id)
    }
}
