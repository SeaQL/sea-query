use std::rc::Rc;
use crate::{TableForeignKey, backend::SchemaBuilder, SchemaStatementBuilder, types::*, prepare::*};

/// Drop a foreign key constraint for an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let foreign_key = ForeignKey::drop()
///     .name("FK_character_font")
///     .table(Char::Table)
///     .to_owned();
///
/// assert_eq!(
///     foreign_key.to_string(MysqlQueryBuilder),
///     r#"ALTER TABLE `character` DROP FOREIGN KEY `FK_character_font`"#
/// );
/// assert_eq!(
///     foreign_key.to_string(PostgresQueryBuilder),
///     r#"ALTER TABLE "character" DROP CONSTRAINT "FK_character_font""#
/// );
/// // Sqlite does not support modification of foreign key constraints to existing tables
/// ```
#[derive(Debug, Clone)]
pub struct ForeignKeyDropStatement {
    pub(crate) foreign_key: TableForeignKey,
    pub(crate) table: Option<Rc<dyn Iden>>,
}

impl Default for ForeignKeyDropStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl ForeignKeyDropStatement {
    /// Construct a new [`ForeignKeyDropStatement`]
    pub fn new() -> Self {
        Self {
            foreign_key: Default::default(),
            table: None,
        }
    }

    /// Set foreign key name
    pub fn name(mut self, name: &str) -> Self {
        self.foreign_key.name(name);
        self
    }

    /// Set key table and referencing table
    pub fn table<T: 'static>(mut self, table: T) -> Self
        where T: Iden {
        self.table = Some(Rc::new(table));
        self
    }

    pub fn to_string<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        <Self as SchemaStatementBuilder>::to_string(self, schema_builder)
    }

    pub fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        <Self as SchemaStatementBuilder>::build(self, schema_builder)
    }

    pub fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        <Self as SchemaStatementBuilder>::build_any(self, schema_builder)
    }
}

impl SchemaStatementBuilder for ForeignKeyDropStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_foreign_key_drop_statement(self, &mut sql);
        sql.result()
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_foreign_key_drop_statement(self, &mut sql);
        sql.result()
    }
}
