//! # Starting the builder can be done in 2 ways
//!
//! ## Using the `Surrealdb<C>` type
//! ```rust
//! use surrealdb::engine::any::connect;
//! use surrealdb_extra::query::select::SelectBuilder;
//!
//! #[tokio::main]
//! async fn main() {
//!
//! let db = connect("mem://").await.unwrap();
//!     db.use_ns("ns").use_db("db").await.unwrap();
//!
//!     let builder = SelectBuilder::new();
//!
//!     let query = builder.what("test").field("test").to_query(&db);
//! }
//! ```
//!
//! ## Using new function inside the builder and passing a reference of type `Surrealdb<C>`
//! ```rust
//! use surrealdb::engine::any::connect;
//! use surrealdb_extra::query::select::SelectBuilder;
//!
//! #[tokio::main]
//! async fn main() {
//!     let db = connect("mem://").await.unwrap();
//!     db.use_ns("ns").use_db("db").await.unwrap();
//!
//!     let builder = SelectBuilder::new();
//!
//!     let query = builder.what("test").field("test").to_query(&db);
//! }
//! ```
//!
//! # For binding first convert the builder to a `Query<>` type and do binding as usual
//!
//! ## Click on the struct for more info

use std::marker::PhantomData;
use surrealdb::{Connection, Surreal};
use surrealdb::method::Query;
use surrealdb::sql::{Explain, Fetchs, Groups, Idioms, Orders, Splits};
use surrealdb::sql::statements::SelectStatement;
use crate::query::parsing::cond::ExtraCond;
use crate::query::parsing::fetch::ExtraFetch;
use crate::query::parsing::field::ExtraField;
use crate::query::parsing::group::ExtraGroup;
use crate::query::parsing::limit::ExtraLimit;
use crate::query::parsing::omit::ExtraOmit;
use crate::query::parsing::order::ExtraOrder;
use crate::query::parsing::split::ExtraSplit;
use crate::query::parsing::start::ExtraStart;
use crate::query::parsing::timeout::ExtraTimeout;
use crate::query::parsing::version::ExtraVersion;
use crate::query::parsing::what::ExtraValue;
use crate::query::parsing::with::ExtraWith;
use crate::query::states::{FilledCond, FilledFields, FilledWhat, NoCond, NoFields, NoWhat};

#[derive(Debug, Clone)]
pub struct SelectBuilder<W, F, C> {
    pub statement: SelectStatement,
    pub(crate) what_state: PhantomData<W>,
    pub(crate) fields_state: PhantomData<F>,
    pub(crate) cond_state: PhantomData<C>,
}

