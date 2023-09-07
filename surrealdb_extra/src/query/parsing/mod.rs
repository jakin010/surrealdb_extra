use surrealdb::sql::{value, Value};

pub mod what;
pub mod idiom;
pub mod field;
pub mod cond;
pub mod omit;
pub mod limit;
pub mod start;
pub mod split;
pub mod group;
pub mod order;
pub mod fetch;
pub mod version;
pub mod timeout;

pub fn str_to_value(val: impl Into<String>) -> Value {
    let val = value(&val.into()).unwrap_or_default();

    if let Value::Idiom(..) = val {
        return val;
    }

    if let Value::Param(..) = val {
        return val;
    }

    Value::Null
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn is_param() {
        let p = "$p";

        let val = str_to_value(p);

        assert!(matches!(val, Value::Param(..)))
    }

    #[test]
    fn is_not_param() {
        let p = "p";

        let val = str_to_value(p);

        assert!(!matches!(val, Value::Param(..)))
    }

    #[test]
    fn is_idiom() {
        let i = "test.test";

        let val = str_to_value(i);

        assert!(matches!(val, Value::Idiom(..)))
    }

    #[test]
    fn is_idiom_part_3() {
        let i = "test.test.test";

        let val = str_to_value(i);

        let Value::Idiom(i) = val else {
            return assert!(false);
        };

        assert_eq!(i.0.len(), 3)
    }

    #[test]
    fn is_not_idiom() {
        let i = "$test";

        let val = str_to_value(i);

        assert!(!matches!(val, Value::Idiom(..)))
    }
}
