use crate::impl_query_statement_builder;

impl_query_statement_builder!(select_statement, SelectStatement);
impl_query_statement_builder!(insert_statement, InsertStatement);
impl_query_statement_builder!(update_statement, UpdateStatement);
impl_query_statement_builder!(delete_statement, DeleteStatement);