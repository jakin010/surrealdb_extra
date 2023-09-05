use surrealdb::sql::{Idiom, Split};
use crate::query::parsing::idiom::ExtraIdiom;

pub struct ExtraSplit(pub Split);

impl From<Split> for ExtraSplit {
    fn from(value: Split) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraSplit {
    fn from(value: &str) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        let split = Split(idiom);

        Self(split)
    }
}

impl From<String> for ExtraSplit {
    fn from(value: String) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        let split = Split(idiom);

        Self(split)
    }
}

impl From<Idiom> for ExtraSplit {
    fn from(value: Idiom) -> Self {
        let split = Split(value);

        Self(split)
    }
}
