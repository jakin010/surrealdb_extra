use std::marker::PhantomData;
use surrealdb::{Connection, Surreal};
use crate::query::create::CreateBuilder;
use crate::query::relate::RelateBuilder;
use crate::query::select::SelectBuilder;
use crate::query::states::{NoCond, NoData, NoFields, NoRelation, NoWhat};
use crate::query::update::UpdateBuilder;

pub trait StatementBuilder<Client>
    where Client: Connection
{
    fn select_builder(&self) -> SelectBuilder<Client, NoWhat, NoFields, NoCond>;
    fn update_builder(&self) -> UpdateBuilder<Client, NoWhat, NoData, NoCond>;
    fn relate_builder(&self) -> RelateBuilder<Client, NoRelation, NoData>;
    fn create_builder(&self) -> CreateBuilder<Client, NoWhat, NoData>;
}

impl<Client: Connection> StatementBuilder<Client> for Surreal<Client>
    where Client: Connection
{
    fn select_builder(&self) -> SelectBuilder<Client, NoWhat, NoFields, NoCond> {
        SelectBuilder {
            statement: Default::default(),
            db: self,
            what_state: PhantomData,
            fields_state: PhantomData,
            cond_state: PhantomData,
        }
    }

    fn update_builder(&self) -> UpdateBuilder<Client, NoWhat, NoData, NoCond> {
        UpdateBuilder {
            statement: Default::default(),
            db: self,
            what_state: PhantomData,
            data_state: PhantomData,
            cond_state: PhantomData,
        }
    }

    fn relate_builder(&self) -> RelateBuilder<Client, NoRelation, NoData> {
        RelateBuilder {
            statement: Default::default(),
            db: self,
            relate_state: PhantomData,
            data_state: PhantomData,
        }
    }

    fn create_builder(&self) -> CreateBuilder<Client, NoWhat, NoData> {
        CreateBuilder {
            statement: Default::default(),
            db: self,
            what_state: PhantomData,
            data_state: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use surrealdb::engine::any::connect;
    use crate::query::statement::StatementBuilder;

    #[tokio::test]
    async fn select_builder() {
        let db = connect("mem://").await.unwrap();

        let _select_builder = db.select_builder();
    }
    #[tokio::test]
    async fn update_builder() {
        let db = connect("mem://").await.unwrap();

        let _update_builder = db.update_builder();
    }
    #[tokio::test]
    async fn relate_builder() {
        let db = connect("mem://").await.unwrap();

        let _relate_builder = db.relate_builder();
    }
    #[tokio::test]
    async fn create_builder() {
        let db = connect("mem://").await.unwrap();

        let _create_builder = db.create_builder();
    }
}
