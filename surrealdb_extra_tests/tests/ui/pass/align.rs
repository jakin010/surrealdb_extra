use surrealdb_extra::repr::SurrealValueRepr;
use surrealdb::types::{Number, SurrealValue, Value};
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(align(4))]
#[repr(u16)]
enum Aligned { A = 1, B = 500 }
fn main() { assert_eq!(Aligned::from_value(Value::Number(Number::Int(500))).unwrap(), Aligned::B); assert_eq!(std::mem::align_of::<Aligned>(), 4); }
