use surrealdb::{sql::{Table, Value, Values, Thing as RecordId}};

#[derive(Debug, Clone)]
pub struct ExtraValue(pub Values);

impl From<Values> for ExtraValue {
    fn from(value: Values) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraValue {
    fn from(value: &str) -> Self {
        let mut table = Table::default();
        table.0 = value.to_string();

        let mut values = Values::default();
        values.0 = vec![Value::Table(table)];

        ExtraValue(values)
    }
}

impl From<String> for ExtraValue {
    fn from(value: String) -> Self {
        let mut table = Table::default();
        table.0 = value;

        let mut values = Values::default();
        values.0 = vec![Value::Table(table)];

        ExtraValue(values)
    }
}

impl From<RecordId> for ExtraValue {
    fn from(value: RecordId) -> Self {
        let mut values = Values::default();        

        values.0 = vec![Value::Thing(value)];
        
        ExtraValue(values)
    }
}
