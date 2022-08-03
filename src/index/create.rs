use super::common::*;
use crate::query::IntoCondition;
use crate::{backend::SchemaBuilder, prepare::*, types::*, SchemaStatementBuilder};
use crate::{ConditionHolder, ConditionalStatement};

/// Create an index for an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
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
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect")"#
/// );
/// ```
/// Create index if not exists
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let index = Index::create()
///     .if_not_exists()
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
///     r#"CREATE INDEX IF NOT EXISTS "idx-glyph-aspect" ON "glyph" ("aspect")"#
/// );
/// assert_eq!(
///     index.to_string(SqliteQueryBuilder),
///     r#"CREATE INDEX IF NOT EXISTS "idx-glyph-aspect" ON "glyph" ("aspect")"#
/// );
/// ```
/// Index with prefix
/// ```
/// use sea_query::{tests_cfg::*, *};
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
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect")"#
/// );
/// ```
/// Index with order
/// ```
/// use sea_query::{tests_cfg::*, *};
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
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect" DESC)"#
/// );
/// ```
/// Index with prefix and order
/// ```
/// use sea_query::{tests_cfg::*, *};
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
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect" ASC)"#
/// );
/// ```
///
/// Partial Index with prefix and order
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let index = Index::create()
///     .name("idx-glyph-aspect")
///     .table(Glyph::Table)
///     .col((Glyph::Aspect, 64, IndexOrder::Asc))
///     .and_where(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
///     .to_owned();
///
/// assert_eq!(
///     index.to_string(PostgresQueryBuilder),
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect" (64) ASC) WHERE "glyph"."aspect" IN ($1, $2)"#
/// );
/// assert_eq!(
///     index.to_string(SqliteQueryBuilder),
///     r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect" ASC) WHERE "glyph"."aspect" IN (?, ?)"#
/// );
/// ```

#[derive(Debug, Clone)]
pub struct IndexCreateStatement {
    pub(crate) table: Option<DynIden>,
    pub(crate) index: TableIndex,
    pub(crate) primary: bool,
    pub(crate) unique: bool,
    pub(crate) index_type: Option<IndexType>,
    pub(crate) if_not_exists: bool,
    pub(crate) r#where: ConditionHolder,
}

/// Specification of a table index
#[derive(Debug, Clone)]
pub enum IndexType {
    BTree,
    FullText,
    Hash,
    Custom(DynIden),
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
            if_not_exists: false,
            r#where: ConditionHolder::new(),
        }
    }

    /// Create index if index not exists
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    /// Set index name
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.index.name(name);
        self
    }

    /// Set target table
    pub fn table<T: 'static>(&mut self, table: T) -> &mut Self
    where
        T: Iden,
    {
        self.table = Some(SeaRc::new(table));
        self
    }

    /// Add index column
    pub fn col<C: 'static>(&mut self, col: C) -> &mut Self
    where
        C: IntoIndexColumn,
    {
        self.index.col(col.into_index_column());
        self
    }

    /// Set index as primary
    pub fn primary(&mut self) -> &mut Self {
        self.primary = true;
        self
    }

    /// Set index as unique
    pub fn unique(&mut self) -> &mut Self {
        self.unique = true;
        self
    }

    /// Set index as full text.
    /// On MySQL, this is `FULLTEXT`.
    /// On PgSQL, this is `GIN`.
    pub fn full_text(&mut self) -> &mut Self {
        self.index_type(IndexType::FullText)
    }

    /// Set index type. Not available on Sqlite.
    pub fn index_type(&mut self, index_type: IndexType) -> &mut Self {
        self.index_type = Some(index_type);
        self
    }

    pub fn is_primary_key(&self) -> bool {
        self.primary
    }

    pub fn is_unique_key(&self) -> bool {
        self.unique
    }

    pub fn get_index_spec(&self) -> &TableIndex {
        &self.index
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            index: self.index.take(),
            primary: self.primary,
            unique: self.unique,
            index_type: self.index_type.take(),
            if_not_exists: self.if_not_exists,
            r#where: self.r#where.clone(),
        }
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

impl ConditionalStatement for IndexCreateStatement {
    fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self {
        self.r#where.add_and_or(condition);
        self
    }

    fn cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.r#where.add_condition(condition.into_condition());
        self
    }
}
