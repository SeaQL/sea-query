//! Schema definition & alternations statements

use crate::{
    ConstraintStatement, ForeignKeyStatement, IndexStatement, TableStatement,
    backend::SchemaBuilder,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SchemaStatement {
    TableStatement(TableStatement),
    IndexStatement(IndexStatement),
    ForeignKeyStatement(ForeignKeyStatement),
    ConstraintStatement(ConstraintStatement),
}

pub trait SchemaStatementBuilder {
    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn build<T>(&self, schema_builder: T) -> String
    where
        T: SchemaBuilder;

    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn to_string<T>(&self, schema_builder: T) -> String
    where
        T: SchemaBuilder,
    {
        self.build(schema_builder)
    }
}
