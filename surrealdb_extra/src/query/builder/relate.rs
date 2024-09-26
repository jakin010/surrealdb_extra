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

#[derive(Debug, Clone, Default)]
pub struct RelateBuilder<T, D> {
    pub statement: RelateStatement,
    pub(crate) relate_state: PhantomData<T>,
    pub(crate) data_state: PhantomData<D>,
}

impl RelateBuilder<NoRelation, NoData> {
    pub fn new() -> Self {
        Self {
            statement: Default::default(),
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `RELATE`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::query::relate::RelateBuilder;
    /// use surrealdb::sql::Thing;
    ///
    ///  RelateBuilder::new().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2")));
    ///  // The above builder becomes `RELATE test:test->test->test2:test2
    ///
    pub fn relation(self, from: impl Into<ExtraValue>, kind: impl Into<ExtraTable>, with: impl Into<ExtraValue>) -> RelateBuilder<FilledRelation, NoData> {
        let Self { mut statement, .. } = self;

        statement.from = from.into().0;
        statement.kind = kind.into().0;
        statement.with = with.into().0;

        RelateBuilder {
            statement,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }
}

impl RelateBuilder<FilledRelation, NoData> {
    /// This function is for `SET`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::query::relate::RelateBuilder;
    /// use surrealdb::sql::Thing;
    ///
    /// RelateBuilder::new().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).set(vec![("test", Operator::Equal, "test")]);
    /// // The above builder becomes `RELATE test:test->test->test2:test2 SET test = 'test'
    ///
    /// RelateBuilder::new().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);
    /// // The above builder becomes `RELATE test:test->test->test2:test2 SET test = 'test', test2 = 'test2'
    ///
    pub fn set(self, set: impl Into<SetExpression>) -> RelateBuilder<FilledRelation, FilledData> {
        let Self { mut statement, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        RelateBuilder {
            statement,
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
    /// use surrealdb_extra::query::relate::RelateBuilder;
    /// use surrealdb::sql::Thing;
    ///
    /// #[derive(Serialize)]
    /// pub struct Test {
    ///     test: String,
    ///     magic: bool
    /// }
    ///
    /// RelateBuilder::new().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).content(Test { test: "test".to_string(), magic: true });
    /// // The above builder becomes `RELATE test:test->test->test2:test2 CONTENT { test: "test", magic: true }
    ///
    pub fn content(self, content: impl Serialize + 'static) -> RelateBuilder<FilledRelation, FilledData> {
        let Self { mut statement, .. } = self;

        let val = to_value(content).unwrap_or_default();

        statement.data = Some(Data::ContentExpression(val));

        RelateBuilder {
            statement,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }
}

impl<D> RelateBuilder<FilledRelation, D> {
    pub fn only(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.only = true;

        Self {
            statement,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `RETURN`
    pub fn output(self, output: impl Into<ExtraOutput>) -> Self {
        let Self { mut statement, .. } = self;

        let output = output.into().0;

        statement.output = Some(output);

        Self {
            statement,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// You can also use the Timeout type inside surrealdb or Duration inside standard for more complex requests
    pub fn timeout(self, timeout: impl Into<ExtraTimeout>) -> Self {
        let Self { mut statement, .. } = self;

        let timeout = timeout.into().0;

        statement.timeout = Some(timeout);

        Self {
            statement,
            relate_state: Default::default(),
            data_state: Default::default(),
        }
    }

    pub fn parallel(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.parallel = true;

        Self {
            statement,
            relate_state: Default::default(),
            data_state: Default::default(),
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
    use surrealdb::sql::Operator;

    use super::*;

    #[test]
    fn relate_table() {
        let relate = RelateBuilder::new().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2")));

        let query = relate.statement.into_query();

        assert!(query.is_ok());
    }

    #[test]
    fn relate_table_data() {
        let relate = RelateBuilder::new().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);

        let query = relate.statement.into_query();

        assert!(query.is_ok());
    }

    #[test]
    fn relate_table_db() {
        let relate = RelateBuilder::new().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2")));

        let query = relate.statement.into_query();

        assert!(query.is_ok());
    }

    #[test]
    fn relate_table_data_db() {
        let relate = RelateBuilder::new().relation(Thing::from(("test", "test")), "test", Thing::from(("test2", "test2"))).set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);

        let query = relate.statement.into_query();

        assert!(query.is_ok());
    }
}
