use surrealdb::sql::{Idiom};

#[derive(Debug, Clone)]
pub struct ExtraOmit(pub Idiom);

impl From<Idiom> for ExtraOmit {
    fn from(value: Idiom) -> Self {
        Self(value)
    }
}
