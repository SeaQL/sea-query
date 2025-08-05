use crate::*;

pub trait TableRefBuilder: QuotedBuilder {
    /// Translate [`TableRef`] that without values into SQL statement.
    fn prepare_table_ref_iden(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        let (table_name, alias) = match table_ref {
            TableRef::Table(table_name, alias) => (table_name, alias),
            TableRef::SubQuery(_, _)
            | TableRef::ValuesList(_, _)
            | TableRef::FunctionCall(_, _) => panic!("TableRef with values is not support"),
        };
        self.prepare_table_name(table_name, sql);
        if let Some(alias) = alias {
            write!(sql, " AS ").unwrap();
            self.prepare_iden(alias, sql);
        }
    }

    /// Translate [`TableName`] into an SQL statement.
    fn prepare_table_name(&self, table_name: &TableName, sql: &mut dyn SqlWriter) {
        let TableName(schema_name, table) = table_name;
        if let Some(SchemaName(database_name, schema)) = schema_name {
            if let Some(DatabaseName(database)) = database_name {
                self.prepare_iden(database, sql);
                write!(sql, ".").unwrap();
            }
            self.prepare_iden(schema, sql);
            write!(sql, ".").unwrap();
        }
        self.prepare_iden(table, sql);
    }
}
