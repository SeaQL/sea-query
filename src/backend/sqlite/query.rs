use super::*;

impl QueryBuilder for SqliteQueryBuilder {
    fn char_length_function(&self) -> &str {
        "LENGTH"
    }

    fn prepare_returning(
        &self,
        returning: &[SelectExpr],
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        if !returning.is_empty() {
            write!(sql, " RETURNING ").unwrap();
            returning.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap()
                }
                self.prepare_select_expr(expr, sql, collector);
                false
            });
        }
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
