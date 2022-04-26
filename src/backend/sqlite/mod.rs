pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Sqlite query builder.
#[derive(Debug)]
pub struct SqliteQueryBuilder;

impl Default for SqliteQueryBuilder {
    fn default() -> Self {
        Self
    }
}

impl GenericBuilder for SqliteQueryBuilder {}

impl SchemaBuilder for SqliteQueryBuilder {}

impl QuotedBuilder for SqliteQueryBuilder {
    fn quote(&self) -> char {
        '"'
    }
}

impl EscapeBuilder for SqliteQueryBuilder {
    fn escape_string(&self, string: &str) -> String {
        string
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "''")
            .replace('\0', "\\0")
            .replace('\x08', "\\b")
            .replace('\x09', "\\t")
            .replace('\x1a', "\\z")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    }

    fn unescape_string(&self, string: &str) -> String {
        let mut escape = false;
        let mut output = String::new();
        for c in string.chars() {
            if !escape && (c == '\\' || c == '\'') {
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
