#[derive(surrealdb_extra::table::Table)]
#[table(crate = "not a path")]
struct S;
fn main() {}
