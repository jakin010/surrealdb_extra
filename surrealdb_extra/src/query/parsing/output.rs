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
        let mut fields = Fields::default();
        fields.0 = value;
        fields.1 = false;

        let value = Output::Fields(fields);

        Self(value)
    }
}

impl From<Field> for ExtraOutput {
    fn from(value: Field) -> Self {
        let mut fields = Fields::default();
        fields.0 = vec![value];
        fields.1 = false;

        Self(Output::Fields(fields))
    }
}

macro_rules! to_output_from_extra_field {
    ($x:ty) => {
        impl From<Vec<$x>> for ExtraOutput {
            fn from(value: Vec<$x>) -> Self {
                let vec_fields: Vec<Field> = value.into_iter().map(|x|  ExtraField::from(x).0).collect();

                let mut fields = Fields::default();
                fields.0 = vec_fields;
                fields.1 = false;

                Self(Output::Fields(fields))
            }
        }
        impl From<$x> for ExtraOutput {
            fn from(value: $x) -> Self {
                let field = ExtraField::from(value).0;


                let mut fields = Fields::default();
                fields.0 = vec![field];
                fields.1 = false;

                Self(Output::Fields(fields))
            }
        }
    };
}

to_output_from_extra_field!(&str);
to_output_from_extra_field!(String);
to_output_from_extra_field!(Value);
to_output_from_extra_field!((&str, &str));
to_output_from_extra_field!((String, String));
to_output_from_extra_field!((String, &str));
to_output_from_extra_field!((&str, String));
