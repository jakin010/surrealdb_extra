use strum::Display;

#[derive(Debug, Display)]
pub enum RelationalOperator {
    #[strum(serialize = "=")]
    Equal,
    #[strum(serialize = "!=")]
    NotEqual,
    #[strum(serialize = "<")]
    LessThan,
    #[strum(serialize = "<=")]
    LessThanOrEqual,
    #[strum(serialize = ">")]
    MoreThan,
    #[strum(serialize = ">=")]
    MoreThanOrEqual,
}

#[derive(Debug, Display, Clone, PartialEq)]
pub enum LogicalOperator {
    #[strum(serialize = "OR")]
    Or,
    #[strum(serialize = "AND")]
    And,
    #[strum(serialize = "")]
    End,
}

#[derive(Debug)]
pub(crate) struct Filter {
    pub key: String,
    pub relational_operator: RelationalOperator,
    pub value: String,
    pub logical_operator: LogicalOperator,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        format!("{} {} ${} {}", &self.key, &self.relational_operator, &self.key, &self.logical_operator)
    }
}
