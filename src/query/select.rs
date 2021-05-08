use std::rc::Rc;
use crate::{backend::QueryBuilder, types::*, expr::*, value::*, prepare::*};
use std::iter::FromIterator;

/// Select rows from an existing table
/// 
/// # Examples
/// 
/// ```
/// use sea_query::{*, tests_cfg::*};
/// 
/// let query = Query::select()
///     .column(Char::Character)
///     .table_column(Font::Table, Font::Name)
///     .from(Char::Table)
///     .left_join(Font::Table, Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id))
///     .and_where(Expr::col(Char::SizeW).is_in(vec![3, 4]))
///     .and_where(Expr::col(Char::Character).like("A%"))
///     .to_owned();
/// 
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` WHERE `size_w` IN (3, 4) AND `character` LIKE 'A%'"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "character" LIKE 'A%'"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` WHERE `size_w` IN (3, 4) AND `character` LIKE 'A%'"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub(crate) distinct: Option<SelectDistinct>,
    pub(crate) selects: Vec<SelectExpr>,
    pub(crate) from: Option<Box<TableRef>>,
    pub(crate) join: Vec<JoinExpr>,
    pub(crate) wherei: Vec<LogicalChainOper>,
    pub(crate) groups: Vec<SimpleExpr>,
    pub(crate) having: Vec<LogicalChainOper>,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
    pub(crate) offset: Option<Value>,
}

/// List of distinct keywords that can be used in select statement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectDistinct {
    All,
    Distinct,
    DistinctRow,
}

/// Select expression used in select statement
#[derive(Debug, Clone)]
pub struct SelectExpr {
    pub expr: SimpleExpr,
    pub alias: Option<Rc<dyn Iden>>,
}

/// Join expression used in select statement
#[derive(Debug, Clone)]
pub struct JoinExpr {
    pub join: JoinType,
    pub table: Box<TableRef>,
    pub on: Option<JoinOn>,
}

impl Into<SelectExpr> for SimpleExpr {
    fn into(self) -> SelectExpr {
        SelectExpr {
            expr: self,
            alias: None,
        }
    }
}

impl Default for SelectStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectStatement {
    /// Construct a new [`SelectStatement`]
    pub fn new() -> Self {
        Self {
            distinct: None,
            selects: Vec::new(),
            from: None,
            join: Vec::new(),
            wherei: Vec::new(),
            groups: Vec::new(),
            having: Vec::new(),
            orders: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// Take the ownership of data in the current [`SelectStatement`]
    pub fn take(&mut self) -> Self {
        Self {
            distinct: self.distinct.take(),
            selects: std::mem::replace(&mut self.selects, Vec::new()),
            from: self.from.take(),
            join: std::mem::replace(&mut self.join, Vec::new()),
            wherei: std::mem::replace(&mut self.wherei, Vec::new()),
            groups: std::mem::replace(&mut self.groups, Vec::new()),
            having: std::mem::replace(&mut self.having, Vec::new()),
            orders: std::mem::replace(&mut self.orders, Vec::new()),
            limit: self.limit.take(),
            offset: self.offset.take(),
        }
    }

    /// A shorthand to express if ... else ... when constructing the select statement.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .conditions(
    ///         true,
    ///         |x| { x.and_where(Expr::col(Char::FontId).eq(5)); },
    ///         |x| { x.and_where(Expr::col(Char::FontId).eq(10)); }
    ///     )
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5"#
    /// );
    /// ```
    pub fn conditions<T, F>(&mut self, b: bool, if_true: T, if_false: F) -> &mut Self
        where T: FnOnce(&mut Self), F: FnOnce(&mut Self) {
        if b {
            if_true(self)
        } else {
            if_false(self)
        }
        self
    }

    /// Add an expression to the select expression list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr(Expr::val(42))
    ///     .expr(Expr::col(Char::Id).max())
    ///     .expr((1..10_i32).fold(Expr::value(0), |expr, i| {
    ///         expr.add(Expr::value(i))
    ///     }))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT 42, MAX(`id`), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT 42, MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT 42, MAX(`id`), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM `character`"#
    /// );
    /// ```
    pub fn expr<T>(&mut self, expr: T) -> &mut Self
        where T: Into<SelectExpr> {
        self.selects.push(expr.into());
        self
    }

    /// Add select expressions from vector of [`SelectExpr`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .exprs(vec![
    ///         Expr::col(Char::Id).max(),
    ///         (1..10_i32).fold(Expr::value(0), |expr, i| {
    ///             expr.add(Expr::value(i))
    ///         }),
    ///     ])
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`id`), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX(`id`), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM `character`"#
    /// );
    /// ```
    pub fn exprs<T, I>(&mut self, exprs: I) -> &mut Self
    where
        T: Into<SelectExpr>,
        I: IntoIterator<Item = T>,
    {
        self.selects
            .append(&mut exprs.into_iter().map(|c| c.into()).collect());
        self
    }

    /// Select distinct
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = Some(SelectDistinct::Distinct);
        self
    }

