use crate::{backend::QueryBuilder, prepare::*, types::*, QuotedBuilder};

/// Helper for constructing any type statement
#[derive(Debug)]
pub struct Type;

#[derive(Clone, Debug)]
pub enum TypeRef {
    Type(DynIden),
    SchemaType(DynIden, DynIden),
    DatabaseSchemaType(DynIden, DynIden, DynIden),
}

pub trait IntoTypeRef {
    fn into_type_ref(self) -> TypeRef;
}

impl IntoTypeRef for TypeRef {
    fn into_type_ref(self) -> TypeRef {
        self
    }
}

impl<I> IntoTypeRef for I
where
    I: IntoIden,
{
    fn into_type_ref(self) -> TypeRef {
        TypeRef::Type(self.into_iden())
    }
}

impl<A, B> IntoTypeRef for (A, B)
where
    A: IntoIden,
    B: IntoIden,
{
    fn into_type_ref(self) -> TypeRef {
        TypeRef::SchemaType(self.0.into_iden(), self.1.into_iden())
    }
}

impl<A, B, C> IntoTypeRef for (A, B, C)
where
    A: IntoIden,
    B: IntoIden,
    C: IntoIden,
{
    fn into_type_ref(self) -> TypeRef {
        TypeRef::DatabaseSchemaType(self.0.into_iden(), self.1.into_iden(), self.2.into_iden())
    }
}

#[derive(Debug, Clone, Default)]
pub struct TypeCreateStatement {
    pub(crate) name: Option<TypeRef>,
    pub(crate) as_type: Option<TypeAs>,
    pub(crate) values: Vec<DynIden>,
}

#[derive(Debug, Clone)]
pub enum TypeAs {
    // Composite,
    Enum,
    /* Range,
     * Base,
     * Array, */
}

#[derive(Debug, Clone, Default)]
pub struct TypeDropStatement {
    pub(crate) names: Vec<TypeRef>,
    pub(crate) option: Option<TypeDropOpt>,
    pub(crate) if_exists: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TypeAlterStatement {
    pub(crate) name: Option<TypeRef>,
    pub(crate) option: Option<TypeAlterOpt>,
}

#[derive(Debug, Clone)]
pub enum TypeDropOpt {
    Cascade,
    Restrict,
}

#[derive(Debug, Clone)]
pub enum TypeAlterOpt {
    Add(DynIden, Option<TypeAlterAddOpt>),
    Rename(DynIden),
    RenameValue(DynIden, DynIden),
}

#[derive(Debug, Clone)]
pub enum TypeAlterAddOpt {
    Before(DynIden),
    After(DynIden),
}

pub trait TypeBuilder: QuotedBuilder {
    /// Translate [`TypeCreateStatement`] into database specific SQL statement.
    fn prepare_type_create_statement(&self, create: &TypeCreateStatement, sql: &mut dyn SqlWriter);

    /// Translate [`TypeDropStatement`] into database specific SQL statement.
    fn prepare_type_drop_statement(&self, drop: &TypeDropStatement, sql: &mut dyn SqlWriter);

    /// Translate [`TypeAlterStatement`] into database specific SQL statement.
    fn prepare_type_alter_statement(&self, alter: &TypeAlterStatement, sql: &mut dyn SqlWriter);

    /// Translate [`TypeRef`] into SQL statement.
    fn prepare_type_ref(&self, type_ref: &TypeRef, sql: &mut dyn SqlWriter) {
        match type_ref {
            TypeRef::Type(name) => {
                name.prepare(sql.as_writer(), self.quote());
            }
            TypeRef::SchemaType(schema, name) => {
                schema.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                name.prepare(sql.as_writer(), self.quote());
            }
            TypeRef::DatabaseSchemaType(database, schema, name) => {
                database.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                schema.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                name.prepare(sql.as_writer(), self.quote());
            }
        }
    }
}

pub trait TypeStatementBuilder {
    fn build<T: TypeBuilder>(&self, type_builder: T) -> String {
        self.build_ref(&type_builder)
    }