impl SelectBuilder<NoWhat, NoFields, NoCond> {
    pub fn new() -> Self {
        Self {
            statement: Default::default(),
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// This functions selects from either the table, table:id or more
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::engine::any::connect;
    /// use surrealdb::sql::Thing;
    /// use surrealdb_extra::query::select::SelectBuilder;
    ///
    /// fn main() {
    ///     SelectBuilder::new().what("test").field("test"); // This becomes `SELECT test FROM test`
    ///
    ///     SelectBuilder::new().what(Thing::from(("test", "test"))).field("test"); // This becomes `SELECT test FROM test:test`
    /// }
    /// ```
    ///
    /// You can also use the Value type inside surrealdb for more complex requests
    pub fn what(self, what: impl Into<ExtraValue>) -> SelectBuilder<FilledWhat, NoFields, NoCond> {
        let Self { mut statement, .. } = self;

        statement.what = what.into().0;

        SelectBuilder {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }
}

impl<F, C> SelectBuilder<FilledWhat, F, C> {
    /// This function selects the fields of a table with alias support or more
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::sql::Field;
    /// use surrealdb_extra::query::select::SelectBuilder;
    ///
    /// fn main() {
    ///     SelectBuilder::new().what("test").field(Field::All); // This becomes `SELECT * FROM test`
    ///
    ///     SelectBuilder::new().what("test").field(Field::All).field(("test", "test.test")); // This becomes `SELECT *, test as test.test FROM test`
    ///
    ///     SelectBuilder::new().what("test").field("test"); // This becomes `SELECT test FROM test`
    ///
    ///     SelectBuilder::new().what("test").field("$test"); // This becomes `SELECT $test FROM test`
    ///
    ///     SelectBuilder::new().what("test").field(("test", "test")); // This becomes `SELECT test as test FROM test`
    ///
    ///     SelectBuilder::new().what("test").field(("test.test", "test")); // This becomes `SELECT test.test as test FROM test`
    ///
    ///     SelectBuilder::new().what("test").field(("test", "test.test")); // This becomes `SELECT test as test.test FROM test`
    /// }
    /// ```
    ///
    /// You can also use the Field type inside surrealdb for more complex requests
    pub fn field(self, field: impl Into<ExtraField>) -> SelectBuilder<FilledWhat, FilledFields, C> {
        let Self { mut statement, .. } = self;

        let field = field.into().0;
        statement.expr.0.push(field);

        SelectBuilder {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }
}

impl SelectBuilder<FilledWhat, FilledFields, NoCond> {
    /// This function is for `WHERE`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::{cond_vec, op};
    /// use surrealdb_extra::query::parsing::cond::Condition;
    /// use surrealdb_extra::query::select::SelectBuilder;
    ///
    /// fn main() {
    ///     SelectBuilder::new().what("test").field("test").condition("test");
    ///     // The above builder becomes `SELECT test FROM test WHERE test`
    ///
    ///     SelectBuilder::new().what("test").field("test").condition(cond_vec![(op!(!), "test")]);
    ///     // The above builder becomes `SELECT test FROM test WHERE !test`
    ///
    ///     SelectBuilder::new().what("test").field("test").condition(cond_vec![("test", op!(>), "$test")]);
    ///     // The above builder becomes `SELECT test FROM test WHERE test > $test`
    ///
    ///     SelectBuilder::new().what("test").field("test").condition(cond_vec![("test", Operator::Equal, "$test")]);
    ///     // The above builder becomes `SELECT test FROM test WHERE test = $test`
    ///
    ///     // For multiple conditions the vector `cond_vec![]` is recommended for usage
    ///     SelectBuilder::new().what("test").field("test")
    ///     .condition(cond_vec![("test1", Operator::Equal, "$test1"), Operator::And, ("test2", Operator::Equal, "$test2"), Operator::Or, "test", Operator::Or, (Operator::Not, "test")]);
    ///     // The above builder becomes `SELECT test FROM test WHERE test1 = $test1 AND test2 = $test2 OR test OR !test`
    ///
    ///     // Other method of using the multi conditions
    ///     SelectBuilder::new().what("test").field("test").condition(vec![Condition::from("test"), Condition::from(Operator::And), Condition::from(("name", Operator::LessThanOrEqual, "$name"))]);
    ///     // The above builder becomes `SELECT test FROM test WHERE test AND name <= $name`
    ///
    ///     // For sub queries
    ///     SelectBuilder::new().what("test").field("test")
    ///     .condition(cond_vec![
    ///         ("test1", Operator::Equal, "$test1"), Operator::And, ("test2", Operator::Equal, "$test2"), Operator::Or, "test", Operator::Or, (Operator::Not, "test"), Operator::And,
    ///         cond_vec![("test1", Operator::Equal, "$test1"), Operator::And, ("test2", Operator::Equal, "$test2")]
    ///     ]);
    ///     // The above builder becomes `SELECT test FROM test WHERE test1 = $test1 AND test2 = $test2 OR test OR !test AND (test1 = $test1 AND test2 = $test2)`
    ///
    ///     // It is also possible to type the condition like normal
    ///     SelectBuilder::new().what("test").field("test")
    ///     .condition("test1 = $test1 AND test2 = $test2 or test or !test");
    ///     // The above builder becomes `SELECT test FROM test WHERE test1 = $test1 AND test2 = $test2 OR test OR !test`
    ///
    /// }
    /// ```
    ///
    /// ## The fastest way to query is to use the string format for conditions at least from benchmarks
    ///
    /// You can also use the Cond/Value type inside surrealdb for more complex requests
    pub fn condition(self, cond: impl Into<ExtraCond>) -> SelectBuilder<FilledWhat, FilledFields, FilledCond> {
        let Self { mut statement, .. } = self;

        let cond = cond.into().0;

        statement.cond = Some(cond);

        SelectBuilder {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

}

impl<C> SelectBuilder<FilledWhat, FilledFields, C> {
    /// You can also use the Idiom type inside surrealdb for more complex requests
    pub fn omit(self, omit: impl Into<ExtraOmit>) -> Self {
        let Self { mut statement, .. } = self;

        let mut omits = statement.omit.unwrap_or(
            Idioms::default()
        );

        omits.0.push(omit.into().0);

        statement.omit = Some(omits);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// You can also use the With type inside surrealdb for more complex requests
    pub fn with(self, with: impl Into<ExtraWith>) -> Self {
        let Self { mut statement, .. } = self;

        statement.with = Some(with.into().0);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// You can also use the Split/Idiom type inside surrealdb for more complex requests
    pub fn split(self, split: impl Into<ExtraSplit>) -> Self {
        let Self { mut statement, .. } = self;

        let mut splits = statement.split.unwrap_or(
            Splits::default()
        );

        splits.0.push(split.into().0);

        statement.split = Some(splits);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// You can also use the Group/Idiom type inside surrealdb for more complex requests
    pub fn group(self, group: impl Into<ExtraGroup>) -> Self {
        let Self { mut statement, .. } = self;

        let mut groups = statement.group.unwrap_or(
            Groups::default()
        );

        groups.0.push(group.into().0);

        statement.group = Some(groups);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }


    /// This function orders the rows
    ///
    /// Example:
    /// ```rust
    /// use surrealdb_extra::query::parsing::order::OrderDirection;
    /// use surrealdb_extra::query::select::SelectBuilder;
    ///
    /// fn main() {
    ///     SelectBuilder::new().what("test").field("test").order(("test", OrderDirection::ASC)); // This becomes `SELECT test FROM test ORDER BY test ASC`
    ///
    ///     SelectBuilder::new().what("test").field(("test", "test")).order(("test".to_string(), OrderDirection::DESC)); // This becomes `SELECT test as test FROM test ORDER BY test DESC`
    ///
    ///     SelectBuilder::new().what("test").field(("test.test", "test")).order(("test1".to_string(), OrderDirection::DESC)).order((("test2", OrderDirection::ASC))); // This becomes `SELECT test.test as test FROM test ORDER BY test1 DESC, test2 ASC`
    /// }
    /// ```
    /// You can also use the Order type inside surrealdb for more complex requests
    pub fn order(self, order: impl Into<ExtraOrder>) -> Self {
        let Self { mut statement, .. } = self;

        let mut orders = statement.order.unwrap_or(
            Orders::default()
        );

        orders.0.push(order.into().0);

        statement.order = Some(orders);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// This function limit amount of rows
    ///
    /// Example:
    /// ```rust
    /// use surrealdb_extra::query::select::SelectBuilder;
    ///
    /// fn main() {
    ///     SelectBuilder::new().what("test").field("test").limit(5); // This becomes `SELECT test FROM test LIMIT 5`
    /// }
    /// ```
    /// You can also use the Limit/Value type inside surrealdb for more complex requests
    pub fn limit(self, limit: impl Into<ExtraLimit>) -> Self {
        let Self { mut statement, .. } = self;

        let limit = limit.into().0;

        statement.limit = Some(limit);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// This function starts rows at x
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::engine::any::connect;
    /// use surrealdb_extra::query::select::SelectBuilder;
    ///
    /// fn main() {
    ///     SelectBuilder::new().what("test").field("test").start(5); // This becomes `SELECT test FROM test START 5`
    /// }
    /// ```
    /// You can also use the Start/Value type inside surrealdb for more complex requests
    pub fn start(self, start: impl Into<ExtraStart>) -> Self {
        let Self { mut statement, .. } = self;

        let start = start.into().0;

        statement.start = Some(start);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// You can also use the Fetch/Idiom type inside surrealdb for more complex requests
    pub fn fetch(self, fetch: impl Into<ExtraFetch>) -> Self {
        let Self { mut statement, .. } = self;

        let mut fetches = statement.fetch.unwrap_or(
            Fetchs::default()
        );

        fetches.0.push(fetch.into().0);

        statement.fetch = Some(fetches);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    /// You can also use the Version type inside surrealdb or `DateTime<Utc>` inside chrono for more complex requests
    pub fn version(self, version: impl Into<ExtraVersion>) -> Self {
        let Self { mut statement, .. } = self;

        let version = version.into().0;

        statement.version = Some(version);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
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
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    pub fn only(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.only = true;

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    pub fn parallel(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.parallel = true;

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    pub fn explain(self) -> Self {
        let Self { mut statement, .. } = self;

        let mut explain = Explain::default();
        explain.0 = true;
        statement.explain = Some(explain);

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
            cond_state: Default::default(),
        }
    }

    pub fn tempfile(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.tempfiles = true;

        Self {
            statement,
            what_state: Default::default(),
            fields_state: Default::default(),
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
    use surrealdb::sql::Thing;
    use surrealdb::sql::{Field, Idiom, Value};
    use super::*;

    #[test]
    fn select_table() {
        let select = SelectBuilder::new().what("test");

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn select_thing() {
        let select = SelectBuilder::new().what(Thing::from(("test", "test")));

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn select_all_field() {
        let select = SelectBuilder::new().what(Thing::from(("test", "test"))).field(Field::All);

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn select_str_fields() {
        let select = SelectBuilder::new().what(Thing::from(("test", "test"))).field("test");

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn select_string_fields() {
        let select = SelectBuilder::new().what("test").field("field.test".to_string());

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn select_str_alias_fields() {
        let select = SelectBuilder::new().what("test").field(("field.test", "test"));

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn select_string_alias_fields() {
        let select = SelectBuilder::new().what("test").field(("field.test".to_string(), "test".to_string()));

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[test]
    fn select_field_with_fields_type() {

        let field = Field::Single {
            expr: Value::Idiom(Idiom::from("test".to_string())),
            alias: None,
        };

        let select = SelectBuilder::new().what("test").field(field);

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }
    #[test]
    fn select_field_with_cond() {

        let field = Field::Single {
            expr: Value::Idiom(Idiom::from("test".to_string())),
            alias: None,
        };

        let select = SelectBuilder::new().what("test").field(field).condition("test");

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }
}
