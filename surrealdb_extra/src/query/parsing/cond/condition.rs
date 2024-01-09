use surrealdb::sql::statements::SelectStatement;
use surrealdb::sql::{Expression, Operator, Subquery, Value};
use crate::query::parsing::cond::ExtraCond;
use crate::query::parsing::str_to_value;

#[derive(Debug, Clone, Default)]
pub enum Condition {
    #[default]
    Null,

    Value(Value),
    ValOpVal(Value, Operator, Value),
    OperatorValue(Operator, Value),
    Operator(Operator),
    SubCond(ExtraCond),
    Subquery(Subquery),
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
            Condition::OperatorValue(o, v) => {
                Value::Expression(
                    Expression::Unary { o, v }.into()
                )
            }
            Condition::SubCond(v) => {
                Value::Subquery(Box::new(Subquery::Value(v.0.0)))
            }
            Condition::Subquery(v) => {
                Value::Subquery(Box::new(v))
            }
            _ => { Value::Null }
        }
    }

    pub fn to_operator(self) -> Operator {
        match self {
            Condition::Operator(o) => { o }
            _ => { Operator::default() }
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Condition::Value(..) => true,
            Condition::ValOpVal(..) => true,
            Condition::OperatorValue(..) => true,
            _ => false,
        }
    }
    pub fn is_operator(&self) -> bool {
        match self {
            Condition::Operator(_) => true,
            _ => false
        }
    }
}

macro_rules! create_from_condition_strings {
    ($l:ty, $r:ty) => {
        impl From<($l, Operator, $r)> for Condition {
            fn from(value: ($l, Operator, $r)) -> Self {

                let l = str_to_value(value.0);
                let o = value.1;
                let r = str_to_value(value.2);

                Self::ValOpVal(l, o, r)
            }
        }
    };
}

macro_rules! create_from_condition_string_value {
    ($l:ty) => {
        impl From<($l, Operator, Value)> for Condition {
            fn from(value: ($l, Operator, Value)) -> Self {

                let l = str_to_value(value.0);
                let o = value.1;

                Self::ValOpVal(l, o, value.2)
            }
        }
    };
}

macro_rules! create_from_condition_string_select {
    ($l:ty) => {
        impl From<($l, Operator, SelectStatement)> for Condition {
            fn from(value: ($l, Operator, SelectStatement)) -> Self {

                let l = str_to_value(value.0);
                let o = value.1;
                let r = Value::Subquery(Box::new(Subquery::Select(value.2)));

                Self::ValOpVal(l, o, r)
            }
        }
    };
}

macro_rules! create_from_condition_value_string {
    ($r:ty) => {
        impl From<(Value, Operator, $r)> for Condition {
            fn from(value: (Value, Operator, $r)) -> Self {

                let o = value.1;
                let r = str_to_value(value.2);

                Self::ValOpVal(value.0, o, r)
            }
        }
    };
}

create_from_condition_strings!(&str, &str);
create_from_condition_strings!(String, String);
create_from_condition_strings!(&str, String);
create_from_condition_strings!(String, &str);

create_from_condition_string_value!(&str);
create_from_condition_string_value!(String);

create_from_condition_string_select!(&str);
create_from_condition_string_select!(String);

create_from_condition_value_string!(&str);
create_from_condition_value_string!(String);

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

impl From<(Value, Operator, Value)> for Condition {
    fn from(value: (Value, Operator, Value)) -> Self {

        let o = value.1;

        Self::ValOpVal(value.0, o, value.2)
    }
}

impl From<(Value, Operator, SelectStatement)> for Condition {
    fn from(value: (Value, Operator, SelectStatement)) -> Self {

        let o = value.1;
        let r = Value::Subquery(Box::new(Subquery::Select(value.2)));

        Self::ValOpVal(value.0, o, r)
    }
}

impl From<ExtraCond> for Condition {
    fn from(value: ExtraCond) -> Self {
        Self::SubCond(value)
    }
}

impl From<SelectStatement> for Condition {
    fn from(value: SelectStatement) -> Self {
        Self::Subquery(Subquery::Select(value))
    }
}

#[macro_export]
macro_rules! cond_vec {
    ($($x:expr),+ $(,)?) => [
        $crate::query::parsing::cond::ExtraCond::from(
            std::collections::VecDeque::<$crate::query::parsing::cond::Condition>::from([$($crate::query::parsing::cond::Condition::from($x)),+])
        )
    ];
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::query::parsing::cond::ExtraCond;
    use crate::op;

    #[test]
    fn from_condition() {
        let vec_cond = vec!["cond1".into(), Operator::And.into(), "cond3".into()];

        let _extra_cond = ExtraCond::from(vec_cond);
    }

    #[test]
    fn condition1() {
        let _ = cond_vec![(op!(!), "test")];
    }

    #[test]
    fn condition3() {
        let _ = cond_vec!["cond1", op!(||), (op!(!), "test")];
    }

    #[test]
    fn condition5() {
        let cond1 = cond_vec!["cond1", op!(and), (op!(!), "test"), op!(or), ("test", op!(!=), "$test")];
        let cond2 = cond_vec!["cond1", op!(AND), "!test", op!(Or), ("test", op!(NotEqual), "$test")];

        assert_eq!(cond1, cond2);
    }


}
