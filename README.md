# SurrealDB Extra

Helpers for working with [SurrealDB](https://surrealdb.com) in Rust: a `Table` trait for your table structs, and a derive that stores fieldless enums as integers.

It builds on [`surrealdb-types`](https://docs.rs/surrealdb-types) (the same types the SurrealDB SDK exposes as `surrealdb::types`), so it works alongside the `surrealdb` crate but doesn't require it.

## Installation

```toml
[dependencies]
surrealdb_extra = "0.13"

# To also get the enum derive:
# surrealdb_extra = { version = "0.13", features = ["repr"] }
```

| Feature | Default | What it adds |
|---|---|---|
| `table` | yes | The `Table` trait and `#[derive(Table)]` |
| `repr` | no | `#[derive(SurrealValueRepr)]` for integer enums |

## Table

`#[derive(Table)]` gives a struct its table name and a helper for making record IDs.

```rust
use surrealdb_extra::table::Table;

#[derive(Table)]
struct UserProfile;            // table name: "user_profile"

#[derive(Table)]
#[table(name = "people")]
struct Person;                 // table name: "people"

assert_eq!(UserProfile::TABLE_NAME, "user_profile");

let id = Person::create_record_id("tobie");   // people:tobie
```

Without `#[table(name = "...")]`, the name is the struct name in snake_case (`HTTPServer` becomes `http_server`).

## Integer enums (`repr` feature)

`#[derive(SurrealValueRepr)]` works like [`serde_repr`](https://docs.rs/serde_repr), but for SurrealDB's `SurrealValue` trait: each variant is stored as its discriminant. No serde involved.

```rust
use surrealdb_extra::repr::SurrealValueRepr;
use surrealdb_extra::types::{Number, SurrealValue, ToSql, Value};

#[derive(Debug, PartialEq, SurrealValueRepr)]
#[repr(u8)]
enum Status {
    Inactive = 0,
    Active = 1,
    Banned = 7,
}

assert_eq!(Status::Banned.into_value(), Value::Number(Number::Int(7)));
assert_eq!(Status::from_value(Value::Number(Number::Int(1))).unwrap(), Status::Active);

// Use the kind in a schema so the database only accepts valid values:
// DEFINE FIELD status ON user TYPE 0 | 1 | 7
assert_eq!(Status::kind_of().to_sql(), "0 | 1 | 7");
```

- Supports `u8` through `u64`, `i8` through `i64`, `usize` and `isize`. Discriminants can be implicit, negative or const expressions.
- Unknown values are an error, unless you mark a fallback variant with `#[surreal_repr(other)]`.
- `#[surreal_repr(kind = "int")]` reports the type as plain `int` instead of the exact values.

## Types re-export

`surrealdb_extra::types` re-exports `surrealdb-types`, so you don't need `surrealdb` just for the types. They're the same items as `surrealdb::types`, so you can mix the two freely.

When you use SurrealDB's own `#[derive(SurrealValue)]` without depending on `surrealdb`, point it at the re-export:

```rust
use surrealdb_extra::types::SurrealValue;

#[derive(SurrealValue)]
#[surreal(crate = "surrealdb_extra::types")]
struct User {
    name: String,
}
```

Two quirks of SurrealDB's derive (not of this crate):

- It refers to `SurrealValue` without a path, so keep `use surrealdb_extra::types::SurrealValue;` in scope.
- For `#[surreal(untagged)]` enums whose variants use `#[surreal(value = ...)]`, it ignores `crate = ...`. Add `use surrealdb_extra::types as surrealdb_types;` in that module.

## Renamed dependency

The derives read your `Cargo.toml`, so renaming the dependency (`extra = { package = "surrealdb_extra" }`) just works. If you reach them through another crate's re-export instead, point them at it with `#[table(crate = "...")]` and `#[surreal_repr(crate = "...::types")]`.

## Documentation

See [docs.rs/surrealdb_extra](https://docs.rs/surrealdb_extra) for the full API.

## License

[MIT](https://github.com/jakin010/surrealdb_extra/blob/main/LICENSE)

## Note

surrealdb_extra is an independent crate and is not affiliated with or endorsed by SurrealDB or its developers.
