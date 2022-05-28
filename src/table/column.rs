use crate::{types::*, value::*};

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
    TinyInteger(Option<u32>),
    SmallInteger(Option<u32>),
    Integer(Option<u32>),
    BigInteger(Option<u32>),
    TinyUnsigned(Option<u32>),
    SmallUnsigned(Option<u32>),
    Unsigned(Option<u32>),
    BigUnsigned(Option<u32>),
    Float(Option<u32>),
    Double(Option<u32>),
    Decimal(Option<(u32, u32)>),
    DateTime(Option<u32>),
    Timestamp(Option<u32>),
    TimestampWithTimeZone(Option<u32>),
    Time(Option<u32>),
    Date,
    Interval(Option<PgInterval>, Option<u32>),
    Binary(BlobSize),
    VarBinary(u32),
    Boolean,
    Money(Option<(u32, u32)>),
    Json,
    JsonBinary,
    Uuid,
    Custom(DynIden),
    Enum(String, Vec<String>),
    Array(Option<String>),
}

/// All column specification keywords
#[derive(Debug, Clone)]
pub enum ColumnSpec {
    Null,
    NotNull,
    Default(Value),
    AutoIncrement,
    UniqueKey,
    PrimaryKey,
    Extra(String),
}

// All interval fields
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum BlobSize {
    Tiny,
    /// MySQL & SQLite support `binary(length)` column type
    Blob(Option<u32>),
    Medium,
    Long,
}

#[cfg(feature = "postgres-interval")]
impl quote::ToTokens for PgInterval {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::{quote, TokenStreamExt};
        tokens.append_all(match self {
            PgInterval::Year => quote! { PgInterval::Year },
            PgInterval::Month => quote! { PgInterval::Month },
            PgInterval::Day => quote! { PgInterval::Day },
            PgInterval::Hour => quote! { PgInterval::Hour },
            PgInterval::Minute => quote! { PgInterval::Minute },
            PgInterval::Second => quote! { PgInterval::Second },
            PgInterval::YearToMonth => quote! { PgInterval::YearToMonth },
            PgInterval::DayToHour => quote! { PgInterval::DayToHour },
            PgInterval::DayToMinute => quote! { PgInterval::DayToMinute },
            PgInterval::DayToSecond => quote! { PgInterval::DayToSecond },
            PgInterval::HourToMinute => quote! { PgInterval::HourToMinute },
            PgInterval::HourToSecond => quote! { PgInterval::HourToSecond },
            PgInterval::MinuteToSecond => quote! { PgInterval::MinuteToSecond },
        });
    }
}

impl ColumnDef {
    /// Construct a table column
    pub fn new<T: 'static>(name: T) -> Self
    where
        T: Iden,
    {
        Self {
            table: None,
            name: SeaRc::new(name),
            types: None,
            spec: Vec::new(),
        }
    }

    /// Construct a table column with column type
    pub fn new_with_type<T: 'static>(name: T, types: ColumnType) -> Self
    where
        T: Iden,
    {
        Self {
            table: None,
            name: SeaRc::new(name),
            types: Some(types),
            spec: Vec::new(),
        }
    }

