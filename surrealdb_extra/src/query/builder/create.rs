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

#[derive(Debug, Clone, Default)]
pub struct CreateBuilder<T, D> {
    pub statement: CreateStatement,
    pub(crate) what_state: PhantomData<T>,
    pub(crate) data_state: PhantomData<D>,
}


impl CreateBuilder<NoWhat, NoData> {
    pub fn new() -> Self {
        Self {
            statement: Default::default(),
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This functions selects from either the table, table:id or more
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::sql::Thing;
    /// use surrealdb_extra::query::create::CreateBuilder;
    ///
    /// CreateBuilder::new().what("test");
    ///
    /// CreateBuilder::new().what(Thing::from(("test", "test")));
    ///
    /// ```
    ///
    /// You can also use the Value type inside surrealdb for more complex requests
    pub fn what(self, what: impl Into<ExtraValue>) -> CreateBuilder<FilledWhat, NoData> {
        let Self { mut statement, .. } = self;

        statement.what = what.into().0;

        CreateBuilder {
            statement,
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }
}

impl CreateBuilder<FilledWhat, NoData> {
    /// This function is for `SET` || `UNSET` || `MERGE` and more
    pub fn data(self, data: impl Into<ExtraData>) -> CreateBuilder<FilledWhat, FilledData> {
        let Self { mut statement, .. } = self;

        let data = data.into().0;

        statement.data = Some(data);

        CreateBuilder {
            statement,
            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `SET`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb::sql::Operator;
    /// use surrealdb_extra::query::create::CreateBuilder;
    ///
    /// CreateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test")]);
    /// // The above builder becomes `CREATE test SET test = 'test'
    ///
    /// CreateBuilder::new().what("test").set(vec![("test", Operator::Equal, "test"), ("test2", Operator::Equal, "test2")]);
    /// // The above builder becomes `CREATE test SET test = 'test', test2 = 'test2'
    ///
    pub fn set(self, set: impl Into<SetExpression>) -> CreateBuilder<FilledWhat, FilledData> {
        let Self { mut statement, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        CreateBuilder {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `UNSET`
    ///
    /// Example:
    /// ```rust
    /// use surrealdb_extra::query::create::CreateBuilder;
    ///
    /// CreateBuilder::new().what("test").unset(vec!["test"]);
    /// // The above builder becomes `CREATE test UNSET test
    ///
    /// CreateBuilder::new().what("test").unset(vec!["test", "test"]);
    /// // The above builder becomes `CREATE test UNSET test, test
    ///
    pub fn unset(self, set: impl Into<UnsetExpression>) -> CreateBuilder<FilledWhat, FilledData> {
        let Self { mut statement, .. } = self;

        let set = set.into().0;

        statement.data = Some(set);

        CreateBuilder {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

    /// This function is for `CONTENT`
    ///
    /// Example:
    /// ```rust
    /// use serde::Serialize;
    /// use surrealdb_extra::query::create::CreateBuilder;
    ///
    /// #[derive(Serialize)]
    /// pub struct Test {
    ///     test: String,
    ///     magic: bool
    /// }
    ///
    /// CreateBuilder::new().what("test").content(Test { test: "test".to_string(), magic: true });
    /// // The above builder becomes `CREATE test CONTENT { test: "test", magic: true }
    ///
    pub fn content(self, content: impl Serialize + 'static) -> CreateBuilder<FilledWhat, FilledData> {
        let Self { mut statement, .. } = self;

        let val = to_value(content).unwrap_or_default();

        statement.data = Some(Data::ContentExpression(val));

        CreateBuilder {
            statement,

            what_state: Default::default(),
            data_state: Default::default(),
        }
    }
}

impl CreateBuilder<FilledWhat, FilledData> {
    pub fn only(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.only = true;

        Self {
            statement,

            what_state: Default::default(),
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

            what_state: Default::default(),
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

            what_state: Default::default(),
            data_state: Default::default(),
        }
    }

    pub fn parallel(self) -> Self {
        let Self { mut statement, .. } = self;

        statement.parallel = true;

        Self {
            statement,

            what_state: Default::default(),
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
    use crate::op;
    use super::*;
    #[derive(Serialize)]
    struct Test {
        test1: String,
        test2: i32
    }

    #[test]
    fn init() {
        let create_builder = CreateBuilder::new().what("test");

        let query = create_builder.statement.into_query();

        assert!(query.is_ok());
    }

    #[test]
    fn with_set() {
        let create_builder = CreateBuilder::new().what("test").set(vec![("test", op!(+=), 4)]);

        let query = create_builder.statement.into_query();

        assert!(query.is_ok());
    }

    #[test]
    fn with_content() {
        let create_builder = CreateBuilder::new().what("test").content(Test {
            test1: "test".to_string(),
            test2: -55
        });

        let query = create_builder.statement.into_query();

        assert!(query.is_ok());
    }
}
