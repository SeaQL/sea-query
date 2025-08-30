//! Schema definition & alternations statements

use crate::{ForeignKeyStatement, IndexStatement, TableStatement, backend::SchemaBuilder};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SchemaStatement {
    TableStatement(TableStatement),
    IndexStatement(IndexStatement),
    ForeignKeyStatement(ForeignKeyStatement),
}

pub trait SchemaStatementBuilder {
    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String;

    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn build_any(&self, schema_builder: &impl SchemaBuilder) -> String;

    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn to_string<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        self.build(schema_builder)
    }
}
