use crate::{
    backend::SchemaBuilder, prepare::*, types::*, SchemaStatementBuilder, TableForeignKey,
};

/// Drop a foreign key constraint for an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
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
    pub(crate) table: Option<DynIden>,
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
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.foreign_key.name(name);
        self
    }

    /// Set key table and referencing table
    pub fn table<T: 'static>(&mut self, table: T) -> &mut Self
    where
        T: Iden,
    {
        self.table = Some(SeaRc::new(table));
        self
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
