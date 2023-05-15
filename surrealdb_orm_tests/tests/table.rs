use surrealdb_orm::Table;

#[derive(Debug, Table)]
#[table(name = "test_test")]
pub struct Test {
    #[table(id)]
    id: String,
    name: String,
}

#[test]
fn table_derive_init() {
//    let t = Test {
//        id: "test".to_string(),
//        name: "test".to_string(),
//    };

    println!("{:#?}", Test::name());
}