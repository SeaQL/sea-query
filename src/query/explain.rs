#[cfg(feature = "backend-mysql")]
use crate::extension::mysql::{ExplainTableTarget, MySqlExplainOptions, MySqlExplainSchemaSpec};
#[cfg(feature = "backend-postgres")]
use crate::extension::postgres::{PgExplainOptions, PgExplainSerialize};
#[cfg(feature = "backend-sqlite")]
use crate::extension::sqlite::SqliteExplainOptions;
use crate::{
    DeleteStatement, InsertStatement, QueryBuilder, QueryStatement, SelectStatement, SqlWriter,
    SqlWriterValues, UpdateStatement, Values,
};

#[cfg(feature = "backend-mysql")]
use crate::{IntoIden, IntoTableRef};

#[cfg(any(feature = "backend-postgres", feature = "backend-mysql"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExplainFormat {
    #[cfg(feature = "backend-postgres")]
    Text,
    #[cfg(feature = "backend-postgres")]
    Xml,
    #[cfg(any(feature = "backend-postgres", feature = "backend-mysql"))]
    Json,
    #[cfg(feature = "backend-postgres")]
    Yaml,
    #[cfg(feature = "backend-mysql")]
    Tree,
    #[cfg(feature = "backend-mysql")]
    Traditional,
}

#[cfg(any(feature = "backend-postgres", feature = "backend-mysql"))]
impl ExplainFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            #[cfg(feature = "backend-postgres")]
            Self::Text => "TEXT",
            #[cfg(feature = "backend-postgres")]
            Self::Xml => "XML",
            #[cfg(any(feature = "backend-postgres", feature = "backend-mysql"))]
            Self::Json => "JSON",
            #[cfg(feature = "backend-postgres")]
            Self::Yaml => "YAML",
            #[cfg(feature = "backend-mysql")]
            Self::Tree => "TREE",
            #[cfg(feature = "backend-mysql")]
            Self::Traditional => "TRADITIONAL",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ExplainableStatement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
}

impl ExplainableStatement {
    pub(crate) fn write_to(&self, query_builder: &impl QueryBuilder, sql: &mut impl SqlWriter) {
        match self {
            Self::Select(statement) => query_builder.prepare_select_statement(statement, sql),
            Self::Insert(statement) => query_builder.prepare_insert_statement(statement, sql),
            Self::Update(statement) => query_builder.prepare_update_statement(statement, sql),
            Self::Delete(statement) => query_builder.prepare_delete_statement(statement, sql),
        }
    }
}

impl From<QueryStatement> for ExplainableStatement {
    fn from(value: QueryStatement) -> Self {
        match value {
            QueryStatement::Select(stmt) => Self::Select(stmt),
            QueryStatement::Insert(stmt) => Self::Insert(stmt),
            QueryStatement::Update(stmt) => Self::Update(stmt),
            QueryStatement::Delete(stmt) => Self::Delete(stmt),
        }
    }
}

impl From<SelectStatement> for ExplainableStatement {
    fn from(value: SelectStatement) -> Self {
        Self::Select(value)
    }
}

impl From<InsertStatement> for ExplainableStatement {
    fn from(value: InsertStatement) -> Self {
        Self::Insert(value)
    }
}

impl From<UpdateStatement> for ExplainableStatement {
    fn from(value: UpdateStatement) -> Self {
        Self::Update(value)
    }
}

impl From<DeleteStatement> for ExplainableStatement {
    fn from(value: DeleteStatement) -> Self {
        Self::Delete(value)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExplainStatement {
    pub(crate) statement: Option<ExplainableStatement>,
    #[cfg(any(feature = "backend-postgres", feature = "backend-mysql"))]
    pub(crate) analyze: Option<bool>,
    #[cfg(any(feature = "backend-postgres", feature = "backend-mysql"))]
    pub(crate) format: Option<ExplainFormat>,
    #[cfg(feature = "backend-postgres")]
    pub(crate) pg_opts: PgExplainOptions,
    #[cfg(feature = "backend-mysql")]
    pub(crate) mysql_opts: MySqlExplainOptions,
    #[cfg(feature = "backend-sqlite")]
    pub(crate) sqlite_opts: SqliteExplainOptions,
}

impl ExplainStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the statement to be explained.
    pub fn statement(mut self, statement: impl Into<ExplainableStatement>) -> Self {
        self.statement = Some(statement.into());
        self
    }
}

#[cfg(any(feature = "backend-postgres", feature = "backend-mysql"))]
impl ExplainStatement {
    /// Set `ANALYZE` to `true`.
    pub fn analyze(mut self) -> Self {
        self.analyze = Some(true);
        self
    }

    /// Set the output format.
    pub fn format(mut self, format: ExplainFormat) -> Self {
        self.format = Some(format);
        self
    }
}

#[cfg(feature = "backend-postgres")]
impl ExplainStatement {
    /// Set `VERBOSE`.
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.pg_opts.verbose = Some(verbose);
        self
    }

    /// Set `COSTS`.
    pub fn costs(mut self, costs: bool) -> Self {
        self.pg_opts.costs = Some(costs);
        self
    }

    /// Set `SETTINGS`.
    pub fn settings(mut self, settings: bool) -> Self {
        self.pg_opts.settings = Some(settings);
        self
    }

    /// Set `GENERIC_PLAN`.
    pub fn generic_plan(mut self, generic_plan: bool) -> Self {
        self.pg_opts.generic_plan = Some(generic_plan);
        self
    }

