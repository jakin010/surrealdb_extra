use surrealdb::{opt::RecordId, sql::Value};

#[derive(Debug, Clone)]
pub struct ExtraValue(pub Value);

impl From<RecordId> for ExtraValue {
    fn from(value: RecordId) -> Self {
        ExtraValue(Value::Thing(value))
    }
}
