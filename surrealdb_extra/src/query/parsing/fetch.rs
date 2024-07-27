use surrealdb::sql::{Fetch, Idiom, Value};
use crate::query::parsing::idiom::ExtraIdiom;

#[derive(Debug, Clone)]
pub struct ExtraFetch(pub Fetch);

impl From<Fetch> for ExtraFetch {
    fn from(value: Fetch) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraFetch {
    fn from(value: &str) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        let mut fetch = Fetch::default();
        fetch.1 = Value::Idiom(idiom);

        Self(fetch)
    }
}

impl From<String> for ExtraFetch {
    fn from(value: String) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        let mut fetch = Fetch::default();
        fetch.1 = Value::Idiom(idiom);

        Self(fetch)
    }
}

impl From<Idiom> for ExtraFetch {
    fn from(value: Idiom) -> Self {
        let mut fetch = Fetch::default();
        fetch.1 = Value::Idiom(value);

        Self(fetch)
    }
}

impl From<Value> for ExtraFetch {
    fn from(value: Value) -> Self {
        let mut fetch = Fetch::default();
        fetch.1 = value;

        Self(fetch)
    }
}
