//! Base types used throughout sea-query.

use crate::backend::query_builder::QueryBuilder;
use crate::{expr::*, query::*};
use std::fmt;

#[cfg(not(feature = "thread-safe"))]
pub use std::rc::Rc as SeaRc;
#[cfg(feature = "thread-safe")]
pub use std::sync::Arc as SeaRc;

macro_rules! iden_trait {
    ($($bounds:ident),*) => {
        /// Identifier
        pub trait Iden where $(Self: $bounds),* {
            fn prepare(&self, s: &mut dyn fmt::Write, q: char) {
                write!(s, "{}{}{}", q, self.quoted(q), q).unwrap();
            }

            fn quoted(&self, q: char) -> String {
                let mut b = [0; 4];
                let qq: &str = q.encode_utf8(&mut b);
                self.to_string().replace(qq, qq.repeat(2).as_str())
            }

            fn to_string(&self) -> String {
                let s = &mut String::new();
                self.unquoted(s);
                s.to_owned()
            }

            fn unquoted(&self, s: &mut dyn fmt::Write);
        }
    };
}

#[cfg(feature = "thread-safe")]
iden_trait!(Send, Sync);
#[cfg(not(feature = "thread-safe"))]
iden_trait!();

pub type DynIden = SeaRc<dyn Iden>;

pub trait IntoIden {
    fn into_iden(self) -> DynIden;
}

pub trait IdenList {
    type IntoIter: Iterator<Item = DynIden>;

    fn into_iter(self) -> Self::IntoIter;
}

impl fmt::Debug for dyn Iden {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unquoted(formatter);
        Ok(())
    }
}

/// Indicates that a SQL type is supported for use in queries.
/// Convert a value to a String for use in queries.
pub trait QueryValue<DB> {
    /// Returns the value as an escaped string safe for use in queries.
    fn query_value(&self) -> String;
}

impl<'a, DB> fmt::Debug for &'a dyn QueryValue<DB> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.query_value())
    }
}

macro_rules! query_value_all {
    ( $ty: ty, |$self: ident| $query_value: block ) => {
        impl<DB> QueryValue<DB> for $ty
        where
            DB: QueryBuilder<DB>,
        {
            fn query_value(& $self) -> String {
                $query_value
            }
        }
    };

    ( $ty: ty, |$self: ident| $query_value: block, quoted ) => {
        impl<DB> QueryValue<DB> for $ty
        where
            DB: QueryBuilder<DB>,
        {
            fn query_value(& $self) -> String {
                let mut buf = String::new();
                let string = { $query_value };
                DB::write_string_quoted(string, &mut buf);
                buf
            }
        }
    };

    ( $ty: ty, |$self: ident| $query_value: block, wrapped ) => {
        impl<DB> QueryValue<DB> for $ty
        where
            DB: QueryBuilder<DB>,
        {
            fn query_value(& $self) -> String {
                QueryValue::<DB>::query_value({ $query_value })
            }
        }
    };
}

query_value_all!(String, |self| { self }, quoted);
query_value_all!(&str, |self| { self }, quoted);
query_value_all!(str, |self| { self.to_string() });
query_value_all!(bool, |self| { self.to_string() });
query_value_all!(f32, |self| { self.to_string() });
query_value_all!(f64, |self| { self.to_string() });
query_value_all!(i16, |self| { self.to_string() });
query_value_all!(i32, |self| { self.to_string() });
query_value_all!(i64, |self| { self.to_string() });
query_value_all!(i8, |self| { self.to_string() });
query_value_all!(u16, |self| { self.to_string() });
query_value_all!(u32, |self| { self.to_string() });
query_value_all!(u64, |self| { self.to_string() });
query_value_all!(u8, |self| { self.to_string() });
query_value_all!([u8], |self| {
    format!(
        "x\'{}\'",
        self.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>(),
    )
});
query_value_all!(Vec<u8>, |self| { self.as_slice() }, wrapped);
query_value_all!(dyn Iden, |self| { &self.to_string() }, wrapped);
query_value_all!(DynIden, |self| { &self.to_string() }, wrapped);
#[cfg(feature = "with-uuid")]
query_value_all!(uuid::Uuid, |self| { &self.to_string() }, quoted);
#[cfg(feature = "with-chrono")]
query_value_all!(
    chrono::DateTime<chrono::FixedOffset>,
    |self| { &self.format("%Y-%m-%d %H:%M:%S").to_string() },
    wrapped
);
#[cfg(feature = "with-chrono")]
query_value_all!(
    chrono::NaiveDateTime,
    |self| { &self.format("%Y-%m-%d %H:%M:%S").to_string() },
    wrapped
);