    /// Add a column to the select expression list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .column(Char::Character)
    ///     .column(Char::SizeW)
    ///     .column(Char::SizeH)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character`"#
    /// );
    /// ```
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .column((Char::Table, Char::Character))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`.`character` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character"."character" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`.`character` FROM `character`"#
    /// );
    /// ```
    pub fn column<C>(&mut self, col: C) -> &mut Self
        where C: IntoColumnRef {
        self.expr(SimpleExpr::Column(col.into_column_ref()))
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`SelectStatement::column`] with a tuple as [`ColumnRef`]"
    )]
    pub fn table_column<T, C>(&mut self, t: T, c: C) -> &mut Self
        where T: IntoIden, C: IntoIden {
        self.column((t.into_iden(), c.into_iden()))
    }

    /// Select columns.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .columns(vec![
    ///         Char::Character,
    ///         Char::SizeW,
    ///         Char::SizeH,
    ///     ])
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character`"#
    /// );
    /// ```
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .columns(vec![
    ///         (Char::Table, Char::Character),
    ///         (Char::Table, Char::SizeW),
    ///         (Char::Table, Char::SizeH),
    ///     ])
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`.`character`, `character`.`size_w`, `character`.`size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character"."character", "character"."size_w", "character"."size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`.`character`, `character`.`size_w`, `character`.`size_h` FROM `character`"#
    /// );
    /// ```
    pub fn columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        self.exprs(
            cols.into_iter()
                .map(|c| SimpleExpr::Column(c.into_column_ref()))
                .collect::<Vec<SimpleExpr>>(),
        )
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`SelectStatement::columns`] with a tuple as [`ColumnRef`]"
    )]
    pub fn table_columns<T, C>(&mut self, cols: Vec<(T, C)>) -> &mut Self
    where
        T: IntoIden,
        C: IntoIden,
    {
        self.columns(
            cols.into_iter()
                .map(|(t, c)| (t.into_iden(), c.into_iden()))
                .collect::<Vec<_>>(),
        )
    }

    /// Select column.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_as(Expr::col(Char::Character), Alias::new("C"))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` AS `C` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" AS "C" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character` AS `C` FROM `character`"#
    /// );
    /// ```
    pub fn expr_as<T, A>(&mut self, expr: T, alias: A) -> &mut Self
        where T: Into<SimpleExpr>, A: IntoIden {
        self.expr(SelectExpr {
            expr: expr.into(),
            alias: Some(alias.into_iden())
        });
        self
    }

    #[deprecated(
        since = "0.6.1",
        note = "Please use the [`SelectStatement::expr_as`] instead"
    )]
    pub fn expr_alias<T, A>(&mut self, expr: T, alias: A) -> &mut Self
        where T: Into<SimpleExpr>, A: IntoIden {
        self.expr_as(expr, alias)
    }

    /// From table.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::FontSize)
    ///     .from(Char::Table)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`"#
    /// );
    /// ```
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::FontSize)
    ///     .from((Char::Table, Glyph::Table))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`.`glyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character"."glyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`.`glyph`"#
    /// );
    /// ```
    pub fn from<R>(&mut self, tbl_ref: R) -> &mut Self
        where R: IntoTableRef {
        self.from_from(tbl_ref.into_table_ref())
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`SelectStatement::from`] with a tuple as [`TableRef`]"
    )]
    /// From schema.table.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::FontSize)
    ///     .from_schema(Char::Table, Glyph::Table)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`.`glyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character"."glyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`.`glyph`"#
    /// );
    /// ```
    pub fn from_schema<S: 'static, T: 'static>(&mut self, schema: S, table: T) -> &mut Self
        where S: IntoIden, T: IntoIden {
        self.from((schema, table))
    }

    /// From table with alias.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::rc::Rc;
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let table_as: Rc<dyn Iden> = Rc::new(Alias::new("char"));
    /// 
    /// let query = Query::select()
    ///     .from_as(Char::Table, table_as.clone())
    ///     .table_column(table_as.clone(), Char::Character)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `char`.`character` FROM `character` AS `char`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "char"."character" FROM "character" AS "char""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `char`.`character` FROM `character` AS `char`"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let table_as = Alias::new("alias");
    /// 
    /// let query = Query::select()
    ///     .from_as((Font::Table, Char::Table), table_as.clone())
    ///     .table_column(table_as, Char::Character)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `alias`.`character` FROM `font`.`character` AS `alias`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "alias"."character" FROM "font"."character" AS "alias""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `alias`.`character` FROM `font`.`character` AS `alias`"#
    /// );
    /// ```
    pub fn from_as<R, A>(&mut self, tbl_ref: R, alias: A) -> &mut Self
        where R: IntoTableRef, A: IntoIden {
        self.from_from(tbl_ref.into_table_ref().alias(alias.into_iden()))
    }

    #[deprecated(
        since = "0.6.1",
        note = "Please use the [`SelectStatement::from_as`] instead"
    )]
    pub fn from_alias<R, A>(&mut self, tbl_ref: R, alias: A) -> &mut Self
        where R: IntoTableRef, A: IntoIden {
        self.from_as(tbl_ref, alias)
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`SelectStatement::from_as`] with a tuple as [`TableRef`]"
    )]
    pub fn from_schema_as<S: 'static, T: 'static, A>(&mut self, schema: S, table: T, alias: A) -> &mut Self
        where S: IntoIden, T: IntoIden, A: IntoIden {
        self.from_as((schema, table), alias)
    }

    /// From sub-query.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .columns(vec![
    ///         Glyph::Image
    ///     ])
    ///     .from_subquery(
    ///         Query::select()
    ///             .columns(vec![
    ///                 Glyph::Image, Glyph::Aspect
    ///             ])
    ///             .from(Glyph::Table)
    ///             .take(),
    ///         Alias::new("subglyph")
    ///     )
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM (SELECT `image`, `aspect` FROM `glyph`) AS `subglyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "image" FROM (SELECT "image", "aspect" FROM "glyph") AS "subglyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `image` FROM (SELECT `image`, `aspect` FROM `glyph`) AS `subglyph`"#
    /// );
    /// ```
    pub fn from_subquery<T>(&mut self, query: SelectStatement, alias: T) -> &mut Self
        where T: IntoIden {
        self.from_from(TableRef::SubQuery(query, alias.into_iden()))
    }

    fn from_from(&mut self, select: TableRef) -> &mut Self {
        self.from = Some(Box::new(select));
        self
    }

    /// Left join.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .table_column(Font::Table, Font::Name)
    ///     .from(Char::Table)
    ///     .left_join(Font::Table, Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// ```
    pub fn left_join<R>(&mut self, tbl_ref: R, condition: SimpleExpr) -> &mut Self 
        where R: IntoTableRef {
        self.join(JoinType::LeftJoin, tbl_ref, condition)
    }

    /// Inner join.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .table_column(Font::Table, Font::Name)
    ///     .from(Char::Table)
    ///     .inner_join(Font::Table, Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` INNER JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` INNER JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// ```
    pub fn inner_join<R>(&mut self, tbl_ref: R, condition: SimpleExpr) -> &mut Self 
        where R: IntoTableRef {
        self.join(JoinType::InnerJoin, tbl_ref, condition)
    }

    /// Join with other table by [`JoinType`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .table_column(Font::Table, Font::Name)
    ///     .from(Char::Table)
    ///     .join(JoinType::RightJoin, Font::Table, Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// ```
    pub fn join<R>(&mut self, join: JoinType, tbl_ref: R, condition: SimpleExpr) -> &mut Self 
        where R: IntoTableRef {
        self.join_join(join, tbl_ref.into_table_ref(), JoinOn::Condition(Box::new(condition)))
    }

    /// Join with other table by [`JoinType`], assigning an alias to the joined table.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .table_column(Font::Table, Font::Name)
    ///     .from(Char::Table)
    ///     .join_as(
    ///         JoinType::RightJoin,
    ///         Font::Table,
    ///         Alias::new("f"),
    ///         Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id)
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` AS `f` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" AS "f" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` AS `f` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// ```
    pub fn join_as<R, A>(&mut self, join: JoinType, tbl_ref: R, alias: A, condition: SimpleExpr) -> &mut Self 
        where R: IntoTableRef, A: IntoIden {
        self.join_join(join, tbl_ref.into_table_ref().alias(alias.into_iden()), JoinOn::Condition(Box::new(condition)))
    }

    #[deprecated(
        since = "0.6.1",
        note = "Please use the [`SelectStatement::join_as`] instead"
    )]
    pub fn join_alias<R, A>(&mut self, join: JoinType, tbl_ref: R, alias: A, condition: SimpleExpr) -> &mut Self 
        where R: IntoTableRef, A: IntoIden {
        self.join_as(join, tbl_ref, alias, condition)
    }

    /// Join with sub-query.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::rc::Rc;
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let sub_glyph: Rc<dyn Iden> = Rc::new(Alias::new("sub_glyph"));
    /// let query = Query::select()
    ///     .column(Font::Name)
    ///     .from(Font::Table)
    ///     .join_subquery(
    ///         JoinType::LeftJoin,
    ///         Query::select()
    ///             .column(Glyph::Id)
    ///             .from(Glyph::Table)
    ///             .take(),
    ///         sub_glyph.clone(),
    ///         Expr::tbl(Font::Table, Font::Id).equals(sub_glyph.clone(), Glyph::Id)
    ///     )
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name" FROM "font" LEFT JOIN (SELECT "id" FROM "glyph") AS "sub_glyph" ON "font"."id" = "sub_glyph"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// ```
    /// 
    pub fn join_subquery<T>(&mut self, join: JoinType, query: SelectStatement, alias: T, condition: SimpleExpr) -> &mut Self
        where T: IntoIden {
        self.join_join(join, TableRef::SubQuery(query, alias.into_iden()), JoinOn::Condition(Box::new(condition)))
    }

    fn join_join(&mut self, join: JoinType, table: TableRef, on: JoinOn) -> &mut Self {
        self.join.push(JoinExpr {
            join,
            table: Box::new(table),
            on: Some(on),
        });
        self
    }

    /// Group by columns.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .table_column(Font::Table, Font::Name)
    ///     .from(Char::Table)
    ///     .join(JoinType::RightJoin, Font::Table, Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id))
    ///     .group_by_columns(vec![
    ///         Char::Character,
    ///     ])
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`"#
    /// );
    /// ```
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .table_column(Font::Table, Font::Name)
    ///     .from(Char::Table)
    ///     .join(JoinType::RightJoin, Font::Table, Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id))
    ///     .group_by_columns(vec![
    ///         (Char::Table, Char::Character),
    ///     ])
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`.`character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`.`character`"#
    /// );
    /// ```
    pub fn group_by_columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        self.add_group_by(
            cols.into_iter()
                .map(|c| SimpleExpr::Column(c.into_column_ref()))
                .collect::<Vec<_>>(),
        )
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`SelectStatement::group_by_columns`] with a tuple as [`ColumnRef`]"
    )]
    pub fn group_by_table_columns<T, C>(&mut self, cols: Vec<(T, C)>) -> &mut Self
    where
        T: IntoIden,
        C: IntoIden,
    {
        self.group_by_columns(
            cols.into_iter()
                .map(|(t, c)| (t.into_iden(), c.into_iden()))
                .collect::<Vec<_>>(),
        )
    }

    /// And where condition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .table_column(Glyph::Table, Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
    ///     .and_where(Expr::tbl(Glyph::Table, Glyph::Image).like("A%"))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `glyph`.`image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "glyph"."image" FROM "glyph" WHERE "glyph"."aspect" IN (3, 4) AND "glyph"."image" LIKE 'A%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `glyph`.`image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    pub fn and_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.wherei.push(LogicalChainOper::And(other));
        self
    }

    /// And where condition, short hand for `if c.is_some() q.and_where(c)`.
    pub fn and_where_option(&mut self, other: Option<SimpleExpr>) -> &mut Self {
        if let Some(other) = other {
            self.wherei.push(LogicalChainOper::And(other));
        }
        self
    }

    /// Or where condition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .table_column(Glyph::Table, Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .or_where(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
    ///     .or_where(Expr::tbl(Glyph::Table, Glyph::Image).like("A%"))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `glyph`.`image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "glyph"."image" FROM "glyph" WHERE "glyph"."aspect" IN (3, 4) OR "glyph"."image" LIKE 'A%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `glyph`.`image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    pub fn or_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.wherei.push(LogicalChainOper::Or(other));
        self
    }

    /// Add group by expressions from vector of [`SelectExpr`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .column(Char::Character)
    ///     .add_group_by(vec![
    ///         Expr::col(Char::SizeW).into(),
    ///         Expr::col(Char::SizeH).into(),
    ///     ])
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` GROUP BY `size_w`, `size_h`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" GROUP BY "size_w", "size_h""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `character` FROM `character` GROUP BY `size_w`, `size_h`"#
    /// );
    /// ```
    pub fn add_group_by<I>(&mut self, expr: I) -> &mut Self
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        self.groups.append(&mut Vec::from_iter(expr.into_iter()));
        self
    }

    /// And having condition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .expr(Expr::col(Glyph::Image).max())
    ///     .from(Glyph::Table)
    ///     .group_by_columns(vec![
    ///         Glyph::Aspect,
    ///     ])
    ///     .and_having(Expr::col(Glyph::Aspect).gt(2))
    ///     .and_having(Expr::col(Glyph::Aspect).lt(8))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `aspect` > 2 AND `aspect` < 8"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2 AND "aspect" < 8"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `aspect` > 2 AND `aspect` < 8"#
    /// );
    /// ```
    pub fn and_having(&mut self, other: SimpleExpr) -> &mut Self {
        self.having.push(LogicalChainOper::And(other));
        self
    }

    /// Or having condition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .expr(Expr::col(Glyph::Image).max())
    ///     .from(Glyph::Table)
    ///     .group_by_columns(vec![
    ///         Glyph::Aspect,
    ///     ])
    ///     .or_having(Expr::col(Glyph::Aspect).lt(1))
    ///     .or_having(Expr::col(Glyph::Aspect).gt(10))
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `aspect` < 1 OR `aspect` > 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" < 1 OR "aspect" > 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `aspect` < 1 OR `aspect` > 10"#
    /// );
    /// ```
    pub fn or_having(&mut self, other: SimpleExpr) -> &mut Self {
        self.having.push(LogicalChainOper::Or(other));
        self
    }

    /// Order by column.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
    ///     .order_by(Glyph::Image, Order::Desc)
    ///     .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" WHERE COALESCE("aspect", 0) > 2 ORDER BY "image" DESC, "glyph"."aspect" ASC"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// ```
    pub fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self 
        where T: IntoColumnRef {
        self.orders.push(OrderExpr {
            expr: SimpleExpr::Column(col.into_column_ref()),
            order,
        });
        self
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`SelectStatement::order_by`] with a tuple as [`ColumnRef`]"
    )]
    pub fn order_by_tbl<T, C>
        (&mut self, table: T, col: C, order: Order) -> &mut Self 
        where T: IntoIden, C: IntoIden {
        self.order_by((table.into_iden(), col.into_iden()), order)
    }

    /// Order by [`SimpleExpr`].
    pub fn order_by_expr(&mut self, expr: SimpleExpr, order: Order) -> &mut Self {
        self.orders.push(OrderExpr {
            expr,
            order,
        });
        self
    }

    /// Order by custom string expression.
    pub fn order_by_customs<T: 'static, I>(&mut self, cols: I) -> &mut Self
    where
        T: ToString,
        I: IntoIterator<Item = (T, Order)>,
    {
        let mut orders = cols
            .into_iter()
            .map(|(c, order)| OrderExpr {
                expr: SimpleExpr::Custom(c.to_string()),
                order,
            })
            .collect();
        self.orders.append(&mut orders);
        self
    }

    /// Order by vector of columns.
    pub fn order_by_columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order)>,
    {
        let mut orders = cols
            .into_iter()
            .map(|(c, order)| OrderExpr {
                expr: SimpleExpr::Column(c.into_column_ref()),
                order,
            })
            .collect();
        self.orders.append(&mut orders);
        self
    }

    #[deprecated(
        since = "0.9.0",
        note = "Please use the [`SelectStatement::order_by_columns`] with a tuple as [`ColumnRef`]"
    )]
    pub fn order_by_table_columns<T, C>(&mut self, cols: Vec<(T, C, Order)>) -> &mut Self
    where
        T: IntoIden,
        C: IntoIden,
    {
        self.order_by_columns(
            cols.into_iter()
                .map(|(t, c, o)| ((t.into_iden(), c.into_iden()), o))
                .collect::<Vec<_>>(),
        )
    }

    /// Limit the number of returned rows.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .limit(10)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10"#
    /// );
    /// ```
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(Value::BigUnsigned(limit));
        self
    }

    /// Offset number of returned rows.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .limit(10)
    ///     .offset(10)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10 OFFSET 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10 OFFSET 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10 OFFSET 10"#
    /// );
    /// ```
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(Value::BigUnsigned(offset));
        self
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
    ///     .order_by(Glyph::Image, Order::Desc)
    ///     .order_by_tbl(Glyph::Table, Glyph::Aspect, Order::Asc)
    ///     .to_owned();
    /// 
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// 
    /// let mut params = Vec::new();
    /// let mut collector = |v| params.push(v);
    /// 
    /// assert_eq!(
    ///     query.build_collect(MysqlQueryBuilder, &mut collector),
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, ?) > ? ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     vec![Value::Int(0), Value::Int(2)]
    /// );
    /// ```
    pub fn build_collect<T: QueryBuilder>(&self, query_builder: T, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_select_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters
    pub fn build_collect_any(&self, query_builder: &dyn QueryBuilder, collector: &mut dyn FnMut(Value)) -> String {
        let mut sql = SqlWriter::new();
        query_builder.prepare_select_statement(self, &mut sql, collector);
        sql.result()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let (query, params) = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
    ///     .order_by(Glyph::Image, Order::Desc)
    ///     .order_by_tbl(Glyph::Table, Glyph::Aspect, Order::Asc)
    ///     .build(MysqlQueryBuilder);
    /// 
    /// assert_eq!(
    ///     query,
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, ?) > ? ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     Values(vec![Value::Int(0), Value::Int(2)])
    /// );
    /// ```
    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values) {
        let mut values = Vec::new();
        let mut collector = |v| values.push(v);
        let sql = self.build_collect(query_builder, &mut collector);
        (sql, Values(values))
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    pub fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values) {
        let mut values = Vec::new();
        let mut collector = |v| values.push(v);
        let sql = self.build_collect_any(query_builder, &mut collector);
        (sql, Values(values))
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
    ///     .order_by(Glyph::Image, Order::Desc)
    ///     .order_by_tbl(Glyph::Table, Glyph::Aspect, Order::Asc)
    ///     .to_string(MysqlQueryBuilder);
    /// 
    /// assert_eq!(
    ///     query,
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// ```
    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
        let (sql, values) = self.build_any(&query_builder);
        inject_parameters(&sql, values.0, &query_builder)
    }
}
