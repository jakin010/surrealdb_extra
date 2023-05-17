use surrealdb::Error;

#[derive(Debug)]
pub enum QueryError {
    TableDoesNotMatchName,
    DB(Error)
}
