//! Translating the SQL AST into engine-specific SQL statements.

use crate::*;

#[cfg(feature = "backend-mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-mysql")))]
mod mysql;
#[cfg(feature = "backend-oracle")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-oracle")))]
mod oracle;
#[cfg(feature = "backend-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
mod postgres;
#[cfg(feature = "backend-sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-sqlite")))]
mod sqlite;

#[cfg(feature = "backend-mysql")]
pub use mysql::*;
#[cfg(feature = "backend-oracle")]
pub use oracle::*;
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
