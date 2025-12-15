//! "Core", low-level identifier types.
use std::{borrow::Cow, fmt::Debug};

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

impl Iden for String {
    fn quoted(&self) -> Cow<'static, str> {
        Cow::Owned(self.clone())
    }

    fn unquoted(&self) -> &str {
        self
    }
}

#[cfg(feature = "thread-safe")]
/// Identifier statically known at compile-time.
pub trait IdenStatic: Iden + Copy + Send + Sync + 'static {
    fn as_str(&self) -> &'static str;
}

#[cfg(not(feature = "thread-safe"))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DynIden(pub(crate) Cow<'static, str>);

impl DynIden {
    pub fn inner(&self) -> Cow<'static, str> {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DynIden {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for DynIden {
    fn as_ref(&self) -> &str {
        &self.0
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
        DynIden(iden.quoted())
    }
}

/// An explicit wrapper for [`Iden`]s which are dynamic user-provided strings.
///
/// Nowadays, `&str` implements [`Iden`] and can be used directly.
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
    fn quoted(&self) -> Cow<'static, str> {
        Cow::Owned(self.0.clone())
    }

    fn unquoted(&self) -> &str {
        &self.0
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

impl Iden for NullAlias {
    fn unquoted(&self) -> &str {
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
