use chrono::{DateTime, Utc};
use surrealdb::sql::Version;

pub struct ExtraVersion(pub(crate) Version);

impl From<Version> for ExtraVersion {
    fn from(value: Version) -> Self {
        Self(value)
    }
}

impl From<DateTime<Utc>> for ExtraVersion {
    fn from(value: DateTime<Utc>) -> Self {
        Self(Version(value.into()))
    }
}
