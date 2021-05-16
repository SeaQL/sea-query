use super::*;

impl QueryBuilder for SqliteQueryBuilder {
    fn char_length_function(&self) -> &str {
        "LENGTH"
    }
}
