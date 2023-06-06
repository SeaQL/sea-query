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
/// use sea_query::extension::postgres::{ExtensionBuilder, ExtensionStatement};
/// use sea_query::tests_cfg::*;
///
/// let mut query = String::new();
/// let stmt = ExtensionStatement::create("ltree")
///     .if_not_exists()
///     .cascade()
///     .schema("public")
///     .version("v0.1.0")
///     .to_owned();
///
/// stmt.prepare_extension_statement(&stmt, &mut query);
///
/// assert_eq!(
///     query,
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

pub trait ExtensionBuilder {
    /// Translate [`ExtensionStatement`] into a PostgreSQL's `CREATE` or `DROP`
    /// `EXTENSION` statement.
    ///
    /// ## Refer
    ///
    /// https://www.postgresql.org/docs/current/sql-createextension.html
    fn prepare_extension_statement(&self, sql: &mut dyn SqlWriter);
}

impl ExtensionBuilder for ExtensionStatement {
    fn prepare_extension_statement(&self, sql: &mut dyn SqlWriter) {
        match self.operation {
            ExtensionOperation::Create => {
                write!(sql, "CREATE EXTENSION ").unwrap();

                if self.if_not_exists {
                    write!(sql, "IF NOT EXISTS ").unwrap()
                }

                write!(sql, "{}", self.name).unwrap();

                if let Some(schema) = self.schema.as_ref() {
                    write!(sql, " WITH SCHEMA {}", schema).unwrap();
                }

                if let Some(version) = self.version.as_ref() {
                    write!(sql, " VERSION {}", version).unwrap();
                }

                if self.cascade {
                    write!(sql, " CASCADE").unwrap();
                }
            }
            ExtensionOperation::Drop => {
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
    /// use sea_query::extension::postgres::{ExtensionBuilder, ExtensionStatement};
    /// use sea_query::tests_cfg::*;
    ///
    /// let mut query = String::new();
    /// let stmt = ExtensionStatement::create("ltree")
    ///     .schema("public")
    ///     .to_owned();
    ///
    /// stmt.prepare_extension_statement(&stmt, &mut query);
    ///
    /// assert_eq!(query, r#"CREATE EXTENSION ltree WITH SCHEMA public"#);
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
    /// use sea_query::extension::postgres::{ExtensionBuilder, ExtensionStatement};
    /// use sea_query::tests_cfg::*;
    ///
    /// let mut query = String::new();
    /// let stmt = ExtensionStatement::create("ltree")
    ///     .version("v0.1.0")
    ///     .to_owned();
    ///
    /// stmt.prepare_extension_statement(&stmt, &mut query);
    ///
    /// assert_eq!(query, r#"CREATE EXTENSION ltree VERSION v0.1.0"#);
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
    /// use sea_query::extension::postgres::{ExtensionBuilder, ExtensionStatement};
    /// use sea_query::tests_cfg::*;
    ///
    /// let mut query = String::new();
    /// let stmt = ExtensionStatement::create("ltree")
    ///     .if_not_exists()
    ///     .to_owned();
    ///
    /// stmt.prepare_extension_statement(&stmt, &mut query);
    ///
    /// assert_eq!(query, r#"CREATE EXTENSION IF NOT EXISTS ltree"#);
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
    /// use sea_query::extension::postgres::{ExtensionBuilder, ExtensionStatement};
    /// use sea_query::tests_cfg::*;
    ///
    /// let mut query = String::new();
    /// let stmt = ExtensionStatement::create("ltree")
    ///     .if_not_exists()
    ///     .to_owned();
    ///
    /// stmt.prepare_extension_statement(&stmt, &mut query);
    ///
    /// assert_eq!(query, r#"CREATE EXTENSION IF NOT EXISTS ltree"#);
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
    /// use sea_query::extension::postgres::{ExtensionBuilder, ExtensionStatement};
    /// use sea_query::tests_cfg::*;
    ///
    /// let mut query = String::new();
    /// let stmt = ExtensionStatement::create("ltree").cascade().to_owned();
    ///
    /// stmt.prepare_extension_statement(&stmt, &mut query);
    ///
    /// assert_eq!(query, r#"CREATE EXTENSION ltree CASCADE"#);
    /// ```
    pub fn cascade(&mut self) -> &mut Self {
        self.cascade = true;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_extension_statement() {
        let mut writer = String::new();
        let stmt = ExtensionStatement::create("ltree")
            .if_not_exists()
            .cascade()
            .schema("public")
            .version("v0.1.0")
            .to_owned();

        stmt.prepare_extension_statement(&mut writer);

        assert_eq!(
            writer,
            r#"CREATE EXTENSION IF NOT EXISTS ltree WITH SCHEMA public VERSION v0.1.0 CASCADE"#
        );
    }

    #[test]
    fn drop_extension_statement() {
        let mut writer = String::new();
        let stmt = ExtensionStatement::drop("ltree")
            .if_exists()
            .cascade()
            .to_owned();

        stmt.prepare_extension_statement(&mut writer);

        assert_eq!(writer, r#"DROP EXTENSION IF EXISTS ltree CASCADE"#);
    }
}
