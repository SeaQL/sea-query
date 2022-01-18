use crate::{
    backend::QueryBuilder, error::*, prepare::*, types::*, value::*, Expr, Query,
    QueryStatementBuilder, SelectExpr, SelectStatement, SimpleExpr,
};

/// Represents a value source that can be used in an insert query.
///
/// [`InsertValueSource`] is a node in the expression tree and can represent a raw value set
/// ('VALUES') or a select query.
#[derive(Debug, Clone)]
enum InsertValueSource {
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
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) columns: Vec<DynIden>,
    pub(crate) source: Option<InsertValueSource>,
    pub(crate) returning: Vec<SelectExpr>,
}

impl InsertStatement {
    /// Construct a new [`InsertStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify which table to insert into.
    ///
    /// # Examples
    ///
    /// See [`InsertStatement::values`]
    #[allow(clippy::wrong_self_convention)]
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

    /// Specify the value source of the insert.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![Glyph::Aspect, Glyph::Image])
    ///     .value_source(InsertValueSource::Select(Box::new(Query::select()
    ///         .column(Glyph::Aspect)
    ///         .column(Glyph::Image)
    ///         .from(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Image).like("0%"))
    ///         .to_owned()))
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
    fn value_source<S>(&mut self, source: S) -> Result<&mut InsertStatement>
    where
        S: Into<InsertValueSource>,
    {
        match source.into() {
            InsertValueSource::Values(values) => {
                for value in values {
                    self.exprs(value)?;
                }
                Ok(self)
            }
            InsertValueSource::Select(select) => self.select_from(*select),
        }
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
}

impl QueryStatementBuilder for InsertStatement {
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

    fn build_collect_any(
        &self,
        query_builder: &dyn QueryBuilder,
        collector: &mut dyn FnMut(Value),
    ) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_insert_statement(self, &mut sql, collector);
        sql.result()
    }
}
