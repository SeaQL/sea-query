use crate::{expr::*, types::*};

/// Specification of a table column
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub(crate) table: Option<TableRef>,
    pub(crate) name: DynIden,
    pub(crate) types: Option<ColumnType>,
    pub(crate) spec: Vec<ColumnSpec>,
}

/// All column types
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ColumnType {
    Char(Option<u32>),
    String(Option<u32>),
    Text,
    TinyInteger,
    SmallInteger,
    Integer,
    BigInteger,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    Decimal(Option<(u32, u32)>),
    DateTime,
    Timestamp,
    TimestampWithTimeZone,
    Time,
    Date,
    Year(Option<MySqlYear>),
    Interval(Option<PgInterval>, Option<u32>),
    Binary(BlobSize),
    VarBinary(u32),
    Bit(Option<u32>),
    VarBit(u32),
    Boolean,
    Money(Option<(u32, u32)>),
    Json,
    JsonBinary,
    Uuid,
    Custom(DynIden),
    Enum {
        name: DynIden,
        variants: Vec<DynIden>,
    },
    Array(RcOrArc<ColumnType>),
    Cidr,
    Inet,
    MacAddr,
    LTree,
}

impl PartialEq for ColumnType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Decimal(l0), Self::Decimal(r0)) => l0 == r0,
            (Self::Year(l0), Self::Year(r0)) => l0 == r0,
            (Self::Interval(l0, l1), Self::Interval(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Binary(l0), Self::Binary(r0)) => l0 == r0,
            (Self::VarBinary(l0), Self::VarBinary(r0)) => l0 == r0,
            (Self::Bit(l0), Self::Bit(r0)) => l0 == r0,
            (Self::VarBit(l0), Self::VarBit(r0)) => l0 == r0,
            (Self::Money(l0), Self::Money(r0)) => l0 == r0,
            (Self::Custom(l0), Self::Custom(r0)) => l0.to_string() == r0.to_string(),
            (
                Self::Enum {
                    name: l_name,
                    variants: l_variants,
                },
                Self::Enum {
                    name: r_name,
                    variants: r_variants,
                },
            ) => {
                l_name.to_string() == r_name.to_string()
                    && l_variants
                        .iter()
                        .map(|v| v.to_string())
                        .eq(r_variants.iter().map(|v| v.to_string()))
            }
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl ColumnType {
    pub fn custom<T>(ty: T) -> ColumnType
    where
        T: Into<String>,
    {
        ColumnType::Custom(Alias::new(ty).into_iden())
    }
}

/// All column specification keywords
#[derive(Debug, Clone)]
pub enum ColumnSpec {
    Null,
    NotNull,
    Default(SimpleExpr),
    AutoIncrement,
    UniqueKey,
    PrimaryKey,
    Check(SimpleExpr),
    Generated { expr: SimpleExpr, stored: bool },
    Extra(String),
    Comment(String),
}

// All interval fields
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PgInterval {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    YearToMonth,
    DayToHour,
    DayToMinute,
    DayToSecond,
    HourToMinute,
    HourToSecond,
    MinuteToSecond,
}

// All MySQL year type field length sizes
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MySqlYear {
    Two,
    Four,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlobSize {
    Tiny,
    /// MySQL & SQLite support `binary(length)` column type
    Blob(Option<u32>),
    Medium,
    Long,
}

impl ColumnDef {
    /// Construct a table column
    pub fn new<T>(name: T) -> Self
    where
        T: IntoIden,
    {
        Self {
            table: None,
            name: name.into_iden(),
            types: None,
            spec: Vec::new(),
        }
    }

    /// Construct a table column with column type
    pub fn new_with_type<T>(name: T, types: ColumnType) -> Self
    where
        T: IntoIden,
    {
        Self {
            table: None,
            name: name.into_iden(),
            types: Some(types),
            spec: Vec::new(),
        }
    }

    /// Set column not null
    pub fn not_null(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::NotNull);
        self
    }

    /// Set column null
    pub fn null(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::Null);
        self
    }

    /// Set default expression of a column
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::create()
    ///     .table(Char::Table)
    ///     .col(ColumnDef::new(Char::FontId).integer().default(12i32))
    ///     .col(
    ///         ColumnDef::new(Char::CreatedAt)
    ///             .timestamp()
    ///             .default(Expr::current_timestamp())
    ///             .not_null(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     [
    ///         "CREATE TABLE `character` (",
    ///         "`font_id` int DEFAULT 12,",
    ///         "`created_at` timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL",
    ///         ")",
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"CREATE TABLE "character" ("#,
    ///         r#""font_id" integer DEFAULT 12,"#,
    ///         r#""created_at" timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn default<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        self.spec.push(ColumnSpec::Default(value.into()));
        self
    }

    /// Set column auto increment
    pub fn auto_increment(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::AutoIncrement);
        self
    }

    /// Set column unique constraint
    pub fn unique_key(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::UniqueKey);
        self
    }

    /// Set column as primary key
    pub fn primary_key(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::PrimaryKey);
        self
    }

    /// Set column type as char with custom length
    pub fn char_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::Char(Some(length)));
        self
    }

    /// Set column type as char
    pub fn char(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Char(None));
        self
    }

    /// Set column type as string with custom length
    pub fn string_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::String(Some(length)));
        self
    }

    /// Set column type as string
    pub fn string(&mut self) -> &mut Self {
        self.types = Some(ColumnType::String(None));
        self
    }

    /// Set column type as text
    pub fn text(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Text);
        self
    }

    /// Set column type as tiny_integer
    pub fn tiny_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TinyInteger);
        self
    }

    /// Set column type as small_integer
    pub fn small_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::SmallInteger);
        self
    }

    /// Set column type as integer
    pub fn integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Integer);
        self
    }

    /// Set column type as big_integer
    pub fn big_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::BigInteger);
        self
    }

    /// Set column type as tiny_unsigned
    pub fn tiny_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TinyUnsigned);
        self
    }

    /// Set column type as small_unsigned
    pub fn small_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::SmallUnsigned);
        self
    }

    /// Set column type as unsigned
    pub fn unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Unsigned);
        self
    }

    /// Set column type as big_unsigned
    pub fn big_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::BigUnsigned);
        self
    }

    /// Set column type as float
    pub fn float(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Float);
        self
    }

    /// Set column type as double
    pub fn double(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Double);
        self
    }

    /// Set column type as decimal with custom precision and scale
    pub fn decimal_len(&mut self, precision: u32, scale: u32) -> &mut Self {
        self.types = Some(ColumnType::Decimal(Some((precision, scale))));
        self
    }

    /// Set column type as decimal
    pub fn decimal(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Decimal(None));
        self
    }

    /// Set column type as date_time
    pub fn date_time(&mut self) -> &mut Self {
        self.types = Some(ColumnType::DateTime);
        self
    }

    /// Set column type as interval type with optional fields and precision. Postgres only
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// assert_eq!(
    ///     Table::create()
    ///         .table(Glyph::Table)
    ///         .col(
    ///             ColumnDef::new(Alias::new("I1"))
    ///                 .interval(None, None)
    ///                 .not_null()
    ///         )
    ///         .col(
    ///             ColumnDef::new(Alias::new("I2"))
    ///                 .interval(Some(PgInterval::YearToMonth), None)
    ///                 .not_null()
    ///         )
    ///         .col(
    ///             ColumnDef::new(Alias::new("I3"))
    ///                 .interval(None, Some(42))
    ///                 .not_null()
    ///         )
    ///         .col(
    ///             ColumnDef::new(Alias::new("I4"))
    ///                 .interval(Some(PgInterval::Hour), Some(43))
    ///                 .not_null()
    ///         )
    ///         .to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"CREATE TABLE "glyph" ("#,
    ///         r#""I1" interval NOT NULL,"#,
    ///         r#""I2" interval YEAR TO MONTH NOT NULL,"#,
    ///         r#""I3" interval(42) NOT NULL,"#,
    ///         r#""I4" interval HOUR(43) NOT NULL"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    pub fn interval(&mut self, fields: Option<PgInterval>, precision: Option<u32>) -> &mut Self {
        self.types = Some(ColumnType::Interval(fields, precision));
        self
    }

    /// Set column type as timestamp
    pub fn timestamp(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Timestamp);
        self
    }

    /// Set column type as timestamp with time zone. Postgres only
    pub fn timestamp_with_time_zone(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TimestampWithTimeZone);
        self
    }

    /// Set column type as time
    pub fn time(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Time);
        self
    }

    /// Set column type as date
    pub fn date(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Date);
        self
    }

    /// Set column type as year
    /// Only MySQL supports year
    pub fn year(&mut self, length: Option<MySqlYear>) -> &mut Self {
        self.types = Some(ColumnType::Year(length));
        self
    }

    /// Set column type as binary with custom length
    pub fn binary_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::Binary(BlobSize::Blob(Some(length))));
        self
    }

    /// Set column type as binary
    pub fn binary(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Binary(BlobSize::Blob(None)));
        self
    }

    /// Set column type as blob, but when given BlobSize::Blob(size) argument, this column map to binary(size) type instead.
    pub fn blob(&mut self, size: BlobSize) -> &mut Self {
        self.types = Some(ColumnType::Binary(size));
        self
    }

    /// Set column type as binary with variable length
    pub fn var_binary(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::VarBinary(length));
        self
    }

    /// Set column type as bit with variable length
    pub fn bit(&mut self, length: Option<u32>) -> &mut Self {
        self.types = Some(ColumnType::Bit(length));
        self
    }

    /// Set column type as varbit with variable length
    pub fn varbit(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::VarBit(length));
        self
    }

    /// Set column type as boolean
    pub fn boolean(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Boolean);
        self
    }

    /// Set column type as money with custom precision and scale
    pub fn money_len(&mut self, precision: u32, scale: u32) -> &mut Self {
        self.types = Some(ColumnType::Money(Some((precision, scale))));
        self
    }

    /// Set column type as money
    pub fn money(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Money(None));
        self
    }

    /// Set column type as json.
    pub fn json(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Json);
        self
    }

    /// Set column type as json binary.
    pub fn json_binary(&mut self) -> &mut Self {
        self.types = Some(ColumnType::JsonBinary);
        self
    }

    /// Set column type as uuid
    pub fn uuid(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Uuid);
        self
    }

    /// Use a custom type on this column.
    pub fn custom<T>(&mut self, name: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.types = Some(ColumnType::Custom(name.into_iden()));
        self
    }

    /// Set column type as enum.
    pub fn enumeration<N, S, V>(&mut self, name: N, variants: V) -> &mut Self
    where
        N: IntoIden,
        S: IntoIden,
        V: IntoIterator<Item = S>,
    {
        self.types = Some(ColumnType::Enum {
            name: name.into_iden(),
            variants: variants.into_iter().map(IntoIden::into_iden).collect(),
        });
        self
    }

    /// Set column type as an array with a specified element type.
    /// This is only supported on Postgres.
    pub fn array(&mut self, elem_type: ColumnType) -> &mut Self {
        self.types = Some(ColumnType::Array(RcOrArc::new(elem_type)));
        self
    }

    /// Set columnt type as cidr.
    /// This is only supported on Postgres.
    pub fn cidr(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Cidr);
        self
    }

    /// Set columnt type as inet.
    /// This is only supported on Postgres.
    pub fn inet(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Inet);
        self
    }

    /// Set columnt type as macaddr.
    /// This is only supported on Postgres.
    pub fn mac_address(&mut self) -> &mut Self {
        self.types = Some(ColumnType::MacAddr);
        self
    }

    /// Set column type as `ltree`
    /// This is only supported on Postgres.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// assert_eq!(
    ///     Table::create()
    ///         .table(Glyph::Table)
    ///         .col(
    ///             ColumnDef::new(Glyph::Id)
    ///                 .integer()
    ///                 .not_null()
    ///                 .auto_increment()
    ///                 .primary_key()
    ///         )
    ///         .col(ColumnDef::new(Glyph::Tokens).ltree())
    ///         .to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"CREATE TABLE "glyph" ("#,
    ///         r#""id" serial NOT NULL PRIMARY KEY,"#,
    ///         r#""tokens" ltree"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn ltree(&mut self) -> &mut Self {
        self.types = Some(ColumnType::LTree);
        self
    }

    /// Set constraints as SimpleExpr
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// assert_eq!(
    ///     Table::create()
    ///         .table(Glyph::Table)
    ///         .col(
    ///             ColumnDef::new(Glyph::Id)
    ///                 .integer()
    ///                 .not_null()
    ///                 .check(Expr::col(Glyph::Id).gt(10))
    ///         )
    ///         .to_string(MysqlQueryBuilder),
    ///     r#"CREATE TABLE `glyph` ( `id` int NOT NULL CHECK (`id` > 10) )"#,
    /// );
    /// ```
    pub fn check<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        self.spec.push(ColumnSpec::Check(value.into()));
        self
    }

    /// Sets the column as generated with SimpleExpr
    pub fn generated<T>(&mut self, expr: T, stored: bool) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        self.spec.push(ColumnSpec::Generated {
            expr: expr.into(),
            stored,
        });
        self
    }

    /// Some extra options in custom string
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// let table = Table::create()
    ///     .table(Char::Table)
    ///     .col(
    ///         ColumnDef::new(Char::Id)
    ///             .uuid()
    ///             .extra("DEFAULT gen_random_uuid()")
    ///             .primary_key()
    ///             .not_null(),
    ///     )
    ///     .col(
    ///         ColumnDef::new(Char::CreatedAt)
    ///             .timestamp_with_time_zone()
    ///             .extra("DEFAULT NOW()")
    ///             .not_null(),
    ///     )
    ///     .to_owned();
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"CREATE TABLE "character" ("#,
    ///         r#""id" uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,"#,
    ///         r#""created_at" timestamp with time zone DEFAULT NOW() NOT NULL"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn extra<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.spec.push(ColumnSpec::Extra(string.into()));
        self
    }

    /// MySQL only.
    pub fn comment<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.spec.push(ColumnSpec::Comment(string.into()));
        self
    }

    pub fn get_column_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_column_type(&self) -> Option<&ColumnType> {
        self.types.as_ref()
    }

    pub fn get_column_spec(&self) -> &Vec<ColumnSpec> {
        self.spec.as_ref()
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            name: std::mem::replace(&mut self.name, SeaRc::new(NullAlias::new())),
            types: self.types.take(),
            spec: std::mem::take(&mut self.spec),
        }
    }
}
