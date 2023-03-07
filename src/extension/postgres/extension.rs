/// Creates a new "CREATE EXTENSION" statement for PostgreSQL
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*, extension::postgres::ExtensionCreateStatement};
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
pub struct ExtensionCreateStatement {
    pub(crate) name: String,
    pub(crate) schema: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) if_not_exists: bool,
    pub(crate) cascade: bool,
}

impl ExtensionCreateStatement {
    pub fn new(name: &str) -> Self {
        ExtensionCreateStatement {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Uses "WITH SCHEMA" on Create Extension Statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::ExtensionCreateStatement};
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
    /// use sea_query::{*, tests_cfg::*, extension::postgres::ExtensionCreateStatement};
    ///
    /// let stmt = ExtensionCreateStatement::new("ltree")
    ///     .version("v0.1.0")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     stmt.to_string(),
    ///     r#"CREATE EXTENSION ltree VERSION v0.1.0"#
    /// );
    /// ```
    pub fn version(&mut self, version: &str) -> &mut Self {
        self.version = Some(version.to_string());
        self
    }

    /// Uses "IF NOT EXISTS" on Create Extension Statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::ExtensionCreateStatement};
    ///
    /// let stmt = ExtensionCreateStatement::new("ltree")
    ///     .if_not_exists()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     stmt.to_string(),
    ///     r#"CREATE EXTENSION IF NOT EXISTS ltree"#
    /// );
    /// ```
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    /// Uses "CASCADE" on Create Extension Statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*, extension::postgres::ExtensionCreateStatement};
    ///
    /// let stmt = ExtensionCreateStatement::new("ltree")
    ///     .cascade()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     stmt.to_string(),
    ///     r#"CREATE EXTENSION ltree CASCADE"#
    /// );
    /// ```
    pub fn cascade(&mut self) -> &mut Self {
        self.cascade = true;
        self
    }
}

impl ToString for ExtensionCreateStatement {
    fn to_string(&self) -> String {
        let mut stmt = String::from("CREATE EXTENSION");

        if self.if_not_exists {
            stmt.push_str(" IF NOT EXISTS");
        }

        stmt.push_str(&format!(" {}", self.name));

        if let Some(schema) = self.schema.as_ref() {
            stmt.push_str(&format!(" WITH SCHEMA {}", schema));
        }

        if let Some(version) = self.version.as_ref() {
            stmt.push_str(&format!(" VERSION {}", version));
        }

        if self.cascade {
            stmt.push_str(" CASCADE");
        }

        stmt
    }
}
