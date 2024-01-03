use std::time::Instant;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use serde::{Deserialize, Serialize};

use surrealdb::engine::any::{Any, connect};
use surrealdb::sql::{Field, Thing, Operator, Value, Expression};
use surrealdb::Surreal;
use surrealdb_extra::query::parsing::cond::Condition;
use tokio::runtime::Runtime;
use surrealdb_extra::{cond_vec, op};
use surrealdb_extra::query::statement::StatementBuilder;
use surrealdb_extra::query::parsing::order::OrderDirection;
use surrealdb_extra::table::Table;

async fn db() -> Surreal<Any> {
    let db = connect("mem://").await.unwrap();

    db.use_ns("test").use_db("test").await.unwrap();

    db
}

#[derive(Debug, Table, Serialize, Deserialize, PartialEq, Clone)]
#[table(name = "test")]
pub struct Test {
    id: Option<Thing>,
    name: String,
    n: i64
}

fn select_builder_from_expr_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "select_builder_from_expr", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _select = db.select_builder().what(Test::TABLE_NAME).field(Field::All).condition(
                        Value::Expression(
                            Box::new(
                                Expression::Binary { 
                                    l: Condition::from(("n", Operator::MoreThan, "$n")).to_value(), 
                                    o: Condition::from(op!(and)).to_operator(), 
                                    r: Value::Expression(
                                        Box::new(
                                            Expression::Binary { 
                                                l: Condition::from(("n", Operator::MoreThan, "$n")).to_value(), 
                                                o: Condition::from(op!(and)).to_operator(), 
                                                r: Value::Expression(
                                                    Box::new(
                                                        Expression::Binary { 
                                                            l: Condition::from(("n", Operator::MoreThan, "$n")).to_value(), 
                                                            o: Condition::from(op!(and)).to_operator(), 
                                                            r: Value::Expression(
                                                                Box::new(
                                                                    Expression::Binary { 
                                                                        l: Condition::from(("n", Operator::MoreThan, "$n")).to_value(), 
                                                                        o: Condition::from(op!(and)).to_operator(), 
                                                                        r: Condition::from(("n", Operator::MoreThan, "$n")).to_value(),
                                                                    }
                                                                )
                                                            ) 
                                                        }
                                                    )
                                                ) 
                                            }
                                        )
                                    ) 
                                }
                            )
                        )
                    ).to_query()
                        .bind(("name", "test"))
                        .bind(("n", 3));
                }
                start.elapsed()
            })
    );
}

fn select_builder_with_cond_5_exact_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "select_builder_with_cond_5_exact", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _select = db.select_builder().what(Test::TABLE_NAME).field(Field::All).condition(cond_vec![
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                        ]).to_query()
                        .bind(("name", "test"))
                        .bind(("n", 3));
                }
                start.elapsed()
            })
    );
}
fn query_with_cond_5_exact_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "query_with_cond_5_exact", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _query = db.query("SELECT * FROM test WHERE n > $n AND n > $n AND n > $n AND n > $n AND n > $n")
                        .bind(("name", "test"))
                        .bind(("n", 3));
                }
                start.elapsed()
            })
    );
}

fn select_builder_with_cond_and_subquery_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "select_builder_with_cond_and_subquery", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _select = db.select_builder().what(Test::TABLE_NAME).field(Field::All).condition(cond_vec![
                            ("name", Operator::Equal, "$name"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            cond_vec![("test", Operator::Equal, "$n"), Operator::And, ("n", Operator::MoreThan, "$n"), Operator::And, ("n", Operator::MoreThan, "$n")]
                        ]).to_query()
                        .bind(("name", "test"))
                        .bind(("n", 3));
                }
                start.elapsed()
            })
    );
}
fn query_with_cond_and_subquery_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "query_with_cond_and_subquery", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _query = db.query("SELECT * FROM test WHERE name = $name AND n > $n AND n > $n AND n > $n AND n > $n AND n > $n AND (test = $n AND n > $n AND n > $n)")
                        .bind(("name", "test"))
                        .bind(("n", 3));
                }
                start.elapsed()
            })
    );
}

fn select_builder_with_cond_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "select_builder_with_cond", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _select = db.select_builder().what(Test::TABLE_NAME).field(Field::All).condition(cond_vec![
                            ("name", Operator::Equal, "$name"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                                Operator::And,
                            ("n", Operator::MoreThan, "$n"),
                        ]).to_query()
                        .bind(("name", "test"))
                        .bind(("n", 3));
                }
                start.elapsed()
            })
    );
}
fn query_with_cond_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "query_with_cond", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _query = db.query("SELECT * FROM test WHERE name = $name AND n > $n AND n > $n AND n > $n AND n > $n AND n > $n")
                        .bind(("name", "test"))
                        .bind(("n", 3));
                }
                start.elapsed()
            })
    );
}

fn select_builder_without_cond_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "select_builder_without_cond", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _select = db.select_builder().what(Test::TABLE_NAME).field(Field::All).to_query();
                }
                start.elapsed()
            })
    );
}
fn query_without_cond_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "query_without_cond", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _query = db.query("SELECT * FROM test");
                }
                start.elapsed()
            })
    );
}

fn select_builder_with_more_options_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "select_builder_with_more_options", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _select = db.select_builder().what(Test::TABLE_NAME).field(Field::All).limit(5).start(2).order(("test", OrderDirection::DESC)).order(("test", OrderDirection::ASC)).to_query();
                }
                start.elapsed()
            })
    );
}
fn query_with_more_options_benchmark(c: &mut Criterion) {

    let r = Runtime::new().unwrap();

    c.bench_function(
        "query_with_more_options", move |b|
            b.to_async(&r).iter_custom(|iters| async move {

                let db = db().await;

                let start = Instant::now();
                for _i in 0..iters {
                    let _query = db.query("SELECT * FROM test LIMIT 5 START 2 ORDER BY test DESC, test ASC");
                }
                start.elapsed()
            })
    );
}

criterion_group!(benches_select_from_expr_with_cond, select_builder_from_expr_benchmark);

criterion_group!(benches_select_with_cond, select_builder_with_cond_benchmark);
criterion_group!(benches_query_with_cond, query_with_cond_benchmark);

criterion_group!(benches_select_subquery, select_builder_with_cond_and_subquery_benchmark);
criterion_group!(benches_query_subquery, query_with_cond_and_subquery_benchmark);

criterion_group!(benches_select_5, select_builder_with_cond_5_exact_benchmark);
criterion_group!(benches_query_5, query_with_cond_5_exact_benchmark);

criterion_group!(benches_select_without_cond, select_builder_without_cond_benchmark);
criterion_group!(benches_query_without_cond, query_without_cond_benchmark);

criterion_group!(benches_select_more_options, select_builder_with_more_options_benchmark);
criterion_group!(benches_query_more_options, query_with_more_options_benchmark);

criterion_main!(
    benches_select_from_expr_with_cond,
    benches_select_with_cond, benches_query_with_cond,
    benches_select_subquery, benches_query_subquery,
    benches_select_5, benches_query_5,
    benches_select_without_cond, benches_query_without_cond,
    benches_select_more_options, benches_query_more_options,
);
