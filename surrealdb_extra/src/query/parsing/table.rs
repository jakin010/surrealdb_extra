use surrealdb::sql::{Table, Value};

#[derive(Debug, Clone)]
pub struct ExtraTable(pub Value);

impl From<&str> for ExtraTable {
    fn from(value: &str) -> Self {
        let mut table = Table::default();
        table.0 = value.to_string();

        ExtraTable(Value::Table(table))
    }
}

impl From<String> for ExtraTable {
    fn from(value: String) -> Self {
        let mut table = Table::default();
        table.0 = value;

        ExtraTable(Value::Table(table))
    }
}
