use surrealdb::sql::Data;

#[derive(Debug, Clone)]
pub struct ExtraData(pub Data);

impl From<Data> for ExtraData {
    fn from(value: Data) -> Self {
        Self(value)
    }
}
