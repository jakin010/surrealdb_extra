mod condition;

use std::collections::VecDeque;
use surrealdb::sql::{Cond, Value, Expression};
use crate::query::parsing::str_to_value;
pub use super::cond::condition::Condition;

#[derive(Debug, Clone, PartialEq)]
pub struct ExtraCond(pub Cond);

impl From<Cond> for ExtraCond {
    fn from(value: Cond) -> Self {
        Self(value)
    }
}

impl From<Value> for ExtraCond {
    fn from(value: Value) -> Self {
        let mut cond = Cond::default();
        cond.0 = value;

        Self(cond)
    }
}

impl From<Expression> for ExtraCond {
    fn from(value: Expression) -> Self {
        let mut cond = Cond::default();
        cond.0 = Value::Expression(Box::new(value));

        Self(cond)
    }
}

impl From<&str> for ExtraCond {
    fn from(value: &str) -> Self {
        let val = str_to_value(value);

        let mut cond = Cond::default();
        cond.0 = val;

        Self(cond)
    }
}

impl From<String> for ExtraCond {
    fn from(value: String) -> Self {
        let val = str_to_value(value);

        let mut cond = Cond::default();
        cond.0 = val;

        Self(cond)
    }
}

impl From<Vec<Condition>> for ExtraCond {
    fn from(value: Vec<Condition>) -> Self {
        let value: VecDeque<Condition> = value.into();

        value.into()
    }
}

impl From<VecDeque<Condition>> for ExtraCond {
    fn from(mut value: VecDeque<Condition>) -> Self {
        let mut cond = Cond::default();

        if value.is_empty() {
            cond.0 = Value::Null;
            return Self(cond)
        }

        let l = value.pop_front().unwrap_or_default().to_value();

        if value.is_empty() {
            cond.0 = l;
            return Self(cond);
        }

        let o = value.pop_front().unwrap_or_default().to_operator();

        let r = value.pop_front().unwrap_or_default().to_value();

        let mut expr = Expression::Binary { l, o, r };

        while value.len() >= 2 {
            let o = value.pop_front().unwrap_or_default().to_operator();

            let v = value.pop_front().unwrap_or_default().to_value();

            expr = Expression::Binary {
                l: Value::Expression(expr.into()),
                o,
                r: v
            };
        }

        cond.0 = Value::Expression(Box::new(expr));
        Self(cond)
    }
}

impl From<Condition> for ExtraCond {
    fn from(value: Condition) -> Self {
        let val = value.to_value();

        let mut cond = Cond::default();
        cond.0 = val;

        Self(cond)
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use surrealdb::sql::Thing;
    use surrealdb::{engine::any::connect, sql::Part};
    use surrealdb::sql::{Field, Operator};
    use crate::{cond_vec, op};
    use crate::query::statement::StatementBuilder;

    use crate::table::Table;

    use super::*;

    #[derive(Debug, Table, Serialize, Deserialize, PartialEq, Clone)]
    #[table(name = "test")]
    pub struct Test {
        id: Option<Thing>,
        name: String,
        n: i64
    }

    #[tokio::test]
    async fn query_and_1cond_builder() {
        let db = connect("mem://").await.unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        let t1 = Test {
            id: None,
            name: "test".to_string(),
            n: 8,
        };

        let t2 = Test {
            id: None,
            name: "test".to_string(),
            n: 21,
        };

        let _ = t1.clone().create(&db).await.unwrap();
        let _ = t2.clone().create(&db).await.unwrap();

        let q = db.query("SELECT name, n FROM test WHERE n > $num").bind(("num", 9));
        let mut res = q.await.unwrap();
        let vec1_t: Vec<Test> = res.take(0).unwrap();

        assert_eq!(vec1_t.len(), 1);

        let select = db.select_builder().what(Test::TABLE_NAME).field("name").field("n").condition(cond_vec![("n", Operator::MoreThan, "$num")]).to_query().bind(("num", 9));
        let mut select_res = select.await.unwrap();
        let vec2_t: Vec<Test> = select_res.take(0).unwrap();

        assert_eq!(vec2_t.len(), 1);

        assert_eq!(vec1_t, vec2_t)
    }

    #[tokio::test]
    async fn select_with_macro() {
        let db = connect("mem://").await.unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        let t1 = Test {
            id: None,
            name: "test".to_string(),
            n: 8,
        };

        let t2 = Test {
            id: None,
            name: "test".to_string(),
            n: 21,
        };

        let t3 = Test {
            id: None,
            name: "test test".to_string(),
            n: 55,
        };

        let _ = t1.clone().create(&db).await.unwrap();
        let _ = t2.clone().create(&db).await.unwrap();
        let _ = t3.clone().create(&db).await.unwrap();

        let select = db.select_builder().what(Test::TABLE_NAME).field(Field::All).condition(cond_vec![
            ("name", Operator::Equal, "$name"),
                Operator::And,
            ("n", Operator::MoreThan, "$n"),
                Operator::And,
            ("n", Operator::MoreThan, "$n"),
                Operator::And,
            ("n", Operator::MoreThan, "$n"),
                Operator::And,
            ("n", Operator::MoreThan, "$n"),
                Operator::And,
            ("n", Operator::MoreThan, "$n"),
        ]).to_query()
            .bind(("name", "test"))
            .bind(("n", 3));

        let mut res = select.await.unwrap();

        let vec: Vec<Test> = res.take(0).unwrap();

        assert_eq!(vec.len(), 2)
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

    #[test]
    fn from_string() {
        let field = "test".to_string();

        let idiom = match ExtraCond::from(field.clone()).0.0 {
            Value::Idiom(i) => i,
            _ => return assert!(false)
        };

        assert_eq!(idiom.0.first().unwrap().clone(), Part::from(field))
    }

    #[test]
    fn cond_from_str() {
        let cond1 = ExtraCond::from("test > test2 AND !test3");
        let cond2  = cond_vec![("test", op!(>), "test2"), op!(and), (op!(!), "test3")];

        assert_eq!(cond1, cond2);
    }
}
