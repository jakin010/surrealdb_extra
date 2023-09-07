use surrealdb::sql::{Limit, Number, Value};

#[derive(Debug, Clone)]
pub struct ExtraLimit(pub Limit);

impl From<Limit> for ExtraLimit {
    fn from(value: Limit) -> Self {
        Self(value)
    }
}

impl From<i64> for ExtraLimit {
    fn from(value: i64) -> Self {
        let val = Value::Number(Number::Int(value));

        Self(Limit(val))
    }
}

impl From<Value> for ExtraLimit {
    fn from(value: Value) -> Self {
        Self(Limit(value))
    }
}
