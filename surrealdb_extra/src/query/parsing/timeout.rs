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
        let mut timeout = Timeout::default();
        timeout.0 = value.into();
        
        Self(timeout)
    }
}
