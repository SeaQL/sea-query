//! Building blocks of SQL statements.
//!
//! [`Expr`] is an arbitrary, dynamically-typed SQL expression.
//! It can be used in select fields, where clauses and many other places.
//!
//! [`ExprTrait`] provides "operator" methods for building expressions.

use crate::{func::*, query::*, types::*, value::*};

/// A legacy compatibility alias for [`Expr`].
///
/// These used to be two separate (but very similar) types.
pub type SimpleExpr = Expr;

/// An arbitrary, dynamically-typed SQL expression.
///
/// It can be used in select fields, where clauses and many other places.
///
/// More concreterly, under the hood [`Expr`]s can be:
///
/// - Rust values
/// - SQL identifiers
/// - SQL function calls
/// - various operators and sub-queries
///
/// If something is not supported here, look into [`BinOper::Custom`],
/// [`Func::cust`], or [`Expr::cust*`][`Expr::cust_with_values`] as a
/// workaround, and consider reporting your issue.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Expr {
    Column(ColumnRef),
    Tuple(Vec<Expr>),
    Unary(UnOper, Box<Expr>),
    FunctionCall(FunctionCall),
    Binary(Box<Expr>, BinOper, Box<Expr>),
    SubQuery(Option<SubQueryOper>, Box<SubQueryStatement>),
    Value(Value),
    Values(Vec<Value>),
    Custom(String),
    CustomWithExpr(String, Vec<Expr>),
    Keyword(Keyword),
    Case(Box<CaseStatement>),
    Constant(Value),
}

/// Whether an identifier should be quoted or not.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum IdenQuoting {
    /// Quote the identifier when building the SQL statement.
    ///
    /// This is the default behavior, and allows to use special characters and reserved keywords in identifiers.
    ///
    /// In some databases, this also makes the identifier case-sensitive, while unquoted identifiers are case-insensitive.
    Quoted,
    /// Don't quote the identifier when building the SQL statement.
    ///
    /// In some databases, unquoted identifiers are case-insensitive, while quoted identifiers are case-sensitive.
    Unquoted,
}

impl<T> From<T> for Expr
where
    T: Into<Value>,
{
    fn from(v: T) -> Self {
        Self::Value(v.into())
    }
}

impl From<Vec<Value>> for Expr {
    fn from(v: Vec<Value>) -> Self {
        Self::Values(v)
    }
}

impl From<SubQueryStatement> for Expr {
    fn from(v: SubQueryStatement) -> Self {
        Self::SubQuery(None, Box::new(v))
    }
}

macro_rules! from_into_subquery_expr {
    ($($ty:ty),+) => {
        $(
            impl From<$ty> for Expr {
                fn from(v: $ty) -> Self {
                    Self::SubQuery(None, Box::new(v.into()))
                }
            }
        )+
    };
}

from_into_subquery_expr!(
    WithQuery,
    DeleteStatement,
    UpdateStatement,
    InsertStatement,
    SelectStatement
);

impl From<FunctionCall> for Expr {
    fn from(func: FunctionCall) -> Self {
        Self::FunctionCall(func)
    }
}

impl From<ColumnRef> for Expr {
    fn from(col: ColumnRef) -> Self {
        Self::Column(col)
    }
}

impl From<Keyword> for Expr {
    fn from(k: Keyword) -> Self {
        Self::Keyword(k)
    }
}

impl From<LikeExpr> for Expr {
    fn from(like: LikeExpr) -> Self {
        match like.escape {
            Some(escape) => Self::Binary(
                Box::new(like.pattern.into()),
                BinOper::Escape,
                Box::new(Expr::Constant(escape.into())),
            ),
            None => like.pattern.into(),
        }
    }
}

impl From<TypeName> for Expr {
    fn from(type_name: TypeName) -> Self {
        Self::TypeName(type_name)
    }
}

