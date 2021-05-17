#[macro_export]
macro_rules! impl_schema_statement_builder {
    ( $mod_name: ident, $struct_name: ident ) => {
        mod $mod_name {

            use crate::{SchemaBuilder, SchemaStatementBuilder, $struct_name};

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
    }
}

#[macro_export]
macro_rules! impl_query_statement_builder {
    ( $mod_name: ident, $struct_name: ident ) => {
        mod $mod_name {

            use crate::{QueryBuilder, QueryStatementBuilder, Values, $struct_name};

            impl $struct_name {

                pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
                    <Self as QueryStatementBuilder>::to_string(self, query_builder)
                }

                pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values) {
                    <Self as QueryStatementBuilder>::build(self, query_builder)
                }

                pub fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values) {
                    <Self as QueryStatementBuilder>::build_any(self, query_builder)
                }
            }
        }
    }
}