//! Base types used throughout sea-query.

use crate::{FunctionCall, ValueTuple, Values, expr::*, query::*};
use std::{borrow::Cow, fmt::Debug, iter::Flatten};

#[cfg(feature = "backend-postgres")]
use crate::extension::postgres::PgBinOper;
#[cfg(feature = "backend-sqlite")]
use crate::extension::sqlite::SqliteBinOper;

/// A reference counted pointer: either [`Rc`][std::rc::Rc] or [`Arc`][std::sync::Arc],
/// depending on the feature flags.
///
/// [`Arc`][std::sync::Arc] is used when `thread-safe` feature is activated.
#[cfg(not(feature = "thread-safe"))]
pub type RcOrArc<T> = std::rc::Rc<T>;
/// A reference counted pointer: either [`Rc`][std::rc::Rc] or [`Arc`][std::sync::Arc],
/// depending on the feature flags.
///
/// [`Arc`][std::sync::Arc] is used when `thread-safe` feature is activated.
#[cfg(feature = "thread-safe")]
pub type RcOrArc<T> = std::sync::Arc<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quote(pub(crate) u8, pub(crate) u8);

/// Identifier
pub trait Iden {
    /// Return the to-be sanitized version of the identifier.
    ///
    /// For example, for MySQL "hel`lo`" would have to be escaped as "hel``lo".
    /// Note that this method doesn't do the actual escape,
    /// as it's backend specific.
    /// It only indicates whether the identifier needs to be escaped.
    ///
    /// If the identifier doesn't need to be escaped, return `'static str`.
    /// This can be deduced at compile-time by the `Iden` macro,
    /// or using the [`is_static_iden`] function.
    ///
    /// `Cow::Owned` would always be escaped.
    fn quoted(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }

    /// A shortcut for writing an [`unquoted`][Iden::unquoted]
    /// identifier into a [`String`].
    ///
    /// We can't reuse [`ToString`] for this, because [`ToString`] uses
    /// the [`Display`][std::fmt::Display] representation. But [`Iden`]
    /// representation is distinct from [`Display`][std::fmt::Display]
    /// and can be different.
    fn to_string(&self) -> String {
        self.unquoted().to_owned()
    }

    /// Write a raw identifier string without quotes.
    ///
    /// We intentionally don't reuse [`Display`][std::fmt::Display] for
    /// this, because we want to allow it to have a different logic.
    fn unquoted(&self) -> &str;
}

/// Identifier statically known at compile-time.
pub trait IdenStatic: Iden + Copy + 'static {
    fn as_str(&self) -> &'static str;
}

/// A prepared (quoted) identifier string.
///
/// The naming is legacy and kept for compatibility.
/// This used to be an alias for a `dyn Iden` object that's lazily rendered later.
///
/// Nowadays, it's an eagerly-rendered string.
/// Most identifiers are static strings that aren't "rendered" at runtime anyway.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynIden(pub(crate) Cow<'static, str>);

impl DynIden {
    pub fn inner(&self) -> Cow<'static, str> {
        self.0.clone()
    }
}

/// A legacy namespace for compatibility.
///
/// It's needed, so that most existing [`SeaRc::new`][SeaRc::new] calls keep working.
///
/// This used to be an actual type
/// (a reference-counted pointer with special impls for `dyn Iden` contents).
/// It's not needed anymore.
#[derive(Debug)]
pub struct SeaRc;

impl SeaRc {
    /// A legacy method, kept for compatibility.
    ///
    /// Nowadays, instead of wrapping an `Iden` object,
    /// it eagerly "renders" it into a string and then drops the object.
    ///
    /// Note that most `Iden`s are statically known
    /// and their representations aren't actually "rendered" and allocated at runtime.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<I>(i: I) -> DynIden
    where
        I: Iden,
    {
        DynIden(i.quoted())
    }

    pub fn clone(iden: &DynIden) -> DynIden {
        iden.clone()
    }
}

