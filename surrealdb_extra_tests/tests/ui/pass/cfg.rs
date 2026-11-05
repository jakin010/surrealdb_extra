use surrealdb_extra::repr::SurrealValueRepr;
use surrealdb::types::{Number, SurrealValue, Value};
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum Cfgd { A = 1, #[cfg(any())] Gone = 2, B = 3 }
fn main() { assert_eq!(Cfgd::from_value(Value::Number(Number::Int(3))).unwrap(), Cfgd::B); }
