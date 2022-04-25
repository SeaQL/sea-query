//! Translating the SQL AST into engine-specific SQL statements.

use crate::*;

#[cfg(feature = "backend-mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-mysql")))]
mod mysql;
#[cfg(feature = "backend-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
mod postgres;
#[cfg(feature = "backend-sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-sqlite")))]
mod sqlite;

#[cfg(feature = "backend-mysql")]
pub use mysql::*;
#[cfg(feature = "backend-postgres")]
pub use postgres::*;
#[cfg(feature = "backend-sqlite")]
pub use sqlite::*;

mod foreign_key_builder;
mod index_builder;
mod query_builder;
mod table_builder;

pub use self::foreign_key_builder::*;
pub use self::index_builder::*;
pub use self::query_builder::*;
pub use self::table_builder::*;

pub trait GenericBuilder: QueryBuilder + SchemaBuilder {}

pub trait SchemaBuilder: TableBuilder + IndexBuilder + ForeignKeyBuilder {}

pub trait QuotedBuilder {
    /// The type of quote the builder uses.
    fn quote(&self) -> char;
}

pub trait EscapeBuilder {
    /// Escape a SQL string literal
    fn escape_string(&self, string: &str) -> String {
        string
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "\\'")
            .replace('\0', "\\0")
            .replace('\x08', "\\b")
            .replace('\x09', "\\t")
            .replace('\x1a', "\\z")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    }

    /// Unescape a SQL string literal
    fn unescape_string(&self, string: &str) -> String {
        let mut escape = false;
        let mut output = String::new();
        for c in string.chars() {
            if !escape && c == '\\' {
                escape = true;
            } else if escape {
                write!(
                    output,
                    "{}",
                    match c {
                        '0' => '\0',
                        'b' => '\x08',
                        't' => '\x09',
                        'z' => '\x1a',
                        'n' => '\n',
                        'r' => '\r',
                        c => c,
                    }
                )
                .unwrap();
                escape = false;
            } else {
                write!(output, "{}", c).unwrap();
            }
        }
        output
    }
}
