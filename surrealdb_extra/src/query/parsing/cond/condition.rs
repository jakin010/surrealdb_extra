use surrealdb::sql::{Expression, Operator, Value};
use crate::query::parsing::str_to_value;

#[derive(Debug, Clone, Default)]
pub enum Condition {
    #[default]
    Null,

    Value(Value),
    ValOpVal(Value, Operator, Value),
    OperatorValue(Operator, Value),
    Operator(Operator),
}

impl Condition {
    pub fn to_value(self) -> Value {
        match self {
            Condition::Value(k) => { k }
            Condition::ValOpVal(l, o, r) => {
                Value::Expression(
                    Expression::Binary { l, o, r }.into()
                )
            }
            Condition::Operator(_) => { Value::Null }
            Condition::OperatorValue(o, v) => {
                Value::Expression(
                    Expression::Unary { o, v }.into()
                )
            }
            _ => { Value::Null }
        }
    }

    pub fn to_operator(self) -> Operator {
        match self {
            Condition::Value(_) => { Operator::default() }
            Condition::ValOpVal(_, _, _) => { Operator::default() }
            Condition::Operator(o) => { o }
            Condition::OperatorValue(_, _) => { Operator::default() }
            _ => { Operator::default() }
        }
    }
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let val = str_to_value(value);

        Self::Value(val)
    }
}

impl From<String> for Condition {
    fn from(value: String) -> Self {
        let val = str_to_value(value);

        Self::Value(val)
    }
}

impl From<Operator> for Condition {
    fn from(value: Operator) -> Self {
        Self::Operator(value)
    }
}

impl From<Value> for Condition {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}

impl From<(Operator, &str)> for Condition {
    fn from(value: (Operator, &str)) -> Self {

        let val = str_to_value(value.1);

        Self::OperatorValue(value.0, val)
    }
}

impl From<(Operator, String)> for Condition {
    fn from(value: (Operator, String)) -> Self {

        let val = str_to_value(value.1);

        Self::OperatorValue(value.0, val)
    }
}

impl From<(Operator, Value)> for Condition {
    fn from(value: (Operator, Value)) -> Self {
        Self::OperatorValue(value.0, value.1)
    }
}

impl From<(&str, Operator, &str)> for Condition {
    fn from(value: (&str, Operator, &str)) -> Self {

        let l = str_to_value(value.0);
        let o = value.1;
        let r = str_to_value(value.2);

        Self::ValOpVal(l, o, r)
    }
}

impl From<(String, Operator, String)> for Condition {
    fn from(value: (String, Operator, String)) -> Self {

        let l = str_to_value(value.0);
        let o = value.1;
        let r = str_to_value(value.2);

        Self::ValOpVal(l, o, r)
    }
}

impl From<(&str, Operator, String)> for Condition {
    fn from(value: (&str, Operator, String)) -> Self {

        let l = str_to_value(value.0);
        let o = value.1;
        let r = str_to_value(value.2);

        Self::ValOpVal(l, o, r)
    }
}

impl From<(String, Operator, &str)> for Condition {
    fn from(value: (String, Operator, &str)) -> Self {

        let l = str_to_value(value.0);
        let o = value.1;
        let r = str_to_value(value.2);

        Self::ValOpVal(l, o, r)
    }
}

impl From<(Value, Operator, Value)> for Condition {
    fn from(value: (Value, Operator, Value)) -> Self {

        let o = value.1;

        Self::ValOpVal(value.0, o, value.2)
    }
}

impl From<(&str, Operator, Value)> for Condition {
    fn from(value: (&str, Operator, Value)) -> Self {
        let l = str_to_value(value.0);

        let o = value.1;

        Self::ValOpVal(l, o, value.2)
    }
}

impl From<(String, Operator, Value)> for Condition {
    fn from(value: (String, Operator, Value)) -> Self {
        let l = str_to_value(value.0);

        let o = value.1;

        Self::ValOpVal(l, o, value.2)
    }
}

impl From<(Value, Operator, &str)> for Condition {
    fn from(value: (Value, Operator, &str)) -> Self {
        let l = value.0;
        let o = value.1;
        let r = str_to_value(value.2);


        Self::ValOpVal(l, o, r)
    }
}

impl From<(Value, Operator, String)> for Condition {
    fn from(value: (Value, Operator, String)) -> Self {
        let l = value.0;
        let o = value.1;
        let r = str_to_value(value.2);


        Self::ValOpVal(l, o, r)
    }
}

#[macro_export]
macro_rules! cond_vec {
    () => (
        std::collections::VecDeque::<$crate::query::parsing::cond::Condition>::new()
    );
    ($($x:expr),+ $(,)?) => (
        std::collections::VecDeque::<$crate::query::parsing::cond::Condition>::from([$($crate::query::parsing::cond::Condition::from($x)),+])
    );
}

#[cfg(test)]
mod test {
    use crate::query::parsing::cond::ExtraCond;
    use super::*;

    #[test]
    fn from_condition() {
        let vec_cond = vec!["cond1".into(), Operator::And.into(), "cond3".into()];

        let _extra_cond = ExtraCond::from(vec_cond);
    }

    #[test]
    fn cond_vec() {
        let v = cond_vec!["test", Operator::Equal, "test"];

        assert_eq!(v.len(), 3);
    }


}
