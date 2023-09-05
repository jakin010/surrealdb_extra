use surrealdb::sql::{Number, Start, Value};

pub struct ExtraStart(pub Start);

impl From<Start> for ExtraStart {
    fn from(value: Start) -> Self {
        Self(value)
    }
}

impl From<i64> for ExtraStart {
    fn from(value: i64) -> Self {
        let val = Value::Number(Number::Int(value));

        Self(Start(val))
    }
}

impl From<Value> for ExtraStart {
    fn from(value: Value) -> Self {
        Self(Start(value))
    }
}
