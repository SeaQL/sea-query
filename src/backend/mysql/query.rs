use super::*;

impl QueryBuilder for MysqlQueryBuilder {
    fn prepare_returning(
        &self,
        _returning: &[SelectExpr],
        _sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
    }
}