impl std::fmt::Display for DynIden {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&'static str> for DynIden {
    fn from(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

pub trait IntoIden {
    fn into_iden(self) -> DynIden;
}

pub trait IdenList {
    type IntoIter: Iterator<Item = DynIden>;

    fn into_iter(self) -> Self::IntoIter;
}

/// An identifier that represents a database name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatabaseName(pub DynIden);

/// A schema name, potentially qualified as `(database.)schema`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaName(pub Option<DatabaseName>, pub DynIden);

/// A table name, potentially qualified as `(database.)(schema.)table`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableName(pub Option<SchemaName>, pub DynIden);

impl TableName {
    /// A flat `(db?, schema?, table)` tuple view, for quick pattern matching.
    ///
    /// Don't use this if you need exhaustiveness.
    /// The return type is too lax and allows invalid shapes like `(Some(_), None, _)`.
    pub(crate) fn as_iden_tuple(&self) -> (Option<&DynIden>, Option<&DynIden>, &DynIden) {
        let TableName(schema_name, table) = self;
        match schema_name {
            None => (None, None, table),
            Some(SchemaName(db_name, schema)) => match db_name {
                None => (None, Some(schema), table),
                Some(DatabaseName(db)) => (Some(db), Some(schema), table),
            },
        }
    }
}

/// A column name, potentially qualified as `(database.)(schema.)(table.)column`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnName(pub Option<TableName>, pub DynIden);

/// Iteration over `[db?, schema?, table?, column]` identifiers.
impl IdenList for ColumnName {
    type IntoIter = Flatten<std::array::IntoIter<Option<DynIden>, 4>>;

    /// Iteration over `[db?, schema?, table?, column]` identifiers.
    fn into_iter(self) -> Self::IntoIter {
        let ColumnName(table_name, column) = self;
        let arr = match table_name {
            None => [None, None, None, Some(column)],
            Some(TableName(schema_name, table)) => match schema_name {
                None => [None, None, Some(table), Some(column)],
                Some(SchemaName(db_name, schema)) => {
                    let db = db_name.map(|db| db.0);
                    [db, Some(schema), Some(table), Some(column)]
                }
            },
        };
        arr.into_iter().flatten()
    }
}

/// Column references.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ColumnRef {
    /// A column name, potentially qualified as `(database.)(schema.)(table.)column`.
    Column(ColumnName),
    /// An `*` expression, potentially qualified as `(database.)(schema.)(table.)*`.
    Asterisk(Option<TableName>),
}

impl ColumnRef {
    #[doc(hidden)]
    /// Returns the unqualified column name if it's not an asterisk.
    pub fn column(&self) -> Option<&DynIden> {
        match self {
            ColumnRef::Column(ColumnName(_table_ref, column_itself)) => Some(column_itself),
            ColumnRef::Asterisk(..) => None,
        }
    }
}

pub trait IntoColumnRef {
    fn into_column_ref(self) -> ColumnRef;
}

/// Table references
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TableRef {
    /// A table identifier with optional Alias. Potentially qualified.
    Table(TableName, Option<DynIden>),
    /// Subquery with alias
    SubQuery(Box<SelectStatement>, DynIden),
    /// Values list with alias
    ValuesList(Vec<ValueTuple>, DynIden),
    /// Function call with alias
    FunctionCall(FunctionCall, DynIden),
}

impl TableRef {
    #[doc(hidden)]
    pub fn sea_orm_table(&self) -> &DynIden {
        match self {
            TableRef::Table(TableName(_, tbl), _)
            | TableRef::SubQuery(_, tbl)
            | TableRef::ValuesList(_, tbl)
            | TableRef::FunctionCall(_, tbl) => tbl,
        }
    }

    #[doc(hidden)]
    pub fn sea_orm_table_alias(&self) -> Option<&DynIden> {
        match self {
            TableRef::Table(_, None) | TableRef::SubQuery(_, _) | TableRef::ValuesList(_, _) => {
                None
            }
            TableRef::Table(_, Some(alias)) | TableRef::FunctionCall(_, alias) => Some(alias),
        }
    }
}

pub trait IntoTableRef {
    fn into_table_ref(self) -> TableRef;
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnOper {
    Not,
}

/// Binary operators.
///
/// If something is not supported here, you can use [`BinOper::Custom`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BinOper {
    And,
    Or,
    Like,
    NotLike,
    Is,
    IsNot,
    In,
    NotIn,
    Between,
    NotBetween,
    Equal,
    NotEqual,
    SmallerThan,
    GreaterThan,
    SmallerThanOrEqual,
    GreaterThanOrEqual,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    LShift,
    RShift,
    As,
    Escape,
    Custom(&'static str),
    #[cfg(feature = "backend-postgres")]
    PgOperator(PgBinOper),
    #[cfg(feature = "backend-sqlite")]
    SqliteOperator(SqliteBinOper),
}

/// Logical chain operator: conjunction or disjunction.
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalChainOper {
    And(Expr),
    Or(Expr),
}

/// Join types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Join,
    CrossJoin,
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
}

/// Nulls order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullOrdering {
    First,
    Last,
}

