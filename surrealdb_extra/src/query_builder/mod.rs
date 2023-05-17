pub mod states;
pub mod filter;
pub mod err;

use std::marker::PhantomData;
use either::{Either, Left, Right};
use surrealdb::{Connection, Surreal};
use crate::query_builder::err::QueryError;
use crate::query_builder::filter::{Filter, LogicalOperator, RelationalOperator};
use crate::query_builder::states::{FieldsQuery, FilterQuery, NoFieldsQuery, NoFilterQuery, NoTableQuery, TableQuery};
use crate::Table;

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
    pub async fn execute<T: Table>(self, db: &Surreal<impl Connection>) -> Result<Vec<T>, QueryError> {

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

        let mut res = query.await.map_err(QueryError::DB)?;

        let table_vec: Vec<T> = res.take(0).map_err(QueryError::DB)?;

        Ok(table_vec)
    }
}

