use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use surrealdb::types::{Number, Object, SurrealValue, Value};
use surrealdb_extra::surreal_value_json::SurrealValueJson;
use surrealdb_extra::table::Table;

#[allow(dead_code)]
#[derive(Debug, Default, Table, Serialize, Deserialize, Clone, PartialEq)]
#[table(name = "test_test")]
pub struct Test1;

#[derive(Debug, Serialize, Deserialize, SurrealValueJson, Eq, PartialEq)]
#[surreal_value_json(kind = "Object")]
struct Test2 {
    name: String,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, SurrealValueJson, Eq, PartialEq)]
#[surreal_value_json(kind = "Int")]
#[repr(u8)]
enum Test3 {
    Test = 1,
}

#[derive(Debug, Serialize, Deserialize, SurrealValueJson)]
struct Test4 {
    name: String,
}

#[test]
fn table_derive_init() {
    assert_eq!("test_test", Test1::TABLE_NAME)
}

#[test]
fn surreal_value_json_into_object() {
    let test = Test2 {
        name: "test".to_string(),
    };

    let val = test.into_value();
    assert!(matches!(val, Value::Object(_)))
}

#[test]
fn surreal_value_json_from_object() {
    let test = Test2 {
        name: "test".to_string(),
    };

    let mut obj = Object::new();
    obj.insert("name", "test");

    let object = Value::Object(obj);

    let res = Test2::from_value(object);

    match res {
        Ok(val) => assert_eq!(val, test),
        Err(_) => assert!(false),
    }
}

#[test]
fn surreal_value_json_into_int() {
    let test = Test3::Test;

    let val = test.into_value();

    match val {
        Value::Number(n) => match n {
            Number::Int(i) => assert_eq!(i, 1),
            _ => assert!(false),
        },
        _ => assert!(false),
    }
}

#[test]
fn surreal_value_json_from_int() {
    let test = Test3::Test;

    let num = json!(1).into_value();
    let res = Test3::from_value(num);

    match res {
        Ok(val) => assert_eq!(val, test),
        Err(_) => assert!(false),
    }
}

#[test]
fn surreal_value_json_from_value_int() {
    let test = Test3::Test;

    let num = Value::Number(Number::Int(1));
    let res = Test3::from_value(num);

    match res {
        Ok(val) => assert_eq!(val, test),
        Err(_) => assert!(false),
    }
}

#[test]
fn surreal_value_json_into_object_no_kind_specified() {
    let test = Test4 {
        name: "test".to_string(),
    };

    let val = test.into_value();
    assert!(matches!(val, Value::Object(_)))
}
