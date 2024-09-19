use surrealdb::{sql::Thing, sql::Value};

#[derive(Debug, Clone)]
pub struct ExtraValue(pub Value);

impl From<Thing> for ExtraValue {
    fn from(value: Thing) -> Self {
        ExtraValue(Value::Thing(value))
    }
}
