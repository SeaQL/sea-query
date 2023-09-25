pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

pub use table::{DefaultTypeMapper, ExactTypeMapper};

/// Sqlite query builder.
#[derive(Default, Debug)]
pub struct SqliteQueryBuilder;

/// An SQLite query builder with custom column typing logic.
#[derive(Default, Debug)]
pub struct SqliteTypedQueryBuilder<TTyping: StaticTypeMapper = DefaultTypeMapper>(
    pub std::marker::PhantomData<TTyping>,
);

const QUOTE: Quote = Quote(b'"', b'"');

/// This macro will repeat the given implementation for both [`SqliteQueryBuilder`] and [`SqliteTypedQueryBuilder`].
/// Why a macro? To prevent future errors if ever we want to add a blanket implementation elsewhere on sea-query types.
macro_rules! sqlite_impl {
    ($($trait:ty { $($toks:tt)* }),*) => {
        $(impl<TTyping: StaticTypeMapper> $trait for SqliteTypedQueryBuilder<TTyping> {
            $($toks)*
        }
        impl $trait for SqliteQueryBuilder {
            $($toks)*
        })*
    }
}

pub(crate) use sqlite_impl;

sqlite_impl! {
    GenericBuilder {},
    SchemaBuilder {},
    QuotedBuilder {
        fn quote(&self) -> Quote {
            QUOTE
        }
    },
    EscapeBuilder {
        fn escape_string(&self, string: &str) -> String {
            string.replace('\'', "''")
        }

        fn unescape_string(&self, string: &str) -> String {
            string.replace("''", "'")
        }
    },
    TableRefBuilder {},
    PrecedenceDecider {
        fn inner_expr_well_known_greater_precedence(
            &self,
            inner: &SimpleExpr,
            outer_oper: &Oper,
        ) -> bool {
            common_inner_expr_well_known_greater_precedence(inner, outer_oper)
        }
    },
    OperLeftAssocDecider {
        fn well_known_left_associative(&self, op: &BinOper) -> bool {
            common_well_known_left_associative(op)
        }
    }
}

pub trait StaticTypeMapper {
    fn prepare_column_type(column_type: &ColumnType, sql: &mut dyn SqlWriter);
}

pub trait SqliteBuilderVariant {
    type TypeMapper: StaticTypeMapper;
}

impl SqliteBuilderVariant for SqliteQueryBuilder {
    type TypeMapper = DefaultTypeMapper;
}

impl<TTyping: StaticTypeMapper> SqliteBuilderVariant for SqliteTypedQueryBuilder<TTyping> {
    type TypeMapper = TTyping;
}