/// Order expression
#[derive(Debug, Clone, PartialEq)]
pub struct OrderExpr {
    pub(crate) expr: Expr,
    pub(crate) order: Order,
    pub(crate) nulls: Option<NullOrdering>,
}

/// Join on types
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum JoinOn {
    Condition(Box<ConditionHolder>),
    Columns(Vec<Expr>),
}

/// Ordering options
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Order {
    Asc,
    Desc,
    Field(Values),
}

/// An explicit wrapper for [`Iden`]s which are dynamic user-provided strings.
///
/// Nowadays, `&str` implements [`Iden`] and can be used directly.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Alias(pub String);

/// Null Alias
#[derive(Default, Debug, Copy, Clone)]
pub struct NullAlias;

/// Asterisk ("*")
///
/// Express the asterisk without table prefix.
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let query = Query::select()
///     .column(Asterisk)
///     .from(Char::Table)
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT * FROM `character`"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"SELECT * FROM "character""#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"SELECT * FROM "character""#
/// );
/// ```
///
/// Express the asterisk with table prefix.
///
/// Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let query = Query::select()
///     .column((Char::Table, Asterisk))
///     .from(Char::Table)
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `character`.* FROM `character`"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"SELECT "character".* FROM "character""#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"SELECT "character".* FROM "character""#
/// );
/// ```
#[derive(Default, Debug, Clone, Copy)]
pub struct Asterisk;

/// Known SQL keywords that can be used as expressions.
///
/// If something is not supported here, you can use [`Keyword::Custom`].
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Keyword {
    Null,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
    Custom(DynIden),
}

/// Like Expression
#[derive(Debug, Clone)]
pub struct LikeExpr {
    pub(crate) pattern: String,
    pub(crate) escape: Option<char>,
}

pub trait IntoLikeExpr {
    fn into_like_expr(self) -> LikeExpr;
}

/// SubQuery operators
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum SubQueryOper {
    Exists,
    Any,
    Some,
    All,
}

// Impl begins

impl Quote {
    pub fn new(c: u8) -> Self {
        Self(c, c)
    }

    pub fn left(&self) -> char {
        char::from(self.0)
    }

    pub fn right(&self) -> char {
        char::from(self.1)
    }
}

impl From<char> for Quote {
    fn from(c: char) -> Self {
        (c as u8).into()
    }
}

impl From<(char, char)> for Quote {
    fn from((l, r): (char, char)) -> Self {
        (l as u8, r as u8).into()
    }
}

impl From<u8> for Quote {
    fn from(u8: u8) -> Self {
        Quote::new(u8)
    }
}

impl From<(u8, u8)> for Quote {
    fn from((l, r): (u8, u8)) -> Self {
        Quote(l, r)
    }
}

impl<T> IntoIden for T
where
    T: Iden,
{
    fn into_iden(self) -> DynIden {
        DynIden(self.quoted())
    }
}

impl IntoIden for DynIden {
    fn into_iden(self) -> DynIden {
        self
    }
}

impl<I> IdenList for I
where
    I: IntoIden,
{
    type IntoIter = std::iter::Once<DynIden>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.into_iden())
    }
}

impl<A, B> IdenList for (A, B)
where
    A: IntoIden,
    B: IntoIden,
{
    type IntoIter = std::array::IntoIter<DynIden, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_iden(), self.1.into_iden()].into_iter()
    }
}

impl<A, B, C> IdenList for (A, B, C)
where
    A: IntoIden,
    B: IntoIden,
    C: IntoIden,
{
    type IntoIter = std::array::IntoIter<DynIden, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_iden(), self.1.into_iden(), self.2.into_iden()].into_iter()
    }
}

