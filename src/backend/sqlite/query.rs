use super::*;

impl QueryBuilder<SqliteQueryBuilder> for SqliteQueryBuilder {
    fn char_length_function(&self) -> &'static str {
        "LENGTH"
    }

    fn prepare_select_lock<'a>(
        &self,
        _select_lock: &LockType,
        _sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(&'a dyn QueryValue<SqliteQueryBuilder>),
    ) {
        // SQLite doesn't supports row locking
    }
}
