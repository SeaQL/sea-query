#[macro_export]
macro_rules! impl_schema_statement_builder {
    ( $mod_name: ident, $struct_name: ident ) => {
        mod $mod_name {

            use crate::{$struct_name, SchemaBuilder, SchemaStatementBuilder};

            impl $struct_name {
                pub fn to_string<T: SchemaBuilder>(&self, schema_builder: T) -> String {
                    <Self as SchemaStatementBuilder>::to_string(self, schema_builder)
                }

                pub fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
                    <Self as SchemaStatementBuilder>::build(self, schema_builder)
                }

                pub fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
                    <Self as SchemaStatementBuilder>::build_any(self, schema_builder)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_query_statement_builder {
    ( $mod_name: ident, $struct_name: ident ) => {
        mod $mod_name {

            use $crate::{
                $struct_name, QueryBuilder, QueryStatementBuilder, QueryStatementWriter, Values,
            };

            impl $struct_name {
                pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
                    <Self as QueryStatementWriter>::to_string(self, query_builder)
                }

                pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values) {
                    <Self as QueryStatementWriter>::build(self, query_builder)
                }

                pub fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values) {
                    <Self as QueryStatementBuilder>::build_any(self, query_builder)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_conditional_statement {
    ( $mod_name: ident, $struct_name: ident ) => {
        mod $mod_name {

            use $crate::{$struct_name, ConditionalStatement, IntoCondition, SimpleExpr};

            impl $struct_name {
                pub fn and_where(&mut self, other: SimpleExpr) -> &mut Self {
                    <Self as ConditionalStatement>::and_where(self, other)
                }

                pub fn and_where_option(&mut self, other: Option<SimpleExpr>) -> &mut Self {
                    <Self as ConditionalStatement>::and_where_option(self, other)
                }

                pub fn cond_where<C>(&mut self, condition: C) -> &mut Self
                where
                    C: IntoCondition,
                {
                    <Self as ConditionalStatement>::cond_where(self, condition)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ordered_statement {
    ( $mod_name: ident, $struct_name: ident ) => {
        mod $mod_name {

            use $crate::{$struct_name, IntoColumnRef, Order, OrderedStatement, SimpleExpr};

            impl $struct_name {
                pub fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self
                where
                    T: IntoColumnRef,
                {
                    <Self as OrderedStatement>::order_by(self, col, order)
                }

                pub fn order_by_expr(&mut self, expr: SimpleExpr, order: Order) -> &mut Self {
                    <Self as OrderedStatement>::order_by_expr(self, expr, order)
                }

                pub fn order_by_customs<T>(&mut self, cols: Vec<(T, Order)>) -> &mut Self
                where
                    T: ToString,
                {
                    <Self as OrderedStatement>::order_by_customs(self, cols)
                }

                pub fn order_by_columns<T>(&mut self, cols: Vec<(T, Order)>) -> &mut Self
                where
                    T: IntoColumnRef,
                {
                    <Self as OrderedStatement>::order_by_columns(self, cols)
                }
            }
        }
    };
}
