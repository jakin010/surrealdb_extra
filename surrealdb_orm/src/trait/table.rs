pub use ::surrealdb_orm_derive::Table;
use ::async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::{Connection, Error, Surreal};
use surrealdb::error::Db;
use surrealdb::sql::Thing;
use crate::query_builder::Query;
use crate::query_builder::states::{NoFieldsQuery, NoFilterQuery, TableQuery};


#[async_trait]
pub trait Table: Serialize + DeserializeOwned + Send + Sync + Sized
{
    fn table_name() -> String;

    fn get_id(&self) -> &Option<::surrealdb::sql::Thing>;

    fn set_id(&mut self, id: ::surrealdb::sql::Thing);

    fn fields() -> Vec<&'static str>;

    async fn create<C: Connection>(self, db: &Surreal<C>) -> Result<Self, Error> {
        let s: Self = db.create(Self::table_name()).content(self).await?;

        Ok(s)
    }

    async fn delete<C: Connection>(id: Thing, db: &Surreal<C>) -> Result<Option<Self>, Error> {
        let id = id.id.to_raw();
        let s: Option<Self> = db.delete((Self::table_name(), id)).await?;

        Ok(s)
    }

    async fn get_all<C: Connection>(db: &Surreal<C>) -> Result<Vec<Self>, Error> {
        let vec_s: Vec<Self> = db.select(Self::table_name()).await?;

        Ok(vec_s)
    }

    async fn get_by_id<C: Connection>(id: Thing, db: &Surreal<C>) -> Result<Option<Self>, Error> {
        let s: Option<Self> = db.select((Self::table_name(), id.id.to_raw())).await?;

        Ok(s)
    }

    async fn update<C: Connection>(self, db: &Surreal<C>) -> Result<Option<Self>, Error> {
        let s: Option<Self> = db
            .update(
                (
                    Self::table_name(),
                    self.get_id().clone().ok_or(Error::Db(Db::IdInvalid { value: "".to_string() }))?.id.to_raw()
                )
            )
            .merge(self)
            .await?;

        Ok(s)
    }

    fn select(id: Option<String>) -> Query<TableQuery, NoFieldsQuery, NoFilterQuery> {
        Query::new().from(Self::table_name(), id)
    }
}

