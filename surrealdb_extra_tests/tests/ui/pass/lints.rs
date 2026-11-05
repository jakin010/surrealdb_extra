#![forbid(non_camel_case_types, non_upper_case_globals)]
#![deny(unused_qualifications, trivial_numeric_casts, unused_results, missing_docs, unreachable_patterns, unused)]
//! lints
use surrealdb_extra::repr::SurrealValueRepr;
use surrealdb::types::{Number, SurrealValue, Value};
/// e
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(i64)]
pub enum L { /// a
 A = 1, /// b
 B = 2 }
fn main() { assert_eq!(L::from_value(Value::Number(Number::Int(2))).unwrap(), L::B); }
