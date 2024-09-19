use std::marker::PhantomData;
use serde::Serialize;
use surrealdb::{Connection, Surreal};
use surrealdb::method::Query;
use surrealdb::sql::statements::RelateStatement;
use surrealdb::sql::{Data, to_value};
use crate::query::parsing::output::ExtraOutput;
use crate::query::parsing::set_expression::SetExpression;
use crate::query::parsing::table::ExtraTable;
use crate::query::parsing::timeout::ExtraTimeout;
use crate::query::parsing::value::ExtraValue;
use crate::query::states::{FilledData, FilledRelation, NoData, NoRelation};

#[derive(Debug, Clone)]
pub struct RelateBuilder<'r, Client, T, D>
    where Client: Connection
{
    pub statement: RelateStatement,
    pub(crate) db: &'r Surreal<Client>,
    pub(crate) relate_state: PhantomData<T>,
    pub(crate) data_state: PhantomData<D>,
}

impl<'r, Client> RelateBuilder<'r, Client, NoRelation, NoData>
    where Client: Connection
{
    pub fn new(db: &'r Surreal<Client>) -> Self {
        Self {
            statement: Default::default(),
            db,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `RELATE`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::engine::any::connect;
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::query::statement::StatementBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     use surrealdb::sql::Thing;
    ///     let db = connect("mem://").await.unwrap();
    ///
    ///     db.relate_builder().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2")));
    ///     // The above builder becomes `RELATE test:test->test->test2:test2
    ///
    /// }
    pub fn relation(self, from: impl Into<ExtraValue>, kind: impl Into<ExtraTable>, with: impl Into<ExtraValue>) -> RelateBuilder<'r, Client, FilledRelation, NoData> {
        let Self { mut statement, db, .. } = self;

        statement.from = from.into().0;
        statement.kind = kind.into().0;
        statement.with = with.into().0;

        RelateBuilder {
            statement,
            db,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }
}

impl<'r, Client> RelateBuilder<'r, Client, FilledRelation, NoData>
    where Client: Connection
{
    /// This function is for `SET`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::engine::any::connect;
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::query::statement::StatementBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     use surrealdb::sql::Thing;
    ///     let db = connect("mem://").await.unwrap();
    ///
    ///     db.relate_builder().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).set(vec![("test", Operator::Equal, "test")]);
    ///     // The above builder becomes `RELATE test:test->test->test2:test2 SET test = 'test'
    ///
    ///     db.relate_builder().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);
    ///     // The above builder becomes `RELATE test:test->test->test2:test2 SET test = 'test', test2 = 'test2'
    ///
    /// }
    pub fn set(self, set: impl Into<SetExpression>) -> RelateBuilder<'r, Client, FilledRelation, FilledData> {
        let Self { mut statement, db, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        RelateBuilder {
            statement,
            db,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `CONTENT`
    ///
    /// Example:
    /// ```rust
    /// use serde::Serialize;
    /// use surrealdb::engine::any::connect;
    /// use surrealdb_extra::query::statement::StatementBuilder;
    /// use surrealdb::sql::Thing;
    ///
    /// #[derive(Serialize)]
    /// pub struct Test {
    ///     test: String,
    ///     magic: bool
    /// }
    /// #[tokio::main]
    /// async fn main() {
    ///
    ///     let db = connect("mem://").await.unwrap();
    ///     db.relate_builder().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).content(Test { test: "test".to_string(), magic: true });
    ///     // The above builder becomes `RELATE test:test->test->test2:test2 CONTENT { test: "test", magic: true }
    ///
    /// }
    pub fn content(self, content: impl Serialize + 'static) -> RelateBuilder<'r, Client, FilledRelation, FilledData> {
        let Self { mut statement, db, .. } = self;

        let val = to_value(content).unwrap_or_default();

        statement.data = Some(Data::ContentExpression(val));

        RelateBuilder {
            statement,
            db,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }
}

impl<'r, Client, D> RelateBuilder<'r, Client, FilledRelation, D>
    where Client: Connection
{
    pub fn only(self) -> Self {
        let Self { mut statement, db, .. } = self;

        statement.only = true;

        Self {
            statement,
            db,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `RETURN`
    pub fn output(self, output: impl Into<ExtraOutput>) -> Self {
        let Self { mut statement, db, .. } = self;

        let output = output.into().0;

        statement.output = Some(output);

        Self {
            statement,
            db,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// You can also use the Timeout type inside surrealdb or Duration inside standard for more complex requests
    pub fn timeout(self, timeout: impl Into<ExtraTimeout>) -> Self {
        let Self { mut statement, db, .. } = self;

        let timeout = timeout.into().0;

        statement.timeout = Some(timeout);

        Self {
            statement,
            db,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    pub fn parallel(self) -> Self {
        let Self { mut statement, db, .. } = self;

        statement.parallel = true;

        Self {
            statement,
            db,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    pub fn to_query(self) -> Query<'r, Client> {
        self.db.query(self.statement)
    }

}

#[cfg(test)]
mod test {
    use surrealdb::engine::any::{Any, connect};
    use surrealdb::opt::IntoQuery;
    use surrealdb::sql::Thing;
    use surrealdb::sql::Operator;
    use surrealdb::Surreal;
    use crate::query::statement::StatementBuilder;

    use super::*;

    async fn db() -> Surreal<Any> {
        let db = connect("mem://").await.unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        db
    }

    #[tokio::test]
    async fn relate_table() {
        let db = db().await;

        let relate = RelateBuilder::new(&db).relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2")));

        let query = relate.statement.into_query();

        assert!(query.is_ok());
    }

    #[tokio::test]
    async fn relate_table_data() {
        let db = db().await;

        let relate = RelateBuilder::new(&db).relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);

        let query = relate.statement.into_query();

        assert!(query.is_ok());
    }

    #[tokio::test]
    async fn relate_table_db() {
        let db = db().await;

        let relate = db.relate_builder().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2")));

        let query = relate.statement.into_query();

        assert!(query.is_ok());
    }

    #[tokio::test]
    async fn relate_table_data_db() {
        let db = db().await;

        let relate = db.relate_builder().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);

        let query = relate.statement.into_query();

        assert!(query.is_ok());
    }
}
