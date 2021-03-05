use std::rc::Rc;
use crate::{backend::QueryBuilder, types::*, expr::*, value::*, prepare::*};

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
#[derive(Clone)]
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectDistinct {
    All,
    Distinct,
    DistinctRow,
}

/// Select expression used in select statement
#[derive(Clone)]
pub struct SelectExpr {
    pub expr: SimpleExpr,
    pub alias: Option<Rc<dyn Iden>>,
}

/// Join expression used in select statement
#[derive(Clone)]
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

    /// Add a new select expression from [`SelectExpr`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr(Expr::col(Char::Id).max())
    ///     .expr((1..10_i32).fold(Expr::value(0), |expr, i| {
    ///         expr.add(Expr::value(i))
    ///     }))
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
    pub fn expr<T: 'static>(&mut self, expr: T) -> &mut Self
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
    pub fn exprs<T: 'static>(&mut self, exprs: Vec<T>) -> &mut Self
        where T: Into<SelectExpr> {
        self.selects.append(&mut exprs.into_iter().map(|c| c.into()).collect());
        self
    }

    /// Select distinct
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = Some(SelectDistinct::Distinct);
        self
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
    pub fn column<C: 'static>(&mut self, col: C) -> &mut Self
        where C: Iden {
        self.column_dyn(Rc::new(col))
    }

    /// Select column, variation of [`SelectStatement::column`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// use std::rc::Rc;
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .column_dyn(Rc::new(Char::Character))
    ///     .column_dyn(Rc::new(Char::SizeW))
    ///     .column_dyn(Rc::new(Char::SizeH))
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
    pub fn column_dyn(&mut self, col: Rc<dyn Iden>) -> &mut Self {
        self.expr(SimpleExpr::Column(col));
        self
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
    pub fn columns<T: 'static>(&mut self, cols: Vec<T>) -> &mut Self
        where T: Iden {
        self.columns_dyn(cols.into_iter().map(|c| Rc::new(c) as Rc<dyn Iden>).collect())
    }

    /// Select column, variation of [`SelectStatement::column`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// use std::rc::Rc;
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .columns_dyn(vec![
    ///         Rc::new(Char::Character),
    ///         Rc::new(Char::SizeW),
    ///         Rc::new(Char::SizeH),
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
    pub fn columns_dyn(&mut self, cols: Vec<Rc<dyn Iden>>) -> &mut Self {
        self.exprs(cols.into_iter().map(|c| SimpleExpr::Column(c)).collect());
        self
    }

    /// Select column with table prefix.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .table_column(Char::Table, Char::Character)
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
    pub fn table_column<T: 'static, C: 'static>(&mut self, t: T, c: C) -> &mut Self
        where T: Iden, C: Iden {
        self.table_column_dyn(Rc::new(t), Rc::new(c))
    }

    /// Select column with table prefix, variation of [`SelectStatement::table_column`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// use std::rc::Rc;
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .table_column_dyn(Rc::new(Char::Table), Rc::new(Char::Character))
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
    pub fn table_column_dyn(&mut self, t: Rc<dyn Iden>, c: Rc<dyn Iden>) -> &mut Self {
        self.expr(SimpleExpr::TableColumn(t, c));
        self
    }

    /// Select columns with table prefix.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .table_columns(vec![
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
    pub fn table_columns<T: 'static, C: 'static>(&mut self, cols: Vec<(T, C)>) -> &mut Self
        where T: Iden, C: Iden {
        self.table_columns_dyn(cols.into_iter().map(
            |(t, c)| (Rc::new(t) as Rc<dyn Iden>, Rc::new(c) as Rc<dyn Iden>)
        ).collect())
    }

    /// Select columns with table prefix, variation of [`SelectStatement::table_columns_dyn`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// use std::rc::Rc;
    /// 
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .table_columns_dyn(vec![
    ///         (Rc::new(Char::Table), Rc::new(Char::Character)),
    ///         (Rc::new(Char::Table), Rc::new(Char::SizeW)),
    ///         (Rc::new(Char::Table), Rc::new(Char::SizeH)),
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
    pub fn table_columns_dyn(&mut self, cols: Vec<(Rc<dyn Iden>, Rc<dyn Iden>)>) -> &mut Self {
        self.exprs(cols.into_iter().map(|(t, c)| SimpleExpr::TableColumn(t, c)).collect());
        self
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
    ///     .expr_alias(Expr::col(Char::Character), Alias::new("C"))
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
    pub fn expr_alias<T, A: 'static>(&mut self, expr: T, alias: A) -> &mut Self
        where T: Into<SimpleExpr>, A: Iden {
        self.expr(SelectExpr {
            expr: expr.into(),
            alias: Some(Rc::new(alias))
        });
        self
    }

    /// From table.
    pub fn from<T: 'static>(&mut self, table: T) -> &mut Self
        where T: Iden {
        self.from_dyn(Rc::new(table))
    }

    /// From table, variation of [`SelectStatement::from`].
    pub fn from_dyn(&mut self, table: Rc<dyn Iden>) -> &mut Self {
        self.from_from(TableRef::Table(table));
        self
    }

    /// From table with alias.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    /// 
    /// let table_alias = Alias::new("char");
    /// 
    /// let query = Query::select()
    ///     .from_alias(Char::Table, table_alias.clone())
    ///     .table_column(table_alias, Char::Character)
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
    pub fn from_alias<T: 'static, Y: 'static>(&mut self, table: T, alias: Y) -> &mut Self
        where T: Iden, Y: Iden {
        self.from_alias_dyn(Rc::new(table), Rc::new(alias))
    }

    /// From table with alias, variation of [`SelectStatement::from_alias`].
    pub fn from_alias_dyn(&mut self, table: Rc<dyn Iden>, alias: Rc<dyn Iden>) -> &mut Self {
        self.from_from(TableRef::TableAlias(table, alias));
        self
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
    pub fn from_subquery<T: 'static>(&mut self, query: SelectStatement, alias: T) -> &mut Self
        where T: Iden {
        self.from_subquery_dyn(query, Rc::new(alias))
    }

    /// From sub-query, variation of [`SelectStatement::from_subquery`].
    pub fn from_subquery_dyn(&mut self, query: SelectStatement, alias: Rc<dyn Iden>) -> &mut Self {
        self.from_from(TableRef::SubQuery(query, alias));
        self
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
    pub fn left_join<T: 'static>(&mut self, table: T, condition: SimpleExpr) -> &mut Self 
        where T: Iden {
        self.left_join_dyn(Rc::new(table), condition)
    }

    /// Left join, variation of [`SelectStatement::left_join`].
    pub fn left_join_dyn(&mut self, table: Rc<dyn Iden>, condition: SimpleExpr) -> &mut Self {
        self.join_join(JoinType::LeftJoin, TableRef::Table(table), JoinOn::Condition(Box::new(condition)));
        self
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
    pub fn inner_join<T: 'static>(&mut self, table: T, condition: SimpleExpr) -> &mut Self 
        where T: Iden {
        self.inner_join_dyn(Rc::new(table), condition)
    }

    /// Inner join, variation of [`SelectStatement::inner_join`].
    pub fn inner_join_dyn(&mut self, table: Rc<dyn Iden>, condition: SimpleExpr) -> &mut Self {
        self.join_join(JoinType::InnerJoin, TableRef::Table(table), JoinOn::Condition(Box::new(condition)));
        self
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
    pub fn join<T: 'static>(&mut self, join: JoinType, table: T, condition: SimpleExpr) -> &mut Self 
        where T: Iden {
        self.join_dyn(join, Rc::new(table), condition)
    }

    /// Join with other table by [`JoinType`], variation of [`SelectStatement::join`].
    pub fn join_dyn(&mut self, join: JoinType, table: Rc<dyn Iden>, condition: SimpleExpr) -> &mut Self {
        self.join_join(join, TableRef::Table(table), JoinOn::Condition(Box::new(condition)));
        self
    }

    /// Join with sub-query.
    /// 
    /// # Examples
    /// 
    /// ...
    /// 
    pub fn join_subquery<T: 'static>(&mut self, join: JoinType, query: SelectStatement, alias: T, condition: SimpleExpr) -> &mut Self
        where T: Iden {
        self.join_subquery_dyn(join, query, Rc::new(alias), condition)
    }

    /// Join with sub-query, variation of [`SelectStatement::join_subquery`].
    pub fn join_subquery_dyn(&mut self, join: JoinType, query: SelectStatement, alias: Rc<dyn Iden>, condition: SimpleExpr) -> &mut Self {
        self.join_join(join, TableRef::SubQuery(query, alias), JoinOn::Condition(Box::new(condition)));
        self
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
    pub fn group_by_columns<T: 'static>(&mut self, cols: Vec<T>) -> &mut Self
        where T: Iden {
        self.group_by_columns_dyn(cols.into_iter().map(|c| Rc::new(c) as Rc<dyn Iden>).collect())
    }

    /// Group by columns, variation of [`SelectStatement::group_by_columns`].
    pub fn group_by_columns_dyn(&mut self, cols: Vec<Rc<dyn Iden>>) -> &mut Self {
        self.add_group_by(cols.into_iter().map(|c| SimpleExpr::Column(c)).collect());
        self
    }

    /// Group by columns with table prefix.
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
    ///     .group_by_table_columns(vec![
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
    pub fn group_by_table_columns<T: 'static>(&mut self, cols: Vec<(T, T)>) -> &mut Self
        where T: Iden {
        self.group_by_table_columns_dyn(cols.into_iter().map(
            |(t, c)| (Rc::new(t) as Rc<dyn Iden>, Rc::new(c) as Rc<dyn Iden>)
        ).collect())
    }

    /// Group by columns with table prefix, variation of [`SelectStatement::group_by_table_columns`].
    pub fn group_by_table_columns_dyn(&mut self, cols: Vec<(Rc<dyn Iden>, Rc<dyn Iden>)>) -> &mut Self {
        self.add_group_by(cols.into_iter().map(|(t, c)| SimpleExpr::TableColumn(t, c)).collect());
        self
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

    /// And where condition, variation of [`SelectStatement::and_where`].
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
    pub fn add_group_by(&mut self, mut expr: Vec<SimpleExpr>) -> &mut Self {
        self.groups.append(&mut expr);
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
    ///     .order_by_tbl(Glyph::Table, Glyph::Aspect, Order::Asc)
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
    pub fn order_by<T: 'static>(&mut self, col: T, order: Order) -> &mut Self 
        where T: Iden {
        self.order_by_dyn(Rc::new(col), order)
    }

    /// Order by column, variation of [`SelectStatement::order_by`]
    pub fn order_by_dyn(&mut self, col: Rc<dyn Iden>, order: Order) -> &mut Self {
        self.orders.push(OrderExpr {
            expr: SimpleExpr::Column(col),
            order,
        });
        self
    }

    /// Order by column with table prefix.
    /// 
    /// # Examples
    /// 
    /// See [`SelectStatement::order_by`].
    pub fn order_by_tbl<T: 'static, C: 'static>
        (&mut self, table: T, col: C, order: Order) -> &mut Self 
        where T: Iden, C: Iden {
        self.order_by_tbl_dyn(Rc::new(table), Rc::new(col), order)
    }

    /// Order by column with table prefix, variation of [`SelectStatement::order_by_tbl`].
    pub fn order_by_tbl_dyn(&mut self, table: Rc<dyn Iden>, col: Rc<dyn Iden>, order: Order) -> &mut Self {
        self.orders.push(OrderExpr {
            expr: SimpleExpr::TableColumn(table, col),
            order,
        });
        self
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
    pub fn order_by_customs<T: 'static>(&mut self, cols: Vec<(T, Order)>) -> &mut Self 
        where T: ToString {
        let mut orders = cols.into_iter().map(
            |(c, order)| OrderExpr {
                expr: SimpleExpr::Custom(c.to_string()),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    /// Order by vector of columns.
    pub fn order_by_columns<T: 'static>(&mut self, cols: Vec<(T, Order)>) -> &mut Self 
        where T: Iden {
        self.order_by_columns_dyn(cols.into_iter().map(
            |(c, order)| (Rc::new(c) as Rc<dyn Iden>, order)
        ).collect())
    }

    /// Order by vector of columns, variation of [`SelectStatement::order_by_columns`].
    pub fn order_by_columns_dyn(&mut self, cols: Vec<(Rc<dyn Iden>, Order)>) -> &mut Self {
        let mut orders = cols.into_iter().map(
            |(c, order)| OrderExpr {
                expr: SimpleExpr::Column(c),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
    }

    /// Order by vector of columns with table prefix.
    pub fn order_by_table_columns<T: 'static, C: 'static>
        (&mut self, cols: Vec<(T, C, Order)>) -> &mut Self 
        where T: Iden, C: Iden {
        self.order_by_table_columns_dyn(cols.into_iter().map(
            |(t, c, order)| (Rc::new(t) as Rc<dyn Iden>, Rc::new(c) as Rc<dyn Iden>, order)
        ).collect())
    }

    /// Order by vector of columns with table prefix, variation of [`SelectStatement::order_by_table_columns`].
    #[allow(clippy::type_complexity)]
    pub fn order_by_table_columns_dyn(&mut self, cols: Vec<(Rc<dyn Iden>, Rc<dyn Iden>, Order)>) -> &mut Self {
        let mut orders = cols.into_iter().map(
            |(t, c, order)| OrderExpr {
                expr: SimpleExpr::TableColumn(t, c),
                order,
            }).collect();
        self.orders.append(&mut orders);
        self
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