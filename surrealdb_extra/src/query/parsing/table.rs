use surrealdb::sql::{Table, Value};

#[derive(Debug, Clone)]
pub struct ExtraTable(pub Value);

impl From<&str> for ExtraTable {
    fn from(value: &str) -> Self {
        ExtraTable(Value::Table(Table(value.to_string())))
    }
}

impl From<String> for ExtraTable {
    fn from(value: String) -> Self {
        ExtraTable(Value::Table(Table(value)))
    }
}
