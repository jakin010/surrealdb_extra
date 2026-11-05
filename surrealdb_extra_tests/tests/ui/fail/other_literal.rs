use surrealdb_extra::repr::SurrealValueRepr;
#[derive(SurrealValueRepr)]
#[repr(i32)]
#[surreal_repr(kind = "literal")]
enum OtherLiteral { A = 1, #[surreal_repr(other)] U = 0 }
fn main() {}
