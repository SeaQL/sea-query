use super::ArrayElement;
use super::*;
use crate::RcOrArc;

// We only implement conversion from Vec<T> to Array when T is not u8.
// This is because for u8's case, there is already conversion to Byte defined above.
// TODO When negative trait becomes a stable feature, following code can be much shorter.
pub trait NotU8 {}

impl NotU8 for bool {}
impl NotU8 for i8 {}
impl NotU8 for i16 {}
impl NotU8 for i32 {}
impl NotU8 for i64 {}
impl NotU8 for u16 {}
impl NotU8 for u32 {}
impl NotU8 for u64 {}
impl NotU8 for f32 {}
impl NotU8 for f64 {}
impl NotU8 for char {}
impl NotU8 for String {}
impl NotU8 for Vec<u8> {}

impl<T: NotU8> NotU8 for Option<T> {}

#[cfg(feature = "with-json")]
impl NotU8 for Json {}

#[cfg(feature = "with-chrono")]
impl NotU8 for NaiveDate {}

#[cfg(feature = "with-chrono")]
impl NotU8 for NaiveTime {}

#[cfg(feature = "with-chrono")]
impl NotU8 for NaiveDateTime {}

#[cfg(feature = "with-chrono")]
impl<Tz> NotU8 for chrono::DateTime<Tz> where Tz: chrono::TimeZone {}

#[cfg(feature = "with-time")]
impl NotU8 for time::Date {}

#[cfg(feature = "with-time")]
impl NotU8 for time::Time {}

#[cfg(feature = "with-time")]
impl NotU8 for PrimitiveDateTime {}

#[cfg(feature = "with-time")]
impl NotU8 for OffsetDateTime {}

#[cfg(feature = "with-jiff")]
impl NotU8 for jiff::civil::Date {}

#[cfg(feature = "with-jiff")]
impl NotU8 for jiff::civil::Time {}

#[cfg(feature = "with-jiff")]
impl NotU8 for jiff::civil::DateTime {}

#[cfg(feature = "with-jiff")]
impl NotU8 for jiff::Timestamp {}

#[cfg(feature = "with-jiff")]
impl NotU8 for jiff::Zoned {}

#[cfg(feature = "with-rust_decimal")]
impl NotU8 for rust_decimal::Decimal {}

#[cfg(feature = "with-bigdecimal")]
impl NotU8 for bigdecimal::BigDecimal {}

#[cfg(feature = "with-uuid")]
impl NotU8 for Uuid {}

#[cfg(feature = "with-uuid")]
impl NotU8 for uuid::fmt::Braced {}

#[cfg(feature = "with-uuid")]
impl NotU8 for uuid::fmt::Hyphenated {}

#[cfg(feature = "with-uuid")]
impl NotU8 for uuid::fmt::Simple {}

#[cfg(feature = "with-uuid")]
impl NotU8 for uuid::fmt::Urn {}

#[cfg(feature = "with-ipnetwork")]
impl NotU8 for IpNetwork {}

#[cfg(feature = "with-mac_address")]
impl NotU8 for MacAddress {}

macro_rules! impl_value_vec {
    ($($ty:ty => $vari:ident)*) => {
        $(
            impl ArrayValue for $ty {
                fn into_array(iter: impl IntoIterator<Item = Option<Self>>) -> Array {
                    let boxed = Box::from_iter(iter);
                    Array::$vari(boxed)
                }
            }

            impl ArrayElement for $ty
            {
                type ArrayValueType = $ty;

                fn into_array_value(self) -> Self::ArrayValueType {
                    self
                }

                fn try_from_value(v: Value) -> Result<Vec<Option<Self>>, ValueTypeErr> {
                    match v {
                        Value::Array(Array::$vari(inner)) => Ok(inner.into_vec()),
                        _ => Err(ValueTypeErr)
                    }
                }
            }
       )*
    }
}

impl_value_vec! {
    bool => Bool
    i8 => TinyInt
    i16 => SmallInt
    i32 => Int
    i64 => BigInt
    u16 => SmallUnsigned
    u32 => Unsigned
    u64 => BigUnsigned
    f32 => Float
    f64 => Double
    std::string::String => String
    char => Char
    Vec<u8> => Bytes
}

// Impls for u8
// because Vec<u8> is already defined as Bytes
impl From<Vec<u8>> for Array {
    fn from(x: Vec<u8>) -> Array {
        let values: Box<[Option<_>]> = x.into_iter().map(Some).collect();

        Array::TinyUnsigned(values)
    }
}
impl From<Box<[u8]>> for Array {
    fn from(x: Box<[u8]>) -> Array {
        let values: Box<[Option<_>]> = x.into_iter().map(Some).collect();

        Array::TinyUnsigned(values)
    }
}

