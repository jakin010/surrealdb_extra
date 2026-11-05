#[derive(surrealdb_extra::repr::SurrealValueRepr)]
#[repr(u8)]
enum E { A, #[surreal_repr(other)] B, #[surreal_repr(other)] C }
fn main() {}
