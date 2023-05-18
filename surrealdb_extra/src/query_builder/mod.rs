//! The `Query` struct provides utilities for building and executing database queries.
//!
//! # Examples
//!
//! Creating a `Query` instance:
//!
//! ``` rust
//! use surrealdb_extra::query_builder::Query;
//!
//! let mut query = Query::new();
//!
//! ```
//! Set the table:
//!
//! ``` rust
//!
//! use surrealdb_extra::query_builder::Query;
//! let query = Query::new().from("test", None);
//!
//! ```
//! Set only som fields to get
//!
//! ``` rust
//!
//! use surrealdb_extra::query_builder::Query;
//! let query = Query::new().from("test", None).field("field1").field("field2");
//!
//! ```
//!
//! Set filter
//!
//! ``` rust
//!
//! use surrealdb_extra::query_builder::filter::{LogicalOperator, RelationalOperator};
//! use surrealdb_extra::query_builder::Query;
//! let query = Query::new().from("test", None)
//!     // To start filtering this function needs to be called
//!     .where_filter()
//!     // Filter is made via tuple of 4
//!     // 1. Field name
//!     // 2. `RelationalOperator` enum which represents the different relational operators available for filter conditions, such as `Equal`, `NotEqual`, `LessThan`, etc.
//!     // 3. Value of the field
//!     // 4. `LogicalOperator` enum which represents the logical operators available for combining filter conditions, including `And` and `Or`.
//!     .filter(("field1", RelationalOperator::Equal, "value1", LogicalOperator::And)).unwrap_right() // If you want to add another filer use the `And` or `Or` `LogicalOperator` and unwrap or handle the error of the right
//!     .filter(("field2", RelationalOperator::NotEqual, "value2", LogicalOperator::End)).unwrap_left(); // If you want to stop filtering use the `End` `LogicalOperator` and unwrap or handle the error of the left
//!
//!
//! ```
//!
//! Set limit
//! ``` rust
//!
//! use surrealdb_extra::query_builder::Query;
//! let query = Query::new().from("test", None).limit(10);
//!
//! ```
//!
//! Execute the query
//! ``` rust
//! use surrealdb::kvs::Datastore;
//! use surrealdb::engine::any::connect;
//! use surrealdb_extra::query_builder::Query;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     Datastore::new("memory").await.unwrap();
//!     let db = connect("mem://").await.unwrap();
//!     db.use_ns("ns").use_db("db").await.unwrap();
//!
//!     let query = Query::new().from("test", None).field("test").execute(&db).await.unwrap();
//!
//! }
//!
//! ```
//!
//! Start query from table
#[cfg_attr(feature = "derive", doc = r##"
``` rust
    use surrealdb_extra::table::Table;
    use surrealdb_extra::query_builder::Query;
    use serde::{Serialize, Deserialize};
    use surrealdb::sql::Thing;
    
    #[derive(Table, Serialize, Deserialize)]
    #[table(name = "struct")]
    pub struct Struct {
        id: Option<Thing>
    };

    let query = Struct::select(None).field("test");

```
"##)]

pub mod states;
pub mod filter;
pub mod err;

use std::marker::PhantomData;
use either::{Either, Left, Right};
use surrealdb::{Connection, Response, Surreal};
use crate::query_builder::err::QueryError;
use crate::query_builder::filter::{Filter, LogicalOperator, RelationalOperator};
use crate::query_builder::states::{FieldsQuery, FilterQuery, NoFieldsQuery, NoFilterQuery, NoTableQuery, TableQuery};

#[derive(Debug)]
pub struct Query<T, F, FT> {
    table: T,
    id: Option<String>,
    fields: F,
    limit: Option<u32>,
    filters: Vec<Filter>,
    phantom: PhantomData<FT>
}

impl Query<NoTableQuery, NoFieldsQuery, NoFilterQuery> {
    pub fn new() -> Self  {
        Self {
            table: NoTableQuery,
            id: None,
            fields: NoFieldsQuery,
            limit: None,
            filters: Vec::new(),
            phantom: PhantomData::default()
        }
    }
}

