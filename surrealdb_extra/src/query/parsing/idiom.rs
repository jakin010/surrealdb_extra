use rayon::prelude::*;
use surrealdb::sql::{Idiom, Part};

pub struct ExtraIdiom(pub(crate) Idiom);

impl From<Idiom> for ExtraIdiom {
    fn from(value: Idiom) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraIdiom {
    fn from(value: &str) -> Self {
        let part: Vec<Part> = value
            .par_split('.')
            .map(|x| Part::from(x))
            .collect();

        let idiom = Idiom(part);

        Self(idiom)
    }
}


impl From<String> for ExtraIdiom {
    fn from(value: String) -> Self {
        let part: Vec<Part> = value
            .par_split('.')
            .map(|x| Part::from(x))
            .collect();

        let idiom = Idiom(part);

        Self(idiom)
    }
}
