use crate::*;

pub trait TableRefBuilder: QuotedBuilder {
    /// Translate [`TableRef`] that without values into SQL statement.
    fn prepare_table_ref_iden(&self, table_ref: &TableRef, sql: &mut impl SqlWriter) {
        let (table_name, alias) = match table_ref {
            TableRef::Table(table_name, alias) => (table_name, alias),
            TableRef::SubQuery(_, _)
            | TableRef::ValuesList(_, _)
            | TableRef::FunctionCall(_, _) => panic!("TableRef with values is not support"),
        };
        self.prepare_table_name(table_name, sql);
        if let Some(alias) = alias {
            sql.write_str(" AS ").unwrap();
            self.prepare_iden(alias, sql);
        }
    }

    /// Translate [`TableName`] into an SQL statement.
    fn prepare_table_name(&self, table_name: &TableName, sql: &mut impl SqlWriter) {
        let TableName(schema_name, table) = table_name;
        if let Some(schema_name) = schema_name {
            self.prepare_schema_name(schema_name, sql);
            sql.write_str(".").unwrap();
        }
        self.prepare_iden(table, sql);
    }

    /// Translate [`SchemaName`] into an SQL statement.
    fn prepare_schema_name(&self, schema_name: &SchemaName, sql: &mut impl SqlWriter) {
        let SchemaName(database_name, schema) = schema_name;
        if let Some(DatabaseName(database)) = database_name {
            self.prepare_iden(database, sql);
            write!(sql, ".").unwrap();
        }
        self.prepare_iden(schema, sql);
    }
}