/// "Operator" methods for building expressions.
///
/// Before `sea_query` 0.32.0 (`sea_orm` 1.1.1),
/// these methods were awailable only on [`Expr`]/[`SimpleExpr`]
/// and you needed to manually construct these types first.
///
/// Now, you can call them directly on any expression type:
///
/// ```no_run
/// # use sea_query::*;
/// #
/// let expr = 1_i32.cast_as("REAL");
/// let expr = Func::char_length("abc").eq(3_i32);
/// let expr = Expr::current_date().cast_as("TEXT").like("2024%");
/// ```
///
/// If some methods are missing, look into [`BinOper::Custom`], [`Func::cust`],
/// or [`Expr::cust*`][`Expr::cust_with_values`] as a workaround, and consider
/// reporting your issue.
pub trait ExprTrait: Sized {
    /// Express an arithmetic addition operation.
    ///
    /// # Examples
    ///
    /// Adding literal values
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.add(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 + 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 + 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 + 1 = 2"#
    /// );
    /// ```
    ///
    /// Adding columns and values
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col(Char::SizeW).add(1))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` + 1 FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "size_w" + 1 FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "size_w" + 1 FROM "character""#
    /// );
    /// ```
    ///
    /// Adding columns
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col(Char::SizeW).add(Expr::col(Char::SizeH)))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` + `size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "size_w" + "size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "size_w" + "size_h" FROM "character""#
    /// );
    /// ```
    fn add<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::Add, right)
    }

    /// Express a `AS enum` expression.
    ///
    /// Type can be qualified with a schema name.
    ///
    /// Unlike [`cast_as`][Expr::cast_as], this method puts the type name in quotes.
    /// This is useful for type names that can be SQL keywords or contain special characters.
    /// In some databases, quoted identifiers are case-sensitive, while unquoted identifiers are case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Char::Table)
    ///     .columns([Char::FontSize])
    ///     .values_panic(["large".as_enum("FontSizeEnum")])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `character` (`font_size`) VALUES ('large')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "character" ("font_size") VALUES (CAST('large' AS "FontSizeEnum"))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "character" ("font_size") VALUES ('large')"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col(Char::FontSize).as_enum("FontSizeEnum[]"))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST("font_size" AS "FontSizeEnum"[]) FROM "character""#
    /// );
    ///
    /// // Also works with a schema-qualified type name:
    ///
    /// let query = Query::insert()
    ///     .into_table(Char::Table)
    ///     .columns([Char::FontSize])
    ///     .values_panic(["large".as_enum(("MySchema", "FontSizeEnum"))])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `character` (`font_size`) VALUES ('large')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "character" ("font_size") VALUES (CAST('large' AS "MySchema"."FontSizeEnum"))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "character" ("font_size") VALUES ('large')"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col(Char::FontSize).as_enum(("MySchema", "FontSizeEnum[]")))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST("font_size" AS "MySchema"."FontSizeEnum"[]) FROM "character""#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn as_enum<N>(self, type_name: N) -> Expr
    where
        N: IntoTypeRef;

    /// Express a logical `AND` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .cond_where(any![
    ///         Expr::col(Char::SizeW).eq(1).and(Expr::col(Char::SizeH).eq(2)),
    ///         Expr::col(Char::SizeW).eq(3).and(Expr::col(Char::SizeH).eq(4)),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w` = 1 AND `size_h` = 2) OR (`size_w` = 3 AND `size_h` = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" = 1 AND "size_h" = 2) OR ("size_w" = 3 AND "size_h" = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" = 1 AND "size_h" = 2) OR ("size_w" = 3 AND "size_h" = 4)"#
    /// );
    /// ```
    fn and<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::And, right)
    }

    /// Express a `BETWEEN` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().between(1, 10))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" BETWEEN 1 AND 10"#
    /// );
    /// ```
    fn between<A, B>(self, a: A, b: B) -> Expr
    where
        A: Into<Expr>,
        B: Into<Expr>,
    {
        self.binary(
            BinOper::Between,
            Expr::Binary(Box::new(a.into()), BinOper::And, Box::new(b.into())),
        )
    }

    /// Create any binary operation
    ///
    /// # Examples
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .cond_where(all![
    ///         Char::SizeW.into_column_ref().binary(BinOper::SmallerThan, 10),
    ///         Char::SizeW.into_column_ref().binary(BinOper::GreaterThan, Char::SizeH.into_column_ref())
    ///     ])
    ///     .to_owned();
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` < 10 AND `size_w` > `size_h`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" < 10 AND "size_w" > "size_h""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" < 10 AND "size_w" > "size_h""#
    /// );
    /// ```
    fn binary<O, R>(self, op: O, right: R) -> Expr
    where
        O: Into<BinOper>,
        R: Into<Expr>;

    /// Express a `CAST AS` expression.
    ///
    /// Type can be qualified with a schema name.
    ///
    /// Unlike [`as_enum`][Expr::as_enum], this method doesn't put the type name in quotes.
    /// This can be used to get case-insensitive behavior in databases where quoted identifiers are case-sensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr("1".cast_as("integer")).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    ///
    /// // Also works with a schema-qualified type name:
    ///
    /// let query = Query::select().expr("1".cast_as(("MySchema", "integer"))).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CAST('1' AS `MySchema`.integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST('1' AS "MySchema".integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CAST('1' AS "MySchema".integer)"#
    /// );
    /// ```
    fn cast_as<N>(self, type_name: N) -> Expr
    where
        N: IntoTypeRef;

    /// Express an arithmetic division operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.div(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 / 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 / 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 / 1 = 2"#
    /// );
    /// ```
    fn div<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::Div, right)
    }

    /// Express an equal (`=`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val("What!").eq("Nothing"))
    ///     .and_where(Char::Id.into_column_ref().eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'What!' = 'Nothing' AND `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'What!' = 'Nothing' AND "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'What!' = 'Nothing' AND "id" = 1"#
    /// );
    /// ```
    ///
    /// Note how you should express a string being a literal vs an identifier.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col("name").eq("Something"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `name` = 'Something'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "name" = 'Something'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "name" = 'Something'"#
    /// );
    /// ```
    fn eq<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::Equal, right)
    }

    /// Express a equal expression between two table columns,
    /// you will mainly use this to relate identical value between two table columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::FontId).into_column_ref().equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" = "font"."id""#
    /// );
    /// ```
    fn equals<C>(self, col: C) -> Expr
    where
        C: IntoColumnRef,
    {
        self.binary(BinOper::Equal, col.into_column_ref())
    }

    /// Express a greater than (`>`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().gt(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" > 2"#
    /// );
    /// ```
    fn gt<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::GreaterThan, right)
    }

    /// Express a greater than or equal (`>=`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().gte(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` >= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" >= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" >= 2"#
    /// );
    /// ```
    fn gte<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::GreaterThanOrEqual, right)
    }

    /// Express a `IN` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Char::SizeW.into_column_ref().in_subquery(
    ///         Query::select()
    ///             .expr(Expr::cust("3 + 2 * 2"))
    ///             .take()
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" IN (SELECT 3 + 2 * 2)"#
    /// );
    /// ```
    fn in_subquery(self, sel: SelectStatement) -> Expr {
        self.binary(BinOper::In, Expr::SubQuery(None, Box::new(sel.into())))
    }

    /// Express a `IN` sub expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::FontId])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         Expr::tuple([
    ///             Expr::col(Char::Character).into(),
    ///             Expr::col(Char::FontId).into(),
    ///         ]).in_tuples([(1, String::from("1")), (2, String::from("2"))])
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font_id` FROM `character` WHERE (`character`, `font_id`) IN ((1, '1'), (2, '2'))"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font_id" FROM "character" WHERE ("character", "font_id") IN ((1, '1'), (2, '2'))"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font_id" FROM "character" WHERE ("character", "font_id") IN ((1, '1'), (2, '2'))"#
    /// );
    /// ```
    fn in_tuples<V, I>(self, v: I) -> Expr
    where
        V: IntoValueTuple,
        I: IntoIterator<Item = V>,
    {
        self.binary(
            BinOper::In,
            Expr::Tuple(
                v.into_iter()
                    .map(|m| Expr::Values(m.into_value_tuple().into_iter().collect()))
                    .collect(),
            ),
        )
    }

    /// Express a `IS` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Ascii).into_column_ref().is(true))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`ascii` IS TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS TRUE"#
    /// );
    /// ```
    fn is<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::Is, right)
    }

    /// Express a `IN` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         (Char::Table, Char::SizeW)
    ///             .into_column_ref()
    ///             .is_in([1, 2, 3]),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `character`.`size_w` IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" IN (1, 2, 3)"#
    /// );
    /// ```
    ///
    /// The same query can be constructed using the `raw_query!` macro.
    ///
    /// ```
    /// use sea_query::Values;
    ///
    /// let ids: Vec<i32> = vec![1, 2, 3];
    /// let query = sea_query::raw_query!(
    ///     SqliteQueryBuilder,
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" IN ({..ids})"#
    /// );
    ///
    /// assert_eq!(
    ///     query.sql,
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" IN (?, ?, ?)"#
    /// );
    /// assert_eq!(query.values, Values(vec![1.into(), 2.into(), 3.into()]));
    /// ```
    ///
    /// Empty value list is converted to an always false expression due to SQL syntax.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         (Char::Table, Char::SizeW)
    ///             .into_column_ref()
    ///             .is_in(Vec::<u8>::new()),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_in<V, I>(self, v: I) -> Expr
    where
        V: Into<Expr>,
        I: IntoIterator<Item = V>,
    {
        self.binary(
            BinOper::In,
            Expr::Tuple(v.into_iter().map(|v| v.into()).collect()),
        )
    }

    /// Express a `IS NOT` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Ascii).into_column_ref().is_not(true))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`ascii` IS NOT TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS NOT TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS NOT TRUE"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_not<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::IsNot, right)
    }

    /// Express a `NOT IN` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         (Char::Table, Char::SizeW)
    ///             .into_column_ref()
    ///             .is_not_in([1, 2, 3]),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `character`.`size_w` NOT IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" NOT IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" NOT IN (1, 2, 3)"#
    /// );
    /// ```
    /// Empty value list
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         (Char::Table, Char::SizeW)
    ///             .into_column_ref()
    ///             .is_not_in(Vec::<u8>::new()),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 1"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_not_in<V, I>(self, v: I) -> Expr
    where
        V: Into<Expr>,
        I: IntoIterator<Item = V>,
    {
        self.binary(
            BinOper::NotIn,
            Expr::Tuple(v.into_iter().map(|v| v.into()).collect()),
        )
    }

    /// Express a `IS NOT NULL` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().is_not_null())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` IS NOT NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NOT NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NOT NULL"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> Expr {
        self.binary(BinOper::IsNot, Keyword::Null)
    }

    /// Express a `IS NULL` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().is_null())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NULL"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> Expr {
        self.binary(BinOper::Is, Keyword::Null)
    }

    /// Express a bitwise left shift.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.left_shift(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 << 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 << 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 << 1 = 2"#
    /// );
    /// ```
    fn left_shift<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::LShift, right)
    }

    /// Express a `LIKE` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Character).into_column_ref().like("Ours'%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`character` LIKE 'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE E'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE 'Ours''%'"#
    /// );
    /// ```
    ///
    /// Like with ESCAPE
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Character).into_column_ref().like(LikeExpr::new(r"|_Our|_").escape('|')))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`character` LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// ```
    fn like<L>(self, like: L) -> Expr
    where
        L: IntoLikeExpr,
    {
        self.binary(BinOper::Like, like.into_like_expr())
    }

    /// Express a less than (`<`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().lt(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` < 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" < 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" < 2"#
    /// );
    /// ```
    fn lt<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::SmallerThan, right)
    }

    /// Express a less than or equal (`<=`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().lte(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` <= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" <= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" <= 2"#
    /// );
    /// ```
    fn lte<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::SmallerThanOrEqual, right)
    }

    /// Express an arithmetic modulo operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.modulo(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 % 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 % 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 % 1 = 2"#
    /// );
    /// ```
    fn modulo<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::Mod, right)
    }

    /// Express an arithmetic multiplication operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.mul(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 * 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 * 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 * 1 = 2"#
    /// );
    /// ```
    fn mul<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::Mul, right)
    }

    /// Express a not equal (`<>`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     // Sometimes, you'll have to qualify the call because of conflicting std traits.
    ///     .and_where(ExprTrait::ne("Morning", "Good"))
    ///     .and_where(Char::Id.into_column_ref().ne(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'Morning' <> 'Good' AND `id` <> 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good' AND "id" <> 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good' AND "id" <> 1"#
    /// );
    /// ```
    fn ne<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::NotEqual, right)
    }

    /// Negates an expression with `NOT`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_null().not())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE NOT `character`.`size_w` IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// ```
    fn not(self) -> Expr {
        self.unary(UnOper::Not)
    }

    /// Express a `NOT BETWEEN` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().not_between(1, 10))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` NOT BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" NOT BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" NOT BETWEEN 1 AND 10"#
    /// );
    /// ```
    fn not_between<A, B>(self, a: A, b: B) -> Expr
    where
        A: Into<Expr>,
        B: Into<Expr>,
    {
        self.binary(
            BinOper::NotBetween,
            Expr::Binary(Box::new(a.into()), BinOper::And, Box::new(b.into())),
        )
    }

    /// Express a not equal expression between two table columns,
    /// you will mainly use this to relate identical value between two table columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::FontId).into_column_ref().not_equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`font_id` <> `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" <> "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" <> "font"."id""#
    /// );
    /// ```
    fn not_equals<C>(self, col: C) -> Expr
    where
        C: IntoColumnRef,
    {
        self.binary(BinOper::NotEqual, col.into_column_ref())
    }

    /// Express a `NOT IN` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Char::SizeW.into_column_ref().not_in_subquery(
    ///         Query::select()
    ///             .expr(Expr::cust("3 + 2 * 2"))
    ///             .take()
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// ```
    fn not_in_subquery(self, sel: SelectStatement) -> Expr {
        self.binary(BinOper::NotIn, Expr::SubQuery(None, Box::new(sel.into())))
    }

    /// Express a `NOT LIKE` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Character).into_column_ref().not_like("Ours'%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`character` NOT LIKE 'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" NOT LIKE E'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" NOT LIKE 'Ours''%'"#
    /// );
    /// ```
    fn not_like<L>(self, like: L) -> Expr
    where
        L: IntoLikeExpr,
    {
        self.binary(BinOper::NotLike, like.into_like_expr())
    }

    /// Express a logical `OR` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(false.or(true))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE FALSE OR TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE FALSE OR TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE FALSE OR TRUE"#
    /// );
    /// ```
    fn or<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::Or, right)
    }

    /// Express a bitwise right shift.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.right_shift(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 >> 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 >> 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 >> 1 = 2"#
    /// );
    /// ```
    fn right_shift<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::RShift, right)
    }

    /// Express an arithmetic subtraction operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.sub(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 - 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 - 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 - 1 = 2"#
    /// );
    /// ```
    fn sub<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::Sub, right)
    }

    /// Apply any unary operator to the expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_null().unary(UnOper::Not))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE NOT `character`.`size_w` IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// ```
    fn unary(self, o: UnOper) -> Expr;

    /// Express a bitwise AND operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(1.bit_and(2).eq(3)).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (1 & 2) = 3"#
    /// );
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.bit_and(1).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (1 & 1) = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 & 1) = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 & 1) = 1"#
    /// );
    /// ```
    fn bit_and<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::BitAnd, right)
    }

    /// Express a bitwise OR operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.bit_or(1).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (1 | 1) = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 | 1) = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 | 1) = 1"#
    /// );
    /// ```
    fn bit_or<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        self.binary(BinOper::BitOr, right)
    }
}

