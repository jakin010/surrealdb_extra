use surrealdb::sql::{Field, Value};
use crate::query::parsing::idiom::ExtraIdiom;
use crate::query::parsing::str_to_value;

#[derive(Debug, Clone)]
pub struct ExtraField(pub Field);

impl From<Field> for ExtraField {
    fn from(value: Field) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraField {
    fn from(value: &str) -> Self {

        let val = str_to_value(value);

        let field = Field::Single {
            expr: val,
            alias: None,
        };

        Self(field)
    }
}

impl From<String> for ExtraField {
    fn from(value: String) -> Self {

        let val = str_to_value(value);

        let field = Field::Single {
            expr: val,
            alias: None,
        };

        Self(field)
    }
}

impl From<Value> for ExtraField {
    fn from(value: Value) -> Self {
        let field = Field::Single {
            expr: value,
            alias: None,
        };

        Self(field)
    }
}

impl From<(&str, &str)> for ExtraField {
    fn from(value: (&str, &str)) -> Self {

        let field = str_to_value(value.0);

        let alias_idiom = ExtraIdiom::from(value.1);

        let field = Field::Single {
            expr: field,
            alias: Some(alias_idiom.0),
        };

        Self(field)
    }
}

impl From<(String, String)> for ExtraField {
    fn from(value: (String, String)) -> Self {

        let field = str_to_value(value.0);

        let alias_idiom = ExtraIdiom::from(value.1);

        let field = Field::Single {
            expr: field,
            alias: Some(alias_idiom.0),
        };

        Self(field)
    }
}

impl From<(&str, String)> for ExtraField {
    fn from(value: (&str, String)) -> Self {

        let field = str_to_value(value.0);

        let alias_idiom = ExtraIdiom::from(value.1);

        let field = Field::Single {
            expr: field,
            alias: Some(alias_idiom.0),
        };

        Self(field)
    }
}

impl From<(String, &str)> for ExtraField {
    fn from(value: (String, &str)) -> Self {

        let field = str_to_value(value.0);

        let alias_idiom = ExtraIdiom::from(value.1);

        let field = Field::Single {
            expr: field,
            alias: Some(alias_idiom.0),
        };

        Self(field)
    }
}

