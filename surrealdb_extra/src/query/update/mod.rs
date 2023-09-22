use std::marker::PhantomData;
use surrealdb::sql::statements::UpdateStatement;
use surrealdb::{Connection, Surreal};
use surrealdb::method::Query;
use crate::query::parsing::cond::ExtraCond;
use crate::query::parsing::data::ExtraData;
use crate::query::parsing::output::ExtraOutput;
use crate::query::parsing::timeout::ExtraTimeout;
use crate::query::parsing::what::ExtraValue;
use crate::query::states::{FilledWhat, NoWhat};


#[derive(Debug, Clone)]
pub struct UpdateBuilder<'r, T, C>
    where C: Connection
{
    pub statement: UpdateStatement,
    pub(crate) db: &'r Surreal<C>,
    pub(crate) what_state: PhantomData<T>,
}

impl<'r, C> UpdateBuilder<'r, NoWhat, C>
    where C: Connection
{
    pub fn new(db: &'r Surreal<C>) -> Self {
        Self {
            statement: Default::default(),
            db,
            what_state: Default::default(),
        }
    }

    /// This functions selects from either the table, table:id or more
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::engine::any::connect;
    /// use surrealdb::sql::Thing;
    /// use surrealdb_extra::query::update::UpdateBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let db = connect("mem://").await.unwrap();
    ///     UpdateBuilder::new(&db).what("test").field("test"); // This becomes `SELECT test FROM test`
    ///
    ///     UpdateBuilder::new(&db).what(Thing::from(("test", "test"))).field("test"); // This becomes `SELECT test FROM test:test`
    /// }
    /// ```
    ///
    /// You can also use the Value type inside surrealdb for more complex requests
    pub fn what(self, what: impl Into<ExtraValue>) -> UpdateBuilder<'r, FilledWhat, C> {
        let Self { mut statement, db, .. } = self;

        statement.what = what.into().0;

        UpdateBuilder {
            statement,
            db,
            what_state: Default::default(),

        }
    }
}

impl<'r, C> UpdateBuilder<'r, FilledWhat, C>
    where C: Connection
{
    pub fn only(self) -> Self {
        let Self { mut statement, db, .. } = self;

        statement.only = true;

        Self {
            statement,
            db,
            what_state: Default::default(),
        }
    }

    pub fn data(self, data: impl Into<ExtraData>) -> Self {
        let Self { mut statement, db, .. } = self;

        let data = data.into().0;

        statement.data = Some(data);

        Self {
            statement,
            db,
            what_state: Default::default(),
        }
    }

    /// This function is for `WHERE`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::engine::any::connect;
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::cond_vec;
    /// use surrealdb_extra::query::parsing::cond::Condition;
    /// use surrealdb_extra::query::update::UpdateBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///
    ///     let db = connect("mem://").await.unwrap();
    ///     UpdateBuilder::new(&db).what("test").condition("test");
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test`
    ///
    ///     UpdateBuilder::new(&db).what("test").condition((Operator::Not, "test"));
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE !test`
    ///
    ///     UpdateBuilder::new(&db).what("test").condition(("test", Operator::MoreThan, "$test"));
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test > $test`
    ///
    ///     UpdateBuilder::new(&db).what("test").condition(("test", Operator::Equal, "$test"));
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test = $test`
    ///
    ///     // For multiple conditions the vector `cond_vec![]` is recommended for usage
    ///     UpdateBuilder::new(&db).what("test")
    ///     .condition(cond_vec![("test1", Operator::Equal, "$test1"), Operator::And, ("test2", Operator::Equal, "$test2"), Operator::Or, "test", Operator::Or, (Operator::Not, "test")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test1 = $test1 AND test2 = $test2 OR test OR !test`
    ///
    ///     // Other method of using the multi conditions
    ///     UpdateBuilder::new(&db).what("test").condition(vec![Condition::from("test"), Condition::from(Operator::And), Condition::from(("name", Operator::LessThanOrEqual, "$name"))]);
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test AND name <= $name`
    ///
    /// }
    /// ```
    /// You can also use the Cond/Value type inside surrealdb for more complex requests
    pub fn condition(self, cond: impl Into<ExtraCond>) -> Self {
        let Self { mut statement, db, .. } = self;

        let cond = cond.into().0;

        statement.cond = Some(cond);

        Self {
            statement,
            db,
            what_state: Default::default(),
        }
    }

    pub fn output(self, output: impl Into<ExtraOutput>) -> Self {
        let Self { mut statement, db, .. } = self;

        let output = output.into().0;

        statement.output = Some(output);

        Self {
            statement,
            db,
            what_state: Default::default(),
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
        }
    }

    pub fn parallel(self) -> Self {
        let Self { mut statement, db, .. } = self;

        statement.parallel = true;

        Self {
            statement,
            db,
            what_state: Default::default(),
        }
    }

    pub fn to_query(self) -> Query<'r, C> {
        self.db.query(self.statement)
    }
}
