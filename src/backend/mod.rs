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
    fn prepare_iden(&self, iden: &DynIden, sql: &mut dyn SqlWriter) {
        let q = self.quote();
        let byte = [q.1];
        let qq: &str = std::str::from_utf8(&byte).unwrap();

        let string;
        let quoted: &str = match &iden.0 {
            Cow::Borrowed(s) => s,
            Cow::Owned(s) => {
                string = s.replace(qq, qq.repeat(2).as_str());
                &string
            }
        };
        write!(sql, "{}{}{}", q.left(), quoted, q.right()).unwrap();
    }
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
                write!(output, "{c}").unwrap();
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
        Self::UnOper(value)
    }
}

impl From<BinOper> for Oper {
    fn from(value: BinOper) -> Self {
        Self::BinOper(value)
    }
}

impl Oper {
    pub(crate) const fn is_logical(&self) -> bool {
        matches!(
            self,
            Self::UnOper(UnOper::Not) | Self::BinOper(BinOper::And) | Self::BinOper(BinOper::Or)
        )
    }

    pub(crate) const fn is_between(&self) -> bool {
        matches!(
            self,
            Self::BinOper(BinOper::Between) | Self::BinOper(BinOper::NotBetween)
        )
    }

    pub(crate) const fn is_like(&self) -> bool {
        matches!(
            self,
            Self::BinOper(BinOper::Like) | Self::BinOper(BinOper::NotLike)
        )
    }

    pub(crate) const fn is_in(&self) -> bool {
        matches!(
            self,
            Self::BinOper(BinOper::In) | Self::BinOper(BinOper::NotIn)
        )
    }

    pub(crate) const fn is_is(&self) -> bool {
        matches!(
            self,
            Self::BinOper(BinOper::Is) | Self::BinOper(BinOper::IsNot)
        )
    }

    pub(crate) const fn is_shift(&self) -> bool {
        matches!(
            self,
            Self::BinOper(BinOper::LShift) | Self::BinOper(BinOper::RShift)
        )
    }

    pub(crate) const fn is_arithmetic(&self) -> bool {
        match self {
            Self::BinOper(b) => {
                matches!(
                    b,
                    BinOper::Mul | BinOper::Div | BinOper::Mod | BinOper::Add | BinOper::Sub
                )
            }
            _ => false,
        }
    }

    pub(crate) const fn is_comparison(&self) -> bool {
        match self {
            Self::BinOper(b) => {
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
