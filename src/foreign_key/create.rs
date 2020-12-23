use crate::{ForeignKeyAction, TableForeignKey, backend::ForeignKeyBuilder, types::*};

/// Create a foreign key constraint for an existing table
/// 
/// # Examples
/// 
/// ```
/// use sea_query::{*, tests_cfg::*};
/// 
/// let foreign_key = ForeignKey::create()
///     .name("FK_character_font")
///     .table(Char::Table, Font::Table)
///     .col(Char::FontId, Font::Id)
///     .on_delete(ForeignKeyAction::Cascade)
///     .on_update(ForeignKeyAction::Cascade)
///     .to_owned();
/// 
/// assert_eq!(
///     foreign_key.to_string(MysqlQueryBuilder),
///     vec![
///         r#"ALTER TABLE `character`"#,
///         r#"ADD CONSTRAINT `FK_character_font`"#,
///         r#"FOREIGN KEY `FK_character_font` (`font_id`) REFERENCES `font` (`id`)"#,
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
/// // Sqlite does not support modification of foreign key constraints to existing tables
/// ```
#[derive(Clone)]
pub struct ForeignKeyCreateStatement {
    pub(crate) foreign_key: TableForeignKey,
    pub(crate) inside_table_creation: bool,
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
            inside_table_creation: false,
        }
    }

    /// Set foreign key name
    pub fn name(mut self, name: &str) -> Self {
        self.foreign_key.name(name);
        self
    }

    /// Set key table and referencing table
    pub fn table<T: 'static, R: 'static>(mut self, table: T, ref_table: R) -> Self
        where T: Iden, R: Iden {
        self.foreign_key.table(table, ref_table);
        self
    }

    /// Set key column and referencing column
    pub fn col<T: 'static, R: 'static>(mut self, column: T, ref_column: R) -> Self
        where T: Iden, R: Iden {
        self.foreign_key.col(column, ref_column);
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
        let mut sql = String::new();
        foreign_key_builder.prepare_foreign_key_create_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, foreign_key_builder: &dyn ForeignKeyBuilder) -> String {
        let mut sql = String::new();
        foreign_key_builder.prepare_foreign_key_create_statement(self, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: ForeignKeyBuilder>(&self, foreign_key_builder: T) -> String {
        self.build(foreign_key_builder)
    }
}