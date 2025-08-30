//! Translating the SQL AST into engine-specific SQL statements.

use crate::*;
use std::borrow::Cow;

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
mod table_ref_builder;

pub use self::foreign_key_builder::*;
pub use self::index_builder::*;
pub use self::query_builder::*;
pub use self::table_builder::*;
pub use self::table_ref_builder::*;

pub trait GenericBuilder: QueryBuilder + SchemaBuilder {}

pub trait SchemaBuilder: TableBuilder + IndexBuilder + ForeignKeyBuilder {}

pub trait QuotedBuilder {
    /// The type of quote the builder uses.
    fn quote(&self) -> Quote;

    /// To prepare iden and write to SQL.
    fn prepare_iden(&self, iden: &DynIden, sql: &mut (impl SqlWriter + ?Sized)) {
        let q = self.quote();
        let qq = q.1 as char;

        sql.write_char(q.left()).unwrap();
        match &iden.0 {
            Cow::Borrowed(s) => sql.write_str(s).unwrap(),
            Cow::Owned(s) => {
                for char in s.chars() {
                    if char == qq {
                        sql.write_char(char).unwrap()
                    }
                    sql.write_char(char).unwrap()
                }
            }
        };
        sql.write_char(q.right()).unwrap();
    }
}

pub trait EscapeBuilder {
    /// Return if string literal needs to be escaped
    fn need_escape(&self, s: &str) -> bool {
        s.chars().any(|c| {
            matches!(
                c,
                '\r' | '\n' | '\x1a' | '\x09' | '\x08' | '\0' | '\'' | '"' | '\\'
            )
        })
    }

    /// Escape a SQL string literal
    fn escape_string(&self, string: &str) -> String {
        let mut escaped = String::with_capacity(string.len() + 8);
        self.write_escaped(&mut escaped, string);
        escaped
    }

    fn write_escaped(&self, buffer: &mut (impl Write + ?Sized), string: &str) {
        for c in string.chars() {
            match c {
                '\\' => buffer.write_str("\\\\"),
                '"' => buffer.write_str("\\\""),
                '\'' => buffer.write_str("\\'"),
                '\0' => buffer.write_str("\\0"),
                '\x08' => buffer.write_str("\\b"),
                '\x09' => buffer.write_str("\\t"),
                '\x1a' => buffer.write_str("\\z"),
                '\n' => buffer.write_str("\\n"),
                '\r' => buffer.write_str("\\r"),
                _ => buffer.write_char(c),
            }
            .unwrap()
        }
    }

    /// Unescape a SQL string literal
    fn unescape_string(&self, string: &str) -> String {
        let mut escape = false;
        let mut output = String::new();
        for c in string.chars() {
            if !escape && c == '\\' {
                escape = true;
            } else if escape {
                output
                    .write_char(match c {
                        '0' => '\0',
                        'b' => '\x08',
                        't' => '\x09',
                        'z' => '\x1a',
                        'n' => '\n',
                        'r' => '\r',
                        c => c,
                    })
                    .unwrap();
                escape = false;
            } else {
                output.write_char(c).unwrap();
            }
        }
        output
    }
}

pub trait PrecedenceDecider {
    // This method decides which precedence relations should lead to dropped parentheses.
    // There will be more fine grained precedence relations than the ones represented here,
    // but dropping parentheses due to these relations can be confusing for readers.
    fn inner_expr_well_known_greater_precedence(&self, inner: &Expr, outer_oper: &Oper) -> bool;
}

pub trait OperLeftAssocDecider {
    // This method decides if the left associativity of an operator should lead to dropped parentheses.
    // Not all known left associative operators are necessarily included here,
    // as dropping them may in some cases be confusing to readers.
    fn well_known_left_associative(&self, op: &BinOper) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum Oper {
    UnOper(UnOper),
    BinOper(BinOper),
}

impl From<UnOper> for Oper {
    fn from(value: UnOper) -> Self {
        Oper::UnOper(value)
    }
}

impl From<BinOper> for Oper {
    fn from(value: BinOper) -> Self {
        Oper::BinOper(value)
    }
}

impl Oper {
    pub(crate) fn is_logical(&self) -> bool {
        matches!(
            self,
            Oper::UnOper(UnOper::Not) | Oper::BinOper(BinOper::And) | Oper::BinOper(BinOper::Or)
        )
    }

    pub(crate) fn is_between(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Between) | Oper::BinOper(BinOper::NotBetween)
        )
    }

    pub(crate) fn is_like(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Like) | Oper::BinOper(BinOper::NotLike)
        )
    }

    pub(crate) fn is_in(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::In) | Oper::BinOper(BinOper::NotIn)
        )
    }

    pub(crate) fn is_is(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Is) | Oper::BinOper(BinOper::IsNot)
        )
    }

    pub(crate) fn is_shift(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::LShift) | Oper::BinOper(BinOper::RShift)
        )
    }

    pub(crate) fn is_arithmetic(&self) -> bool {
        match self {
            Oper::BinOper(b) => {
                matches!(
                    b,
                    BinOper::Mul | BinOper::Div | BinOper::Mod | BinOper::Add | BinOper::Sub
                )
            }
            _ => false,
        }
    }

    pub(crate) fn is_comparison(&self) -> bool {
        match self {
            Oper::BinOper(b) => {
                matches!(
                    b,
                    BinOper::SmallerThan
                        | BinOper::SmallerThanOrEqual
                        | BinOper::Equal
                        | BinOper::GreaterThanOrEqual
                        | BinOper::GreaterThan
                        | BinOper::NotEqual
                )
            }
            _ => false,
        }
    }
}
