use std::rc::Rc;
use crate::{backend::SchemaBuilder, SchemaStatementBuilder, types::*, prepare::*};
use super::common::*;

/// Create an index for an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let index = Index::create()
///     .name("idx-glyph-aspect")
///     .table(Glyph::Table)
///     .col(Glyph::Aspect)
///     .to_owned();
///
/// assert_eq!(
///     index.to_string(MysqlQueryBuilder),
///     r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect`)"#
/// );
/// assert_eq!(
///     index.to_string(PostgresQueryBuilder),
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect")"#
/// );
/// assert_eq!(
///     index.to_string(SqliteQueryBuilder),
///     r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect`)"#
/// );
/// ```
/// Index with prefix
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let index = Index::create()
///     .name("idx-glyph-aspect")
///     .table(Glyph::Table)
///     .col((Glyph::Aspect, 128))
///     .to_owned();
///
/// assert_eq!(
///     index.to_string(MysqlQueryBuilder),
///     r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect` (128))"#
/// );
/// assert_eq!(
///     index.to_string(PostgresQueryBuilder),
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect" (128))"#
/// );
/// assert_eq!(
///     index.to_string(SqliteQueryBuilder),
///     r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect`)"#
/// );
/// ```
/// Index with order
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let index = Index::create()
///     .name("idx-glyph-aspect")
///     .table(Glyph::Table)
///     .col((Glyph::Aspect, IndexOrder::Desc))
///     .to_owned();
///
/// assert_eq!(
///     index.to_string(MysqlQueryBuilder),
///     r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect` DESC)"#
/// );
/// assert_eq!(
///     index.to_string(PostgresQueryBuilder),
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect" DESC)"#
/// );
/// assert_eq!(
///     index.to_string(SqliteQueryBuilder),
///     r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect` DESC)"#
/// );
/// ```
/// Index with prefix and order
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let index = Index::create()
///     .name("idx-glyph-aspect")
///     .table(Glyph::Table)
///     .col((Glyph::Aspect, 64, IndexOrder::Asc))
///     .to_owned();
///
/// assert_eq!(
///     index.to_string(MysqlQueryBuilder),
///     r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect` (64) ASC)"#
/// );
/// assert_eq!(
///     index.to_string(PostgresQueryBuilder),
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect" (64) ASC)"#
/// );
/// assert_eq!(
///     index.to_string(SqliteQueryBuilder),
///     r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect` ASC)"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct IndexCreateStatement {
    pub(crate) table: Option<Rc<dyn Iden>>,
    pub(crate) index: TableIndex,
    pub(crate) primary: bool,
    pub(crate) unique: bool,
    pub(crate) index_type: Option<IndexType>,
}

/// Specification of a table index
#[derive(Debug, Clone)]
pub enum IndexType {
    BTree,
    FullText,
    Hash,
    Custom(Rc<dyn Iden>),
}

impl Default for IndexCreateStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexCreateStatement {
    /// Construct a new [`IndexCreateStatement`]
    pub fn new() -> Self {
        Self {
            table: None,
            index: Default::default(),
            primary: false,
            unique: false,
            index_type: None,
        }
    }

    /// Set index name
    pub fn name(mut self, name: &str) -> Self {
        self.index.name(name);
        self
    }

    /// Set target table
    pub fn table<T: 'static>(mut self, table: T) -> Self
        where T: Iden {
        self.table = Some(Rc::new(table));
        self
    }

    /// Add index column
    pub fn col<C: 'static>(mut self, col: C) -> Self
        where C: IntoIndexColumn {
        self.index.col(col.into_index_column());
        self
    }

    /// Set index as primary
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }

    /// Set index as unique
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    /// Set index as full text.
    /// On MySQL, this is `FULLTEXT`.
    /// On PgSQL, this is `GIN`.
    pub fn full_text(self) -> Self {
        self.index_type(IndexType::FullText)
    }

    /// Set index type. Not available on Sqlite.
    pub fn index_type(mut self, index_type: IndexType) -> Self {
        self.index_type = Some(index_type);
        self
    }
}

impl SchemaStatementBuilder for IndexCreateStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_index_create_statement(self, &mut sql);
        sql.result()
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_index_create_statement(self, &mut sql);
        sql.result()
    }
}