/// This generic implementation covers all expression types,
/// including [ColumnRef], [Value], [FunctionCall], [Expr]...
impl<T> ExprTrait for T
where
    T: Into<Expr>,
{
    fn as_enum<N>(self, type_name: N) -> Expr
    where
        N: IntoTypeRef,
    {
        Expr::FunctionCall(Func::cast_as_quoted(self, type_name))
    }

    fn binary<O, R>(self, op: O, right: R) -> Expr
    where
        O: Into<BinOper>,
        R: Into<Expr>,
    {
        Expr::Binary(Box::new(self.into()), op.into(), Box::new(right.into()))
    }

    fn cast_as<N>(self, type_name: N) -> Expr
    where
        N: Into<TypeRef>,
    {
        Expr::FunctionCall(Func::cast_as(self, type_name))
    }

    fn unary(self, op: UnOper) -> Expr {
        Expr::Unary(op, Box::new(self.into()))
    }
}

impl Expr {
    #[deprecated(since = "0.29.0", note = "Please use the [`Asterisk`]")]
    pub fn asterisk() -> Self {
        Self::col(Asterisk)
    }

    /// Express the target column without table prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::SizeW).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// ```
    pub fn col<T>(n: T) -> Self
    where
        T: IntoColumnRef,
    {
        Self::Column(n.into_column_ref())
    }