/// Column references
#[derive(Debug, Clone)]
pub enum ColumnRef {
    Column(DynIden),
    TableColumn(DynIden, DynIden),
}

pub trait IntoColumnRef {
    fn into_column_ref(self) -> ColumnRef;
}

/// Table references
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum TableRef<'a, DB> {
    Table(DynIden),
    SchemaTable(DynIden, DynIden),
    TableAlias(DynIden, DynIden),
    SchemaTableAlias(DynIden, DynIden, DynIden),
    SubQuery(SelectStatement<'a, DB>, DynIden),
}

pub trait IntoTableRef<'a, DB> {
    fn into_table_ref(self) -> TableRef<'a, DB>;
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOper {
    Not,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    As,
    #[cfg(feature = "backend-postgres")]
    Matches,
    #[cfg(feature = "backend-postgres")]
    Contains,
    #[cfg(feature = "backend-postgres")]
    Contained,
    #[cfg(feature = "backend-postgres")]
    Concatenate,
}

/// Logical chain operator
#[derive(Debug, Clone)]
pub enum LogicalChainOper<'a, DB> {
    And(SimpleExpr<'a, DB>),
    Or(SimpleExpr<'a, DB>),
}

/// Join types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Join,
    InnerJoin,
    LeftJoin,
    RightJoin,
}

/// Order expression
#[derive(Debug, Clone)]
pub struct OrderExpr<'a, DB> {
    pub(crate) expr: SimpleExpr<'a, DB>,
    pub(crate) order: Order,
}

/// Join on types
#[derive(Debug, Clone)]
pub enum JoinOn<'a, DB> {
    Condition(Box<ConditionHolder<'a, DB>>),
    Columns(Vec<SimpleExpr<'a, DB>>),
}

/// Ordering options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Asc,
    Desc,
}

/// Helper for create name alias
#[derive(Debug, Clone)]
pub struct Alias(String);

/// Null Alias
#[derive(Debug, Copy, Clone)]
pub struct NullAlias;

/// Common SQL Keywords
#[derive(Debug, Clone)]
pub enum Keyword {
    Null,
    Custom(DynIden),
}

// Impl begins

impl<T: 'static> IntoIden for T
where
    T: Iden,
{
    fn into_iden(self) -> DynIden {
        SeaRc::new(self)
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
    type IntoIter = std::vec::IntoIter<DynIden>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0.into_iden(), self.1.into_iden()].into_iter()
    }
}

impl<A, B, C> IdenList for (A, B, C)
where
    A: IntoIden,
    B: IntoIden,
    C: IntoIden,
{
    type IntoIter = std::vec::IntoIter<DynIden>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0.into_iden(), self.1.into_iden(), self.2.into_iden()].into_iter()
    }
}

impl IntoColumnRef for ColumnRef {
    fn into_column_ref(self) -> ColumnRef {
        self
    }
}

impl<T: 'static> IntoColumnRef for T
where
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Column(self.into_iden())
    }
}

impl<S: 'static, T: 'static> IntoColumnRef for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::TableColumn(self.0.into_iden(), self.1.into_iden())
    }
}

impl<'a, DB> IntoTableRef<'a, DB> for TableRef<'a, DB> {
    fn into_table_ref(self) -> TableRef<'a, DB> {
        self
    }
}

