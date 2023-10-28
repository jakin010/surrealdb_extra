use rayon::prelude::*;
use surrealdb::sql::{Data, Idiom};
use crate::query::parsing::idiom::ExtraIdiom;

#[derive(Debug, Clone)]
pub struct UnsetExpression(pub Data);

impl From<Vec<&str>> for UnsetExpression {
    fn from(value: Vec<&str>) -> Self {
        let value: Vec<Idiom> = value.into_par_iter()
            .map(|e|  {
                ExtraIdiom::from(e).0
            }).collect();

        Self(Data::UnsetExpression(value))
    }
}

impl From<Vec<String>> for UnsetExpression {
    fn from(value: Vec<String>) -> Self {
        let value: Vec<Idiom> = value.into_par_iter()
            .map(|e|  {
                ExtraIdiom::from(e).0
            }).collect();

        Self(Data::UnsetExpression(value))
    }
}

impl From<Vec<Idiom>> for UnsetExpression {
    fn from(value: Vec<Idiom>) -> Self {
        Self(Data::UnsetExpression(value))
    }
}
