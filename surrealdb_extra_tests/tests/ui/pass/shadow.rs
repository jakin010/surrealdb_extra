// Same-named enum as the hidden struct, and user-shadowed `core`/`std`
#![allow(dead_code)]
use surrealdb_extra::repr::SurrealValueRepr;
use surrealdb::types::{Number, SurrealValue, Value};
mod core {}
mod std {}
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum S { A = 1 }
fn main() { assert_eq!(S::from_value(Value::Number(Number::Int(1))).unwrap(), S::A); }
