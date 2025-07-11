use crate::*;

pub trait TableRefBuilder: QuotedBuilder {
    /// Translate [`TableRef`] that without values into SQL statement.
    fn prepare_table_ref_iden(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            TableRef::Table(iden) => {
                self.prepare_dyn_iden(iden, sql);
            }
            TableRef::SchemaTable(schema, table) => {
                self.prepare_dyn_iden(schema, sql);
                write!(sql, ".").unwrap();
                self.prepare_dyn_iden(table, sql);
            }
            TableRef::DatabaseSchemaTable(database, schema, table) => {
                self.prepare_dyn_iden(database, sql);
                write!(sql, ".").unwrap();
                self.prepare_dyn_iden(schema, sql);
                write!(sql, ".").unwrap();
                self.prepare_dyn_iden(table, sql);
            }
            TableRef::TableAlias(iden, alias) => {
                self.prepare_dyn_iden(iden, sql);
                write!(sql, " AS ").unwrap();
                self.prepare_dyn_iden(alias, sql);
            }
            TableRef::SchemaTableAlias(schema, table, alias) => {
                self.prepare_dyn_iden(schema, sql);
                write!(sql, ".").unwrap();
                self.prepare_dyn_iden(table, sql);
                write!(sql, " AS ").unwrap();
                self.prepare_dyn_iden(alias, sql);
            }
            TableRef::DatabaseSchemaTableAlias(database, schema, table, alias) => {
                self.prepare_dyn_iden(database, sql);
                write!(sql, ".").unwrap();
                self.prepare_dyn_iden(schema, sql);
                write!(sql, ".").unwrap();
                self.prepare_dyn_iden(table, sql);
                write!(sql, " AS ").unwrap();
                self.prepare_dyn_iden(alias, sql);
            }
            TableRef::SubQuery(_, _)
            | TableRef::ValuesList(_, _)
            | TableRef::FunctionCall(_, _) => {
                panic!("TableRef with values is not support")
            }
        }
    }
}
