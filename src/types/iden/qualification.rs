//! Conversion traits/impls for "potentially qualified" names like `(schema?).(table?).column`.

use super::*;

// -------------------------- MaybeQualifiedOnce -------------------------------

/// A name that can be unqualified (`foo`) or qualified once (`foo.bar`).
///
/// This is mostly a "private" helper trait to provide reusable conversions.
pub trait MaybeQualifiedOnce {
    /// Represent a maybe-qualified name as a `(foo?, bar)` tuple.
    fn into_2_parts(self) -> (Option<DynIden>, DynIden);
}

/// Only the "base", no qualification (`foo`).
impl<T> MaybeQualifiedOnce for T
where
    T: IntoIden,
{
    fn into_2_parts(self) -> (Option<DynIden>, DynIden) {
        (None, self.into_iden())
    }
}

/// With a qualification (`foo.bar`).
impl<S, T> MaybeQualifiedOnce for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_2_parts(self) -> (Option<DynIden>, DynIden) {
        let (qual, base) = self;
        (Some(qual.into_iden()), base.into_iden())
    }
}

// ------------------------- MaybeQualifiedTwice -------------------------------

/// A name that can be unqualified (`foo`), qualified once (`foo.bar`), or twice (`foo.bar.baz`).
///
/// This is mostly a "private" helper trait to provide reusable conversions.
pub trait MaybeQualifiedTwice {
    /// Represent a maybe-qualified name as a `(foo?, bar?, baz)` tuple.
    ///
    /// To be precise, it's actually `((foo?, bar)?, baz)` to rule out invalid states like `(Some, None, Some)`.
    fn into_3_parts(self) -> (Option<(Option<DynIden>, DynIden)>, DynIden);
}

/// From 1 or 2 parts (`foo` or `foo.bar`).
impl<T> MaybeQualifiedTwice for T
where
    T: MaybeQualifiedOnce,
{
    fn into_3_parts(self) -> (Option<(Option<DynIden>, DynIden)>, DynIden) {
        let (middle, base) = self.into_2_parts();
        let qual = middle.map(|middle| (None, middle));
        (qual, base)
    }
}

/// Fully-qualified from 3 parts (`foo.bar.baz`).
impl<S, T, U> MaybeQualifiedTwice for (S, T, U)
where
    S: IntoIden,
    T: IntoIden,
    U: IntoIden,
{
    fn into_3_parts(self) -> (Option<(Option<DynIden>, DynIden)>, DynIden) {
        let (q2, q1, base) = self;
        let (q2, q1, base) = (q2.into_iden(), q1.into_iden(), base.into_iden());
        let q = (Some(q2), q1);
        (Some(q), base)
    }
}

// -------------------------------- impls --------------------------------------

/// Construct a [`SchemaName`] from 1-2 parts (`(database?).schema`)
impl<T> From<T> for SchemaName
where
    T: MaybeQualifiedOnce,
{
    fn from(value: T) -> Self {
        let (db, schema) = value.into_2_parts();
        let db_name = db.map(DatabaseName);
        SchemaName(db_name, schema)
    }
}

/// Construct a [`TypeRef`] from 1-3 parts (`(database?).(schema?).type`)
impl<T> From<T> for TypeRef
where
    T: MaybeQualifiedTwice,
{
    fn from(value: T) -> Self {
        let (schema_parts, r#type) = value.into_3_parts();
        let schema_name = schema_parts.map(|schema_parts| match schema_parts {
            (Some(db), schema) => SchemaName(Some(DatabaseName(db)), schema),
            (None, schema) => SchemaName(None, schema),
        });
        TypeRef(schema_name, r#type)
    }
}

/// Construct a [`TableName`] from 1-3 parts (`(database?).(schema?).table`)
impl<T> From<T> for TableName
where
    T: MaybeQualifiedTwice,
{
    fn from(value: T) -> Self {
        let (schema_parts, table) = value.into_3_parts();
        let schema_name = schema_parts.map(|schema_parts| match schema_parts {
            (Some(db), schema) => SchemaName(Some(DatabaseName(db)), schema),
            (None, schema) => SchemaName(None, schema),
        });
        TableName(schema_name, table)
    }
}

/// Construct a [`ColumnName`] from 1-3 parts (`(schema?).(table?).column`)
impl<T> From<T> for ColumnName
where
    T: MaybeQualifiedTwice,
{
    fn from(value: T) -> Self {
        let (table_parts, column) = value.into_3_parts();
        let table_name = table_parts.map(|table_parts| match table_parts {
            (Some(schema), table) => TableName(Some(schema.into()), table),
            (None, table) => TableName(None, table),
        });
        ColumnName(table_name, column)
    }
}