impl IntoColumnRef for ColumnRef {
    fn into_column_ref(self) -> ColumnRef {
        self
    }
}

impl<T> From<T> for DatabaseName
where
    T: IntoIden,
{
    fn from(iden: T) -> Self {
        DatabaseName(iden.into_iden())
    }
}

impl<T> From<T> for SchemaName
where
    T: IntoIden,
{
    fn from(iden: T) -> Self {
        SchemaName(None, iden.into_iden())
    }
}

impl<S, T> From<(S, T)> for SchemaName
where
    S: IntoIden,
    T: IntoIden,
{
    fn from((db, schema): (S, T)) -> Self {
        SchemaName(Some(db.into()), schema.into_iden())
    }
}

impl<T> From<T> for TableName
where
    T: IntoIden,
{
    fn from(iden: T) -> Self {
        TableName(None, iden.into_iden())
    }
}

impl<S, T> From<(S, T)> for TableName
where
    S: IntoIden,
    T: IntoIden,
{
    fn from((schema, table): (S, T)) -> Self {
        TableName(Some(schema.into()), table.into_iden())
    }
}

impl<S, T, U> From<(S, T, U)> for TableName
where
    S: IntoIden,
    T: IntoIden,
    U: IntoIden,
{
    fn from((db, schema, table): (S, T, U)) -> Self {
        TableName(Some((db, schema).into()), table.into_iden())
    }
}

impl<T> From<T> for ColumnName
where
    T: IntoIden,
{
    fn from(iden: T) -> Self {
        ColumnName(None, iden.into_iden())
    }
}

impl<S, T> From<(S, T)> for ColumnName
where
    S: IntoIden,
    T: IntoIden,
{
    fn from((table, column): (S, T)) -> Self {
        ColumnName(Some(table.into()), column.into_iden())
    }
}

impl<S, T, U> From<(S, T, U)> for ColumnName
where
    S: IntoIden,
    T: IntoIden,
    U: IntoIden,
{
    fn from((schema, table, column): (S, T, U)) -> Self {
        ColumnName(Some((schema, table).into()), column.into_iden())
    }
}

impl<T> IntoColumnRef for T
where
    T: Into<ColumnName>,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Column(self.into())
    }
}

impl IntoColumnRef for Asterisk {
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Asterisk(None)
    }
}

impl<T> IntoColumnRef for (T, Asterisk)
where
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Asterisk(Some(self.0.into()))
    }
}

impl IntoTableRef for TableRef {
    fn into_table_ref(self) -> TableRef {
        self
    }
}

impl<T> IntoTableRef for T
where
    T: Into<TableName>,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::Table(self.into(), None)
    }
}

impl TableRef {
    /// Add or replace the current alias
    pub fn alias<A>(self, alias: A) -> Self
    where
        A: IntoIden,
    {
        match self {
            Self::Table(table, _) => Self::Table(table, Some(alias.into_iden())),
            Self::SubQuery(statement, _) => Self::SubQuery(statement, alias.into_iden()),
            Self::ValuesList(values, _) => Self::ValuesList(values, alias.into_iden()),
            Self::FunctionCall(func, _) => Self::FunctionCall(func, alias.into_iden()),
        }
    }
}

impl Alias {
    pub fn new<T>(n: T) -> Self
    where
        T: Into<String>,
    {
        Self(n.into())
    }
}

impl Iden for Alias {
    fn quoted(&self) -> Cow<'static, str> {
        Cow::Owned(self.0.clone())
    }

    fn unquoted(&self) -> &str {
        &self.0
    }
}

impl IntoIden for String {
    fn into_iden(self) -> DynIden {
        DynIden(Cow::Owned(self))
    }
}

impl Iden for &'static str {
    fn quoted(&self) -> Cow<'static, str> {
        if is_static_iden(self) {
            Cow::Borrowed(self)
        } else {
            Cow::Owned(String::from(*self))
        }
    }

    fn unquoted(&self) -> &str {
        self
    }
}

