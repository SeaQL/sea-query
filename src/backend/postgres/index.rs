use super::*;

impl IndexBuilder for PostgresQueryBuilder {
    fn prepare_index_create_statement(&mut self, create: &IndexCreateStatement, sql: &mut dyn FmtWrite) {
        write!(sql, "CREATE INDEX ").unwrap();
        if let Some(name) = &create.index.name {
            write!(sql, "\"{}\" ", name).unwrap();
        }

        write!(sql, "ON ").unwrap();
        if let Some(table) = &create.table {
            table.prepare(sql, '"');
        }

        write!(sql, " (").unwrap();
        create.index.columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql, '"');
            false
        });
        write!(sql, ")").unwrap();
    }

    fn prepare_index_drop_statement(&mut self, drop: &IndexDropStatement, sql: &mut dyn FmtWrite) {
        write!(sql, "DROP INDEX ").unwrap();
        if let Some(name) = &drop.index.name {
            write!(sql, "\"{}\"", name).unwrap();
        }
    }
}