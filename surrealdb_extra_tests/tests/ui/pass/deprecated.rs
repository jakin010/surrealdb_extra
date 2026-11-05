#![deny(deprecated)]
use surrealdb_extra::repr::SurrealValueRepr;
use surrealdb::types::{Number, SurrealValue, Value};
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum Dep { A = 1, #[deprecated] Old = 2 }
fn main() { assert_eq!(Dep::from_value(Value::Number(Number::Int(1))).unwrap(), Dep::A); }
