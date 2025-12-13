//! "Core", low-level identifier types.
use std::{borrow::Cow, fmt::Debug};

/// A Rust type that represents an SQL identifier.
///
/// This could be something like a cheap enum that's rendered into an SQL string later.
/// In those cases, prefer implementing [`IdenStatic`] instead if implementing [`Iden`] directly.
pub trait Iden {
    /// A shortcut for writing an [`unquoted`][Iden::unquoted]
    /// identifier into an owned [`String`].
    ///
    /// We can't reuse [`ToString`] for this, because [`ToString`] uses
    /// the [`Display`][std::fmt::Display] representation. But [`Iden`]
    /// representation is distinct from [`Display`][std::fmt::Display]
    /// and can be different.
    fn to_string(&self) -> String {
        self.unquoted().into()
    }

    /// Return a raw identifier string without quotes.
    ///
    /// We intentionally don't reuse [`Display`][std::fmt::Display] for
    /// this, because we want to allow it to have a different logic.
    fn unquoted(&self) -> Cow<'static, str>;
}

impl Iden for String {
    fn unquoted(&self) -> Cow<'static, str> {
        Cow::Owned(self.clone())
    }
}

impl<T> Iden for T
where
    T: IdenStatic,
{
    fn unquoted(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.as_str())
    }
}

/// An SQL identifier ([`Iden`]) that's statically known at compile-time.
///
/// When possible, prefer implementing [`IdenStatic`] instead of implementing [`Iden`] directly.
pub trait IdenStatic {
    /// Return a raw identifier string without quotes, just like [`Iden::unquoted`].
    ///
    /// With an additional guarantee that it's a statically known string that doesn't need to be allocated at runtime.
    fn as_str(&self) -> &'static str;
}

impl IdenStatic for &'static str {
    fn as_str(&self) -> &'static str {
        self
    }
}

/// A string that represents an SQL identifier (like a column name).
///
/// At this stage, it's a raw unprepared string without quotes.
/// The SQL codegen backend will quote it later, in a DB-specific manner.
///
/// ## Why it's called `DynIden`
///
/// The naming is legacy and kept for compatibility.
/// `DynIden` used to be an alias for a `dyn Iden` object that's lazily rendered later.
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

impl std::fmt::Display for DynIden {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub trait IntoIden: Into<DynIden> {
    fn into_iden(self) -> DynIden;
}

impl<T> IntoIden for T
where
    T: Into<DynIden>,
{
    fn into_iden(self) -> DynIden {
        self.into()
    }
}

impl<T> From<T> for DynIden
where
    T: Iden,
{
    fn from(iden: T) -> Self {
        DynIden(iden.unquoted())
    }
}

/// An explicit wrapper for [`Iden`]s which are dynamic user-provided strings.
///
/// Nowadays, `String`/`&str` implement [`Iden`] and can be used directly.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Alias(pub String);

impl Alias {
    pub fn new<T>(n: T) -> Self
    where
        T: Into<String>,
    {
        Self(n.into())
    }
}

impl Iden for Alias {
    fn unquoted(&self) -> Cow<'static, str> {
        Cow::Owned(self.0.clone())
    }
}

/// Null Alias
#[derive(Default, Debug, Copy, Clone)]
pub struct NullAlias;

impl NullAlias {
    pub fn new() -> Self {
        Self
    }
}

impl IdenStatic for NullAlias {
    fn as_str(&self) -> &'static str {
        ""
    }
}

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