impl From<Vec<Option<u8>>> for Array {
    fn from(x: Vec<Option<u8>>) -> Array {
        Array::TinyUnsigned(x.into_boxed_slice())
    }
}
impl From<Box<[Option<u8>]>> for Array {
    fn from(x: Box<[Option<u8>]>) -> Array {
        Array::TinyUnsigned(x)
    }
}

impl From<Vec<Option<u8>>> for Value {
    fn from(x: Vec<Option<u8>>) -> Value {
        Value::Array(Array::TinyUnsigned(x.into_boxed_slice()))
    }
}

impl ValueType for Vec<Option<u8>> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Array(Array::TinyUnsigned(inner)) => Ok(inner.into_vec()),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Vec<u8>).to_owned()
    }

    fn array_type() -> ArrayType {
        <u8>::array_type()
    }

    fn column_type() -> ColumnType {
        use ColumnType::*;
        Array(RcOrArc::new(<u8>::column_type()))
    }
}

#[cfg(feature = "with-json")]
impl_value_vec! {
    serde_json::Value => Json
}

impl From<(Arc<str>, Vec<Option<Arc<Enum>>>)> for Value {
    fn from(x: (Arc<str>, Vec<Option<Arc<Enum>>>)) -> Value {
        Value::Array(Array::Enum(Box::new((x.0, x.1.into_boxed_slice()))))
    }
}

#[cfg(feature = "with-chrono")]
impl_value_vec! {
    NaiveDate => ChronoDate
    NaiveTime => ChronoTime
    NaiveDateTime => ChronoDateTime
    chrono::DateTime<chrono::Utc> => ChronoDateTimeUtc
    chrono::DateTime<chrono::Local> => ChronoDateTimeLocal
    chrono::DateTime<chrono::FixedOffset> => ChronoDateTimeWithTimeZone
}

#[cfg(feature = "with-time")]
impl_value_vec! {
    time::Date => TimeDate
    time::Time => TimeTime
    PrimitiveDateTime => TimeDateTime
    OffsetDateTime => TimeDateTimeWithTimeZone
}

#[cfg(feature = "with-jiff")]
impl_value_vec! {
    jiff::civil::Date => JiffDate
    jiff::civil::Time => JiffTime
    jiff::civil::DateTime => JiffDateTime
    jiff::Timestamp => JiffTimestamp
    jiff::Zoned => JiffZoned
}

#[cfg(feature = "with-rust_decimal")]
impl_value_vec! {
    rust_decimal::Decimal => Decimal
}

#[cfg(feature = "with-bigdecimal")]
impl_value_vec! {
    bigdecimal::BigDecimal => BigDecimal
}

#[cfg(feature = "with-uuid")]
impl_value_vec! {
    uuid::Uuid => Uuid
}

// uuid::fmt::* types use UUID arrays
macro_rules! impl_uuid_fmt_pg_array_element {
    ($ty:ty, $method:ident) => {
        #[cfg(feature = "with-uuid")]
        impl ArrayElement for $ty {
            type ArrayValueType = Uuid;

            fn into_array_value(self) -> Self::ArrayValueType {
                self.into_uuid()
            }

            fn try_from_value(v: Value) -> Result<Vec<Option<Self>>, ValueTypeErr> {
                match v {
                    Value::Array(Array::Uuid(inner)) => Ok(inner
                        .into_vec()
                        .into_iter()
                        .map(|opt| opt.map(Self::from))
                        .collect()),
                    Value::Array(Array::Null(_)) => Ok(vec![]),
                    _ => Err(ValueTypeErr),
                }
            }
        }
    };
}

impl_uuid_fmt_pg_array_element!(uuid::fmt::Braced, braced);
impl_uuid_fmt_pg_array_element!(uuid::fmt::Hyphenated, hyphenated);
impl_uuid_fmt_pg_array_element!(uuid::fmt::Simple, simple);
impl_uuid_fmt_pg_array_element!(uuid::fmt::Urn, urn);

#[cfg(feature = "with-ipnetwork")]
impl_value_vec! {
    IpNetwork => IpNetwork
}

#[cfg(feature = "with-mac_address")]
impl_value_vec! {
    MacAddress => MacAddress
}

impl<T> Nullable for Vec<T>
where
    T: Into<Value> + NotU8 + ValueType,
{
    fn null() -> Value {
        Value::Array(Array::Null(T::array_type()))
    }
}

impl Value {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn as_ref_array(&self) -> Option<&Array> {
        match self {
            Self::Array(v) if !v.is_null() => Some(v),
            Self::Array(_) => None,
            _ => panic!("not Value::Array"),
        }
    }
}
