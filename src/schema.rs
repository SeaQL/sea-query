//! Schema definition & alternations statements

use crate::{backend::SchemaBuilder, ForeignKeyStatement, IndexStatement, TableStatement};

#[derive(Debug, Clone)]
pub enum SchemaStatement {
    TableStatement(TableStatement),
    IndexStatement(IndexStatement),
    ForeignKeyStatement(ForeignKeyStatement),
}

pub trait SchemaStatementBuilder {
    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn build(&self, schema_builder: &dyn SchemaBuilder) -> String;

    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String;

    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn to_string(&self, schema_builder: &dyn SchemaBuilder) -> String {
        self.build(schema_builder)
    }
}
