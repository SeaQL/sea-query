use std::rc::Rc;
use crate::{TableIndex, backend::IndexBuilder, types::*};

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
#[derive(Clone)]
pub struct IndexCreateStatement {
    pub(crate) table: Option<Rc<dyn Iden>>,
    pub(crate) index: TableIndex,
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

    /// Set index column
    pub fn col<T: 'static>(mut self, column: T) -> Self
        where T: Iden {
        self.index.col(column);
        self
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T: IndexBuilder>(&self, index_builder: T) -> String {
        let mut sql = String::new();
        index_builder.prepare_index_create_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, index_builder: Box<dyn IndexBuilder>) -> String {
        let mut sql = String::new();
        index_builder.prepare_index_create_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: IndexBuilder>(&self, index_builder: T) -> String {
        self.build(index_builder)
    }
}