    /// Express the target column without table prefix, returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::column(Char::SizeW).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::column((Char::Table, Char::SizeW)).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// ```
    pub fn column<T>(n: T) -> Self
    where
        T: IntoColumnRef,
    {
        Self::Column(n.into_column_ref())
    }

    /// Wraps tuple of `Expr`, can be used for tuple comparison
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         Expr::tuple([Expr::col(Char::SizeW).into(), Expr::value(100)])
    ///             .lt(Expr::tuple([Expr::value(500), Expr::value(100)])))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w`, 100) < (500, 100)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w", 100) < (500, 100)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w", 100) < (500, 100)"#
    /// );
    /// ```
    pub fn tuple<I>(n: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::Tuple(n.into_iter().collect::<Vec<Self>>())
    }

    #[deprecated(since = "0.29.0", note = "Please use the [`Asterisk`]")]
    pub fn table_asterisk<T>(t: T) -> Self
    where
        T: IntoIden,
    {
        Self::col((t.into_iden(), Asterisk))
    }

    /// Express a [`Value`], returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).into())
    ///     .and_where(Expr::val(2.5).into())
    ///     .and_where(Expr::val("3").into())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// ```
    pub fn val<V>(v: V) -> Self
    where
        V: Into<Value>,
    {
        Self::from(v)
    }