    fn build_ref<T: TypeBuilder>(&self, type_builder: &T) -> String {
        let mut sql = String::with_capacity(256);
        self.build_collect_ref(type_builder, &mut sql)
    }

    fn build_collect<T: TypeBuilder>(&self, type_builder: T, sql: &mut dyn SqlWriter) -> String {
        self.build_collect_ref(&type_builder, sql)
    }

    fn build_collect_ref<T: TypeBuilder>(
        &self,
        type_builder: &T,
        sql: &mut dyn SqlWriter,
    ) -> String;

    /// Build corresponding SQL statement and return SQL string
    fn to_string<T>(&self, type_builder: T) -> String
    where
        T: TypeBuilder + QueryBuilder,
    {
        self.build_ref(&type_builder)
    }
}

impl Type {
    /// Construct type [`TypeCreateStatement`]
    pub fn create() -> TypeCreateStatement {
        TypeCreateStatement::new()
    }

    /// Construct type [`TypeDropStatement`]
    pub fn drop() -> TypeDropStatement {
        TypeDropStatement::new()
    }

    /// Construct type [`TypeAlterStatement`]
    pub fn alter() -> TypeAlterStatement {
        TypeAlterStatement::new()
    }
}

impl TypeCreateStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create enum as custom type
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// enum FontFamily {
    ///     Type,
    ///     Serif,
    ///     Sans,
    ///     Monospace,
    /// }
    ///
    /// impl Iden for FontFamily {
    ///     fn unquoted(&self, s: &mut dyn std::fmt::Write) {
    ///         write!(
    ///             s,
    ///             "{}",
    ///             match self {
    ///                 Self::Type => "font_family",
    ///                 Self::Serif => "serif",
    ///                 Self::Sans => "sans",
    ///                 Self::Monospace => "monospace",
    ///             }
    ///         )
    ///         .unwrap();
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     Type::create()
    ///         .as_enum(FontFamily::Type)
    ///         .values(vec![
    ///             FontFamily::Serif,
    ///             FontFamily::Sans,
    ///             FontFamily::Monospace
    ///         ])
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"CREATE TYPE "font_family" AS ENUM ('serif', 'sans', 'monospace')"#
    /// );
    /// ```
    pub fn as_enum<T: 'static>(&mut self, name: T) -> &mut Self
    where
        T: IntoTypeRef,
    {
        self.name = Some(name.into_type_ref());
        self.as_type = Some(TypeAs::Enum);
        self
    }

    pub fn values<T, I>(&mut self, values: I) -> &mut Self
    where
        T: IntoIden,
        I: IntoIterator<Item = T>,
    {
        for v in values.into_iter() {
            self.values.push(v.into_iden());
        }
        self
    }
}

impl TypeStatementBuilder for TypeCreateStatement {
    fn build_collect_ref<T: TypeBuilder>(
        &self,
        type_builder: &T,
        sql: &mut dyn SqlWriter,
    ) -> String {
        type_builder.prepare_type_create_statement(self, sql);
        sql.to_string()
    }
}

impl TypeDropStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Drop a type
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// struct FontFamily;
    ///
    /// impl Iden for FontFamily {
    ///     fn unquoted(&self, s: &mut dyn std::fmt::Write) {
    ///         write!(s, "{}", "font_family").unwrap();
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     Type::drop()
    ///         .if_exists()
    ///         .name(FontFamily)
    ///         .restrict()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"DROP TYPE IF EXISTS "font_family" RESTRICT"#
    /// );
    /// ```
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: IntoTypeRef,
    {
        self.names.push(name.into_type_ref());
        self
    }

    pub fn names<T, I>(&mut self, names: I) -> &mut Self
    where
        T: IntoTypeRef,
        I: IntoIterator<Item = T>,
    {
        for n in names.into_iter() {
            self.names.push(n.into_type_ref());
        }
        self
    }

    /// Set `IF EXISTS`
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }

    /// Set `CASCADE`
    pub fn cascade(&mut self) -> &mut Self {
        self.option = Some(TypeDropOpt::Cascade);
        self
    }

    /// Set `RESTRICT`
    pub fn restrict(&mut self) -> &mut Self {
        self.option = Some(TypeDropOpt::Restrict);
        self
    }
}

