use super::*;

/// All schema definition & operation statements
#[derive(Debug, Clone)]
pub enum SchemaStatement {
    TableStatement(TableStatement),
    IndexStatement(IndexStatement),
    ForeignKeyStatement(ForeignKeyStatement),
}