    /// Wrap an expression to perform some operation on it later.
    ///
    /// Since `sea_query` 0.32.0 (`sea_orm` 1.1.1), **this is not necessary** in most cases!
    ///
    /// Some SQL operations used to be defined only as inherent methods on [`Expr`].
    /// Thus, to use them, you needed to manually convert from other types to [`Expr`].
    /// But now these operations are also defined as [`ExprTrait`] methods
    /// that can be called directly on any expression type,
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     // This is the old style, when `Expr::expr` was necessary:
    ///     .and_where(Expr::expr(Expr::col(Char::SizeW).if_null(0)).gt(2))
    ///     .to_owned();
    ///
    /// // But since 0.32.0, this compiles too:
    /// let _ = Expr::col(Char::SizeW).if_null(0).gt(2);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE IFNULL(`size_w`, 0) > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE COALESCE("size_w", 0) > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE IFNULL("size_w", 0) > 2"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     // This is the old style, when `Expr::expr` was necessary:
    ///     .and_where(Expr::expr(Func::lower(Expr::col(Char::Character))).is_in(["a", "b"]))
    ///     .to_owned();
    ///
    /// // But since 0.32.0, this compiles too:
    /// let _ = Func::lower(Expr::col(Char::Character)).is_in(["a", "b"]);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE LOWER(`character`) IN ('a', 'b')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE LOWER("character") IN ('a', 'b')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE LOWER("character") IN ('a', 'b')"#
    /// );
    /// ```
    #[allow(clippy::self_named_constructors)]
    pub fn expr<T>(expr: T) -> Self
    where
        T: Into<Self>,
    {
        expr.into()
    }

