use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("{0}")]
    DB(::surrealdb::Error)
}
