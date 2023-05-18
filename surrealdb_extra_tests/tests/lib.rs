use serde::{Deserialize, Serialize};
use surrealdb::engine::any::{Any, connect};
use surrealdb::kvs::Datastore;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb_extra::query_builder::filter::{LogicalOperator, RelationalOperator};
use surrealdb_extra::table::Table;

#[allow(dead_code)]
#[derive(Debug, Default, Table, Serialize, Deserialize, Clone, PartialEq)]
#[table(name = "test_test")]
pub struct Test {
    id: Option<Thing>,
    name: String,
    n: Option<usize>,
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
        ..Test::default()
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
        ..Test::default()
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
        ..Test::default()
    };

    let tc = t.create(&db).await.unwrap();

    let op_t = Test::get_by_id(tc.get_id().clone().unwrap(), &db).await.unwrap();

    assert!(op_t.is_some());
    assert_eq!(op_t.unwrap().get_id().clone().unwrap(), tc.get_id().clone().unwrap())
}

#[tokio::test]
async fn table_delete() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
        ..Test::default()
    };

    let tc = t.create(&db).await.unwrap();

    assert!(tc.id.is_some());

    let td = Test::delete(tc.get_id().clone().unwrap(), &db).await.unwrap();

    let op_td = Test::get_by_id(td.unwrap().get_id().clone().unwrap(), &db).await.unwrap();

    assert!(op_td.is_none())
}

#[tokio::test]
async fn table_get_all() {
    let db = database().await;

    for _ in 0..10 {
        let t = Test {
            id: None,
            name: "test data".to_string(),
            ..Test::default()
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
        ..Test::default()
    };

    let mut tc = t.create(&db).await.unwrap();

    tc.name = "test".to_string();

    let tu = tc.clone().update(&db).await.unwrap();

    assert!(tu.is_some());
    assert_eq!(tu.unwrap().name, tc.name);
}

#[tokio::test]
async fn table_select_limit() {
    let db = database().await;

    for n in 0..10 {
        let t = Test {
            id: None,
            name: "test data".to_string(),
            n: Some(n),
            ..Test::default()
        };

        let _ = t.create(&db).await.unwrap();
    }

    let mut vt = Test::select(None)
        .field("name")
        .field("n")
        .limit(5)
        .execute(&db).await.unwrap();

    let res: Vec<Test> = vt.take(0).unwrap();
    
    assert_eq!(res.len(), 5);
}

#[tokio::test]
async fn table_select_name() {
    let db = database().await;

    let t = Test {
        name: "test data".to_string(),
        ..Test::default()
    };

    let _ = t.clone().create(&db).await.unwrap();

    let mut vt = Test::select(None)
        .field("name")
        .execute(&db).await.unwrap();

    let res: Vec<Test> = vt.take(0).unwrap();

    assert_eq!(t, res.get(0).unwrap().clone())
}

#[tokio::test]
async fn table_select_filter() {
    let db = database().await;

    let t = Test {
        name: "test data".to_string(),
        ..Test::default()
    };

    let _ = t.clone().create(&db).await.unwrap();

    let t2 = Test {
        name: "test lala".to_string(),
        ..Test::default()
    };

    let _ = t2.create(&db).await.unwrap();

    let mut vt = Test::select(None)
        .field("name")
        .where_filter()
        .filter(("name", RelationalOperator::Equal, "test data", LogicalOperator::End)).unwrap_left()
        .execute(&db).await.unwrap();

    let res: Vec<Test> = vt.take(0).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(t, res.get(0).unwrap().clone())
}

#[tokio::test]
async fn table_select_filter_id() {
    let db = database().await;

    let t = Test {
        id: Some((Test::table_name(), "test1".to_string()).into()),
        name: "test data".to_string(),
        ..Test::default()
    };

    let _ = t.clone().create(&db).await.unwrap();

    let t2 = Test {
        name: "test lala".to_string(),
        ..Test::default()
    };

    let _ = t2.create(&db).await.unwrap();

    let mut vt = Test::select(Some("test1".into()))
        .field("id")
        .field("name")
        .where_filter()
        .filter(("name", RelationalOperator::Equal, "test data", LogicalOperator::End)).unwrap_left()
        .execute(&db).await.unwrap();

    let res: Vec<Test> = vt.take(0).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(t, res.get(0).unwrap().clone())
}
