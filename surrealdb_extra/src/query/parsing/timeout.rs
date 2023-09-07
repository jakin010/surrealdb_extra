use std::time;
use surrealdb::sql::Timeout;

#[derive(Debug, Clone)]
pub struct ExtraTimeout(pub Timeout);

impl From<Timeout> for ExtraTimeout {
    fn from(value: Timeout) -> Self {
        Self(value)
    }
}

impl From<time::Duration> for ExtraTimeout {
    fn from(value: time::Duration) -> Self {
        Self(Timeout(value.into()))
    }
}
