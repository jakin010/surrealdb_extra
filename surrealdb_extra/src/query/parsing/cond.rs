use surrealdb::sql::{Cond, Value, Expression, Idiom, Operator};

use super::idiom::ExtraIdiom;

pub struct ExtraCond(pub Cond);

impl From<Cond> for ExtraCond {
    fn from(value: Cond) -> Self {
        Self(value)
    }
}

impl From<Value> for ExtraCond {
    fn from(value: Value) -> Self {
        Self(Cond(value))
    }
}

impl From<Expression> for ExtraCond {
    fn from(value: Expression) -> Self {
        Self(Cond(Value::Expression(Box::new(value))))
    }
}

impl From<&str> for ExtraCond {
    fn from(value: &str) -> Self {

        let idiom = ExtraIdiom::from(value).0;

        Self(Cond(Value::Idiom(idiom)))
    }
}


// impl From<(Operator, &str)> for ExtraCond {
//     fn from(value: (Operator, &str)) -> Self {
//         let idiom = ExtraIdiom::from(value.1).0;

//         let expr = Expression::Unary { o: value.0, v: Value::Idiom(idiom) };

//         Self(Cond(Value::Expression(Box::new(expr))))
//     }
// }

impl From<String> for ExtraCond {
    fn from(value: String) -> Self {

        let idiom = ExtraIdiom::from(value).0;

        Self(Cond(Value::Idiom(idiom)))
    }
}

// impl From<(Operator, String)> for ExtraCond {
//     fn from(value: (Operator, String)) -> Self {
//         let idiom = ExtraIdiom::from(value.1).0;

//         let expr = Expression::Unary { o: value.0, v: Value::Idiom(idiom) };

//         Self(Cond(Value::Expression(Box::new(expr))))
//     }
// }

// impl From<Idiom> for ExtraCond {
//     fn from(value: Idiom) -> Self {
//         Self(Cond(Value::Idiom(value)))
//     }
// }

// impl From<(Operator, Idiom)> for ExtraCond {
//     fn from(value: (Operator, Idiom)) -> Self {

//         let expr = Expression::Unary { o: value.0, v: Value::Idiom(value.1) };

//         Self(Cond(Value::Expression(Box::new(expr))))
//     }
// }

#[derive(Debug, Clone)]
pub enum Condition {
    Field(Value),
    FieldValue(Value, Operator, Value),
    OperatorField(Operator, Value),
    Operator(Operator),
}

impl Condition {
    pub fn to_value(self) -> Value {
        match self {
            Condition::Field(k) => { k }
            Condition::FieldValue(k, o, v) => { 
                Value::Expression(
                    Expression::Binary { l: k, o, r: v }.into()
                )
             }
            Condition::Operator(_) => { Value::Null }
            Condition::OperatorField(o, k) => {
                Value::Expression(
                    Expression::Unary { o, v: k }.into()
                )
            } 
        }
    }

    pub fn to_oparator(self) -> Operator {
        match self {
            Condition::Field(_) => { Operator::default() }
            Condition::FieldValue(_, _, _) => { Operator::default() }
            Condition::Operator(o) => { o }
            Condition::OperatorField(_, _) => { Operator::default() } 
        }
    }
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        Self::Field(Value::Idiom(idiom))
    }
}

impl From<String> for Condition {
    fn from(value: String) -> Self {
        let idiom = ExtraIdiom::from(value).0;

        Self::Field(Value::Idiom(idiom))
    }
}

impl From<Operator> for Condition {
    fn from(value: Operator) -> Self {
        Self::Operator(value)
    }
}

impl From<Vec<Condition>> for ExtraCond {
    fn from(mut value: Vec<Condition>) -> Self {
        
        // let value = value.reverse();

        let mut expr = Expression::default();

        let l = value.remove(0);
        let l_val = l.to_value();

        let o = value.remove(0);
        let o_oparator = o.to_oparator();

        let r = value.remove(0);
        let r_val = r.to_value();

        expr = Expression::Binary { l: l_val, o: o_oparator, r: r_val };

        for v in value.windows(2) {

            let oparator = &v[0];
            
            // let value = match v {
            //     Condition::Field(k) => { }
            //     Condition::FieldValue(k, o, v) => {}
            //     Condition::Operator(o) => { }
            //     Condition::OperatorField(o, k) => {} 
            // };

            break;
        }
        
        dbg!(&expr);

        Self(Cond(Value::Expression(Box::new(expr))))
    }
}

// impl From<impl Into<Condition>> for ExtraCond {
//     fn from(value: impl Into<Condition>) -> Self {

//         let expr = Expression::Unary { o: value.0, v: Value::Idiom(value.1) };

//         Self(Cond(Value::Expression(Box::new(expr))))
//     }
// }

#[cfg(test)]
mod test {

    use surrealdb::{engine::any::connect, sql::Part};

    use super::*;

    #[tokio::test]
    async fn query() {
        let db = connect("mem://").await.unwrap();

        let q = db.query("SELECT * FROM test WHERE test = 1 AND test = 2 AND test = 3 AND test = 4");

        dbg!(q);
    }

    #[test]
    fn from_condition() {
        let cond = Condition::Field(Value::Idiom(ExtraIdiom::from("value").0));

        let vec_cond = vec![cond];

        let _extra_cond = ExtraCond::from(vec_cond);
    }

    #[test]
    fn from_str() {
        let field = "test";

        let idiom = match ExtraCond::from(field).0.0 {
            Value::Idiom(i) => i,
            _ => return assert!(false)
        };

        assert_eq!(idiom.0.first().unwrap().clone(), Part::from(field))
    }
}