impl<'a, DB, T: 'static> IntoTableRef<'a, DB> for T
where
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef<'a, DB> {
        TableRef::Table(self.into_iden())
    }
}

impl<'a, DB, S: 'static, T: 'static> IntoTableRef<'a, DB> for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef<'a, DB> {
        TableRef::SchemaTable(self.0.into_iden(), self.1.into_iden())
    }
}

impl<'a, DB> TableRef<'a, DB> {
    /// Add or replace the current alias
    pub fn alias<A: 'static>(self, alias: A) -> Self
    where
        A: IntoIden,
    {
        match self {
            Self::Table(table) => Self::TableAlias(table, alias.into_iden()),
            Self::TableAlias(table, _) => Self::TableAlias(table, alias.into_iden()),
            Self::SchemaTable(schema, table) => {
                Self::SchemaTableAlias(schema, table, alias.into_iden())
            }
            Self::SchemaTableAlias(schema, table, _) => {
                Self::SchemaTableAlias(schema, table, alias.into_iden())
            }
            Self::SubQuery(statement, _) => Self::SubQuery(statement, alias.into_iden()),
        }
    }
}

impl Alias {
    pub fn new(n: &str) -> Self {
        Self(n.to_owned())
    }
}

impl Iden for Alias {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        write!(s, "{}", self.0).unwrap();
    }
}

impl NullAlias {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for NullAlias {
    fn default() -> Self {
        Self
    }
}

impl Iden for NullAlias {
    fn unquoted(&self, _s: &mut dyn fmt::Write) {}
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_identifier() {
        #[cfg(feature = "backend-mysql")]
        let query_mysql = MySqlQuery::select()
            .column(Alias::new("hello-World_"))
            .to_owned();

        #[cfg(feature = "backend-postgres")]
        let query_postgres = PgQuery::select()
            .column(Alias::new("hello-World_"))
            .to_owned();

        #[cfg(feature = "backend-sqlite")]
        let query_sqlite = SqliteQuery::select()
            .column(Alias::new("hello-World_"))
            .to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query_mysql.to_string(), r#"SELECT `hello-World_`"#);
        #[cfg(feature = "backend-postgres")]
        assert_eq!(query_postgres.to_string(), r#"SELECT "hello-World_""#);
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query_sqlite.to_string(), r#"SELECT `hello-World_`"#);
    }

    #[test]
    fn test_quoted_identifier_1() {
        #[cfg(feature = "backend-mysql")]
        let query_mysql = MySqlQuery::select().column(Alias::new("hel`lo")).to_owned();

        #[cfg(feature = "backend-postgres")]
        let query_postgres = PgQuery::select().column(Alias::new("hel`lo")).to_owned();

        #[cfg(feature = "backend-sqlite")]
        let query_sqlite = SqliteQuery::select()
            .column(Alias::new("hel`lo"))
            .to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query_mysql.to_string(), r#"SELECT `hel``lo`"#);
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query_postgres.to_string(), r#"SELECT "hel`lo""#);
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query_sqlite.to_string(), r#"SELECT `hel``lo`"#);
    }

    #[test]
    fn test_quoted_identifier_2() {
        #[cfg(feature = "backend-mysql")]
        let query_mysql = MySqlQuery::select()
            .column(Alias::new("hel``lo"))
            .to_owned();

        #[cfg(feature = "backend-postgres")]
        let query_postgres = PgQuery::select().column(Alias::new("hel``lo")).to_owned();

        #[cfg(feature = "backend-sqlite")]
        let query_sqlite = SqliteQuery::select()
            .column(Alias::new("hel``lo"))
            .to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query_mysql.to_string(), r#"SELECT `hel````lo`"#);
        #[cfg(feature = "backend-postgres")]
        assert_eq!(query_postgres.to_string(), r#"SELECT "hel``lo""#);
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query_sqlite.to_string(), r#"SELECT `hel````lo`"#);
    }
}
