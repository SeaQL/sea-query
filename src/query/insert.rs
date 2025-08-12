use crate::{
    Expr, OnConflict, QueryStatement, QueryStatementBuilder, QueryStatementWriter, ReturningClause,
    SelectStatement, SubQueryStatement, Values, WithClause, WithQuery, backend::QueryBuilder,
    error::*, prepare::*, types::*,
};
use inherent::inherent;

/// Represents a value source that can be used in an insert query.
///
/// [`InsertValueSource`] is a node in the expression tree and can represent a raw value set
/// ('VALUES') or a select query.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InsertValueSource {
    Values(Vec<Vec<Expr>>),
    Select(Box<SelectStatement>),
}

/// Insert any new rows into an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{audit::*, tests_cfg::*, *};
///
/// let query = Query::insert()
///     .into_table(Glyph::Table)
///     .columns([Glyph::Aspect, Glyph::Image])
///     .values_panic([5.15.into(), "12A".into()])
///     .values_panic([4.21.into(), "123".into()])
///     .take();
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
/// assert_eq!(
///     query.audit().unwrap().inserted_tables(),
///     [Glyph::Table.into_iden()]
/// );
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct InsertStatement {
    pub(crate) replace: bool,
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) columns: Vec<DynIden>,
    pub(crate) source: Option<InsertValueSource>,
    pub(crate) on_conflict: Option<OnConflict>,
    pub(crate) returning: Option<ReturningClause>,
    pub(crate) default_values: Option<u32>,
    pub(crate) with: Option<WithClause>,
}

