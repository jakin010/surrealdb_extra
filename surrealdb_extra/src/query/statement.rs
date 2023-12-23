use std::marker::PhantomData;
use surrealdb::{Connection, Surreal};
use crate::query::relate::RelateBuilder;
use crate::query::select::SelectBuilder;
use crate::query::states::{NoCond, NoFields, NoRelation, NoWhat};
use crate::query::update::UpdateBuilder;

pub trait StatementBuilder<Client>
    where Client: Connection
{
    fn select_builder(&self) -> SelectBuilder<Client, NoWhat, NoFields, NoCond>;
    fn update_builder(&self) -> UpdateBuilder<Client, NoWhat, NoCond>;

    fn relate_builder(&self) -> RelateBuilder<NoRelation, Client>;
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

    fn update_builder(&self) -> UpdateBuilder<Client, NoWhat, NoCond> {
        UpdateBuilder {
            statement: Default::default(),
            db: self,
            what_state: PhantomData,
            cond_state: PhantomData,
        }
    }

    fn relate_builder(&self) -> RelateBuilder<
    NoRelation, Client> {
        RelateBuilder {
            statement: Default::default(),
            db: self,
            relate_state: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use surrealdb::engine::any::connect;
    use crate::query::statement::StatementBuilder;

    #[tokio::test]
    async fn select_builder() {
        let db = connect("mem://").await.unwrap();

        let select_builder = db.select_builder();

        let db_type_id = db.type_id();
        let select_type_id = select_builder.db.type_id();

        assert_eq!(db_type_id, select_type_id);
    }
    #[tokio::test]
    async fn update_builder() {
        let db = connect("mem://").await.unwrap();

        let update_builder = db.update_builder();

        let db_type_id = db.type_id();
        let update_type_id = update_builder.db.type_id();

        assert_eq!(db_type_id, update_type_id);
    }
    #[tokio::test]
    async fn relate_builder() {
        let db = connect("mem://").await.unwrap();

        let relate_builder = db.relate_builder();

        let db_type_id = db.type_id();
        let relate_type_id = relate_builder.db.type_id();

        assert_eq!(db_type_id, relate_type_id);
    }
}
