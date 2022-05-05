use crate::{
    backend::QueryBuilder, error::*, prepare::*, types::*, value::*, Expr, OnConflict, Query,
    QueryStatementBuilder, QueryStatementWriter, SelectExpr, SelectStatement, SimpleExpr,
    SubQueryStatement, WithClause, WithQuery,
};

/// Represents a value source that can be used in an insert query.
///
/// [`InsertValueSource`] is a node in the expression tree and can represent a raw value set
/// ('VALUES') or a select query.
#[derive(Debug, Clone)]
pub(crate) enum InsertValueSource {
    Values(Vec<Vec<SimpleExpr>>),
    Select(Box<SelectStatement>),
}

/// Insert any new rows into an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let query = Query::insert()
///     .into_table(Glyph::Table)
///     .columns(vec![Glyph::Aspect, Glyph::Image])
///     .values_panic(vec![5.15.into(), "12A".into()])
///     .values_panic(vec![4.21.into(), "123".into()])
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// ```
#[derive(Debug, Default, Clone)]
pub struct InsertStatement {
    pub(crate) replace: bool,
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) columns: Vec<DynIden>,
    pub(crate) source: Option<InsertValueSource>,
    pub(crate) on_conflict: Option<OnConflict>,
    pub(crate) returning: Vec<SelectExpr>,
    pub(crate) default_values: Option<u32>,
}

