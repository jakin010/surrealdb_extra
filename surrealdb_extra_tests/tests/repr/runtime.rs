use surrealdb::types::{Decimal, Kind, KindLiteral, Number, SurrealValue, Value};
use surrealdb_extra::repr::SurrealValueRepr;

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum Status {
    Inactive = 0,
    Active = 1,
    Banned = 7,
}

// implicit discriminants (0, 1, then 10, 11) + const expression
const BASE: u16 = 10;
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u16)]
enum Implicit {
    A,
    B,
    C = BASE,
    D,
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(i8)]
enum Signed {
    Neg = -128,
    MinusOne = -1,
    Zero = 0,
    Max = 127,
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(i32)]
enum WithOther {
    A = 1,
    B = 2,
    #[surreal_repr(other)]
    Unknown = -1,
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u64)]
enum Big {
    Small = 1,
    Huge = u64::MAX,
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
#[surreal_repr(kind = "int")]
enum IntKind {
    A = 3,
    B = 4,
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum One {
    Only = 5,
}

// explicit crate path
pub(crate) mod alias {
    pub use surrealdb::types as t;
}
#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
#[surreal_repr(crate = "crate::repr::runtime::alias::t")]
enum Pathed {
    X = 9,
}

// Not Copy / Clone: into_value must not need them.
fn int(i: i64) -> Value {
    Value::Number(Number::Int(i))
}

#[test]
fn round_trip() {
    for s in [Status::Inactive, Status::Active, Status::Banned] {
        let d = match s {
            Status::Inactive => 0,
            Status::Active => 1,
            Status::Banned => 7,
        };
        let v = s.into_value();
        assert_eq!(v, int(d));
        assert!(matches!(v, Value::Number(Number::Int(_))));
        let back = Status::from_value(v).unwrap();
        assert_eq!(back as i64, d);
    }
}

#[test]
fn implicit_and_const_discriminants() {
    assert_eq!(Implicit::A.into_value(), int(0));
    assert_eq!(Implicit::B.into_value(), int(1));
    assert_eq!(Implicit::C.into_value(), int(10));
    assert_eq!(Implicit::D.into_value(), int(11));
    assert_eq!(Implicit::from_value(int(11)).unwrap(), Implicit::D);
}

#[test]
fn negative_i8() {
    assert_eq!(Signed::Neg.into_value(), int(-128));
    assert_eq!(Signed::from_value(int(-128)).unwrap(), Signed::Neg);
    assert_eq!(Signed::from_value(int(-1)).unwrap(), Signed::MinusOne);
    assert_eq!(Signed::from_value(int(127)).unwrap(), Signed::Max);
    assert_eq!(Signed::from_value(int(0)).unwrap(), Signed::Zero);
    let e = Signed::from_value(int(-129)).unwrap_err().to_string();
    assert!(e.contains("out of range"), "{e}");
}

#[test]
fn accepts_other_widths_and_whole_decimal() {
    assert_eq!(
        Status::from_value(Value::Number(Number::U8(7))).unwrap(),
        Status::Banned
    );
    assert_eq!(
        Status::from_value(Value::Number(Number::I32(1))).unwrap(),
        Status::Active
    );
    assert_eq!(
        Status::from_value(Value::Number(Number::Decimal(Decimal::from(7)))).unwrap(),
        Status::Banned
    );
}

#[test]
fn unknown_value_error() {
    let e = Status::from_value(int(3)).unwrap_err().to_string();
    println!("unknown: {e}");
    assert!(
        e.contains("Status") && e.contains("expected one of: 0, 1, 7"),
        "{e}"
    );
    let e = Status::from_value(int(300)).unwrap_err().to_string();
    println!("range: {e}");
    assert!(e.contains("300") && e.contains("u8"), "{e}");
}

#[test]
fn wrong_type_error() {
    let e = Status::from_value(Value::String("Active".into()))
        .unwrap_err()
        .to_string();
    println!("string: {e}");
    assert!(e.to_lowercase().contains("string"), "{e}");
    assert!(Status::from_value(Value::Bool(true)).is_err());
    assert!(Status::from_value(Value::None).is_err());
    // floats rejected, like the built-in integer impls
    let e = Status::from_value(Value::Number(Number::Float(1.0)))
        .unwrap_err()
        .to_string();
    println!("float: {e}");
    assert!(e.contains("0 | 1 | 7"), "{e}");
}

#[test]
fn fallback_variant() {
    assert_eq!(WithOther::from_value(int(2)).unwrap(), WithOther::B);
    assert_eq!(WithOther::from_value(int(42)).unwrap(), WithOther::Unknown);
    assert_eq!(WithOther::Unknown.into_value(), int(-1));
    // out-of-repr range still errors, fallback isn't reached (serde_repr semantics)
    assert!(WithOther::from_value(int(i64::from(i32::MAX) + 1)).is_err());
    assert!(WithOther::from_value(Value::String("x".into())).is_err());
    assert_eq!(WithOther::kind_of(), Kind::Int);
}

#[test]
fn kind_of_shapes() {
    let lit = |i| Kind::Literal(KindLiteral::Integer(i));
    assert_eq!(
        Status::kind_of(),
        Kind::Either(vec![lit(0), lit(1), lit(7)])
    );
    assert_eq!(
        Implicit::kind_of(),
        Kind::Either(vec![lit(0), lit(1), lit(10), lit(11)])
    );
    assert_eq!(One::kind_of(), lit(5));
    assert_eq!(IntKind::kind_of(), Kind::Int);
    assert_eq!(Big::kind_of(), Kind::Either(vec![lit(1), Kind::Number]));
    println!(
        "Status kind: {}",
        surrealdb::types::ToSql::to_sql(&Status::kind_of())
    );
}

#[test]
fn is_value_consistent_with_from_value() {
    assert!(Status::is_value(&int(7)));
    assert!(Status::is_value(&Value::Number(Number::U8(1))));
    assert!(!Status::is_value(&int(3)));
    assert!(!Status::is_value(&Value::Number(Number::Float(1.0))));
    assert!(!Status::is_value(&Value::String("1".into())));
    // default is_kind would have said yes to the float:
    assert!(Value::Number(Number::Float(1.0)).is_kind(&Status::kind_of()));
    assert!(WithOther::is_value(&int(99)));
}

#[test]
fn u64_above_i64_max() {
    let v = Big::Huge.into_value();
    println!("u64::MAX -> {v:?}");
    assert!(matches!(v, Value::Number(Number::Decimal(_))));
    assert_eq!(Big::from_value(v).unwrap(), Big::Huge);
    assert_eq!(Big::Small.into_value(), int(1));
}

#[test]
fn crate_path_override() {
    assert_eq!(Pathed::X.into_value(), int(9));
    assert_eq!(Pathed::from_value(int(9)).unwrap(), Pathed::X);
}
