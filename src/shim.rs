#[macro_export]
macro_rules! impl_schema_statement_builder {
    ( $mod_name: ident, $struct_name: ident ) => {
        mod $mod_name {

            use crate::{$struct_name, SchemaBuilder, SchemaStatementBuilder};

            impl $struct_name {
                pub fn to_string<T: SchemaBuilder>(&self) -> String {
                    <Self as SchemaStatementBuilder>::to_string::<T>(self)
                }

                pub fn build<T: SchemaBuilder>(&self) -> String {
                    <Self as SchemaStatementBuilder>::build::<T>(self)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_query_statement_builder {
    ( $mod_name: ident, $struct_name: ident ) => {
        mod $mod_name {

            use crate::{$struct_name, QueryBuilder, QueryStatementBuilder, QueryValue};

            impl<'a, DB> $struct_name<'a, DB>
            where
                DB: QueryBuilder<DB> + Default,
            {
                pub fn to_string(&'a self) -> String
                where
                    DB: Clone,
                {
                    <Self as QueryStatementBuilder<DB>>::to_string(self)
                }

                pub fn build(&'a self) -> (String, Vec<&dyn QueryValue<DB>>) {
                    <Self as QueryStatementBuilder<DB>>::build(self)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_conditional_statement {
    ( $mod_name: ident, $struct_name: ident ) => {
        #[allow(deprecated)]
        mod $mod_name {

            use crate::{ConditionalStatement, SimpleExpr, IntoCondition, $struct_name};

            impl<'a, DB> $struct_name<'a, DB> {
                pub fn and_where(&mut self, other: SimpleExpr<'a, DB>) -> &mut Self {
                    <Self as ConditionalStatement<DB>>::and_where(self, other)
                }

                pub fn and_where_option(&mut self, other: Option<SimpleExpr<'a, DB>>) -> &mut Self {
                    <Self as ConditionalStatement<DB>>::and_where_option(self, other)
                }

                #[deprecated(
                    since = "0.12.0",
                    note = "Please use [`ConditionalStatement::cond_where`]. Calling `or_where` after `and_where` will panic."
                )]
                pub fn or_where(&mut self, other: SimpleExpr<'a, DB>) -> &mut Self {
                    <Self as ConditionalStatement<DB>>::or_where(self, other)
                }

                pub fn cond_where<C>(&mut self, condition: C) -> &mut Self where C: IntoCondition<'a, DB> {
                    <Self as ConditionalStatement<DB>>::cond_where(self, condition)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_ordered_statement {
    ( $mod_name: ident, $struct_name: ident ) => {
        #[allow(deprecated)]
        mod $mod_name {

            use crate::{OrderedStatement, IntoColumnRef, IntoIden, Order, SimpleExpr, $struct_name};

            impl<'a, DB> $struct_name<'a, DB> {
                pub fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self
                    where T: IntoColumnRef {
                    <Self as OrderedStatement<DB>>::order_by(self, col, order)
                }

                #[deprecated(
                    since = "0.9.0",
                    note = "Please use the [`OrderedStatement::order_by`] with a tuple as [`ColumnRef`]"
                )]
                pub fn order_by_tbl<T, C>
                    (&mut self, table: T, col: C, order: Order) -> &mut Self
                    where T: IntoIden, C: IntoIden {
                    <Self as OrderedStatement<DB>>::order_by_tbl(self, table, col, order)
                }

                pub fn order_by_expr(&mut self, expr: SimpleExpr<'a, DB>, order: Order) -> &mut Self {
                    <Self as OrderedStatement<DB>>::order_by_expr(self, expr, order)
                }

                pub fn order_by_customs<T>(&mut self, cols: Vec<(T, Order)>) -> &mut Self
                    where T: ToString {
                    <Self as OrderedStatement<DB>>::order_by_customs(self, cols)
                }

                pub fn order_by_columns<T>(&mut self, cols: Vec<(T, Order)>) -> &mut Self
                    where T: IntoColumnRef {
                    <Self as OrderedStatement<DB>>::order_by_columns(self, cols)
                }

                #[deprecated(
                    since = "0.9.0",
                    note = "Please use the [`OrderedStatement::order_by_columns`] with a tuple as [`ColumnRef`]"
                )]
                pub fn order_by_table_columns<T, C>
                    (&mut self, cols: Vec<(T, C, Order)>) -> &mut Self
                    where T: IntoIden, C: IntoIden {
                    <Self as OrderedStatement<DB>>::order_by_table_columns(self, cols)
                }
            }
        }
    }
}
