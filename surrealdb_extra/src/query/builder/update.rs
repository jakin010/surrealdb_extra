use std::marker::PhantomData;
use serde::Serialize;
use surrealdb::sql::statements::UpdateStatement;
use surrealdb::{Connection, Surreal};
use surrealdb::method::Query;
use surrealdb::sql::{Data, to_value};
use crate::query::parsing::cond::ExtraCond;
use crate::query::parsing::data::ExtraData;
use crate::query::parsing::output::ExtraOutput;
use crate::query::parsing::set_expression::SetExpression;
use crate::query::parsing::timeout::ExtraTimeout;
use crate::query::parsing::unset_expression::UnsetExpression;
use crate::query::parsing::what::ExtraValue;
use crate::query::states::{FilledCond, FilledData, FilledWhat, NoCond, NoData, NoWhat};


#[derive(Debug, Clone)]
pub struct UpdateBuilder<T, D, C> {
    pub statement: UpdateStatement,
    pub(crate) what_state: PhantomData<T>,
    pub(crate) data_state: PhantomData<D>,
    pub(crate) cond_state: PhantomData<C>,
}

impl UpdateBuilder<NoWhat, NoData, NoCond> {
    pub fn new() -> Self {
        Self {
            statement: Default::default(),
            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
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
    /// fn main() {
    ///     UpdateBuilder::new().what("test");
    ///
    ///     UpdateBuilder::new().what(Thing::from(("test", "test")));
    /// }
    /// ```
    ///
    /// You can also use the Value type inside surrealdb for more complex requests
    pub fn what(self, what: impl Into<ExtraValue>) -> UpdateBuilder<FilledWhat, NoData, NoCond> {
        let Self { mut statement, .. } = self;

        statement.what = what.into().0;

        UpdateBuilder {
            statement,
            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }
}

impl UpdateBuilder<FilledWhat, NoData, NoCond> {

    /// This function is for `SET` || `UNSET` || `MERGE` and more
    pub fn data(self, data: impl Into<ExtraData>) -> UpdateBuilder<FilledWhat, FilledData, NoCond> {
        let Self { mut statement, .. } = self;

        let data = data.into().0;

        statement.data = Some(data);

        UpdateBuilder {
            statement,
            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// This function is for `SET`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::query::update::UpdateBuilder;
    ///
    /// fn main() {
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test'
    ///
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test', test2 = 'test2'
    ///
    /// }
    pub fn set(self, set: impl Into<SetExpression>) -> UpdateBuilder<FilledWhat, FilledData, NoCond> {
        let Self { mut statement, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        UpdateBuilder {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// This function is for `UNSET`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb_extra::query::update::UpdateBuilder;
    ///
    /// fn main() {
    ///     UpdateBuilder::new().what("test").unset(vec!["test"]);
    ///     // The above builder becomes `UPDATE test UNSET test
    ///
    ///     UpdateBuilder::new().what("test").unset(vec!["test", "test"]);
    ///     // The above builder becomes `UPDATE test UNSET test, test
    ///
    /// }
    pub fn unset(self, set: impl Into<UnsetExpression>) -> UpdateBuilder<FilledWhat, FilledData, NoCond> {
        let Self { mut statement, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        UpdateBuilder {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// This function is for `CONTENT`
    ///
    /// Example:
    /// ```rust
    /// use serde::Serialize;
    /// use surrealdb_extra::query::update::UpdateBuilder;
    ///
    /// #[derive(Serialize)]
    /// pub struct Test {
    ///     test: String,
    ///     magic: bool
    /// }
    ///
    /// fn main() {
    ///     UpdateBuilder::new().what("test").content(Test { test: "test".to_string(), magic: true });
    ///     // The above builder becomes `UPDATE test CONTENT { test: "test", magic: true }
    ///
    /// }
    pub fn content(self, content: impl Serialize + 'static) -> UpdateBuilder<FilledWhat, FilledData, NoCond> {
        let Self { mut statement, .. } = self;

        let val = to_value(content).unwrap_or_default();

        statement.data = Some(Data::ContentExpression(val));

        UpdateBuilder {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }
}

impl UpdateBuilder<FilledWhat, FilledData, NoCond> {
    /// This function is for `WHERE`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::{cond_vec, op};
    /// use surrealdb_extra::query::parsing::cond::Condition;
    /// use surrealdb_extra::query::update::UpdateBuilder;
    ///
    /// fn main() {
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")]).condition("test");
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test`
    ///
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")]).condition(cond_vec![(Operator::Not, "test")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE !test`
    ///
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")]).condition(cond_vec![("test", op!(>), "$test")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test > $test`
    ///
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")]).condition(cond_vec![("test", Operator::Equal, "$test")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test = $test`
    ///
    ///     // For multiple conditions the vector `cond_vec![]` is recommended for usage
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")])
    ///     .condition(cond_vec![("test1", Operator::Equal, "$test1"), Operator::And, ("test2", Operator::Equal, "$test2"), Operator::Or, "test", Operator::Or, (Operator::Not, "test")]);
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test1 = $test1 AND test2 = $test2 OR test OR !test`
    ///
    ///     // Other method of using the multi conditions
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")]).condition(vec![Condition::from("test"), Condition::from(Operator::And), Condition::from(("name", Operator::LessThanOrEqual, "$name"))]);
    ///     // The above builder becomes `UPDATE test SET test = 'test' WHERE test AND name <= $name`
    ///
    ///     // It is also possible to type the condition like normal
    ///     UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")])
    ///     .condition("test1 = $test1 AND test2 = $test2 or test or !test");
    ///      // The above builder becomes `UPDATE test SET test = 'test' WHERE test1 = $test1 AND test2 = $test2 OR test OR !test`
    ///
    /// }
    /// ```
    ///
    /// ## The fastest way to query is to use the string format for conditions at least from benchmarks
    ///
    /// You can also use the Cond/Value type inside surrealdb for more complex requests
    pub fn condition(self, cond: impl Into<ExtraCond>) -> UpdateBuilder<FilledWhat, FilledData, FilledCond> {
        let Self { mut statement, .. } = self;

        let cond = cond.into().0;

        statement.cond = Some(cond);

        UpdateBuilder {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }
}

impl<C> UpdateBuilder<FilledWhat, FilledData, C> {
    pub fn only(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.only = true;

        Self {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// This function is for `RETURN`
    pub fn output(self, output: impl Into<ExtraOutput>) -> Self {
        let Self { mut statement, .. } = self;

        let output = output.into().0;

        statement.output = Some(output);

        Self {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// You can also use the Timeout type inside surrealdb or Duration inside standard for more complex requests
    pub fn timeout(self, timeout: impl Into<ExtraTimeout>) -> Self {
        let Self { mut statement, .. } = self;

        let timeout = timeout.into().0;

        statement.timeout = Some(timeout);

        Self {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    pub fn parallel(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.parallel = true;

        Self {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// Converts the builder to query type
    pub fn to_query(self, db: &Surreal<impl Connection>) -> Query<impl Connection> {
        db.query(self.statement)
    }
}

#[cfg(test)]
mod test {
    use surrealdb::opt::IntoQuery;
    use surrealdb::sql::Operator;
    use serde::Serialize;
    use super::*;

    #[derive(Serialize)]
    struct Test {
        test1: String,
        test2: String
    }

    #[test]
    fn update_builder() {
        let update = UpdateBuilder::new().what("test");

        let query = update.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn update_builder_with_set_and_cond() {
        let update = UpdateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")]).condition("test");

        let query = update.statement.into_query();

        assert!(query.is_ok());
    }

    #[test]
    fn update_builder_with_unset_and_cond() {
        let update = UpdateBuilder::new().what("test").unset(vec!["test", "test"]).condition("test");

        let query = update.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn update_builder_with_content() {
        let test = Test {
            test1: "test1".to_string(),
            test2: "test2".to_string(),
        };

        let update = UpdateBuilder::new().what("test").content(test);

        let query = update.statement.into_query();

        assert!(query.is_ok())
    }
}
