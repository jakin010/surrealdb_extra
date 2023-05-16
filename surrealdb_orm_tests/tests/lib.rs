use serde::{Deserialize, Serialize};
use surrealdb::engine::any::{Any, connect};
use surrealdb::kvs::Datastore;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb_orm::Table;

#[allow(dead_code)]
#[derive(Debug, Default, Table, Serialize, Deserialize, Clone)]
#[table(name = "test_test")]
pub struct Test {
    id: Option<Thing>,
    name: String,
}

async fn database() -> Surreal<Any> {
    let _ = Datastore::new("memory").await.unwrap();
    let db = connect("mem://").await.unwrap();
    db.use_ns("ns").use_db("db").await.unwrap();

    db
}

#[test]
fn table_derive_init() {
    assert_eq!("test_test", Test::table_name())
}

#[test]
fn table_derive_get_id() {
    let t = Test {
        id: Some(Thing::from(("test", "test"))),
        name: "".to_string(),
    };
    assert_eq!(t.get_id().clone().unwrap(), Thing::from(("test", "test")))
}

#[test]
fn table_derive_set_id() {
    let mut t = Test {
        name: "".to_string(),
        ..Test::default()
    };

    t.set_id(Thing::from(("test", "test")));

    assert_eq!(t.get_id().clone().unwrap(), Thing::from(("test", "test")))
}

#[tokio::test]
async fn table_create() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test".to_string(),
    };

    let tc = t.clone().create(&db).await.unwrap();

    assert_eq!(t.name, tc.name);
}

#[tokio::test]
async fn table_db_get_by_id() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
    };

    let tc = t.create(&db).await.unwrap();

    let op_t = Test::get_by_id(tc.id.clone().unwrap(), &db).await.unwrap();

    assert!(op_t.is_some());
    assert_eq!(op_t.unwrap().id.unwrap(), tc.id.unwrap())
}

#[tokio::test]
async fn table_delete() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
    };

    let tc = t.create(&db).await.unwrap();

    assert!(tc.id.is_some());

    let td = Test::delete(tc.id.unwrap(), &db).await.unwrap();

    let op_td = Test::get_by_id(td.unwrap().id.clone().unwrap(), &db).await.unwrap();

    assert!(op_td.is_none())
}

#[tokio::test]
async fn table_get_all() {
    let db = database().await;

    for _ in 0..10 {
        let t = Test {
            id: None,
            name: "test data".to_string(),
        };

        let _ = t.create(&db).await.unwrap();
    }

    let vt = Test::get_all(&db).await.unwrap();

    assert_eq!(vt.len(), 10);
}

#[tokio::test]
async fn table_update() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
    };

    let mut tc = t.create(&db).await.unwrap();

    tc.name = "test".to_string();

    let tu = tc.clone().update(&db).await.unwrap();

    assert!(tu.is_some());
    assert_eq!(tu.unwrap().name, tc.name);
}

#[tokio::test]
async fn table_select() {
    let db = database().await;

    let t = Test::select().field("t").field("field");

    println!("{:#?}", t)
}