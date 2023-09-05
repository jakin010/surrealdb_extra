use surrealdb::sql::{Idiom};

pub struct ExtraOmit(pub(crate) Idiom);

impl From<Idiom> for ExtraOmit {
    fn from(value: Idiom) -> Self {
        Self(value)
    }
}
