use surrealdb::{opt::RecordId, sql::{Table, Value, Values}};

#[derive(Debug, Clone)]
pub struct ExtraValue(pub Values);

impl From<Values> for ExtraValue {
    fn from(value: Values) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraValue {
    fn from(value: &str) -> Self {
        let values = Values(
            vec![
                Value::Table(Table(value.to_string()))
            ]
        );

        ExtraValue(values)
    }
}

impl From<String> for ExtraValue {
    fn from(value: String) -> Self {
        let values = Values(
            vec![
                Value::Table(Table(value))
            ]
        );

        ExtraValue(values)
    }
}

impl From<RecordId> for ExtraValue {
    fn from(value: RecordId) -> Self {
        let values = Values(
            vec![
                Value::Thing(value)
            ]
        );

        ExtraValue(values)
    }
}
