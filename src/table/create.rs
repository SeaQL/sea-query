use crate::{
    backend::SchemaBuilder, foreign_key::*, index::*, prepare::*, types::*, ColumnDef,
    SchemaStatementBuilder,
};

/// Create a table
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let table = Table::create()
///     .table(Char::Table)
///     .if_not_exists()
///     .col(ColumnDef::new(Char::Id).integer().not_null().auto_increment().primary_key())
///     .col(ColumnDef::new(Char::FontSize).integer().not_null())
///     .col(ColumnDef::new(Char::Character).string().not_null())
///     .col(ColumnDef::new(Char::SizeW).integer().not_null())
///     .col(ColumnDef::new(Char::SizeH).integer().not_null())
///     .col(ColumnDef::new(Char::FontId).integer().default(Value::Int(None)))
///     .foreign_key(
///         ForeignKey::create()
///             .name("FK_2e303c3a712662f1fc2a4d0aad6")
///             .from(Char::Table, Char::FontId)
///             .to(Font::Table, Font::Id)
///             .on_delete(ForeignKeyAction::Cascade)
///             .on_update(ForeignKeyAction::Cascade)
///     )
///     .to_owned();
///
/// assert_eq!(
///     table.to_string(MysqlQueryBuilder),
///     vec![
///         r#"CREATE TABLE IF NOT EXISTS `character` ("#,
///             r#"`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,"#,
///             r#"`font_size` int NOT NULL,"#,
///             r#"`character` varchar(255) NOT NULL,"#,
///             r#"`size_w` int NOT NULL,"#,
///             r#"`size_h` int NOT NULL,"#,
///             r#"`font_id` int DEFAULT NULL,"#,
///             r#"CONSTRAINT `FK_2e303c3a712662f1fc2a4d0aad6`"#,
///                 r#"FOREIGN KEY (`font_id`) REFERENCES `font` (`id`)"#,
///                 r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
///         r#")"#,
///     ].join(" ")
/// );
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     vec![
///         r#"CREATE TABLE IF NOT EXISTS "character" ("#,
///             r#""id" serial NOT NULL PRIMARY KEY,"#,
///             r#""font_size" integer NOT NULL,"#,
///             r#""character" varchar NOT NULL,"#,
///             r#""size_w" integer NOT NULL,"#,
///             r#""size_h" integer NOT NULL,"#,
///             r#""font_id" integer DEFAULT NULL,"#,
///             r#"CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#,
///                 r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
///                 r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
///         r#")"#,
///     ].join(" ")
/// );
/// assert_eq!(
///     table.to_string(SqliteQueryBuilder),
///     vec![
///        r#"CREATE TABLE IF NOT EXISTS "character" ("#,
///            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
///            r#""font_size" integer NOT NULL,"#,
///            r#""character" text NOT NULL,"#,
///            r#""size_w" integer NOT NULL,"#,
///            r#""size_h" integer NOT NULL,"#,
///            r#""font_id" integer DEFAULT NULL,"#,
///            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
///        r#")"#,
///     ].join(" ")
/// );
/// ```
#[derive(Debug, Clone)]
pub struct TableCreateStatement {
    pub(crate) table: Option<TableRef>,
    pub(crate) columns: Vec<ColumnDef>,
    pub(crate) options: Vec<TableOpt>,
    pub(crate) partitions: Vec<TablePartition>,
    pub(crate) indexes: Vec<IndexCreateStatement>,
    pub(crate) foreign_keys: Vec<ForeignKeyCreateStatement>,
    pub(crate) if_not_exists: bool,
}

/// All available table options
#[derive(Debug, Clone)]
pub enum TableOpt {
    Engine(String),
    Collate(String),
    CharacterSet(String),
}

/// All available table partition options
#[derive(Debug, Clone)]
pub enum TablePartition {}

impl Default for TableCreateStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl TableCreateStatement {
    /// Construct create table statement
    pub fn new() -> Self {
        Self {
            table: None,
            columns: Vec::new(),
            options: Vec::new(),
            partitions: Vec::new(),
            indexes: Vec::new(),
            foreign_keys: Vec::new(),
            if_not_exists: false,
        }
    }

    #[deprecated(
        since = "0.9.6",
        note = "Please use the [`TableCreateStatement::if_not_exists`]"
    )]
    pub fn create_if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    /// Create table if table not exists
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    /// Set table name
    pub fn table<T>(&mut self, table: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(table.into_table_ref());
        self
    }

    /// Add a new table column
    pub fn col(&mut self, column: &mut ColumnDef) -> &mut Self {
        let mut column = column.take();
        column.table = self.table.clone();
        self.columns.push(column);
        self
    }

    /// Add an index. MySQL only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Table::create()
    ///         .table(Glyph::Table)
    ///         .col(ColumnDef::new(Glyph::Id).integer().not_null())
    ///         .index(Index::create().unique().name("idx-glyph-id").col(Glyph::Id))
    ///         .to_string(MysqlQueryBuilder),
    ///     vec![
    ///         "CREATE TABLE `glyph` (",
    ///         "`id` int NOT NULL,",
    ///         "UNIQUE KEY `idx-glyph-id` (`id`)",
    ///         ")",
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn index(&mut self, index: &mut IndexCreateStatement) -> &mut Self {
        self.indexes.push(index.take());
        self
    }

    /// Add an primary key.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let mut statement = Table::create();
    /// statement
    ///     .table(Glyph::Table)
    ///     .col(ColumnDef::new(Glyph::Id).integer().not_null())
    ///     .col(ColumnDef::new(Glyph::Image).string().not_null())
    ///     .primary_key(Index::create().col(Glyph::Id).col(Glyph::Image));
    /// assert_eq!(
    ///     statement.to_string(MysqlQueryBuilder),
    ///     vec![
    ///         "CREATE TABLE `glyph` (",
    ///         "`id` int NOT NULL,",
    ///         "`image` varchar(255) NOT NULL,",
    ///         "PRIMARY KEY (`id`, `image`)",
    ///         ")",
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     statement.to_string(PostgresQueryBuilder),
    ///     vec![
    ///         "CREATE TABLE \"glyph\" (",
    ///         "\"id\" integer NOT NULL,",
    ///         "\"image\" varchar NOT NULL,",
    ///         "PRIMARY KEY (\"id\", \"image\")",
    ///         ")",
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     statement.to_string(SqliteQueryBuilder),
    ///     vec![
    ///         r#"CREATE TABLE "glyph" ("#,
    ///         r#""id" integer NOT NULL,"#,
    ///         r#""image" text NOT NULL,"#,
    ///         r#"PRIMARY KEY ("id", "image")"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn primary_key(&mut self, index: &mut IndexCreateStatement) -> &mut Self {
        let mut index = index.take();
        index.primary = true;
        self.indexes.push(index);
        self
    }

    /// Add a foreign key
    pub fn foreign_key(&mut self, foreign_key: &mut ForeignKeyCreateStatement) -> &mut Self {
        self.foreign_keys.push(foreign_key.take());
        self
    }

    /// Set database engine. MySQL only.
    pub fn engine(&mut self, string: &str) -> &mut Self {
        self.opt(TableOpt::Engine(string.into()));
        self
    }

    /// Set database collate. MySQL only.
    pub fn collate(&mut self, string: &str) -> &mut Self {
        self.opt(TableOpt::Collate(string.into()));
        self
    }

    /// Set database character set. MySQL only.
    pub fn character_set(&mut self, string: &str) -> &mut Self {
        self.opt(TableOpt::CharacterSet(string.into()));
        self
    }

    fn opt(&mut self, option: TableOpt) -> &mut Self {
        self.options.push(option);
        self
    }

    #[allow(dead_code)]
    fn partition(&mut self, partition: TablePartition) -> &mut Self {
        self.partitions.push(partition);
        self
    }

    pub fn get_table_name(&self) -> Option<&TableRef> {
        self.table.as_ref()
    }

    pub fn get_columns(&self) -> &Vec<ColumnDef> {
        self.columns.as_ref()
    }

    pub fn get_foreign_key_create_stmts(&self) -> &Vec<ForeignKeyCreateStatement> {
        self.foreign_keys.as_ref()
    }

    pub fn get_indexes(&self) -> &Vec<IndexCreateStatement> {
        self.indexes.as_ref()
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            columns: std::mem::take(&mut self.columns),
            options: std::mem::take(&mut self.options),
            partitions: std::mem::take(&mut self.partitions),
            indexes: std::mem::take(&mut self.indexes),
            foreign_keys: std::mem::take(&mut self.foreign_keys),
            if_not_exists: self.if_not_exists,
        }
    }
}

impl SchemaStatementBuilder for TableCreateStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_create_statement(self, &mut sql);
        sql.result()
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_create_statement(self, &mut sql);
        sql.result()
    }
}
