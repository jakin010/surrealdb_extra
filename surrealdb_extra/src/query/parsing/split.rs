use surrealdb::sql::{Idiom, Split};
use crate::query::parsing::idiom::ExtraIdiom;

#[derive(Debug, Clone)]
pub struct ExtraSplit(pub Split);

impl From<Split> for ExtraSplit {
    fn from(value: Split) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraSplit {
    fn from(value: &str) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        let mut split = Split::default();
        split.0 = idiom;

        Self(split)
    }
}

impl From<String> for ExtraSplit {
    fn from(value: String) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        let mut split = Split::default();
        split.0 = idiom;

        Self(split)
    }
}

impl From<Idiom> for ExtraSplit {
    fn from(value: Idiom) -> Self {
        let mut split = Split::default();
        split.0 = value;

        Self(split)
    }
}
