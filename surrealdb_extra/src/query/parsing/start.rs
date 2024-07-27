use surrealdb::sql::{Number, Start, Value};

#[derive(Debug, Clone)]
pub struct ExtraStart(pub Start);

impl From<Start> for ExtraStart {
    fn from(value: Start) -> Self {
        Self(value)
    }
}

impl From<i64> for ExtraStart {
    fn from(value: i64) -> Self {
        let val = Value::Number(Number::Int(value));

        let mut start = Start::default();
        start.0 = val;
        Self(start)
    }
}

impl From<Value> for ExtraStart {
    fn from(value: Value) -> Self {
        let mut start = Start::default();
        start.0 = value;
        
        Self(start)
    }
}
