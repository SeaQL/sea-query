use crate::{QueryBuilder, QuotedBuilder, SqlWriter};

/// Creates a new "CREATE or DROP EXTENSION" statement for PostgreSQL
///
/// # Exampl
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Extension;

impl Extension {
    /// Creates a new [`ExtensionCreateStatement`]
    pub fn create() -> ExtensionCreateStatement {
        ExtensionCreateStatement::new()
    }

    /// Creates a new [`ExtensionDropStatement`]
    pub fn drop() -> ExtensionDropStatement {
        ExtensionDropStatement::new()
    }
}

/// Creates a new "CREATE EXTENSION" statement for PostgreSQL
///
/// # Synopsis
///
/// ```ignore
/// CREATE EXTENSION [ IF NOT EXISTS ] extension_name
///     [ WITH ] [ SCHEMA schema_name ]
///              [ VERSION version ]
///              [ CASCADE ]
/// ```
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
/// stmt.prepare_extension_statement(&mut query);
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
pub struct ExtensionCreateStatement {
    pub(crate) name: String,
    pub(crate) schema: Option<String>,
    pub(crate) version: Option<String>,

    /// Conditional to execute query based on existance of the extension.
    pub(crate) if_not_exists: bool,

    /// Determines the presence of the `RESTRICT` statement
    pub(crate) cascade: bool,
}

impl ExtensionCreateStatement {
    pub fn new() -> Self {
        Self::default()
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
    /// stmt.prepare_extension_statement(&mut query);
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
    /// let stmt = ExtensionCreateStatement::create("ltree")
    ///     .version("v0.1.0")
    ///     .to_owned();
    ///
    /// stmt.prepare_extension_statement(&mut query);
    ///
    /// assert_eq!(query, r#"CREATE EXTENSION ltree VERSION v0.1.0"#);
    /// ```
    pub fn version(&mut self, version: &str) -> &mut Self {
        self.version = Some(version.to_string());
        self
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
    /// let stmt = ExtensionCreateStatement::create("ltree")
    ///     .cascade()
    ///     .to_owned();
    ///
    /// stmt.prepare_extension_statement(&mut query);
    ///
    /// assert_eq!(query, r#"CREATE EXTENSION ltree CASCADE"#);
    /// ```
    pub fn cascade(&mut self) -> &mut Self {
        self.cascade = true;
        self
    }
}

/// Creates a new "DROP EXTENSION" statement for PostgreSQL
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
/// stmt.prepare_extension_statement(&mut query);
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
pub struct ExtensionDropStatement {
    pub(crate) name: String,
    pub(crate) schema: Option<String>,
    pub(crate) version: Option<String>,

    /// Conditional to execute query based on existance of the extension.
    pub(crate) if_exists: bool,

    /// Determines the presence of the `RESTRICT` statement.
    pub(crate) restrict: bool,

    /// Determines the presence of the `RESTRICT` statement
    pub(crate) cascade: bool,
}

impl ExtensionDropStatement {
    pub fn new() -> Self {
        Self::default()
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
    /// stmt.prepare_extension_statement(&mut query);
    ///
    /// assert_eq!(query, r#"DROP EXTENSION IF EXISTS ltree"#);
    /// ```
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }
}

pub trait ExtensionBuilder: QuotedBuilder {
    /// Translate [`ExtensionCreateStatement`] into database specific SQL statement.
    fn prepare_extension_create_statement(
        &self,
        create: &ExtensionCreateStatement,
        sql: &mut dyn SqlWriter,
    );

    /// Translate [`ExtensionDropStatement`] into database specific SQL statement.
    fn prepare_extension_drop_statement(
        &self,
        drop: &ExtensionDropStatement,
        sql: &mut dyn SqlWriter,
    );
}

macro_rules! impl_extension_statement_builder {
    ( $struct_name: ident, $func_name: ident ) => {
        impl $struct_name {
            pub fn build_ref<T: ExtensionBuilder>(&self, extension_builder: &T) -> String {
                let mut sql = String::with_capacity(256);
                self.build_collect_ref(extension_builder, &mut sql)
            }

            pub fn build_collect<T: ExtensionBuilder>(
                &self,
                extension_builder: T,
                sql: &mut dyn SqlWriter,
            ) -> String {
                self.build_collect_ref(&extension_builder, sql)
            }

            pub fn build_collect_ref<T: ExtensionBuilder>(
                &self,
                extension_builder: &T,
                sql: &mut dyn SqlWriter,
            ) -> String {
                extension_builder.$func_name(self, sql);
                sql.to_string()
            }

            /// Build corresponding SQL statement and return SQL string
            pub fn to_string<T>(&self, extension_builder: T) -> String
            where
                T: ExtensionBuilder + QueryBuilder,
            {
                self.build_ref(&extension_builder)
            }
        }
    };
}

impl_extension_statement_builder!(ExtensionCreateStatement, prepare_extension_create_statement);
impl_extension_statement_builder!(ExtensionDropStatement, prepare_extension_drop_statement);

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn create_extension_statement() {
//         let mut writer = String::new();
//         let stmt = ExtensionStatement::create("ltree")
//             .if_not_exists()
//             .cascade()
//             .schema("public")
//             .version("v0.1.0")
//             .to_owned();

//         stmt.prepare_extension_statement(&mut writer);

//         assert_eq!(
//             writer,
//             r#"CREATE EXTENSION IF NOT EXISTS ltree WITH SCHEMA public VERSION v0.1.0 CASCADE"#
//         );
//     }

//     #[test]
//     fn drop_extension_statement() {
//         let mut writer = String::new();
//         let stmt = ExtensionStatement::drop("ltree")
//             .if_exists()
//             .cascade()
//             .to_owned();

//         stmt.prepare_extension_statement(&mut writer);

//         assert_eq!(writer, r#"DROP EXTENSION IF EXISTS ltree CASCADE"#);
//     }
// }
