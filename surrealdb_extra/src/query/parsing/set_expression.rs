use surrealdb::sql::{Data, Idiom, Operator, Value};
use crate::query::parsing::idiom::ExtraIdiom;
use crate::query::parsing::str_to_value;

#[derive(Debug, Clone)]
pub struct SetExpression(pub Data);

impl From<Vec<(&str, Operator, Value)>> for SetExpression {
    fn from(value: Vec<(&str, Operator, Value)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (ExtraIdiom::from(e.0).0, e.1, e.2)
            }).collect();

        Self(Data::SetExpression(value))
    }
}

impl From<Vec<(&str, Operator, &str)>> for SetExpression {
    fn from(value: Vec<(&str, Operator, &str)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (ExtraIdiom::from(e.0).0, e.1, str_to_value(e.2))
            }).collect();

        Self(Data::SetExpression(value))
    }
}

impl From<Vec<(&str, Operator, String)>> for SetExpression {
    fn from(value: Vec<(&str, Operator, String)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (ExtraIdiom::from(e.0).0, e.1, str_to_value(e.2))
            }).collect();

        Self(Data::SetExpression(value))
    }
}

impl From<Vec<(String, Operator, &str)>> for SetExpression {
    fn from(value: Vec<(String, Operator, &str)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (ExtraIdiom::from(e.0).0, e.1, str_to_value(e.2))
            }).collect();

        Self(Data::SetExpression(value))
    }
}

impl From<Vec<(String, Operator, Value)>> for SetExpression {
    fn from(value: Vec<(String, Operator, Value)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (ExtraIdiom::from(e.0).0, e.1, e.2)
            }).collect();

        Self(Data::SetExpression(value))
    }
}

impl From<Vec<(String, Operator, String)>> for SetExpression {
    fn from(value: Vec<(String, Operator, String)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (ExtraIdiom::from(e.0).0, e.1, str_to_value(e.2))
            }).collect();

        Self(Data::SetExpression(value))
    }
}

impl From<Vec<(Idiom, Operator, String)>> for SetExpression {
    fn from(value: Vec<(Idiom, Operator, String)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (e.0, e.1, str_to_value(e.2))
            }).collect();

        Self(Data::SetExpression(value))
    }
}

impl From<Vec<(Idiom, Operator, &str)>> for SetExpression {
    fn from(value: Vec<(Idiom, Operator, &str)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (e.0, e.1, str_to_value(e.2))
            }).collect();

        Self(Data::SetExpression(value))
    }
}

impl From<Vec<(Idiom, Operator, Value)>> for SetExpression {
    fn from(value: Vec<(Idiom, Operator, Value)>) -> Self {
        let value: Vec<(Idiom, Operator, Value)> = value.into_iter()
            .map(|e|  {
                (e.0, e.1, e.2)
            }).collect();

        Self(Data::SetExpression(value))
    }
}
