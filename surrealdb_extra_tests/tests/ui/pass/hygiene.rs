#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]
use surrealdb_extra::repr::SurrealValueRepr;
use surrealdb::types::{Number, SurrealValue, Value as V};
// Items in scope whose names match the macro's local bindings
const other: u8 = 200;
const d: u8 = 201;
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum H { A = 1 }
fn main() { assert_eq!(H::from_value(V::Number(Number::Int(1))).unwrap(), H::A); }
