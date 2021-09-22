use crate::{
    backend::QueryBuilder, error::*, prepare::*, types::*, Expr, Query, QueryStatementBuilder,
    Queryable, SelectExpr, SelectStatement, SimpleExpr,
};

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
///     query.to_string(),
///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// ```
#[derive(Debug, Default, Clone)]
pub struct InsertStatement<'a, DB> {
    pub(crate) table: Option<Box<TableRef<'a, DB>>>,
    pub(crate) columns: Vec<DynIden>,
    pub(crate) values: Vec<Vec<SimpleExpr<'a, DB>>>,
    pub(crate) returning: Vec<SelectExpr<'a, DB>>,
}

impl<'a, DB> InsertStatement<'a, DB>
where
    Self: Default,
    DB: QueryBuilder<DB>,
{
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
        T: IntoTableRef<'a, DB>,
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
    ///     query.to_string(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// ```
    pub fn values(&mut self, values: &[&'a dyn QueryValue<DB>]) -> Result<&mut Self>
    where
        DB: Default,
    {
        let values = values
            .iter()
            .map(|v| Expr::val(&**v).into())
            .collect::<Vec<SimpleExpr<'a, DB>>>();
        if self.columns.len() != values.len() {
            return Err(Error::ColValNumMismatch {
                col_len: self.columns.len(),
                val_len: values.len(),
            });
        }
        self.values.push(values);
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
    ///     query.to_string(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, CAST('2020-02-02 00:00:00' AS DATE))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, CAST('2020-02-02 00:00:00' AS DATE))"#
    /// );
    /// ```
    pub fn exprs<I>(&mut self, values: I) -> Result<&mut Self>
    where
        I: IntoIterator<Item = SimpleExpr<'a, DB>>,
    {
        let values = values.into_iter().collect::<Vec<SimpleExpr<'a, DB>>>();
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
    pub fn values_panic(&mut self, values: &[&'a dyn QueryValue<DB>]) -> &mut Self
    where
        DB: Default,
    {
        self.values(values).unwrap()
    }

    /// Specify a row of values to be inserted, variation of [`InsertStatement::exprs`].
    pub fn exprs_panic<I>(&mut self, values: I) -> &mut Self
    where
        I: IntoIterator<Item = SimpleExpr<'a, DB>>,
    {
        self.exprs(values).unwrap()
    }

    /// RETURNING expressions. Postgres only.
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
    ///     query.to_string(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A')"
    /// );
    /// ```
    pub fn returning(&mut self, select: SelectStatement<'a, DB>) -> &mut Self {
        self.returning = select.selects;
        self
    }

    /// RETURNING a column after insertion. Postgres only. This is equivalent to MySQL's LAST_INSERT_ID.
    /// Wrapper over [`InsertStatement::returning()`].
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
    ///     query.to_string(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A')"
    /// );
    /// ```
    pub fn returning_col<C>(&mut self, col: C) -> &mut Self
    where
        DB: Default,
        Query: Queryable<DB>,
        C: IntoIden,
    {
        self.returning(Query::select().column(col.into_iden()).take())
    }
}

impl<'a, DB> QueryStatementBuilder<'a, DB> for InsertStatement<'a, DB>
where
    DB: QueryBuilder<DB> + Default + 'a,
{
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
    fn build_collect(&'a self, collector: &mut dyn FnMut(&'a dyn QueryValue<DB>)) -> String {
        let mut sql = SqlWriter::new();
        DB::default().prepare_insert_statement(self, &mut sql, collector);
        sql.result()
    }
}
