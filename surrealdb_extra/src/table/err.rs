use thiserror::Error;

#[derive(Debug, Error)]
pub enum TableError {
    #[error("Id of table is empty")]
    IdEmpty,
    #[error("{0}")]
    Db(#[from] surrealdb::Error),
    #[error("Empty table")]
    EmptyTable
}
