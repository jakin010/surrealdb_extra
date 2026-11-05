use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use surrealdb::types::{SurrealValue, ToSql};
use surrealdb_extra::repr::SurrealValueRepr;

/// The embedded engine needs more stack than the default 2 MB test thread in debug builds.
fn run(fut: impl std::future::Future<Output = ()> + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_stack_size(32 * 1024 * 1024)
                .build()
                .unwrap()
                .block_on(fut)
        })
        .unwrap()
        .join()
        .unwrap();
}

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum Status {
    Inactive = 0,
    Active = 1,
    Banned = 7,
}

#[derive(Debug, PartialEq, SurrealValue)]
struct User {
    name: String,
    status: Status,
}

#[test]
fn db_round_trip() {
    run(db_round_trip_impl());
}

async fn db_round_trip_impl() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("t").use_db("t").await.unwrap();
    let ty = Status::kind_of().to_sql();
    db.query(format!("DEFINE TABLE user SCHEMAFULL; DEFINE FIELD name ON user TYPE string; DEFINE FIELD status ON user TYPE {ty};"))
        .await.unwrap().check().unwrap();

    let created: Option<User> = db
        .create(("user", "a"))
        .content(User {
            name: "a".into(),
            status: Status::Banned,
        })
        .await
        .unwrap();
    assert_eq!(created.unwrap().status, Status::Banned);

    let mut res = db
        .query("SELECT VALUE status FROM user:a; SELECT VALUE type::of(status) FROM ONLY user:a")
        .await
        .unwrap();
    let raw: Vec<i64> = res.take(0).unwrap();
    assert_eq!(raw, vec![7]);
    let st: Vec<Status> = db
        .query("SELECT VALUE status FROM user")
        .await
        .unwrap()
        .take(0)
        .unwrap();
    assert_eq!(st, vec![Status::Banned]);
    let ty_of: Option<String> = res.take(1).unwrap();
    println!("stored type: {ty_of:?}, schema TYPE {ty}");

    // Schema enforces the literal union: 3 is rejected by the DB itself.
    let err = db
        .query("CREATE user:b SET name = 'b', status = 3")
        .await
        .unwrap()
        .check();
    println!(
        "schema reject: {:?}",
        err.as_ref().err().map(|e| e.to_string())
    );
    assert!(err.is_err());

    // Bind as parameter
    let found: Vec<User> = db
        .query("SELECT * OMIT id FROM user WHERE status = $s")
        .bind(("s", Status::Banned))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    assert_eq!(found.len(), 1);
}

mod edge_cases {
    use surrealdb::Surreal;
    use surrealdb::engine::local::Mem;
    use surrealdb::types::{SurrealValue, ToSql};
    use surrealdb_extra::repr::SurrealValueRepr;

    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(i64)]
    enum Ext {
        Min = i64::MIN,
        Max = i64::MAX,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u64)]
    enum Big {
        A = 1,
        B = u64::MAX,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(u8)]
    enum WO {
        A = 1,
        #[surreal_repr(other)]
        U = 0,
    }
    #[derive(Debug, PartialEq, SurrealValueRepr)]
    #[repr(i8)]
    enum Neg {
        A = -5,
        B = 3,
    }

    #[derive(Debug, PartialEq, SurrealValue)]
    struct R {
        e: Ext,
        b: Big,
        w: WO,
        n: Neg,
        o: Option<Neg>,
    }

    #[test]
    fn adv_db() {
        super::run(adv_db_impl());
    }

    async fn adv_db_impl() {
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("t").use_db("t").await.unwrap();
        let q = format!(
            "DEFINE TABLE r SCHEMAFULL; DEFINE FIELD e ON r TYPE {}; DEFINE FIELD b ON r TYPE {}; DEFINE FIELD w ON r TYPE {}; DEFINE FIELD n ON r TYPE {}; DEFINE FIELD o ON r TYPE {};",
            Ext::kind_of().to_sql(),
            Big::kind_of().to_sql(),
            WO::kind_of().to_sql(),
            Neg::kind_of().to_sql(),
            <Option<Neg>>::kind_of().to_sql()
        );
        println!("{q}");
        let r = db.query(q).await.unwrap().check();
        println!("define: {:?}", r.as_ref().err().map(|e| e.to_string()));
        r.unwrap();
        for (id, e, b, n) in [
            ("a", Ext::Min, Big::B, Neg::A),
            ("b", Ext::Max, Big::A, Neg::B),
        ] {
            let res: Result<Option<R>, _> = db
                .create(("r", id))
                .content(R {
                    e,
                    b,
                    w: WO::A,
                    n,
                    o: Some(Neg::A),
                })
                .await;
            println!("create {id}: {:?}", res.as_ref().map_err(|e| e.to_string()));
            assert!(res.unwrap().is_some());
        }
        // DB accepts w = 300 (schema int) but Rust cannot read it back
        let r = db
            .query("CREATE r:c SET e = 9223372036854775807, b = 1, w = 300, n = 3")
            .await
            .unwrap()
            .check();
        println!(
            "w=300 create: {:?}",
            r.as_ref().err().map(|e| e.to_string())
        );
        let back: Result<Option<R>, _> = db.select(("r", "c")).await;
        println!("read back w=300: {:?}", back.map_err(|e| e.to_string()));
        let all: Result<Vec<R>, _> = db.select("r").await;
        println!(
            "select all: {:?}",
            all.map(|v| v.len()).map_err(|e| e.to_string())
        );
    }
}
