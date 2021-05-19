use crate::{impl_conditional_statement, impl_ordered_statement, impl_query_statement_builder};

impl_query_statement_builder!(select_statement_builder, SelectStatement);
impl_query_statement_builder!(insert_statement_builder, InsertStatement);
impl_query_statement_builder!(update_statement_builder, UpdateStatement);
impl_query_statement_builder!(delete_statement_builder, DeleteStatement);

impl_ordered_statement!(select_statement_ordered, SelectStatement);
impl_ordered_statement!(update_statement_ordered, UpdateStatement);
impl_ordered_statement!(delete_statement_ordered, DeleteStatement);

impl_conditional_statement!(select_statement_conditional, SelectStatement);
impl_conditional_statement!(update_statement_conditional, UpdateStatement);
impl_conditional_statement!(delete_statement_conditional, DeleteStatement);