impl InsertStatement {
    /// Construct a new [`InsertStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Take the ownership of data in the current [`SelectStatement`]
    pub fn take(&mut self) -> Self {
        Self {
            replace: self.replace,
            table: self.table.take(),
            columns: std::mem::take(&mut self.columns),
            source: self.source.take(),
            on_conflict: self.on_conflict.take(),
            returning: self.returning.take(),
            default_values: self.default_values.take(),
            with: self.with.take(),
        }
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
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic([5.15.into(), "12A".into()])
    ///     .take();
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

    /// Specify a select query whose values to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{audit::*, tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .select_from(Query::select()
    ///         .column(Glyph::Aspect)
    ///         .column(Glyph::Image)
    ///         .from(Glyph::Table)
    ///         .and_where(Expr::col(Glyph::Image).like("0%"))
    ///         .take()
    ///     )
    ///     .unwrap()
    ///     .take();
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
    /// assert_eq!(
    ///     query.audit().unwrap().selected_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// assert_eq!(
    ///     query.audit().unwrap().inserted_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Image])
    ///     .select_from(
    ///         Query::select()
    ///             .expr(Expr::val("hello"))
    ///             .cond_where(Cond::all().not().add(Expr::exists(
    ///                 Query::select().expr(Expr::val("world")).take(),
    ///             )))
    ///             .take(),
    ///     )
    ///     .unwrap()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") SELECT 'hello' WHERE NOT EXISTS(SELECT 'world')"#
    /// );
    /// ```
    /// use sea_query::{audit::*, tests_cfg::*, *};
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Image])
    ///     .select_from(
    ///         Query::select()
    ///             .expr(Font::Name)
    ///             .from(Font::Table)
    ///             .take(),
    ///     )
    ///     .unwrap()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") SELECT "name" FROM "font""#
    /// );
    /// assert_eq!(
    ///     query.audit().unwrap().selected_tables(),
    ///     [Font::Table.into_iden()]
    /// );
    /// assert_eq!(
    ///     query.audit().unwrap().inserted_tables(),
    ///     [Glyph::Table.into_iden()]
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
    /// Return error when number of values not matching number of columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values([
    ///         2.into(),
    ///         Func::cast_as("2020-02-02 00:00:00", "DATE").into(),
    ///     ])
    ///     .unwrap()
    ///     .take();
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
    ///
    /// assert!(
    ///     Query::insert()
    ///         .into_table(Glyph::Table)
    ///         .columns([Glyph::Aspect, Glyph::Image])
    ///         .values([1.into()])
    ///         .is_err()
    /// );
    /// ```
    pub fn values<I>(&mut self, values: I) -> Result<&mut Self>
    where
        I: IntoIterator<Item = Expr>,
    {
        let values = values.into_iter().collect::<Vec<Expr>>();
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
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic([2.1345.into(), "24B".into()])
    ///     .values_panic([5.15.into(), "12A".into()])
    ///     .take();
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
    ///
    /// The same query can be constructed using the `raw_query!` macro.
    ///
    /// ```
    /// use sea_query::Values;
    ///
    /// let values = vec![(2.1345, "24B"), (5.15, "12A")];
    /// let query = sea_query::raw_query!(
    ///     PostgresQueryBuilder,
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES {..(values.0:1),}"#
    /// );
    ///
    /// assert_eq!(
    ///     query.sql,
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES ($1, $2), ($3, $4)"#
    /// );
    /// assert_eq!(
    ///     query.values,
    ///     Values(vec![2.1345.into(), "24B".into(), 5.15.into(), "12A".into()])
    /// );
    ///
    /// // same as above but with named fields:
    ///
    /// struct Item<'a> {
    ///     aspect: f64,
    ///     image: &'a str,
    /// };
    ///
    /// let values = vec![
    ///     Item {
    ///         aspect: 2.1345,
    ///         image: "24B",
    ///     },
    ///     Item {
    ///         aspect: 5.15,
    ///         image: "12A",
    ///     },
    /// ];
    ///
    /// let new_query = sea_query::raw_query!(
    ///     PostgresQueryBuilder,
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES {..(values.aspect, values.image),}"#
    /// );
    ///
    /// assert_eq!(query.sql, new_query.sql);
    /// assert_eq!(query.values, new_query.values);
    /// ```
    pub fn values_panic<I>(&mut self, values: I) -> &mut Self
    where
        I: IntoIterator<Item = Expr>,
    {
        self.values(values).unwrap()
    }

    /// Add rows to be inserted from an iterator, variation of [`InsertStatement::values_panic`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{audit::*, tests_cfg::*, *};
    ///
    /// let rows = vec![[2.1345.into(), "24B".into()], [5.15.into(), "12A".into()]];
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_from_panic(rows)
    ///     .take();
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
    /// assert_eq!(
    ///     query.audit().unwrap().inserted_tables(),
    ///     [Glyph::Table.into_iden()]
    /// );
    /// ```
    pub fn values_from_panic<I, J>(&mut self, values_iter: J) -> &mut Self
    where
        I: IntoIterator<Item = Expr>,
        J: IntoIterator<Item = I>,
    {
        values_iter.into_iter().for_each(|values| {
            self.values_panic(values);
        });
        self
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
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Image])
    ///     .values_panic(["12A".into()])
    ///     .returning(Query::returning().columns([Glyph::Id]))
    ///     .take();
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
    pub fn returning(&mut self, returning: ReturningClause) -> &mut Self {
        self.returning = Some(returning);
        self
    }

    /// RETURNING expressions for a column.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Image])
    ///     .values_panic(["12A".into()])
    ///     .returning_col(Glyph::Id)
    ///     .take();
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
        C: IntoColumnRef,
    {
        self.returning(ReturningClause::Columns(vec![col.into_column_ref()]))
    }

    /// RETURNING expressions all columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Image])
    ///     .values_panic(["12A".into()])
    ///     .returning_all()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A')"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING *"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING *"#
    /// );
    /// ```
    pub fn returning_all(&mut self) -> &mut Self {
        self.returning(ReturningClause::All)
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
    ///         .take();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column(Glyph::Id)
    ///         .column(Glyph::Image)
    ///         .column(Glyph::Aspect)
    ///         .table_name("cte")
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let select = SelectStatement::new()
    ///         .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    ///         .from("cte")
    ///         .take();
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

    /// Create a Common Table Expression by specifying a [CommonTableExpression] or [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, IntoCondition, IntoIden, tests_cfg::*};
    ///
    /// let select = SelectStatement::new()
    ///         .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    ///         .from(Glyph::Table)
    ///         .take();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column(Glyph::Id)
    ///         .column(Glyph::Image)
    ///         .column(Glyph::Aspect)
    ///         .table_name("cte")
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let select = SelectStatement::new()
    ///         .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    ///         .from("cte")
    ///         .take();
    ///     let mut query = Query::insert();
    ///     query
    ///         .with_cte(with_clause)
    ///         .into_table(Glyph::Table)
    ///         .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    ///         .select_from(select)
    ///         .unwrap();
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
    pub fn with_cte<C: Into<WithClause>>(&mut self, clause: C) -> &mut Self {
        self.with = Some(clause.into());
        self
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
    ///     .take();
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
    ///     .columns([Glyph::Image])
    ///     .values_panic(["ABC".into()])
    ///     .take();
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
    ///     .take();
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
    ///     .columns([Glyph::Image])
    ///     .values_panic(["ABC".into()])
    ///     .take();
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

#[inherent]
impl QueryStatementBuilder for InsertStatement {
    pub fn build_collect_any_into(
        &self,
        query_builder: &dyn QueryBuilder,
        sql: &mut dyn SqlWriter,
    ) {
        query_builder.prepare_insert_statement(self, sql);
    }

    pub fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values);
    pub fn build_collect_any(
        &self,
        query_builder: &dyn QueryBuilder,
        sql: &mut dyn SqlWriter,
    ) -> String;
}

impl From<InsertStatement> for QueryStatement {
    fn from(s: InsertStatement) -> Self {
        Self::Insert(s)
    }
}

impl From<InsertStatement> for SubQueryStatement {
    fn from(s: InsertStatement) -> Self {
        Self::InsertStatement(s)
    }
}

#[inherent]
impl QueryStatementWriter for InsertStatement {
    pub fn build_collect_into<T: QueryBuilder>(&self, query_builder: T, sql: &mut dyn SqlWriter) {
        query_builder.prepare_insert_statement(self, sql);
    }

    pub fn build_collect<T: QueryBuilder>(
        &self,
        query_builder: T,
        sql: &mut dyn SqlWriter,
    ) -> String;
    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values);
    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String;
}
