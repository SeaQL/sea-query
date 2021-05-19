use crate::impl_schema_statement_builder;

impl_schema_statement_builder!(
    foreign_key_create_statement_builder,
    ForeignKeyCreateStatement
);
impl_schema_statement_builder!(foreign_key_drop_statement_builder, ForeignKeyDropStatement);
