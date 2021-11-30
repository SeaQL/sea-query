use super::*;

impl QueryBuilder for SqliteQueryBuilder {
    fn char_length_function(&self) -> &str {
        "LENGTH"
    }

    fn prepare_select_lock(
        &self,
        _select_lock: &LockType,
        _sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
        // SQLite doesn't supports row locking
    }
}
