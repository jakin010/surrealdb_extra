#[cfg(feature = "derive")]
mod r#trait;
#[cfg(feature = "derive")]
pub use r#trait::Table;

pub mod query_builder;

