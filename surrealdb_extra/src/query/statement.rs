use std::marker::PhantomData;
use surrealdb::{Connection, Surreal};
use crate::query::select::SelectBuilder;
use crate::query::select::states::{NoFields, NoWhat};

pub trait StatementBuilder<C>
    where C: Connection
{
    fn select_builder(&self) -> SelectBuilder<NoWhat, NoFields, C>;
}

impl<C: Connection> StatementBuilder<C> for Surreal<C>
    where C: Connection
{
    fn select_builder(&self) -> SelectBuilder<NoWhat, NoFields, C> {
        SelectBuilder {
            statement: Default::default(),
            db: self,
            what_state: PhantomData,
            fields_state: PhantomData,
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
}
