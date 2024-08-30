use std::marker::PhantomData;
use serde::Serialize;
use surrealdb::{Connection, Surreal};
use surrealdb::method::Query;
use surrealdb::sql::statements::CreateStatement;
use surrealdb::sql::{Data, to_value};
use crate::query::parsing::data::ExtraData;
use crate::query::parsing::output::ExtraOutput;
use crate::query::parsing::set_expression::SetExpression;
use crate::query::parsing::timeout::ExtraTimeout;
use crate::query::parsing::unset_expression::UnsetExpression;
use crate::query::parsing::what::ExtraValue;
use crate::query::states::{FilledData, FilledWhat, NoData, NoWhat};

#[derive(Debug, Clone)]
pub struct CreateBuilder<'r, Client, T, D>
    where Client: Connection
{
    pub statement: CreateStatement,
    pub(crate) db: &'r Surreal<Client>,
    pub(crate) what_state: PhantomData<T>,
    pub(crate) data_state: PhantomData<D>,
}


impl<'r, Client> CreateBuilder<'r, Client, NoWhat, NoData>
    where Client: Connection
{
    pub fn new(db: &'r Surreal<Client>) -> Self {
        Self {
            statement: Default::default(),
            db,
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This functions selects from either the table, table:id or more
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::engine::any::connect;
    /// use surrealdb_extra::query::create::CreateBuilder;
    /// use surrealdb::sql::Thing;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let db = connect("mem://").await.unwrap();
    ///     CreateBuilder::new(&db).what("test");
    ///
    ///     CreateBuilder::new(&db).what(Thing::from(("test", "test")));
    /// }
    /// ```
    ///
    /// You can also use the Value type inside surrealdb for more complex requests
    pub fn what(self, what: impl Into<ExtraValue>) -> CreateBuilder<'r, Client, FilledWhat, NoData> {
        let Self { mut statement, db, .. } = self;

        statement.what = what.into().0;

        CreateBuilder {
            statement,
            db,
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }
}

impl<'r, Client> CreateBuilder<'r, Client, FilledWhat, NoData>
    where Client: Connection
{
    /// This function is for `SET` || `UNSET` || `MERGE` and more
    pub fn data(self, data: impl Into<ExtraData>) -> CreateBuilder<'r, Client, FilledWhat, FilledData> {
        let Self { mut statement, db, .. } = self;

        let data = data.into().0;

        statement.data = Some(data);

        CreateBuilder {
            statement,
            db,
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

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
    ///     let db = connect("mem://").await.unwrap();
    ///
    ///     db.create_builder().what("test").set(vec![("test", Operator::Equal, "test")]);
    ///     // The above builder becomes `CREATE test SET test = 'test'
    ///
    ///     db.create_builder().what("test").set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);
    ///     // The above builder becomes `CREATE test SET test = 'test', test2 = 'test2'
    ///
    /// }
    pub fn set(self, set: impl Into<SetExpression>) -> CreateBuilder<'r, Client, FilledWhat, FilledData> {
        let Self { mut statement, db, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        CreateBuilder {
            statement,
            db,
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `UNSET`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::engine::any::connect;
    /// use surrealdb_extra::query::statement::StatementBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let db = connect("mem://").await.unwrap();
    ///
    ///     db.create_builder().what("test").unset(vec!["test"]);
    ///     // The above builder becomes `CREATE test UNSET test
    ///
    ///     db.create_builder().what("test").unset(vec!["test", "test"]);
    ///     // The above builder becomes `CREATE test UNSET test, test
    ///
    /// }
    pub fn unset(self, set: impl Into<UnsetExpression>) -> CreateBuilder<'r, Client, FilledWhat, FilledData> {
        let Self { mut statement, db, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        CreateBuilder {
            statement,
            db,
            what_state: Default::default(),
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
    ///
    /// #[derive(Serialize)]
    /// pub struct Test {
    ///     test: String,
    ///     magic: bool
    /// }
    /// #[tokio::main]
    /// async fn main() {
    ///     let db = connect("mem://").await.unwrap();
    ///
    ///     db.create_builder().what("test").content(Test { test: "test".to_string(), magic: true });
    ///     // The above builder becomes `CREATE test CONTENT { test: "test", magic: true }
    ///
    /// }
    pub fn content(self, content: impl Serialize + 'static) -> CreateBuilder<'r, Client, FilledWhat, FilledData> {
        let Self { mut statement, db, .. } = self;

        let val = to_value(content).unwrap_or_default();

        statement.data = Some(Data::ContentExpression(val));

        CreateBuilder {
            statement,
            db,
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }
}

impl<'r, Client> CreateBuilder<'r, Client, FilledWhat, FilledData>
    where Client: Connection
{
    pub fn only(self) -> Self {
        let Self { mut statement, db, .. } = self;

        statement.only = true;

        Self {
            statement,
            db,
            what_state: Default::default(),
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
            what_state: Default::default(),
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
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

    pub fn parallel(self) -> Self {
        let Self { mut statement, db, .. } = self;

        statement.parallel = true;

        Self {
            statement,
            db,
            what_state: Default::default(),
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
    use crate::op;
    use crate::query::statement::StatementBuilder;
    use super::*;

    async fn db() -> Surreal<Any> {
        let db = connect("mem://").await.unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        db
    }

    #[derive(Serialize)]
    struct Test {
        test1: String,
        test2: i32
    }

    #[tokio::test]
    async fn init() {
        let db = db().await;

        let create_builder = CreateBuilder::new(&db).what("test");

        let query = create_builder.statement.into_query();

        assert!(query.is_ok());
    }

    #[tokio::test]
    async fn with_set() {
        let db = db().await;

        let create_builder = db.create_builder().what("test").set(vec![("test", op!(+=), 4)]);

        let query = create_builder.statement.into_query();

        assert!(query.is_ok());
    }

    #[tokio::test]
    async fn with_content() {
        let db = db().await;

        let create_builder = db.create_builder().what("test").content(Test {
            test1: "test".to_string(),
            test2: -55
        });

        let query = create_builder.statement.into_query();

        assert!(query.is_ok());
    }
}
