// The embedded database makes this test binary large enough that the macOS linker warns
// about its unwind table. It is harmless.
#![allow(linker_messages)]

pub(crate) mod repr;

use serde::{Deserialize, Serialize};
use surrealdb_extra::table::Table;

#[allow(dead_code)]
#[derive(Debug, Default, Table, Serialize, Deserialize, Clone, PartialEq)]
#[table(name = "test_test")]
pub struct Test1;

#[test]
fn table_derive_init() {
    assert_eq!("test_test", Test1::TABLE_NAME)
}

#[allow(dead_code)]
#[derive(Table)]
struct UserProfile;

#[allow(dead_code)]
#[derive(Table)]
struct HTTPServer;

#[allow(dead_code, non_camel_case_types)]
#[derive(Table)]
struct r#type;

#[test]
fn table_default_name_is_snake_case() {
    assert_eq!("user_profile", UserProfile::TABLE_NAME);
    assert_eq!("http_server", HTTPServer::TABLE_NAME);
    assert_eq!("type", r#type::TABLE_NAME);
}

#[test]
fn table_default_name_record_id() {
    let id = UserProfile::create_record_id("a");
    assert_eq!(id.table.as_str(), "user_profile");
}
