use extra::repr::SurrealValueRepr;
use extra::table::Table;
use extra::types::{Number, SurrealValue, Value};

#[derive(Table)]
struct UserProfile;

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(i16)]
enum Level {
    Low = -1,
    High = 10,
}

#[derive(SurrealValue)]
#[surreal(crate = "extra::types")]
struct User {
    level: Level,
}

#[test]
fn table_with_renamed_dependency() {
    assert_eq!(UserProfile::TABLE_NAME, "user_profile");
    let id = UserProfile::create_record_id("a");
    assert_eq!(id.table.as_str(), "user_profile");
}

#[test]
fn repr_with_renamed_dependency() {
    assert_eq!(Level::Low.into_value(), Value::Number(Number::Int(-1)));
    assert_eq!(
        Level::from_value(Value::Number(Number::Int(10))).unwrap(),
        Level::High
    );
}

#[test]
fn upstream_derive_through_reexport() {
    let value = User { level: Level::High }.into_value();
    let back = User::from_value(value).unwrap();
    assert_eq!(back.level, Level::High);
}
