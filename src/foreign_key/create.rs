use crate::{ForeignKeyAction, TableForeignKey, backend::ForeignKeyBuilder, types::*, prepare::*};

/// Create a foreign key constraint for an existing table. Unsupported by Sqlite
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let foreign_key = ForeignKey::create()
///     .name("FK_character_font")
///     .from(Char::Table, Char::FontId)
///     .to(Font::Table, Font::Id)
///     .on_delete(ForeignKeyAction::Cascade)
///     .on_update(ForeignKeyAction::Cascade)
///     .to_owned();
///
/// assert_eq!(
///     foreign_key.to_string(MysqlQueryBuilder),
///     vec![
///         r#"ALTER TABLE `character`"#,
///         r#"ADD CONSTRAINT `FK_character_font`"#,
///         r#"FOREIGN KEY (`font_id`) REFERENCES `font` (`id`)"#,
///         r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
///     ].join(" ")
/// );
/// assert_eq!(
///     foreign_key.to_string(PostgresQueryBuilder),
///     vec![
///         r#"ALTER TABLE "character" ADD CONSTRAINT "FK_character_font""#,
///         r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
///         r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
///     ].join(" ")
/// );
/// ```
///
/// Composite key
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let foreign_key = ForeignKey::create()
///     .name("FK_character_glyph")
///     .from(Char::Table, (Char::FontId, Char::Id))
///     .to(Glyph::Table, (Char::FontId, Glyph::Id))
///     .on_delete(ForeignKeyAction::Cascade)
///     .on_update(ForeignKeyAction::Cascade)
///     .to_owned();
///
/// assert_eq!(
///     foreign_key.to_string(MysqlQueryBuilder),
///     vec![
///         r#"ALTER TABLE `character`"#,
///         r#"ADD CONSTRAINT `FK_character_glyph`"#,
///         r#"FOREIGN KEY (`font_id`, `id`) REFERENCES `glyph` (`font_id`, `id`)"#,
///         r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
///     ].join(" ")
/// );
/// assert_eq!(
///     foreign_key.to_string(PostgresQueryBuilder),
///     vec![
///         r#"ALTER TABLE "character" ADD CONSTRAINT "FK_character_glyph""#,
///         r#"FOREIGN KEY ("font_id", "id") REFERENCES "glyph" ("font_id", "id")"#,
///         r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
///     ].join(" ")
/// );
/// ```
#[derive(Debug, Clone)]
pub struct ForeignKeyCreateStatement {
    pub(crate) foreign_key: TableForeignKey,
}

impl Default for ForeignKeyCreateStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl ForeignKeyCreateStatement {
    /// Construct a new [`ForeignKeyCreateStatement`]
    pub fn new() -> Self {
        Self {
            foreign_key: Default::default(),
        }
    }

    /// Set foreign key name
    pub fn name(mut self, name: &str) -> Self {
        self.foreign_key.name(name);
        self
    }

    /// Set key table and referencing table
    #[deprecated(
        since = "0.10.2",
        note = "Please use the [`ForeignKeyCreateStatement::from`] and [`ForeignKeyCreateStatement::to`]"
    )]
    pub fn table<T: 'static, R: 'static>(mut self, table: T, ref_table: R) -> Self
        where T: Iden, R: Iden {
        self.foreign_key.from_tbl(table);
        self.foreign_key.to_tbl(ref_table);
        self
    }

    /// Set key column and referencing column
    #[deprecated(
        since = "0.10.2",
        note = "Please use the [`ForeignKeyCreateStatement::from`] and [`ForeignKeyCreateStatement::to`]"
    )]
    pub fn col<T: 'static, R: 'static>(mut self, column: T, ref_column: R) -> Self
        where T: Iden, R: Iden {
        self.foreign_key.from_col(column);
        self.foreign_key.to_col(ref_column);
        self
    }

    /// Set key table and columns
    pub fn from<T, C>(mut self, table: T, columns: C) -> Self
        where T: IntoIden, C: IdenList {
        self.foreign_key.from_tbl(table);
        for col in columns.into_iter() {
            self.foreign_key.from_col(col);
        }
        self
    }

    /// Set referencing table and columns
    pub fn to<T, C>(mut self, table: T, columns: C) -> Self
        where T: IntoIden, C: IdenList {
        self.foreign_key.to_tbl(table);
        for col in columns.into_iter() {
            self.foreign_key.to_col(col);
        }
        self
    }

    /// Set key table
    pub fn from_tbl<T>(mut self, table: T) -> Self
        where T: IntoIden {
        self.foreign_key.from_tbl(table);
        self
    }

    /// Set referencing table
    pub fn to_tbl<R>(mut self, ref_table: R) -> Self
        where R: IntoIden {
        self.foreign_key.to_tbl(ref_table);
        self
    }

    /// Add key column
    pub fn from_col<T>(mut self, column: T) -> Self
        where T: IntoIden {
        self.foreign_key.from_col(column);
        self
    }

    /// Add referencing column
    pub fn to_col<R>(mut self, ref_column: R) -> Self
        where R: IntoIden {
        self.foreign_key.to_col(ref_column);
        self
    }

    /// Set on delete action
    pub fn on_delete(mut self, action: ForeignKeyAction) -> Self {
        self.foreign_key.on_delete(action);
        self
    }

    /// Set on update action
    pub fn on_update(mut self, action: ForeignKeyAction) -> Self {
        self.foreign_key.on_update(action);
        self
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T: ForeignKeyBuilder>(&self, foreign_key_builder: T) -> String {
        let mut sql = SqlWriter::new();
        foreign_key_builder.prepare_foreign_key_create_statement(self, &mut sql);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, foreign_key_builder: &dyn ForeignKeyBuilder) -> String {
        let mut sql = SqlWriter::new();
        foreign_key_builder.prepare_foreign_key_create_statement(self, &mut sql);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: ForeignKeyBuilder>(&self, foreign_key_builder: T) -> String {
        self.build(foreign_key_builder)
    }
}
