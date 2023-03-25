use crate::SqlWriter;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) enum ExtensionOperation {
    #[default]
    Create,
    Drop,
}

/// Creates a new "CREATE EXTENSION" statement for PostgreSQL
///
/// # Examples
///
/// ```
/// use sea_query::{extension::postgres::ExtensionCreateStatement, tests_cfg::*, *};
///
/// let stmt = ExtensionCreateStatement::new("ltree")
///     .if_not_exists()
///     .cascade()
///     .schema("public")
///     .version("v0.1.0")
///     .to_owned();
///
/// assert_eq!(
///     stmt.to_string(),
///     r#"CREATE EXTENSION IF NOT EXISTS ltree WITH SCHEMA public VERSION v0.1.0 CASCADE"#
/// );
/// ```
///
/// # References
///
/// [Refer to the PostgreSQL Documentation][1]
///
/// [1]: https://www.postgresql.org/docs/current/sql-createextension.html
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExtensionStatement {
    pub(crate) name: String,
    pub(crate) schema: Option<String>,
    pub(crate) version: Option<String>,
    /// Conditional to execute query based on existance of the extension.
    ///
    /// This is only used for `CREATE EXTENSION` and is not compatible with
    /// `DROP EXTENSION`.
    pub(crate) if_not_exists: bool,
    pub(crate) with: bool,
    /// Conditional to execute query based on existance of the extension.
    ///
    /// This is only used for `DROP EXTENSION` and is not compatible with
    /// `CREATE EXTENSION`.
    pub(crate) if_exists: bool,
    /// Determines the presence of the `RESTRICT` statement.
    ///
    /// This is only used for `DROP EXTENSION` and is not compatible with
    /// `CREATE EXTENSION`.
    pub(crate) restrict: bool,
    /// Determines the presence of the `RESTRICT` statement
    pub(crate) cascade: bool,
    pub(crate) operation: ExtensionOperation,
}

pub trait CreateExtensionBuilder {
    /// Translate [`ExtensionStatement`] into a PostgreSQL's `CREATE EXTENSION` statement
    ///
    /// PostgreSQL Syntax
    ///
    /// ```ignore
    /// CREATE EXTENSION [ IF NOT EXISTS ] extension_name
    ///     [ WITH ] [ SCHEMA schema_name ]
    ///              [ VERSION version ]
    ///              [ CASCADE ]
    /// ```
    ///
    /// ## Refer
    ///
    /// https://www.postgresql.org/docs/current/sql-createextension.html
    fn prepare_extension_create_statement(
        &self,
        stmt: &ExtensionStatement,
        sql: &mut dyn SqlWriter,
    );

    /// Translate [`ExtensionStatement`] into a PostgreSQL's `DROP EXTENSION` statement
    ///
    /// PostgreSQL Syntax
    ///
    /// ```ignore
    /// DROP EXTENSION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
    /// ```
    ///
    /// ## Refer
    ///
    ///  https://www.postgresql.org/docs/current/sql-createextension.html
    fn prepare_extension_drop_statement(&self, stmt: &ExtensionStatement, sql: &mut dyn SqlWriter);
}

impl CreateExtensionBuilder for ExtensionStatement {
    fn prepare_extension_create_statement(
        &self,
        stmt: &ExtensionStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "CREATE EXTENSION ").unwrap();

        if self.if_not_exists {
            write!(sql, "IF NOT EXISTS ").unwrap()
        }

        write!(sql, "{}", self.name).unwrap();

        if self.with {
            write!(sql, " WITH ").unwrap();
        }

        if let Some(schema) = self.schema.as_ref() {
            write!(sql, " SCHEMA {} ", schema).unwrap();
        }

        if let Some(version) = self.version.as_ref() {
            write!(sql, " VERSION {} ", version).unwrap();
        }

        if self.cascade {
            write!(sql, " CASCADE").unwrap();
        }
    }

    fn prepare_extension_drop_statement(&self, stmt: &ExtensionStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "DROP EXTENSION ").unwrap();

        if self.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }

        write!(sql, "{}", self.name).unwrap();

        if self.cascade {
            write!(sql, " CASCADE").unwrap();
        }

        if self.restrict {
            write!(sql, "  RESTRICT").unwrap();
        }
    }
}

impl ExtensionStatement {
    pub fn create(name: &str) -> Self {
        ExtensionStatement {
            name: name.to_string(),
            operation: ExtensionOperation::Create,
            ..Default::default()
        }
    }

    pub fn drop(name: &str) -> Self {
        ExtensionStatement {
            name: name.to_string(),
            operation: ExtensionOperation::Drop,
            ..Default::default()
        }
    }

    /// Uses "WITH SCHEMA" on Create Extension Statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::ExtensionCreateStatement, tests_cfg::*, *};
    ///
    /// let stmt = ExtensionCreateStatement::new("ltree")
    ///     .schema("public")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     stmt.to_string(),
    ///     r#"CREATE EXTENSION ltree WITH SCHEMA public"#
    /// );
    /// ```
    pub fn schema(&mut self, schema: &str) -> &mut Self {
        self.schema = Some(schema.to_string());
        self
    }

    /// Uses "VERSION" on Create Extension Statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::ExtensionCreateStatement, tests_cfg::*, *};
    ///
    /// let stmt = ExtensionCreateStatement::new("ltree")
    ///     .version("v0.1.0")
    ///     .to_owned();
    ///
    /// assert_eq!(stmt.to_string(), r#"CREATE EXTENSION ltree VERSION v0.1.0"#);
    /// ```
    pub fn version(&mut self, version: &str) -> &mut Self {
        self.version = Some(version.to_string());
        self
    }

    /// Uses "IF EXISTS" on Drop Extension Statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::ExtensionCreateStatement, tests_cfg::*, *};
    ///
    /// let stmt = ExtensionCreateStatement::new("ltree")
    ///     .if_not_exists()
    ///     .to_owned();
    ///
    /// assert_eq!(stmt.to_string(), r#"CREATE EXTENSION IF NOT EXISTS ltree"#);
    /// ```
    pub fn if_not_exists(&mut self) -> &mut Self {
        if matches!(self.operation, ExtensionOperation::Create) {
            self.if_not_exists = true;
            return self;
        }

        panic!("IF NOT EXISTS parameter is not compatible with DROP EXTENSION");
    }

    /// Uses "IF NOT EXISTS" on Create Extension Statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::ExtensionCreateStatement, tests_cfg::*, *};
    ///
    /// let stmt = ExtensionCreateStatement::new("ltree")
    ///     .if_not_exists()
    ///     .to_owned();
    ///
    /// assert_eq!(stmt.to_string(), r#"DROP EXTENSION IF EXISTS ltree"#);
    /// ```
    pub fn if_exists(&mut self) -> &mut Self {
        if matches!(self.operation, ExtensionOperation::Drop) {
            self.if_exists = true;
            return self;
        }

        panic!("IF EXISTS parameter is not compatible with CREATE EXTENSION");
    }

    /// Uses "CASCADE" on Create Extension Statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::ExtensionCreateStatement, tests_cfg::*, *};
    ///
    /// let stmt = ExtensionCreateStatement::new("ltree").cascade().to_owned();
    ///
    /// assert_eq!(stmt.to_string(), r#"CREATE EXTENSION ltree CASCADE"#);
    /// ```
    pub fn cascade(&mut self) -> &mut Self {
        self.cascade = true;
        self
    }
}
