use std::rc::Rc;
use crate::{types::*, value::*};

/// Specification of a table column
#[derive(Clone)]
pub struct ColumnDef {
    pub(crate) table: Option<Rc<dyn Iden>>,
    pub(crate) name: Rc<dyn Iden>,
    pub(crate) types: Option<ColumnType>,
    pub(crate) spec: Vec<ColumnSpec>,
}

impl ColumnDef {
    /// Construct a table column
    pub fn new<T: 'static>(name: T) -> Self
        where T: Iden{
        Self {
            table: None,
            name: Rc::new(name),
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
        where T: Into<Value> {
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
        self.types = Some(ColumnType::Char(length));
        self
    }

    /// Set column type as char
    pub fn char(mut self) -> Self {
        self.types = Some(ColumnType::CharDefault);
        self
    }

    /// Set column type as string with custom length
    pub fn string_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::String(length));
        self
    }

    /// Set column type as string
    pub fn string(mut self) -> Self {
        self.types = Some(ColumnType::StringDefault);
        self
    }

    /// Set column type as text
    pub fn text(mut self) -> Self {
        self.types = Some(ColumnType::Text);
        self
    }

    /// Set column type as tiny_integer with custom length
    pub fn tiny_integer_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::TinyInteger(length));
        self
    }

    /// Set column type as tiny_integer
    pub fn tiny_integer(mut self) -> Self {
        self.types = Some(ColumnType::TinyIntegerDefault);
        self
    }

    /// Set column type as small_integer with custom length
    pub fn small_integer_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::SmallInteger(length));
        self
    }

    /// Set column type as small_integer
    pub fn small_integer(mut self) -> Self {
        self.types = Some(ColumnType::SmallIntegerDefault);
        self
    }

    /// Set column type as integer with custom length
    pub fn integer_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::Integer(length));
        self
    }

    /// Set column type as integer
    pub fn integer(mut self) -> Self {
        self.types = Some(ColumnType::IntegerDefault);
        self
    }

    /// Set column type as big_integer with custom length
    pub fn big_integer_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::BigInteger(length));
        self
    }

    /// Set column type as big_integer
    pub fn big_integer(mut self) -> Self {
        self.types = Some(ColumnType::BigIntegerDefault);
        self
    }

    /// Set column type as float with custom precision
    pub fn float_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::Float(precision));
        self
    }

    /// Set column type as float
    pub fn float(mut self) -> Self {
        self.types = Some(ColumnType::FloatDefault);
        self
    }

    /// Set column type as double with custom precision
    pub fn double_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::Double(precision));
        self
    }

    /// Set column type as double
    pub fn double(mut self) -> Self {
        self.types = Some(ColumnType::DoubleDefault);
        self
    }

    /// Set column type as decimal with custom precision and scale
    pub fn decimal_len(mut self, precision: u32, scale: u32) -> Self {
        self.types = Some(ColumnType::Decimal(precision, scale));
        self
    }

    /// Set column type as decimal
    pub fn decimal(mut self) -> Self {
        self.types = Some(ColumnType::DecimalDefault);
        self
    }

    /// Set column type as date_time with custom precision
    pub fn date_time_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::DateTime(precision));
        self
    }

    /// Set column type as date_time
    pub fn date_time(mut self) -> Self {
        self.types = Some(ColumnType::DateTimeDefault);
        self
    }

    /// Set column type as timestamp with custom precision
    pub fn timestamp_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::Timestamp(precision));
        self
    }

    /// Set column type as timestamp
    pub fn timestamp(mut self) -> Self {
        self.types = Some(ColumnType::TimestampDefault);
        self
    }

    /// Set column type as time with custom precision
    pub fn time_len(mut self, precision: u32) -> Self {
        self.types = Some(ColumnType::Time(precision));
        self
    }

    /// Set column type as time
    pub fn time(mut self) -> Self {
        self.types = Some(ColumnType::TimeDefault);
        self
    }

    /// Set column type as date
    pub fn date(mut self) -> Self {
        self.types = Some(ColumnType::Date);
        self
    }

    /// Set column type as binary with custom length
    pub fn binary_len(mut self, length: u32) -> Self {
        self.types = Some(ColumnType::Binary(length));
        self
    }

    /// Set column type as binary
    pub fn binary(mut self) -> Self {
        self.types = Some(ColumnType::BinaryDefault);
        self
    }

    /// Set column type as boolean
    pub fn boolean(mut self) -> Self {
        self.types = Some(ColumnType::Boolean);
        self
    }

    /// Set column type as money with custom precision ans scale
    pub fn money_len(mut self, precision: u32, scale: u32) -> Self {
        self.types = Some(ColumnType::Money(precision, scale));
        self
    }

    /// Set column type as money
    pub fn money(mut self) -> Self {
        self.types = Some(ColumnType::MoneyDefault);
        self
    }

    /// Set column type as json
    pub fn json(mut self) -> Self {
        self.types = Some(ColumnType::Json);
        self
    }

    pub fn custom<T: 'static>(mut self, n: T) -> Self
        where T: Iden {
        self.types = Some(ColumnType::Custom(Rc::new(n)));
        self
    }
}

/// All available column types
#[derive(Clone)]
pub enum ColumnType {
    Char(u32),
    CharDefault,
    String(u32),
    StringDefault,
    Text,
    TinyInteger(u32),
    TinyIntegerDefault,
    SmallInteger(u32),
    SmallIntegerDefault,
    Integer(u32),
    IntegerDefault,
    BigInteger(u32),
    BigIntegerDefault,
    Float(u32),
    FloatDefault,
    Double(u32),
    DoubleDefault,
    Decimal(u32, u32),
    DecimalDefault,
    DateTime(u32),
    DateTimeDefault,
    Timestamp(u32),
    TimestampDefault,
    Time(u32),
    TimeDefault,
    Date,
    Binary(u32),
    BinaryDefault,
    Boolean,
    Money(u32, u32),
    MoneyDefault,
    Json,
    Custom(Rc<dyn Iden>),
}

/// All available column specification keywords
#[derive(Clone)]
pub enum ColumnSpec {
    Null,
    NotNull,
    Default(Value),
    AutoIncrement,
    UniqueKey,
    PrimaryKey,
}