impl TypeStatementBuilder for TypeDropStatement {
    fn build_collect_ref<T: TypeBuilder>(
        &self,
        type_builder: &T,
        sql: &mut dyn SqlWriter,
    ) -> String {
        type_builder.prepare_type_drop_statement(self, sql);
        sql.to_string()
    }
}

impl TypeAlterStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Change the definition of a type
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// enum FontFamily {
    ///     Type,
    ///     Serif,
    ///     Sans,
    ///     Monospace,
    /// }
    ///
    /// impl Iden for FontFamily {
    ///     fn unquoted(&self, s: &mut dyn std::fmt::Write) {
    ///         write!(
    ///             s,
    ///             "{}",
    ///             match self {
    ///                 Self::Type => "font_family",
    ///                 Self::Serif => "serif",
    ///                 Self::Sans => "sans",
    ///                 Self::Monospace => "monospace",
    ///             }
    ///         )
    ///         .unwrap();
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(FontFamily::Type)
    ///         .add_value(Alias::new("cursive"))
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font_family" ADD VALUE 'cursive'"#
    /// );
    /// ```
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: IntoTypeRef,
    {
        self.name = Some(name.into_type_ref());
        self
    }

    pub fn add_value<T>(self, value: T) -> Self
    where
        T: IntoIden,
    {
        self.alter_option(TypeAlterOpt::Add(value.into_iden(), None))
    }

    /// Add a enum value before an existing value
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .add_value(Alias::new("weight"))
    ///         .before(Font::Variant)
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" ADD VALUE 'weight' BEFORE 'variant'"#
    /// )
    /// ```
    pub fn before<T>(mut self, value: T) -> Self
    where
        T: IntoIden,
    {
        if let Some(option) = self.option {
            self.option = Some(option.before(value));
        }
        self
    }

    pub fn after<T>(mut self, value: T) -> Self
    where
        T: IntoIden,
    {
        if let Some(option) = self.option {
            self.option = Some(option.after(value));
        }
        self
    }

    pub fn rename_to<T>(self, name: T) -> Self
    where
        T: IntoIden,
    {
        self.alter_option(TypeAlterOpt::Rename(name.into_iden()))
    }

    /// Rename a enum value
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .rename_value(Alias::new("variant"), Alias::new("language"))
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" RENAME VALUE 'variant' TO 'language'"#
    /// )
    /// ```
    pub fn rename_value<T, V>(self, existing: T, new_name: V) -> Self
    where
        T: IntoIden,
        V: IntoIden,
    {
        self.alter_option(TypeAlterOpt::RenameValue(
            existing.into_iden(),
            new_name.into_iden(),
        ))
    }

    fn alter_option(mut self, option: TypeAlterOpt) -> Self {
        self.option = Some(option);
        self
    }
}

impl TypeStatementBuilder for TypeAlterStatement {
    fn build_collect_ref<T: TypeBuilder>(
        &self,
        type_builder: &T,
        sql: &mut dyn SqlWriter,
    ) -> String {
        type_builder.prepare_type_alter_statement(self, sql);
        sql.to_string()
    }
}

impl TypeAlterOpt {
    /// Changes only `ADD VALUE x` options into `ADD VALUE x BEFORE` options, does nothing otherwise
    pub fn before<T>(self, value: T) -> Self
    where
        T: IntoIden,
    {
        match self {
            TypeAlterOpt::Add(iden, _) => {
                Self::Add(iden, Some(TypeAlterAddOpt::Before(value.into_iden())))
            }
            _ => self,
        }
    }

    /// Changes only `ADD VALUE x` options into `ADD VALUE x AFTER` options, does nothing otherwise
    pub fn after<T>(self, value: T) -> Self
    where
        T: IntoIden,
    {
        match self {
            TypeAlterOpt::Add(iden, _) => {
                Self::Add(iden, Some(TypeAlterAddOpt::After(value.into_iden())))
            }
            _ => self,
        }
    }
}
