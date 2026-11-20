use crate::ColumnType;
use crate::{PostgresQueryBuilder, QueryBuilder, QuotedBuilder, prepare::*, types::*};

/// Helper for constructing any type statement
#[derive(Debug)]
pub struct Type;

// A similar type used to be defined here. Let's keep exporting it for compatibility.
pub use crate::TypeRef;

// This trait used to be defined here. Let's keep exporting it for compatibility.
pub use crate::IntoTypeRef;

#[derive(Debug, Clone)]
pub(crate) struct CompositeFieldType {
    pub(crate) name: DynIden,
    pub(crate) col_type: ColumnType,
    pub(crate) collation: Option<(Option<DynIden>, DynIden)>,
}

impl<N: IntoIden> From<(N, ColumnType)> for CompositeFieldType {
    fn from((name, col_type): (N, ColumnType)) -> Self {
        Self {
            name: name.into_iden(),
            col_type,
            collation: None,
        }
    }
}

impl<N: IntoIden, C: MaybeQualifiedOnce> From<(N, ColumnType, C)> for CompositeFieldType {
    fn from((name, col_type, collation): (N, ColumnType, C)) -> Self {
        Self {
            collation: Some(collation.into_2_parts()),
            ..Self::from((name, col_type))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TypeCreateStatement {
    pub(crate) name: Option<TypeRef>,
    pub(crate) as_type: Option<TypeAs>,
    pub(crate) values: Vec<DynIden>,
    pub(crate) fields: Vec<CompositeFieldType>,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TypeAs {
    Composite,
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
#[non_exhaustive]
pub enum TypeDropOpt {
    Cascade,
    Restrict,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TypeAlterOpt {
    Add {
        value: DynIden,
        placement: Option<TypeAlterAddOpt>,
        if_not_exists: bool,
    },
    Rename(DynIden),
    RenameValue(DynIden, DynIden),
    AddAttribute {
        name: DynIden,
        col_type: ColumnType,
    },
    DropAttribute(DynIden),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TypeAlterAddOpt {
    Before(DynIden),
    After(DynIden),
}

/// An enum type creation statement
#[derive(Debug, Clone)]
pub struct EnumCreateStatement {
    name: TypeRef,
    values: Vec<DynIden>,
}

impl From<EnumCreateStatement> for TypeCreateStatement {
    fn from(statement: EnumCreateStatement) -> Self {
        Self {
            name: Some(statement.name),
            as_type: Some(TypeAs::Enum),
            values: statement.values,
            ..Self::default()
        }
    }
}

/// A composite type creation statement
#[derive(Debug, Clone)]
pub struct CompositeTypeCreateStatement {
    name: TypeRef,
    fields: Vec<CompositeFieldType>,
}

mod composite_field {
    use super::*;

    pub trait IntoCompositeField {
        fn add_to(self, statement: &mut CompositeTypeCreateStatement);
    }

    impl<T> IntoCompositeField for T
    where
        CompositeFieldType: From<T>,
    {
        fn add_to(self, statement: &mut CompositeTypeCreateStatement) {
            statement.fields.push(CompositeFieldType::from(self));
        }
    }
}

impl CompositeTypeCreateStatement {
    /// Add a field as `(name, type)` or `(name, type, collation)`.
    /// A collation can be qualified as `(schema, name)`.
    pub fn field(mut self, field: impl composite_field::IntoCompositeField) -> Self {
        field.add_to(&mut self);
        self
    }
}

impl From<CompositeTypeCreateStatement> for TypeCreateStatement {
    fn from(statement: CompositeTypeCreateStatement) -> Self {
        Self {
            name: Some(statement.name),
            as_type: Some(TypeAs::Composite),
            fields: statement.fields,
            ..Self::default()
        }
    }
}

pub trait TypeBuilder: QuotedBuilder {
    /// Translate [`TypeCreateStatement`] into database specific SQL statement.
    fn prepare_type_create_statement(&self, create: &TypeCreateStatement, sql: &mut impl SqlWriter);

    /// Translate [`TypeDropStatement`] into database specific SQL statement.
    fn prepare_type_drop_statement(&self, drop: &TypeDropStatement, sql: &mut impl SqlWriter);

    /// Translate [`TypeAlterStatement`] into database specific SQL statement.
    fn prepare_type_alter_statement(&self, alter: &TypeAlterStatement, sql: &mut impl SqlWriter);
}

impl Type {
    /// Construct type [`TypeCreateStatement`]
    pub fn create() -> TypeCreateStatement {
        TypeCreateStatement::new()
    }

    /// Create an enum type
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// assert_eq!(
    ///     Type::create_enum(("app", "status"), ["pending", "approved"])
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"CREATE TYPE "app"."status" AS ENUM ('pending', 'approved')"#,
    /// );
    /// ```
    ///
    /// ```compile_fail,E0277
    /// use sea_query::{extension::postgres::Type, ColumnType};
    ///
    /// Type::create_enum("status", [("code", ColumnType::Integer)]);
    /// ```
    pub fn create_enum(
        name: impl IntoTypeRef,
        values: impl IntoIterator<Item = impl IntoIden>,
    ) -> EnumCreateStatement {
        EnumCreateStatement {
            name: name.into_type_ref(),
            values: values.into_iter().map(IntoIden::into_iden).collect(),
        }
    }

    /// Create a composite type
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// assert_eq!(
    ///     Type::create_composite(("app", "record"))
    ///         .field(("code", ColumnType::Integer))
    ///         .field(("label", ColumnType::Text, ("pg_catalog", "C")))
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"CREATE TYPE "app"."record" AS ("code" integer, "label" text COLLATE "pg_catalog"."C")"#,
    /// );
    /// ```
    ///
    /// ```compile_fail,E0277
    /// use sea_query::extension::postgres::Type;
    ///
    /// Type::create_composite("record").field("pending");
    /// ```
    pub fn create_composite(name: impl IntoTypeRef) -> CompositeTypeCreateStatement {
        CompositeTypeCreateStatement {
            name: name.into_type_ref(),
            fields: Vec::new(),
        }
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
    /// #[derive(Iden)]
    /// enum FontFamily {
    ///     #[iden = "font_family"]
    ///     Type,
    ///     Serif,
    ///     Sans,
    ///     Monospace,
    /// }
    ///
    /// assert_eq!(
    ///     Type::create()
    ///         .as_enum(FontFamily::Type)
    ///         .values([FontFamily::Serif, FontFamily::Sans, FontFamily::Monospace])
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"CREATE TYPE "font_family" AS ENUM ('serif', 'sans', 'monospace')"#
    /// );
    /// ```
    pub fn as_enum<T>(&mut self, name: T) -> &mut Self
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
    ///     fn unquoted(&self) -> &str {
    ///         "font_family"
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

    /// Drop multiple types
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// #[derive(Iden)]
    /// enum KycStatus {
    ///     #[iden = "kyc_status"]
    ///     Type,
    ///     Pending,
    ///     Approved,
    /// }
    ///
    /// #[derive(Iden)]
    /// enum FontFamily {
    ///     #[iden = "font_family"]
    ///     Type,
    ///     Aerial,
    ///     Forte,
    /// }
    ///
    /// assert_eq!(
    ///     Type::drop()
    ///         .if_exists()
    ///         .names([KycStatus::Type.into_iden(), FontFamily::Type.into_iden()])
    ///         .cascade()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"DROP TYPE IF EXISTS "kyc_status", "font_family" CASCADE"#
    /// );
    /// ```
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
    ///     fn unquoted(&self) -> &str {
    ///         match self {
    ///             Self::Type => "font_family",
    ///             Self::Serif => "serif",
    ///             Self::Sans => "sans",
    ///             Self::Monospace => "monospace",
    ///         }
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(FontFamily::Type)
    ///         .add_value("cursive")
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
        self.alter_option(TypeAlterOpt::Add {
            value: value.into_iden(),
            placement: None,
            if_not_exists: false,
        })
    }

    /// Add an enum value if it does not already exist
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .add_value_if_not_exists("weight")
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" ADD VALUE IF NOT EXISTS 'weight'"#
    /// );
    /// ```
    pub fn add_value_if_not_exists(self, value: impl IntoIden) -> Self {
        self.add_value(value).if_not_exists()
    }

    /// Add a enum value before an existing value
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .add_value("weight")
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

    /// Add a enum value if not already exists
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .add_value("weight")
    ///         .if_not_exists()
    ///         .after(Font::Variant)
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" ADD VALUE IF NOT EXISTS 'weight' AFTER 'variant'"#
    /// )
    /// ```
    pub fn if_not_exists(mut self) -> Self {
        if let Some(option) = self.option {
            self.option = Some(option.if_not_exists());
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
    ///         .rename_value("variant", "language")
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

    /// Set the attribute to add
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .add_attribute(Font::Variant, ColumnType::Text)
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" ADD ATTRIBUTE "variant" text"#
    /// )
    /// ```
    pub fn add_attribute(self, name: impl IntoIden, col_type: ColumnType) -> Self {
        self.alter_option(TypeAlterOpt::AddAttribute {
            name: name.into_iden(),
            col_type,
        })
    }

    /// Set the attribute to drop
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .drop_attribute(Font::Variant)
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" DROP ATTRIBUTE "variant""#
    /// )
    /// ```
    pub fn drop_attribute(self, name: impl IntoIden) -> Self {
        self.alter_option(TypeAlterOpt::DropAttribute(name.into_iden()))
    }

    fn alter_option(mut self, option: TypeAlterOpt) -> Self {
        self.option = Some(option);
        self
    }
}

impl TypeAlterOpt {
    /// Changes only `ADD VALUE x` options into `ADD VALUE x BEFORE` options, does nothing otherwise
    pub fn before<T>(self, value: T) -> Self
    where
        T: IntoIden,
    {
        match self {
            TypeAlterOpt::Add {
                value: iden,
                if_not_exists,
                ..
            } => Self::Add {
                value: iden,
                if_not_exists,
                placement: Some(TypeAlterAddOpt::Before(value.into_iden())),
            },
            _ => self,
        }
    }

    /// Changes only `ADD VALUE x` options into `ADD VALUE x AFTER` options, does nothing otherwise
    pub fn after<T>(self, value: T) -> Self
    where
        T: IntoIden,
    {
        match self {
            TypeAlterOpt::Add {
                value: iden,
                if_not_exists,
                ..
            } => Self::Add {
                value: iden,
                if_not_exists,
                placement: Some(TypeAlterAddOpt::After(value.into_iden())),
            },
            _ => self,
        }
    }

    /// Changes only `ADD VALUE x` options into `ADD VALUE IF NOT EXISTS x` options, does nothing otherwise
    pub fn if_not_exists(self) -> Self {
        match self {
            TypeAlterOpt::Add {
                value, placement, ..
            } => Self::Add {
                value,
                placement,
                if_not_exists: true,
            },
            _ => self,
        }
    }
}

macro_rules! impl_type_statement_builder {
    ( $struct_name: ident, $func_name: ident ) => {
        impl $struct_name {
            pub fn build_ref<T: TypeBuilder>(&self, type_builder: &T) -> String {
                let mut sql = String::with_capacity(256);
                self.build_collect_ref(type_builder, &mut sql)
            }

            pub fn build_collect<T: TypeBuilder>(
                &self,
                type_builder: T,
                sql: &mut impl SqlWriter,
            ) -> String {
                self.build_collect_ref(&type_builder, sql)
            }

            pub fn build_collect_ref<T: TypeBuilder>(
                &self,
                type_builder: &T,
                sql: &mut impl SqlWriter,
            ) -> String {
                type_builder.$func_name(self, sql);
                sql.to_string()
            }

            /// Build corresponding SQL statement and return SQL string
            pub fn to_string<T>(&self, type_builder: T) -> String
            where
                T: TypeBuilder + QueryBuilder,
            {
                self.build_ref(&type_builder)
            }
        }
    };
}

macro_rules! impl_postgres_type_statement_builder {
    ( $struct_name: ident, $func_name: ident, $members: ident ) => {
        impl $struct_name {
            pub fn build_ref(&self, type_builder: &PostgresQueryBuilder) -> String {
                let mut sql = String::with_capacity(256);
                self.build_collect_ref(type_builder, &mut sql)
            }

            pub fn build_collect(
                &self,
                type_builder: PostgresQueryBuilder,
                sql: &mut impl SqlWriter,
            ) -> String {
                self.build_collect_ref(&type_builder, sql)
            }

            pub fn build_collect_ref(
                &self,
                type_builder: &PostgresQueryBuilder,
                sql: &mut impl SqlWriter,
            ) -> String {
                type_builder.$func_name(Some(&self.name), &self.$members, sql);
                sql.to_string()
            }

            /// Build corresponding SQL statement and return SQL string
            pub fn to_string(&self, type_builder: PostgresQueryBuilder) -> String {
                self.build_ref(&type_builder)
            }
        }
    };
}

impl_type_statement_builder!(TypeCreateStatement, prepare_type_create_statement);
impl_type_statement_builder!(TypeAlterStatement, prepare_type_alter_statement);
impl_type_statement_builder!(TypeDropStatement, prepare_type_drop_statement);

impl_postgres_type_statement_builder!(
    EnumCreateStatement,
    prepare_enum_type_create_statement,
    values
);
impl_postgres_type_statement_builder!(
    CompositeTypeCreateStatement,
    prepare_composite_type_create_statement,
    fields
);
