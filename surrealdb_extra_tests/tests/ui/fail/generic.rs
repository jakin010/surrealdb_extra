#[derive(surrealdb_extra::repr::SurrealValueRepr)]
#[repr(u8)]
enum E<T> { A, B, #[doc(hidden)] _P = 2 }
fn main() {}
