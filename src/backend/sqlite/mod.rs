pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Sqlite query builder.
#[derive(Default, Debug)]
pub struct SqliteQueryBuilder;

const QUOTE: Quote = Quote(b'"', b'"');

impl GenericBuilder for SqliteQueryBuilder {}

impl SchemaBuilder for SqliteQueryBuilder {}

impl QuotedBuilder for SqliteQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for SqliteQueryBuilder {
    fn write_escaped(&self, buffer: &mut impl Write, string: &str) {
        for char in string.chars() {
            match char {
                '\'' => buffer.write_str("''"),
                c => buffer.write_char(c),
            }
            .unwrap()
        }
    }

    fn unescape_string(&self, string: &str) -> String {
        string.replace("''", "'")
    }
}

impl TableRefBuilder for SqliteQueryBuilder {
    // SQLite does not support a fully qualified db.schema.table reference - fail if db is provided
    fn prepare_schema_name(&self, schema_name: &SchemaName, sql: &mut impl SqlWriter) {
        match schema_name {
            SchemaName(None, schema) => self.prepare_iden(schema, sql),
            _ => panic!("Sqlite does not support fully qualified db.schema.table syntax"),
        }
    }
}

impl PrecedenceDecider for SqliteQueryBuilder {
    fn inner_expr_well_known_greater_precedence(&self, inner: &Expr, outer_oper: &Oper) -> bool {
        common_inner_expr_well_known_greater_precedence(inner, outer_oper)
    }
}

impl OperLeftAssocDecider for SqliteQueryBuilder {
    fn well_known_left_associative(&self, op: &BinOper) -> bool {
        common_well_known_left_associative(op)
    }
}
