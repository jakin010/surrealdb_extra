#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "table")]
pub mod table;

#[cfg_attr(docsrs, doc(cfg(feature = "query")))]
#[cfg(feature = "query")]
pub mod query;

#[cfg_attr(docsrs, doc(cfg(feature = "query")))]
#[cfg(feature = "query")]
pub use ::paste::item;