    /// Express a [`Value`], returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::value(1))
    ///     .and_where(Expr::value(2.5))
    ///     .and_where(Expr::value("3"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// ```
    pub fn value<V>(v: V) -> Self
    where
        V: Into<Self>,
    {
        v.into()
    }

    /// Express any custom expression in [`&str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::cust("1 = 1"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 = 1"#
    /// );
    /// ```
    pub fn cust<T>(s: T) -> Self
    where
        T: Into<String>,
    {
        Self::Custom(s.into())
    }

    /// Express any custom expression with [`Value`]. Use this if your expression needs variables.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(1))
    ///     .and_where(Expr::cust_with_values("6 = ? * ?", [2, 3]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `id` = 1 AND (6 = 2 * 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND (6 = 2 * 3)"#
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(1))
    ///     .and_where(Expr::cust_with_values("6 = $2 * $1", [3, 2]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND (6 = 2 * 3)"#
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("6 = ? * ?", [2, 3]))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT 6 = 2 * 3"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT 6 = 2 * 3"#);
    /// ```
    /// Postgres only: use `$$` to escape `$`
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("$1 $$ $2", ["a", "b"]))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT 'a' $ 'b'"#);
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("data @? ($1::JSONPATH)", ["hello"]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT data @? ('hello'::JSONPATH)"#
    /// );
    /// ```
    pub fn cust_with_values<T, V, I>(s: T, v: I) -> Self
    where
        T: Into<String>,
        V: Into<Value>,
        I: IntoIterator<Item = V>,
    {
        Self::CustomWithExpr(
            s.into(),
            v.into_iter()
                .map(|v| Into::<Value>::into(v).into())
                .collect(),
        )
    }

    /// Express any custom expression with [`Expr`]. Use this if your expression needs other expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::val(1).add(2))
    ///     .expr(Expr::cust_with_expr("data @? ($1::JSONPATH)", "hello"))
    ///     .to_owned();
    /// let (sql, values) = query.build(PostgresQueryBuilder);
    ///
    /// assert_eq!(sql, r#"SELECT $1 + $2, data @? ($3::JSONPATH)"#);
    /// assert_eq!(
    ///     values,
    ///     Values(vec![1i32.into(), 2i32.into(), "hello".into()])
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_expr(
    ///         "json_agg(DISTINCT $1)",
    ///         Expr::col(Char::Character),
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT json_agg(DISTINCT "character")"#
    /// );
    /// ```
    pub fn cust_with_expr<T, E>(s: T, expr: E) -> Self
    where
        T: Into<String>,
        E: Into<Self>,
    {
        Self::CustomWithExpr(s.into(), vec![expr.into()])
    }

    /// Express any custom expression with [`Expr`]. Use this if your expression needs other expressions.
    pub fn cust_with_exprs<T, I>(s: T, v: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = Expr>,
    {
        Self::CustomWithExpr(s.into(), v.into_iter().collect())
    }

    /// Express a `MAX` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).max())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn max(self) -> Self {
        Func::max(self).into()
    }

    /// Express a `MIN` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).min())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MIN(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MIN("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MIN("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn min(self) -> Self {
        Func::min(self).into()
    }

    /// Express a `SUM` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).sum())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT SUM(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT SUM("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT SUM("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn sum(self) -> Self {
        Func::sum(self).into()
    }

    /// Express a `COUNT` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).count())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COUNT(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COUNT("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COUNT("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn count(self) -> Self {
        Func::count(self).into()
    }

    /// Express a `COUNT` function with the DISTINCT modifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).count_distinct())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT `character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT "character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT "character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn count_distinct(self) -> Self {
        Func::count_distinct(self).into()
    }

    /// Express a `IF NULL` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).if_null(0))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT IFNULL(`character`.`size_w`, 0) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COALESCE("character"."size_w", 0) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT IFNULL("character"."size_w", 0) FROM "character""#
    /// );
    /// ```
    pub fn if_null<V>(self, v: V) -> Self
    where
        V: Into<Self>,
    {
        Func::if_null(self, v).into()
    }

    /// Express a `EXISTS` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(Expr::exists(Query::select().column(Char::Id).from(Char::Table).take()), "character_exists")
    ///     .expr_as(Expr::exists(Query::select().column(Glyph::Id).from(Glyph::Table).take()), "glyph_exists")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT `id` FROM `character`) AS `character_exists`, EXISTS(SELECT `id` FROM `glyph`) AS `glyph_exists`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT "id" FROM "character") AS "character_exists", EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT "id" FROM "character") AS "character_exists", EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// ```
    pub fn exists(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Exists), Box::new(sel.into()))
    }

    /// Express a `ANY` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Id)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(Expr::any(
    ///         Query::select().column(Char::Id).from(Char::Table).take(),
    ///     )))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `id` = ANY(SELECT `id` FROM `character`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "id" = ANY(SELECT "id" FROM "character")"#
    /// );
    /// ```
    pub fn any(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Any), Box::new(sel.into()))
    }

    /// Express a `SOME` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Id)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).ne(Expr::some(
    ///         Query::select().column(Char::Id).from(Char::Table).take(),
    ///     )))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `id` <> SOME(SELECT `id` FROM `character`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "id" <> SOME(SELECT "id" FROM "character")"#
    /// );
    /// ```
    pub fn some(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Some), Box::new(sel.into()))
    }

    /// Express a `ALL` sub-query expression.
    pub fn all(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::All), Box::new(sel.into()))
    }

    /// Adds new `CASE WHEN` to existing case statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         Expr::case(
    ///                 Expr::col((Glyph::Table, Glyph::Aspect)).is_in([2, 4]),
    ///                 true
    ///              )
    ///             .finally(false),
    ///          "is_even"
    ///     )
    ///     .from(Glyph::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" IN (2, 4)) THEN TRUE ELSE FALSE END) AS "is_even" FROM "glyph""#
    /// );
    /// ```
    pub fn case<C, T>(cond: C, then: T) -> CaseStatement
    where
        C: IntoCondition,
        T: Into<Self>,
    {
        CaseStatement::new().case(cond, then)
    }

    /// Keyword `CURRENT_DATE`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select().expr(Expr::current_date()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT CURRENT_DATE"#);
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_DATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_DATE"#
    /// );
    /// ```
    pub fn current_date() -> Self {
        Self::Keyword(Keyword::CurrentDate)
    }

    /// Keyword `CURRENT_TIMESTAMP`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select().expr(Expr::current_time()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT CURRENT_TIME"#);
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_TIME"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_TIME"#
    /// );
    /// ```
    pub fn current_time() -> Self {
        Self::Keyword(Keyword::CurrentTime)
    }

    /// Keyword `CURRENT_TIMESTAMP`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{Expr, MysqlQueryBuilder, PostgresQueryBuilder, Query, SqliteQueryBuilder};
    ///
    /// let query = Query::select().expr(Expr::current_timestamp()).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// ```
    pub fn current_timestamp() -> Self {
        Self::Keyword(Keyword::CurrentTimestamp)
    }

    /// Keyword `DEFAULT`.
    ///
    /// SQLite does not support VALUES ​​(DEFAULT).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{
    ///     Expr, MysqlQueryBuilder, PostgresQueryBuilder, Query, SqliteQueryBuilder, tests_cfg::*,
    /// };
    ///
    /// let query = Query::insert()
    ///     .columns([Char::Id])
    ///     .values_panic([Expr::keyword_default()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT (`id`) VALUES (DEFAULT)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT ("id") VALUES (DEFAULT)"#
    /// );
    /// ```
    pub fn keyword_default() -> Self {
        Self::Keyword(Keyword::Default)
    }

    /// Custom keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::custom_keyword("test"))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT test"#);
    /// assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT test"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT test"#);
    /// ```
    pub fn custom_keyword<T>(i: T) -> Self
    where
        T: IntoIden,
    {
        Self::Keyword(Keyword::Custom(i.into_iden()))
    }
}
impl Expr {
    pub(crate) fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_, _, _))
    }

    pub(crate) fn get_bin_oper(&self) -> Option<&BinOper> {
        match self {
            Self::Binary(_, oper, _) => Some(oper),
            _ => None,
        }
    }
}
