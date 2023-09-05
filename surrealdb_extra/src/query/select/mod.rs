pub(crate) mod states;

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
use crate::query::select::states::{FilledFields, FilledWhat, NoFields, NoWhat};

#[derive(Debug, Clone)]
pub struct SelectBuilder<'r, T, F, C>
    where C: Connection
{
    pub(crate) statement: SelectStatement,
    pub(crate) db: &'r Surreal<C>,
    pub(crate) what_state: PhantomData<T>,
    pub(crate) fields_state: PhantomData<F>,
}

impl<'r, C> SelectBuilder<'r, NoWhat, NoFields, C>
    where C: Connection
{
    pub fn new(db: &'r Surreal<C>) -> Self {
        Self {
            statement: Default::default(),
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn what(self, what: impl Into<ExtraValue>) -> SelectBuilder<'r, FilledWhat, NoFields, C> {
        let Self { mut statement, db, .. } = self;

        statement.what = what.into().0;

        SelectBuilder {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }
}

impl<'r, F, C> SelectBuilder<'r, FilledWhat, F, C>
    where C: Connection
{
    pub fn field(self, field: impl Into<ExtraField>) -> SelectBuilder<'r, FilledWhat, FilledFields, C> {
        let Self { mut statement, db, .. } = self;

        let field = field.into().0;
        statement.expr.0.push(field);

        SelectBuilder {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }
}

impl<'r, C> SelectBuilder<'r, FilledWhat, FilledFields, C>
    where C: Connection
{
    pub fn omit(self, omit: impl Into<ExtraOmit>) -> Self {
        let Self { mut statement, db, .. } = self;

        let mut omits = statement.omit.unwrap_or(
            Idioms::default()
        );

        omits.0.push(omit.into().0);

        statement.omit = Some(omits);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn condition(self, cond: impl Into<ExtraCond>) -> Self {
        let Self { mut statement, db, .. } = self;

        let cond = cond.into().0;

        statement.cond = Some(cond);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn split(self, split: impl Into<ExtraSplit>) -> Self {
        let Self { mut statement, db, .. } = self;

        let mut splits = statement.split.unwrap_or(
            Splits::default()
        );

        splits.0.push(split.into().0);

        statement.split = Some(splits);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn group(self, group: impl Into<ExtraGroup>) -> Self {
        let Self { mut statement, db, .. } = self;

        let mut groups = statement.group.unwrap_or(
            Groups::default()
        );

        groups.0.push(group.into().0);

        statement.group = Some(groups);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn order(self, order: impl Into<ExtraOrder>) -> Self {
        let Self { mut statement, db, .. } = self;

        let mut orders = statement.order.unwrap_or(
            Orders::default()
        );

        orders.0.push(order.into().0);

        statement.order = Some(orders);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn limit(self, limit: impl Into<ExtraLimit>) -> Self {
        let Self { mut statement, db, .. } = self;

        let limit = limit.into().0;

        statement.limit = Some(limit);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn start(self, start: impl Into<ExtraStart>) -> Self {
        let Self { mut statement, db, .. } = self;

        let start = start.into().0;

        statement.start = Some(start);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn fetch(self, fetch: impl Into<ExtraFetch>) -> Self {
        let Self { mut statement, db, .. } = self;

        let mut fetches = statement.fetch.unwrap_or(
            Fetchs::default()
        );

        fetches.0.push(fetch.into().0);

        statement.fetch = Some(fetches);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn version(self, version: impl Into<ExtraVersion>) -> Self {
        let Self { mut statement, db, .. } = self;

        let version = version.into().0;

        statement.version = Some(version);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn timeout(self, timeout: impl Into<ExtraTimeout>) -> Self {
        let Self { mut statement, db, .. } = self;

        let timeout = timeout.into().0;

        statement.timeout = Some(timeout);

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn parallel(self) -> Self {
        let Self { mut statement, db, .. } = self;

        statement.parallel = true;

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
        }
    }

    pub fn explain(self) -> Self {
        let Self { mut statement, db, .. } = self;

        statement.explain = Some(Explain(true));

        Self {
            statement,
            db,
            what_state: Default::default(),
            fields_state: Default::default(),
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
    use surrealdb::sql::{Field, Idiom, Thing, Value};
    use super::*;

    async fn db() -> Surreal<Any> {
        let db = connect("mem://").await.unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        db
    }

    #[tokio::test]
    async fn select_table() {
        let db = db().await;

        let select = SelectBuilder::new(&db).what("test");

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn select_thing() {
        let db = db().await;

        let select = SelectBuilder::new(&db).what(Thing::from(("test", "test")));

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn select_all_field() {
        let db = db().await;

        let select = SelectBuilder::new(&db).what(Thing::from(("test", "test"))).field(Field::All);

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn select_str_fields() {
        let db = db().await;

        let select = SelectBuilder::new(&db).what(Thing::from(("test", "test"))).field("test");

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn select_string_fields() {
        let db = db().await;

        let select = SelectBuilder::new(&db).what("test").field("field.test".to_string());

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn select_str_alias_fields() {
        let db = db().await;

        let select = SelectBuilder::new(&db).what("test").field(("field.test", "test"));

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn select_string_alias_fields() {
        let db = db().await;

        let select = SelectBuilder::new(&db).what("test").field(("field.test".to_string(), "test".to_string()));

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }

    #[tokio::test]
    async fn select_field_with_fields_type() {

        let field = Field::Single {
            expr: Value::Idiom(Idiom::from("test".to_string())),
            alias: None,
        };

        let db = db().await;

        let select = SelectBuilder::new(&db).what("test").field(field);

        let query = select.statement.into_query();

        assert!(query.is_ok())
    }
}