impl Default for Query<NoTableQuery, NoFieldsQuery, NoFilterQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F> Query<NoTableQuery, F, NoFilterQuery> {
    pub fn from(self, table: impl Into<String>, id: Option<String>) -> Query<TableQuery, F, NoFilterQuery> {
        let Self { fields, limit, filters, .. } = self;

        Query {
            table: TableQuery(table.into()),
            id,
            fields,
            limit,
            filters,
            phantom: PhantomData::default()
        }
    }
}

impl<T, F> Query<T, F, NoFilterQuery> {
    pub fn limit(self, limit: u32) -> Self {
        let Self { table, id, fields, filters, .. } = self;

        Query {
            table,
            id,
            fields,
            limit: Some(limit),
            filters,
            phantom: PhantomData::default()
        }
    }

    pub fn where_filter(self) -> Query<T, F, FilterQuery> {
        let Self { table, id, fields, limit, filters, .. } = self;

        Query {
            table,
            id,
            fields,
            limit,
            filters,
            phantom: PhantomData::default()
        }
    }
}

impl<T> Query<T, NoFieldsQuery, NoFilterQuery> {
    pub fn field(self, field: impl Into<String>) -> Query<T, FieldsQuery, NoFilterQuery> {
        let Self { table, id, limit, filters, .. } = self;

        Query {
            table,
            id,
            fields: FieldsQuery(vec![field.into()]),
            limit,
            filters,
            phantom: PhantomData::default()
        }
    }
}

impl<T> Query<T, FieldsQuery, NoFilterQuery> {
    pub fn field(self, field: impl Into<String>) -> Self {
        let Self { table, id, mut fields, limit, filters, .. } = self;

        fields.0.push(field.into());

        Query {
            table,
            id,
            fields,
            limit,
            filters,
            phantom: PhantomData::default()
        }
    }
}

impl<T, F> Query<T, F, FilterQuery> {
    pub fn filter(self, filter: (impl Into<String>, RelationalOperator, impl Into<String>, LogicalOperator)) -> Either<Query<T, F, NoFilterQuery>, Query<T, F, FilterQuery>> {
        let Self { table, id, fields, limit, mut filters, .. } = self;

        let f = Filter {
            key: filter.0.into(),
            relational_operator: filter.1,
            value: filter.2.into(),
            logical_operator: filter.3.clone(),
        };

        filters.push(f);

        if filter.3 == LogicalOperator::End {
            Left(
                Query {
                    table,
                    id,
                    fields,
                    limit,
                    filters,
                    phantom: PhantomData::default()
                }
            )
        } else {
            Right(
                Query {
                    table,
                    id,
                    fields,
                    limit,
                    filters,
                    phantom: PhantomData::default()
                }
            )
        }
    }
}

impl Query<TableQuery, FieldsQuery, NoFilterQuery> {
    pub async fn execute(self, db: &Surreal<impl Connection>) -> Result<Response, QueryError> {

        let mut query = String::from("SELECT");

        if self.fields.0.contains(&"*".to_string()) {
            query.push_str(" *")
        } else {
            let fields_str = self.fields.0.join(",");

            query.push_str(&format!(" {}", fields_str))
        }

        if self.id.is_some() {
            query.push_str(" FROM type::thing($tb, $id)")
        } else {
            query.push_str(" FROM type::table($tb)");
        }

        if !self.filters.is_empty() {
            query.push_str(" WHERE");

            for filter in &self.filters {
                query.push_str(&format!(" {}", filter.to_string()))
            }
        }

        if self.limit.is_some() {
            query.push_str(&format!(" LIMIT {}", self.limit.unwrap()))
        }

        let mut query = db.query(query);

        if let Some(id) = self.id {
            query = query.bind(("id", id));
        }

        query = query.bind(("tb", self.table.0));

        for filter in self.filters {
            query = query.bind((filter.key, filter.value));
        }

        let res = query.await.map_err(QueryError::DB)?;

        Ok(res)
    }
}
