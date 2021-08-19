use crate::{
    backend::QueryBuilder, error::*, prepare::*, types::*, value::*, Query, QueryStatementBuilder,
    SelectExpr, SelectStatement,
};

/// Insert any new rows into an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::insert()
///     .into_table(Glyph::Table)
///     .columns(vec![
///         Glyph::Aspect,
///         Glyph::Image,
///     ])
///     .values_panic(vec![
///         5.15.into(),
///         "12A".into(),
///     ])
///     .values_panic(vec![
///         4.21.into(),
///         "123".into(),
///     ])
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
///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// ```
#[derive(Debug, Default, Clone)]
pub struct InsertStatement {
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) columns: Vec<DynIden>,
    pub(crate) values: Vec<Vec<Value>>,
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
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Aspect,
    ///         Glyph::Image,
    ///     ])
    ///     .values(vec![
    ///         2.1345.into(),
    ///         "24B".into(),
    ///     ])
    ///     .unwrap()
    ///     .values_panic(vec![
    ///         5.15.into(),
    ///         "12A".into(),
    ///     ])
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
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// ```
    pub fn values<I>(&mut self, values: I) -> Result<&mut Self>
    where
        I: IntoIterator<Item = Value>,
    {
        let values = values.into_iter().collect::<Vec<_>>();
        if self.columns.len() != values.len() {
            return Err(Error::ColValNumMismatch {
                col_len: self.columns.len(),
                val_len: values.len(),
            });
        }
        self.values.push(values);
        Ok(self)
    }

    /// Specify a row of values to be inserted, variation of [`InsertStatement::values`].
    pub fn values_panic<I>(&mut self, values: I) -> &mut Self
    where
        I: IntoIterator<Item = Value>,
    {
        self.values(values).unwrap()
    }

    /// RETURNING expressions. Postgres only.
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Image,
    ///     ])
    ///     .values_panic(vec![
    ///         "12A".into(),
    ///     ])
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
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A')"
    /// );
    /// ```
    pub fn returning(&mut self, select: SelectStatement) -> &mut Self {
        self.returning = select.selects;
        self
    }

    /// RETURNING a column after insertion. Postgres only. This is equivalent to MySQL's LAST_INSERT_ID.
    /// Wrapper over [`InsertStatement::returning()`].
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Image,
    ///     ])
    ///     .values_panic(vec![
    ///         "12A".into(),
    ///     ])
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
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A')"
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
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns(vec![
    ///         Glyph::Aspect,
    ///         Glyph::Image,
    ///     ])
    ///     .values_panic(vec![
    ///         3.1415.into(),
    ///         "041080".into(),
    ///     ])
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
