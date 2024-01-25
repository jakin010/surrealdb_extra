use surrealdb::sql::{Data, Idiom, Operator, Value};
use crate::query::parsing::idiom::ExtraIdiom;

#[derive(Debug, Clone)]
pub struct SetExpression(pub Data);

impl<I: Into<ExtraIdiom>, V: Into<Value>> From<Vec<(I, Operator, V)>> for SetExpression {
    fn from(value: Vec<(I, Operator, V)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e| {
                (e.0.into().0, e.1, e.2.into())
            }).collect();

        Self(Data::SetExpression(value))
    }
}
