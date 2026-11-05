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
