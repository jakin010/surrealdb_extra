use surrealdb::sql::{Group, Idiom};
use crate::query::parsing::idiom::ExtraIdiom;

#[derive(Debug, Clone)]
pub struct ExtraGroup(pub Group);

impl From<Group> for ExtraGroup {
    fn from(value: Group) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraGroup {
    fn from(value: &str) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        let mut group = Group::default();
        group.0 = idiom;

        Self(group)
    }
}

impl From<String> for ExtraGroup {
    fn from(value: String) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        let mut group = Group::default();
        group.0 = idiom;

        Self(group)
    }
}

impl From<Idiom> for ExtraGroup {
    fn from(value: Idiom) -> Self {
        let mut group = Group::default();
        group.0 = value;

        Self(group)
    }
}
