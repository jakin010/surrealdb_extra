use rayon::prelude::*;
use surrealdb::sql::{Field, Fields, Output, Value};
use crate::query::parsing::field::ExtraField;

#[derive(Debug, Clone)]
pub struct ExtraOutput(pub Output);

impl From<Output> for ExtraOutput {
    fn from(value: Output) -> Self {
        Self(value)
    }
}

impl From<Fields> for ExtraOutput {
    fn from(value: Fields) -> Self {
        let value = Output::Fields(value);

        Self(value)
    }
}

impl From<Vec<Field>> for ExtraOutput {
    fn from(value: Vec<Field>) -> Self {
        let value = Output::Fields(Fields(value, false));

        Self(value)
    }
}

impl From<Field> for ExtraOutput {
    fn from(value: Field) -> Self {
        let fields = Fields(vec![value], false);

        Self(Output::Fields(fields))
    }
}

impl From<&str> for ExtraOutput {
    fn from(value: &str) -> Self {
        let field = ExtraField::from(value).0;

        let fields = Fields(vec![field], false);

        Self(Output::Fields(fields))
    }
}

impl From<String> for ExtraOutput {
    fn from(value: String) -> Self {
        let field = ExtraField::from(value).0;

        let fields = Fields(vec![field], false);

        Self(Output::Fields(fields))
    }
}

impl From<Value> for ExtraOutput {
    fn from(value: Value) -> Self {
        let field = ExtraField::from(value).0;

        let fields = Fields(vec![field], false);

        Self(Output::Fields(fields))
    }
}

impl From<(&str, &str)> for ExtraOutput {
    fn from(value: (&str, &str)) -> Self {
        let field = ExtraField::from(value).0;

        let fields = Fields(vec![field], false);

        Self(Output::Fields(fields))
    }
}

impl From<(String, String)> for ExtraOutput {
    fn from(value: (String, String)) -> Self {
        let field = ExtraField::from(value).0;

        let fields = Fields(vec![field], false);

        Self(Output::Fields(fields))
    }
}

impl From<(&str, String)> for ExtraOutput {
    fn from(value: (&str, String)) -> Self {
        let field = ExtraField::from(value).0;

        let fields = Fields(vec![field], false);

        Self(Output::Fields(fields))
    }
}

impl From<(String, &str)> for ExtraOutput {
    fn from(value: (String, &str)) -> Self {
        let field = ExtraField::from(value).0;

        let fields = Fields(vec![field], false);

        Self(Output::Fields(fields))
    }
}
