#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "surreal_value_json")]
pub mod surreal_value_json;
