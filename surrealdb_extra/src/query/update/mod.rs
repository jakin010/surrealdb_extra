use std::marker::PhantomData;
use surrealdb::sql::statements::UpdateStatement;
use surrealdb::{Connection, Surreal};
use surrealdb::method::Query;
use crate::query::parsing::cond::ExtraCond;
use crate::query::parsing::data::ExtraData;
use crate::query::parsing::output::ExtraOutput;
use crate::query::parsing::set_expression::SetExpression;
use crate::query::parsing::timeout::ExtraTimeout;
use crate::query::parsing::unset_expression::UnsetExpression;
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
    ///     UpdateBuilder::new(&db).what("test");
    ///
    ///     UpdateBuilder::new(&db).what(Thing::from(("test", "test")));
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

    /// This function is for `SET` || `UNSET` || `MERGE` and more
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
    ///     db.update_builder().what("test").set(vec![("test", Operator::Equal, "test")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test'
    ///
    ///     db.update_builder().what("test").set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test', test2 = 'test2'
    ///
    /// }
    pub fn set(self, set: impl Into<SetExpression>) -> Self {
        let Self { mut statement, db, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        Self {
            statement,
            db,
            what_state: Default::default(),
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
    ///     db.update_builder().what("test").unset(vec!["test"]);
    ///     // The above builder becomes `UPDATE test UNSET test
    ///
    ///     db.update_builder().what("test").unset(vec!["test", "test"]);
    ///     // The above builder becomes `UPDATE test UNSET test, test
    ///
    /// }
    pub fn unset(self, set: impl Into<UnsetExpression>) -> Self {
        let Self { mut statement, db, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

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
    ///     UpdateBuilder::new(&db).what("test").set(vec![("test", Operator::Equal, "test")]).condition("test");
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test`
    ///
    ///     UpdateBuilder::new(&db).what("test").set(vec![("test", Operator::Equal, "test")]).condition((Operator::Not, "test"));
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE !test`
    ///
    ///     UpdateBuilder::new(&db).what("test").set(vec![("test", Operator::Equal, "test")]).condition(("test", Operator::MoreThan, "$test"));
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test > $test`
    ///
    ///     UpdateBuilder::new(&db).what("test").set(vec![("test", Operator::Equal, "test")]).condition(("test", Operator::Equal, "$test"));
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test = $test`
    ///
    ///     // For multiple conditions the vector `cond_vec![]` is recommended for usage
    ///     UpdateBuilder::new(&db).what("test").set(vec![("test", Operator::Equal, "test")])
    ///     .condition(cond_vec![("test1", Operator::Equal, "$test1"), Operator::And, ("test2", Operator::Equal, "$test2"), Operator::Or, "test", Operator::Or, (Operator::Not, "test")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test1 = $test1 AND test2 = $test2 OR test OR !test`
    ///
    ///     // Other method of using the multi conditions
    ///     UpdateBuilder::new(&db).what("test").set(vec![("test", Operator::Equal, "test")]).condition(vec![Condition::from("test"), Condition::from(Operator::And), Condition::from(("name", Operator::LessThanOrEqual, "$name"))]);
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

    /// This function is for `RETURN`
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

#[cfg(test)]
mod test {
    use surrealdb::engine::any::{Any, connect};
    use surrealdb::opt::IntoQuery;
    use surrealdb::sql::Operator;
    use super::*;

    async fn db() -> Surreal<Any> {
        let db = connect("mem://").await.unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        db
    }

    #[tokio::test]
    async fn update_builder() {
        let db = db().await;

        let update = UpdateBuilder::new(&db).what("test");

        let query = update.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn update_builder_with_set_and_cond() {
        let db = db().await;

        let update = UpdateBuilder::new(&db).what("test").set(vec![("test", Operator::Equal, "test")]).condition("test");

        let query = update.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn update_builder_with_unset_and_cond() {
        let db = db().await;

        let update = UpdateBuilder::new(&db).what("test").unset(vec!["test", "test"]).condition("test");

        let query = update.statement.into_query();

        assert!(query.is_ok())
    }
}
