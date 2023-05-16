use surrealdb::Error;

#[derive(Debug)]
pub enum QueryError {
    FieldDoesNotExist(String),
    TableDoesNotMatchName,
    DB(Error)
}
