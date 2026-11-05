#[derive(surrealdb_extra::repr::SurrealValueRepr)]
#[repr(u8)]
enum E { A, B(u8) }
fn main() {}
