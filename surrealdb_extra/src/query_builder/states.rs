#[derive(Debug)]
pub struct NoTableQuery;
#[derive(Debug)]
pub struct TableQuery(pub String);

#[derive(Debug)]
pub struct NoFieldsQuery;
#[derive(Debug)]
pub struct FieldsQuery(pub Vec<String>);

#[derive(Debug)]
pub struct NoFilterQuery;
#[derive(Debug)]
pub struct FilterQuery;
