use surrealdb::sql::{Idiom};

pub struct ExtraOmit(pub Idiom);

impl From<Idiom> for ExtraOmit {
    fn from(value: Idiom) -> Self {
        Self(value)
    }
}
