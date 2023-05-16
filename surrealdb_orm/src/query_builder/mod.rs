#[derive(Debug)]
pub struct NoTableQuery;
#[derive(Debug)]
pub struct TableQuery(String);

#[derive(Debug)]
pub struct NoFieldsQuery;
#[derive(Debug)]
pub struct FieldsQuery(Vec<String>);

#[derive(Debug)]
pub struct Query<T, F> {
    table: T,
    fields: F
}

impl Query<NoTableQuery, NoFieldsQuery> {
    pub fn new() -> Self  {
        Self { 
            table: NoTableQuery, 
            fields: NoFieldsQuery 
        }
    }
}

impl<F> Query<NoTableQuery, F> {
    pub fn from(self, table: impl Into<String>) -> Query<TableQuery, F> {
        let Self { fields, .. } = self;

        Query { 
            table: TableQuery(table.into()), 
            fields 
        }
    }
}

impl <T> Query<T, NoFieldsQuery> {
    pub fn field(self, field: impl Into<String>) -> Query<T, FieldsQuery> {
        let Self { table, .. } = self;

        Query { 
            table, 
            fields: FieldsQuery(vec![field.into()])
        }
    }
}

impl <T> Query<T, FieldsQuery> {
    pub fn field(self, field: impl Into<String>) -> Self {
        let Self { table, mut fields } = self;

        let _ = fields.0.push(field.into());

        Query { 
            table, 
            fields
        }
    }
}