use surrealdb::sql::{Field, Value};
use crate::query::parsing::idiom::ExtraIdiom;

pub struct ExtraField(pub(crate) Field);

impl From<Field> for ExtraField {
    fn from(value: Field) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraField {
    fn from(value: &str) -> Self {

        let idiom = ExtraIdiom::from(value);

        let field = Field::Single {
            expr: Value::Idiom(idiom.0),
            alias: None,
        };

        Self(field)
    }
}

impl From<String> for ExtraField {
    fn from(value: String) -> Self {

        let idiom = ExtraIdiom::from(value);

        let field = Field::Single {
            expr: Value::Idiom(idiom.0),
            alias: None,
        };

        Self(field)
    }
}

impl From<(&str, &str)> for ExtraField {
    fn from(value: (&str, &str)) -> Self {

        let field_idiom = ExtraIdiom::from(value.0);
        let alias_idiom = ExtraIdiom::from(value.1);

        let field = Field::Single {
            expr: Value::Idiom(field_idiom.0),
            alias: Some(alias_idiom.0),
        };

        Self(field)
    }
}

impl From<(String, String)> for ExtraField {
    fn from(value: (String, String)) -> Self {

        let field_idiom = ExtraIdiom::from(value.0);
        let alias_idiom = ExtraIdiom::from(value.1);

        let field = Field::Single {
            expr: Value::Idiom(field_idiom.0),
            alias: Some(alias_idiom.0),
        };

        Self(field)
    }
}

impl From<(&str, String)> for ExtraField {
    fn from(value: (&str, String)) -> Self {

        let field_idiom = ExtraIdiom::from(value.0);
        let alias_idiom = ExtraIdiom::from(value.1);

        let field = Field::Single {
            expr: Value::Idiom(field_idiom.0),
            alias: Some(alias_idiom.0),
        };

        Self(field)
    }
}

impl From<(String, &str)> for ExtraField {
    fn from(value: (String, &str)) -> Self {

        let field_idiom = ExtraIdiom::from(value.0);
        let alias_idiom = ExtraIdiom::from(value.1);

        let field = Field::Single {
            expr: Value::Idiom(field_idiom.0),
            alias: Some(alias_idiom.0),
        };

        Self(field)
    }
}

