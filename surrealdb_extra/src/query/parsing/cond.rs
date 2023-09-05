use surrealdb::sql::{Cond, Value};

pub struct ExtraCond(pub(crate) Cond);

impl From<Cond> for ExtraCond {
    fn from(value: Cond) -> Self {
        Self(value)
    }
}

impl From<Value> for ExtraCond {
    fn from(value: Value) -> Self {
        Self(Cond(value))
    }
}
