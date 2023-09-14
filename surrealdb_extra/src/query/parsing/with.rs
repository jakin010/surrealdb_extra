use surrealdb::sql::With;

#[derive(Debug, Clone)]
pub struct ExtraWith(pub With);

impl From<With> for ExtraWith {
    fn from(value: With) -> Self {
        Self(value)
    }
}

impl From<Vec<String>> for ExtraWith {
    fn from(value: Vec<String>) -> Self {

        let with = With::Index(value);

        Self(with)
    }
}
