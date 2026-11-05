#![allow(clippy::enum_clike_unportable_variant, clippy::enum_variant_names)]

use surrealdb::types::{Decimal, Kind, Number, SurrealValue, ToSql, Value};
use surrealdb_extra::repr::SurrealValueRepr;

fn int(i: i64) -> Value {
    Value::Number(Number::Int(i))
}

const SHIFT: u8 = 3;
const fn five() -> u8 {
    5
}
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum Exprs {
    A = 1 << SHIFT,
    B,
    C = five() * 2,
    D = (1 + 1),
    E,
}

#[test]
fn discriminant_exprs() {
    assert_eq!(Exprs::A.into_value(), int(8));
    assert_eq!(Exprs::B.into_value(), int(9));
    assert_eq!(Exprs::C.into_value(), int(10));
    assert_eq!(Exprs::D.into_value(), int(2));
    assert_eq!(Exprs::E.into_value(), int(3));
    for (i, e) in [
        (8, Exprs::A),
        (9, Exprs::B),
        (10, Exprs::C),
        (2, Exprs::D),
        (3, Exprs::E),
    ] {
        assert_eq!(Exprs::from_value(int(i)).unwrap(), e);
    }
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(i64)]
enum Extremes {
    Min = i64::MIN,
    MinPlus,
    Max = i64::MAX,
}

#[test]
fn i64_extremes() {
    assert_eq!(Extremes::Min.into_value(), int(i64::MIN));
    assert_eq!(Extremes::MinPlus.into_value(), int(i64::MIN + 1));
    assert_eq!(Extremes::from_value(int(i64::MAX)).unwrap(), Extremes::Max);
    assert_eq!(Extremes::from_value(int(i64::MIN)).unwrap(), Extremes::Min);
    println!("i64 kind: {}", Extremes::kind_of().to_sql());
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(usize)]
enum Ptr {
    A = 0,
    B = usize::MAX,
}
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(isize)]
enum IPtr {
    A = isize::MIN,
    B = -1,
    C,
}

#[test]
fn ptr_reprs() {
    let v = Ptr::B.into_value();
    println!("usize::MAX -> {v:?}");
    assert_eq!(Ptr::from_value(v).unwrap(), Ptr::B);
    assert_eq!(
        Ptr::from_value(Value::Number(Number::U64(u64::MAX))).unwrap(),
        Ptr::B
    );
    assert_eq!(IPtr::from_value(int(i64::MIN)).unwrap(), IPtr::A);
    assert_eq!(IPtr::C.into_value(), int(0));
    println!("usize kind: {:?}", Ptr::kind_of());
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u64)]
enum U64 {
    A = 5,
    B = u64::MAX,
}

#[test]
fn u64_inputs() {
    assert_eq!(
        U64::from_value(Value::Number(Number::U64(u64::MAX))).unwrap(),
        U64::B
    );
    assert_eq!(
        U64::from_value(Value::Number(Number::Decimal(Decimal::from(u64::MAX)))).unwrap(),
        U64::B
    );
    assert!(U64::from_value(int(-1)).is_err());
    assert!(U64::is_value(&Value::Number(Number::U64(u64::MAX))));
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum Status {
    A = 0,
    B = 1,
    C = 7,
}

#[test]
fn float_decimal_none_null() {
    use std::str::FromStr;
    let d = |s: &str| Value::Number(Number::Decimal(Decimal::from_str(s).unwrap()));
    assert_eq!(Status::from_value(d("7")).unwrap(), Status::C);
    assert_eq!(Status::from_value(d("7.000")).unwrap(), Status::C);
    let e = Status::from_value(d("7.5")).unwrap_err().to_string();
    println!("7.5 -> {e}");
    let e = Status::from_value(d("-0")).map(|x| format!("{x:?}"));
    println!("-0 -> {e:?}");
    for v in [
        Value::None,
        Value::Null,
        Value::Number(Number::Float(7.0)),
        Value::Number(Number::Float(f64::NAN)),
        Value::Number(Number::F32(1.0)),
        Value::Bool(true),
        Value::String("7".into()),
    ] {
        let r = Status::from_value(v.clone());
        println!("{v:?} -> {:?}", r.as_ref().map_err(|e| e.to_string()));
        assert!(r.is_err());
        assert!(!Status::is_value(&v));
    }
}

#[test]
fn option_kind_sql() {
    println!(
        "Option<Status> kind: {}",
        <Option<Status>>::kind_of().to_sql()
    );
    println!("Vec<Status> kind: {}", <Vec<Status>>::kind_of().to_sql());
    assert_eq!(<Option<Status>>::from_value(Value::None).unwrap(), None);
    assert_eq!(
        <Option<Status>>::from_value(int(7)).unwrap(),
        Some(Status::C)
    );
    let r = <Option<Status>>::from_value(Value::Null);
    println!("Option from Null -> {:?}", r.map_err(|e| e.to_string()));
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum WithOtherU8 {
    A = 1,
    #[surreal_repr(other)]
    U = 0,
}
#[test]
fn other_int_kind_accepts_out_of_range_in_db() {
    // Kind::Int lets 300 through the schema, but from_value rejects it
    assert_eq!(WithOtherU8::kind_of(), Kind::Int);
    println!(
        "300 -> {:?}",
        WithOtherU8::from_value(int(300)).map_err(|e| e.to_string())
    );
    println!(
        "-1 -> {:?}",
        WithOtherU8::from_value(int(-1)).map_err(|e| e.to_string())
    );
}

// Enums named like prelude / surreal types
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum Option2 {
    None = 0,
    Some = 1,
}
#[allow(dead_code, non_camel_case_types)]
mod names {
    use super::*;
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum Result {
        Ok = 0,
        Err = 1,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum Value {
        Number = 0,
        None = 1,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum Kind {
        Int = 0,
        Literal = 1,
        Either = 2,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum Error {
        A,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum Vec {
        A,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum String {
        A,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum TryFrom {
        A,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum SurrealValue {
        A,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum Ok {
        Ok,
        Err,
        Some,
        None,
    }
    // variants named like the generated locals
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    pub enum Locals {
        value,
        other,
        d,
        expected,
        i,
        Self_,
    }
}

#[test]
fn prelude_names() {
    assert_eq!(Option2::from_value(int(1)).unwrap(), Option2::Some);
    assert_eq!(
        <names::Result as SurrealValue>::from_value(int(1)).unwrap(),
        names::Result::Err
    );
    assert_eq!(
        <names::Value as SurrealValue>::into_value(names::Value::None),
        int(1)
    );
    assert_eq!(
        <names::Ok as SurrealValue>::from_value(int(3)).unwrap(),
        names::Ok::None
    );
    assert_eq!(
        <names::Locals as SurrealValue>::from_value(int(1)).unwrap(),
        names::Locals::other
    );
}

// local enum inside fn, and two enums in same scope
#[test]
fn local_enum() {
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    enum L {
        A = 3,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    enum M {
        A = 4,
    }
    assert_eq!(L::from_value(int(3)).unwrap(), L::A);
    assert_eq!(M::A.into_value(), int(4));
}

// enum that is Copy + Clone + Hash etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SurrealValueRepr)]
#[repr(u8)]
#[non_exhaustive]
pub enum Full {
    A = 1,
}
