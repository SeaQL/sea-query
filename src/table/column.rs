use crate::{types::*, value::*};

/// Specification of a table column
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub(crate) table: Option<DynIden>,
    pub(crate) name: DynIden,
    pub(crate) types: Option<ColumnType>,
    pub(crate) spec: Vec<ColumnSpec>,
}

/// All column types
#[derive(Debug, Clone)]
pub enum ColumnType {
    Char(Option<u32>),
    String(Option<u32>),
    Text,
    TinyInteger(Option<u32>),
    SmallInteger(Option<u32>),
    Integer(Option<u32>),
    BigInteger(Option<u32>),
    Float(Option<u32>),
    Double(Option<u32>),
    Decimal(Option<(u32, u32)>),
    DateTime(Option<u32>),
    Timestamp(Option<u32>),
    Time(Option<u32>),
    Date,
    Binary(Option<u32>),
    Boolean,
    Money(Option<(u32, u32)>),
    Json,
    JsonBinary,
    Custom(DynIden),
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

    /// Set column not null
    pub fn not_null(mut self) -> Self {
        self.spec.push(ColumnSpec::NotNull);
        self
    }

    /// Set default value of a column
    pub fn default<T>(mut self, value: T) -> Self
    where
        T: Into<Value>,
    {
        self.spec.push(ColumnSpec::Default(value.into()));
        self
    }

    /// Set column auto increment
    pub fn auto_increment(mut self) -> Self {
        self.spec.push(ColumnSpec::AutoIncrement);
        self
    }

    /// Set column unique constraint
    pub fn unique_key(mut self) -> Self {
        self.spec.push(ColumnSpec::UniqueKey);
        self
    }

    /// Set column as primary key
    pub fn primary_key(mut self) -> Self {
        self.spec.push(ColumnSpec::PrimaryKey);
        self
    }

    /// Set column type as char with custom length
    pub fn char_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::Char(Some(length)));
        self
    }

    /// Set column type as char
    pub fn char(mut self) -> Self {
        self.types = Some(ColumnType::Char(None));
        self
    }

    /// Set column type as string with custom length
    pub fn string_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::String(Some(length)));
        self
    }

    /// Set column type as string
    pub fn string(mut self) -> Self {
        self.types = Some(ColumnType::String(None));
        self
    }

    /// Set column type as text
    pub fn text(mut self) -> Self {
        self.types = Some(ColumnType::Text);
        self
    }

    /// Set column type as tiny_integer with custom length
    pub fn tiny_integer_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::TinyInteger(Some(length)));
        self
    }

    /// Set column type as tiny_integer
    pub fn tiny_integer(mut self) -> Self {
        self.types = Some(ColumnType::TinyInteger(None));
        self
    }

    /// Set column type as small_integer with custom length
    pub fn small_integer_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::SmallInteger(Some(length)));
        self
    }

    /// Set column type as small_integer
    pub fn small_integer(mut self) -> Self {
        self.types = Some(ColumnType::SmallInteger(None));
        self
    }

    /// Set column type as integer with custom length
    pub fn integer_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::Integer(Some(length)));
        self
    }

    /// Set column type as integer
    pub fn integer(mut self) -> Self {
        self.types = Some(ColumnType::Integer(None));
        self
    }

    /// Set column type as big_integer with custom length
    pub fn big_integer_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::BigInteger(Some(length)));
        self
    }

    /// Set column type as big_integer
    pub fn big_integer(mut self) -> Self {
        self.types = Some(ColumnType::BigInteger(None));
        self
    }

    /// Set column type as float with custom precision
    pub fn float_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::Float(Some(precision)));
        self
    }

    /// Set column type as float
    pub fn float(mut self) -> Self {
        self.types = Some(ColumnType::Float(None));
        self
    }

    /// Set column type as double with custom precision
    pub fn double_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::Double(Some(precision)));
        self
    }

    /// Set column type as double
    pub fn double(mut self) -> Self {
        self.types = Some(ColumnType::Double(None));
        self
    }

    /// Set column type as decimal with custom precision and scale
    pub fn decimal_len(mut self, precision: u32, scale: u32) -> Self {
        self.types = Some(ColumnType::Decimal(Some((precision, scale))));
        self
    }

    /// Set column type as decimal
    pub fn decimal(mut self) -> Self {
        self.types = Some(ColumnType::Decimal(None));
        self
    }

    /// Set column type as date_time with custom precision
    pub fn date_time_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::DateTime(Some(precision)));
        self
    }

    /// Set column type as date_time
    pub fn date_time(mut self) -> Self {
        self.types = Some(ColumnType::DateTime(None));
        self
    }

    /// Set column type as timestamp with custom precision
    pub fn timestamp_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::Timestamp(Some(precision)));
        self
    }

    /// Set column type as timestamp
    pub fn timestamp(mut self) -> Self {
        self.types = Some(ColumnType::Timestamp(None));
        self
    }

    /// Set column type as time with custom precision
    pub fn time_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::Time(Some(precision)));
        self
    }

    /// Set column type as time
    pub fn time(mut self) -> Self {
        self.types = Some(ColumnType::Time(None));
        self
    }

    /// Set column type as date
    pub fn date(mut self) -> Self {
        self.types = Some(ColumnType::Date);
        self
    }

    /// Set column type as binary with custom length
    pub fn binary_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::Binary(Some(length)));
        self
    }

    /// Set column type as binary
    pub fn binary(mut self) -> Self {
        self.types = Some(ColumnType::Binary(None));
        self
    }

    /// Set column type as boolean
    pub fn boolean(mut self) -> Self {
        self.types = Some(ColumnType::Boolean);
        self
    }

    /// Set column type as money with custom precision ans scale
    pub fn money_len(mut self, precision: u32, scale: u32) -> Self {
        self.types = Some(ColumnType::Money(Some((precision, scale))));
        self
    }

    /// Set column type as money
    pub fn money(mut self) -> Self {
        self.types = Some(ColumnType::Money(None));
        self
    }

    /// Set column type as json.
    /// On MySQL, this is equivalent to json_binary. On MariaDB, this is equivalent to text.
    /// On PgSQL, this is equivalent to json.
    pub fn json(mut self) -> Self {
        self.types = Some(ColumnType::Json);
        self
    }

    /// Set column type as json binary.
    /// On MySQL, this is equivalent to json. On MariaDB, this is equivalent to text.
    /// On PgSQL, this is equivalent to jsonb.
    pub fn json_binary(mut self) -> Self {
        self.types = Some(ColumnType::JsonBinary);
        self
    }

    /// Use a custom type on this column.
    pub fn custom<T: 'static>(mut self, n: T) -> Self
    where
        T: Iden,
    {
        self.types = Some(ColumnType::Custom(SeaRc::new(n)));
        self
    }

    /// Some extra options in custom string
    pub fn extra(mut self, string: String) -> Self {
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
}