    /// Set `BUFFERS`.
    pub fn buffers(mut self, buffers: bool) -> Self {
        self.pg_opts.buffers = Some(buffers);
        self
    }

    /// Set `SERIALIZE TEXT`.
    pub fn serialize_text(mut self) -> Self {
        self.pg_opts.serialize = Some(PgExplainSerialize::Text);
        self
    }

    /// Set `SERIALIZE BINARY`.
    pub fn serialize_binary(mut self) -> Self {
        self.pg_opts.serialize = Some(PgExplainSerialize::Binary);
        self
    }

    /// Set `SERIALIZE NONE`.
    pub fn serialize_none(mut self) -> Self {
        self.pg_opts.serialize = Some(PgExplainSerialize::None);
        self
    }

    /// Set `WAL`.
    pub fn wal(mut self, wal: bool) -> Self {
        self.pg_opts.wal = Some(wal);
        self
    }

    /// Set `TIMING`.
    pub fn timing(mut self, timing: bool) -> Self {
        self.pg_opts.timing = Some(timing);
        self
    }

    /// Set `SUMMARY`.
    pub fn summary(mut self, summary: bool) -> Self {
        self.pg_opts.summary = Some(summary);
        self
    }

    /// Set `MEMORY`.
    pub fn memory(mut self, memory: bool) -> Self {
        self.pg_opts.memory = Some(memory);
        self
    }
}

#[cfg(feature = "backend-mysql")]
impl ExplainStatement {
    /// Create a MySQL `EXPLAIN` for `tbl_name [col_name | wild]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sea_query::{ExplainStatement, MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     ExplainStatement::table("glyph").to_string(MysqlQueryBuilder),
    ///     "EXPLAIN `glyph`"
    /// );
    /// assert_eq!(
    ///     ExplainStatement::table_with_column("glyph", "size").to_string(MysqlQueryBuilder),
    ///     "EXPLAIN `glyph` `size`"
    /// );
    /// assert_eq!(
    ///     ExplainStatement::table_with_wildcard("glyph", "size_%").to_string(MysqlQueryBuilder),
    ///     "EXPLAIN `glyph` 'size_%'"
    /// );
    /// ```
    pub fn table(table: impl IntoTableRef) -> Self {
        let mut statement = Self::new();
        statement.mysql_opts.table = Some(table.into_table_ref());
        statement.mysql_opts.target = None;
        statement
    }

    pub fn table_with_column(table: impl IntoTableRef, column: impl IntoIden) -> Self {
        let mut statement = Self::new();
        statement.mysql_opts.table = Some(table.into_table_ref());
        statement.mysql_opts.target = Some(ExplainTableTarget::Column(column.into_iden()));
        statement
    }

    pub fn table_with_wildcard(table: impl IntoTableRef, wildcard: &'static str) -> Self {
        let mut statement = Self::new();
        statement.mysql_opts.table = Some(table.into_table_ref());
        statement.mysql_opts.target = Some(ExplainTableTarget::Wildcard(wildcard));
        statement
    }

    /// Store the EXPLAIN output into a MySQL user variable.
    pub fn into_variable(mut self, variable: impl Into<String>) -> Self {
        self.mysql_opts.into_variable = Some(variable.into());
        self
    }

    /// Set the column name for `EXPLAIN tbl_name col_name`.
    pub fn column(mut self, column: impl IntoIden) -> Self {
        self.mysql_opts.target = Some(ExplainTableTarget::Column(column.into_iden()));
        self
    }

    /// Explain a statement for a specific connection id.
    pub fn for_connection(mut self, id: u64) -> Self {
        self.mysql_opts.for_connection = Some(id);
        self
    }

    /// Explain a statement for a specific schema.
    pub fn for_schema(mut self, schema: impl IntoIden) -> Self {
        self.mysql_opts.schema_spec = Some(MySqlExplainSchemaSpec::Schema(schema.into_iden()));
        self
    }

    /// Explain a statement for a specific database.
    pub fn for_database(mut self, database: impl IntoIden) -> Self {
        self.mysql_opts.schema_spec = Some(MySqlExplainSchemaSpec::Database(database.into_iden()));
        self
    }

    // set_table removed: prefer constructors on ExplainStatement.
}

#[cfg(feature = "backend-sqlite")]
impl ExplainStatement {
    /// Use `EXPLAIN QUERY PLAN`.
    pub fn query_plan(mut self) -> Self {
        self.sqlite_opts.query_plan = true;
        self
    }
}

impl ExplainStatement {
    /// Build SQL into the provided writer.
    pub fn build_collect_into(&self, query_builder: impl QueryBuilder, sql: &mut impl SqlWriter) {
        query_builder.prepare_explain_statement(self, sql);
    }

    /// Build SQL into the provided writer and return the resulting string.
    pub fn build_collect(
        &self,
        query_builder: impl QueryBuilder,
        sql: &mut impl SqlWriter,
    ) -> String {
        self.build_collect_into(query_builder, sql);
        sql.to_string()
    }

    /// Build SQL and collect values.
    pub fn build(&self, query_builder: impl QueryBuilder) -> (String, Values) {
        let (placeholder, numbered) = query_builder.placeholder();
        let mut sql = SqlWriterValues::new(placeholder, numbered);
        self.build_collect_into(query_builder, &mut sql);
        sql.into_parts()
    }

    /// Build the SQL string.
    pub fn to_string(&self, query_builder: impl QueryBuilder) -> String {
        let mut sql = String::with_capacity(256);
        self.build_collect_into(query_builder, &mut sql);
        sql
    }
}