    /// Set column not null
    pub fn not_null(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::NotNull);
        self
    }

    /// Set default value of a column
    pub fn default<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Value>,
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

    /// Set column type as tiny_integer with custom length
    pub fn tiny_integer_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::TinyInteger(Some(length)));
        self
    }

    /// Set column type as tiny_integer
    pub fn tiny_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TinyInteger(None));
        self
    }

    /// Set column type as small_integer with custom length
    pub fn small_integer_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::SmallInteger(Some(length)));
        self
    }

    /// Set column type as small_integer
    pub fn small_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::SmallInteger(None));
        self
    }

    /// Set column type as integer with custom length
    pub fn integer_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::Integer(Some(length)));
        self
    }

    /// Set column type as integer
    pub fn integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Integer(None));
        self
    }

    /// Set column type as big_integer with custom length
    pub fn big_integer_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::BigInteger(Some(length)));
        self
    }

    /// Set column type as big_integer
    pub fn big_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::BigInteger(None));
        self
    }

    /// Set column type as tiny_unsigned with custom length
    pub fn tiny_unsigned_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::TinyUnsigned(Some(length)));
        self
    }

    /// Set column type as tiny_unsigned
    pub fn tiny_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TinyUnsigned(None));
        self
    }

    /// Set column type as small_unsigned with custom length
    pub fn small_unsigned_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::SmallUnsigned(Some(length)));
        self
    }

    /// Set column type as small_unsigned
    pub fn small_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::SmallUnsigned(None));
        self
    }

    /// Set column type as unsigned with custom length
    pub fn unsigned_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::Unsigned(Some(length)));
        self
    }

    /// Set column type as unsigned
    pub fn unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Unsigned(None));
        self
    }

    /// Set column type as big_unsigned with custom length
    pub fn big_unsigned_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::BigUnsigned(Some(length)));
        self
    }

    /// Set column type as big_unsigned
    pub fn big_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::BigUnsigned(None));
        self
    }

    /// Set column type as float with custom precision
    pub fn float_len(&mut self, precision: u32) -> &mut Self {
        self.types = Some(ColumnType::Float(Some(precision)));
        self
    }

    /// Set column type as float
    pub fn float(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Float(None));
        self
    }

    /// Set column type as double with custom precision
    pub fn double_len(&mut self, precision: u32) -> &mut Self {
        self.types = Some(ColumnType::Double(Some(precision)));
        self
    }

    /// Set column type as double
    pub fn double(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Double(None));
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

    /// Set column type as date_time with custom precision
    pub fn date_time_len(&mut self, precision: u32) -> &mut Self {
        self.types = Some(ColumnType::DateTime(Some(precision)));
        self
    }

    /// Set column type as date_time
    pub fn date_time(&mut self) -> &mut Self {
        self.types = Some(ColumnType::DateTime(None));
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
    ///     vec![
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

    /// Set column type as timestamp with custom precision
    pub fn timestamp_len(&mut self, precision: u32) -> &mut Self {
        self.types = Some(ColumnType::Timestamp(Some(precision)));
        self
    }

    /// Set column type as timestamp
    pub fn timestamp(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Timestamp(None));
        self
    }

    /// Set column type as timestamp with time zone. Postgres only
    pub fn timestamp_with_time_zone(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TimestampWithTimeZone(None));
        self
    }

    /// Set column type as timestamp with time zone plus custom precision
    pub fn timestamp_with_time_zone_len(&mut self, precision: u32) -> &mut Self {
        self.types = Some(ColumnType::TimestampWithTimeZone(Some(precision)));
        self
    }

    /// Set column type as time with custom precision
    pub fn time_len(&mut self, precision: u32) -> &mut Self {
        self.types = Some(ColumnType::Time(Some(precision)));
        self
    }

    /// Set column type as time
    pub fn time(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Time(None));
        self
    }

    /// Set column type as date
    pub fn date(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Date);
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
    /// On MySQL, this is equivalent to `json_binary`. On MariaDB, this is equivalent to `text`.
    /// On PgSQL, this is equivalent to `json`.
    pub fn json(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Json);
        self
    }

    /// Set column type as json binary.
    /// On MySQL, this is equivalent to `json`. On MariaDB, this is equivalent to `text`.
    /// On PgSQL, this is equivalent to `jsonb`.
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
    pub fn custom<T: 'static>(&mut self, n: T) -> &mut Self
    where
        T: Iden,
    {
        self.types = Some(ColumnType::Custom(SeaRc::new(n)));
        self
    }

    /// Set column type as enum.
    pub fn enumeration<N, S, V>(&mut self, name: N, variants: V) -> &mut Self
    where
        N: ToString,
        S: ToString,
        V: IntoIterator<Item = S>,
    {
        self.types = Some(ColumnType::Enum(
            name.to_string(),
            variants.into_iter().map(|v| v.to_string()).collect(),
        ));
        self
    }

    /// Set column type as an array with a specified element type.
    /// This is only supported on Postgres.
    pub fn array(&mut self, elem_type: String) -> &mut Self {
        self.types = Some(ColumnType::Array(Some(elem_type)));
        self
    }

    /// Some extra options in custom string
    pub fn extra(&mut self, string: String) -> &mut Self {
        self.spec.push(ColumnSpec::Extra(string));
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
