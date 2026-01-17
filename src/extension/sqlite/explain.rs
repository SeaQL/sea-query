use crate::SqlWriter;

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct SqliteExplainOptions {
    pub(crate) query_plan: bool,
}

impl SqliteExplainOptions {
    pub(crate) fn write_to(&self, sql: &mut impl SqlWriter) {
        if self.query_plan {
            sql.write_str(" QUERY PLAN").unwrap();
        }
    }
}