/// Return whether this identifier needs to be escaped.
/// Right now we're very safe and only return true for identifiers
/// composed of `a-zA-Z0-9_`.
///
/// ```
/// use sea_query::is_static_iden;
///
/// assert!(is_static_iden("abc"));
/// assert!(is_static_iden("a_b_c"));
/// assert!(!is_static_iden("a-b-c"));
/// assert!(is_static_iden("abc123"));
/// assert!(!is_static_iden("123abc"));
/// assert!(!is_static_iden("a|b|c"));
/// assert!(!is_static_iden("a'b'c"));
/// ```
pub const fn is_static_iden(string: &str) -> bool {
    let bytes = string.as_bytes();
    if bytes.is_empty() {
        return true;
    }

    // can only begin with [a-z_]
    if bytes[0] == b'_' || (bytes[0] as char).is_ascii_alphabetic() {
        // good
    } else {
        return false;
    }

    let mut i = 1;
    while i < bytes.len() {
        if bytes[i] == b'_' || (bytes[i] as char).is_ascii_alphanumeric() {
            // good
        } else {
            return false;
        }
        i += 1;
    }

    true
}

impl NullAlias {
    pub fn new() -> Self {
        Self
    }
}

impl Iden for NullAlias {
    fn unquoted(&self) -> &str {
        ""
    }
}

impl LikeExpr {
    pub fn new<T>(pattern: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pattern: pattern.into(),
            escape: None,
        }
    }

    #[deprecated(since = "0.29.0", note = "Please use the [`LikeExpr::new`] method")]
    pub fn str<T>(pattern: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pattern: pattern.into(),
            escape: None,
        }
    }

    pub fn escape(self, c: char) -> Self {
        Self {
            pattern: self.pattern,
            escape: Some(c),
        }
    }
}

impl IntoLikeExpr for LikeExpr {
    fn into_like_expr(self) -> LikeExpr {
        self
    }
}

impl<T> IntoLikeExpr for T
where
    T: Into<String>,
{
    fn into_like_expr(self) -> LikeExpr {
        LikeExpr::new(self)
    }
}

#[cfg(test)]
mod tests {
    pub use crate::{tests_cfg::*, *};
    pub use Character as CharReexport;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_identifier() {
        let query = Query::select().column("hello-World_").to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hello-World_`");
        #[cfg(feature = "backend-postgres")]
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            r#"SELECT "hello-World_""#
        );
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(
            query.to_string(SqliteQueryBuilder),
            r#"SELECT "hello-World_""#
        );
    }

    #[test]
    fn test_quoted_identifier_1() {
        let query = Query::select().column("hel`lo").to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hel``lo`");
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT "hel`lo""#);

        let query = Query::select().column("hel\"lo").to_owned();

        #[cfg(feature = "backend-postgres")]
        assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT "hel""lo""#);
    }

    #[test]
    fn test_quoted_identifier_2() {
        let query = Query::select().column("hel``lo").to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hel````lo`");
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT "hel``lo""#);

        let query = Query::select().column("hel\"\"lo").to_owned();

        #[cfg(feature = "backend-postgres")]
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            r#"SELECT "hel""""lo""#
        );
    }

    #[test]
    fn test_cmp_identifier() {
        type CharLocal = Character;

        assert_eq!(
            ColumnRef::Column(Character::Id.into()),
            ColumnRef::Column(Character::Id.into())
        );
        assert_eq!(
            ColumnRef::Column(Character::Id.into()),
            ColumnRef::Column(Char::Id.into())
        );
        assert_eq!(
            ColumnRef::Column(Character::Id.into()),
            ColumnRef::Column(CharLocal::Id.into())
        );
        assert_eq!(
            ColumnRef::Column(Character::Id.into()),
            ColumnRef::Column(CharReexport::Id.into())
        );
        assert_eq!(
            ColumnRef::Column("id".into()),
            ColumnRef::Column("id".into())
        );
        assert_ne!(
            ColumnRef::Column("id".into()),
            ColumnRef::Column("id_".into())
        );
        assert_eq!(
            ColumnRef::Column(Character::Id.into()),
            ColumnRef::Column("id".into())
        );
        assert_ne!(
            ColumnRef::Column(Character::Id.into()),
            ColumnRef::Column(Character::Table.into())
        );
        assert_eq!(
            ColumnRef::Column(Character::Id.into()),
            ColumnRef::Column(Font::Id.into())
        );
    }
}