impl InsertStatement {
    /// Construct a new [`InsertStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Use REPLACE instead of INSERT
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .replace()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![Glyph::Aspect, Glyph::Image])
    ///     .values_panic(vec![5.15.into(), "12A".into()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"REPLACE INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"REPLACE INTO "glyph" ("aspect", "image") VALUES (5.15, '12A')"#
    /// );
    /// ```
    #[cfg(any(feature = "backend-sqlite", feature = "backend-mysql"))]
    pub fn replace(&mut self) -> &mut Self {
        self.replace = true;
        self
    }

    /// Specify which table to insert into.
    ///
    /// # Examples
    ///
    /// See [`InsertStatement::values`]
    pub fn into_table<T>(&mut self, tbl_ref: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    /// Specify what columns to insert.
    ///
    /// # Examples
    ///
    /// See [`InsertStatement::values`]
    pub fn columns<C, I>(&mut self, columns: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        self.columns = columns.into_iter().map(|c| c.into_iden()).collect();
        self
    }

    /// Specify a row of values to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![Glyph::Aspect, Glyph::Image])
    ///     .values(vec![2.1345.into(), "24B".into()])
    ///     .unwrap()
    ///     .values_panic(vec![5.15.into(), "12A".into()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// ```
    pub fn values<I>(&mut self, values: I) -> Result<&mut Self>
    where
        I: IntoIterator<Item = Value>,
    {
        let values = values
            .into_iter()
            .map(|v| Expr::val(v).into())
            .collect::<Vec<SimpleExpr>>();
        if self.columns.len() != values.len() {
            return Err(Error::ColValNumMismatch {
                col_len: self.columns.len(),
                val_len: values.len(),
            });
        }

        let values_source = if let Some(InsertValueSource::Values(values)) = &mut self.source {
            values
        } else {
            self.source = Some(InsertValueSource::Values(Default::default()));
            if let Some(InsertValueSource::Values(values)) = &mut self.source {
                values
            } else {
                unreachable!();
            }
        };
        values_source.push(values);
        Ok(self)
    }

    /// Specify a select query whose values to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![Glyph::Aspect, Glyph::Image])
    ///     .select_from(Query::select()
    ///         .column(Glyph::Aspect)
    ///         .column(Glyph::Image)
    ///         .from(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Image).like("0%"))
    ///         .to_owned()
    ///     )
    ///     .unwrap()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) SELECT `aspect`, `image` FROM `glyph` WHERE `image` LIKE '0%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") SELECT "aspect", "image" FROM "glyph" WHERE "image" LIKE '0%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") SELECT "aspect", "image" FROM "glyph" WHERE "image" LIKE '0%'"#
    /// );
    /// ```
    pub fn select_from<S>(&mut self, select: S) -> Result<&mut Self>
    where
        S: Into<SelectStatement>,
    {
        let statement = select.into();

        if self.columns.len() != statement.selects.len() {
            return Err(Error::ColValNumMismatch {
                col_len: self.columns.len(),
                val_len: statement.selects.len(),
            });
        }

        self.source = Some(InsertValueSource::Select(Box::new(statement)));
        Ok(self)
    }

    /// Specify a row of values to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![Glyph::Aspect, Glyph::Image])
    ///     .exprs(vec![
    ///         Expr::val(2).into(),
    ///         Func::cast_as("2020-02-02 00:00:00", Alias::new("DATE")),
    ///     ])
    ///     .unwrap()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, CAST('2020-02-02 00:00:00' AS DATE))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, CAST('2020-02-02 00:00:00' AS DATE))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, CAST('2020-02-02 00:00:00' AS DATE))"#
    /// );
    /// ```
    pub fn exprs<I>(&mut self, values: I) -> Result<&mut Self>
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        let values = values.into_iter().collect::<Vec<SimpleExpr>>();
        if self.columns.len() != values.len() {
            return Err(Error::ColValNumMismatch {
                col_len: self.columns.len(),
                val_len: values.len(),
            });
        }
        if !values.is_empty() {
            let values_source = if let Some(InsertValueSource::Values(values)) = &mut self.source {
                values
            } else {
                self.source = Some(InsertValueSource::Values(Default::default()));
                if let Some(InsertValueSource::Values(values)) = &mut self.source {
                    values
                } else {
                    unreachable!();
                }
            };
            values_source.push(values);
        }
        Ok(self)
    }

    /// Specify a row of values to be inserted, variation of [`InsertStatement::values`].
    pub fn values_panic<I>(&mut self, values: I) -> &mut Self
    where
        I: IntoIterator<Item = Value>,
    {
        self.values(values).unwrap()
    }

    /// Specify a row of values to be inserted, variation of [`InsertStatement::exprs`].
    pub fn exprs_panic<I>(&mut self, values: I) -> &mut Self
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        self.exprs(values).unwrap()
    }

    /// ON CONFLICT expression
    ///
    /// # Examples
    ///
    /// - [`OnConflict::update_columns`]: Update column value of existing row with inserting value
    /// - [`OnConflict::update_values`]: Update column value of existing row with value
    /// - [`OnConflict::update_exprs`]: Update column value of existing row with expression
    pub fn on_conflict(&mut self, on_conflict: OnConflict) -> &mut Self {
        self.on_conflict = Some(on_conflict);
        self
    }

    /// RETURNING expressions.
    ///
    /// ## Note:
    /// Works on
    /// * PostgreSQL
    /// * SQLite
    ///     - SQLite version >= 3.35.0
    ///     - **Note that sea-query won't try to enforce either of these constraints**
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![Glyph::Image])
    ///     .values_panic(vec!["12A".into()])
    ///     .returning(Query::select().column(Glyph::Id).take())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A')"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// ```
    pub fn returning(&mut self, select: SelectStatement) -> &mut Self {
        self.returning = select.selects;
        self
    }

    /// RETURNING a column after insertion. This is equivalent to MySQL's LAST_INSERT_ID.
    /// Wrapper over [`InsertStatement::returning()`].
    ///
    /// ## Note:
    /// Works on
    /// * PostgreSQL
    /// * SQLite
    ///     - SQLite version >= 3.35.0
    ///     - **Note that sea-query won't try to enforce either of these constraints**
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![Glyph::Image])
    ///     .values_panic(vec!["12A".into()])
    ///     .returning_col(Glyph::Id)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A')"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// ```
    pub fn returning_col<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoIden,
    {
        self.returning(Query::select().column(col.into_iden()).take())
    }

    /// Create a [WithQuery] by specifying a [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, IntoCondition, IntoIden, tests_cfg::*};
    ///
    /// let select = SelectStatement::new()
    ///         .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    ///         .from(Glyph::Table)
    ///         .to_owned();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column(Glyph::Id)
    ///         .column(Glyph::Image)
    ///         .column(Glyph::Aspect)
    ///         .table_name(Alias::new("cte"))
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let select = SelectStatement::new()
    ///         .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    ///         .from(Alias::new("cte"))
    ///         .to_owned();
    ///     let mut insert = Query::insert();
    ///     insert
    ///         .into_table(Glyph::Table)
    ///         .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    ///         .select_from(select)
    ///         .unwrap();
    ///     let query = insert.with(with_clause);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"WITH `cte` (`id`, `image`, `aspect`) AS (SELECT `id`, `image`, `aspect` FROM `glyph`) INSERT INTO `glyph` (`id`, `image`, `aspect`) SELECT `id`, `image`, `aspect` FROM `cte`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph") INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph") INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#
    /// );
    /// ```
    pub fn with(self, clause: WithClause) -> WithQuery {
        clause.query(self)
    }

    /// Insert with default values if columns and values are not supplied.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// // Insert default
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .or_default_values()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` VALUES ()"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" VALUES (DEFAULT)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" DEFAULT VALUES"#
    /// );
    ///
    /// // Ordinary insert as columns and values are supplied
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .or_default_values()
    ///     .columns(vec![Glyph::Image])
    ///     .values_panic(vec!["ABC".into()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`image`) VALUES ('ABC')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('ABC')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('ABC')"#
    /// );
    /// ```
    pub fn or_default_values(&mut self) -> &mut Self {
        self.default_values = Some(1);
        self
    }

    /// Insert multiple rows with default values if columns and values are not supplied.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// // Insert default
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .or_default_values_many(3)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` VALUES (), (), ()"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" VALUES (DEFAULT), (DEFAULT), (DEFAULT)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" DEFAULT VALUES"#
    /// );
    ///
    /// // Ordinary insert as columns and values are supplied
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .or_default_values_many(3)
    ///     .columns(vec![Glyph::Image])
    ///     .values_panic(vec!["ABC".into()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`image`) VALUES ('ABC')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('ABC')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('ABC')"#
    /// );
    /// ```
    pub fn or_default_values_many(&mut self, num_rows: u32) -> &mut Self {
        self.default_values = Some(num_rows);
        self
    }
}

impl QueryStatementBuilder for InsertStatement {
    fn build_collect_any_into(
        &self,
        query_builder: &dyn QueryBuilder,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        query_builder.prepare_insert_statement(self, sql, collector);
    }

    fn into_sub_query_statement(self) -> SubQueryStatement {
        SubQueryStatement::InsertStatement(self)
    }
}

impl QueryStatementWriter for InsertStatement {
    /// Build corresponding SQL statement for certain database backend and collect query parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![Glyph::Aspect, Glyph::Image])
    ///     .values_panic(vec![3.1415.into(), "041080".into()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (3.1415, '041080')"#
    /// );
    ///
    /// let mut params = Vec::new();
    /// let mut collector = |v| params.push(v);
    ///
    /// assert_eq!(
    ///     query.build_collect(MysqlQueryBuilder, &mut collector),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (?, ?)"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     vec![
    ///         Value::Double(Some(3.1415)),
    ///         Value::String(Some(Box::new(String::from("041080")))),
    ///     ]
    /// );
    /// ```
    fn build_collect<T: QueryBuilder>(
        &self,
        query_builder: T,
        collector: &mut dyn FnMut(Value),
    ) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_insert_statement(self, &mut sql, collector);
        sql.result()
    }
}
