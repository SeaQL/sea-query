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
impl<Tz> NotU8 for DateTime<Tz> where Tz: chrono::TimeZone {}

#[cfg(feature = "with-time")]
impl NotU8 for time::Date {}

#[cfg(feature = "with-time")]
impl NotU8 for time::Time {}

#[cfg(feature = "with-time")]
impl NotU8 for PrimitiveDateTime {}

#[cfg(feature = "with-time")]
impl NotU8 for OffsetDateTime {}

#[cfg(feature = "with-rust_decimal")]
impl NotU8 for Decimal {}

#[cfg(feature = "with-bigdecimal")]
impl NotU8 for BigDecimal {}

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
            impl From<Vec<$ty>> for Array {
                fn from(x: Vec<$ty>) -> Array {
                    let values: Vec<Option<_>> = x
                        .into_iter()
                        .map(Some)
                        .collect();

                    Array::$vari(values.into_boxed_slice())
                }
            }


            impl From<Vec<Option<$ty>>> for Array {
                fn from(x: Vec<Option<$ty>>) -> Array {
                    Array::$vari(x.into_boxed_slice())
                }
            }

            impl From<Vec<$ty>> for Value {
                fn from(x: Vec<$ty>) -> Value {
                    let values: Vec<Option<_>> = x
                        .into_iter()
                        .map(Some)
                        .collect();

                    Value::Array(
                        Some(Array::$vari(values.into_boxed_slice()))
                    )
                }
            }

            impl From<Vec<Option<$ty>>> for Value {
                fn from(x: Vec<Option<$ty>>) -> Value {
                    Value::Array(Some(Array::$vari(x.into())))
                }
            }

            impl ValueType for Vec<Option<$ty>>
            {
                fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                    match v {
                        Value::Array(Some(Array::$vari(inner))) => {
                            Ok(inner.into_vec())
                        }
                        _ => Err(ValueTypeErr),
                    }
                }

                fn type_name() -> String {
                    stringify!(Vec<$ty>).to_owned()
                }

                fn array_type() -> ArrayType {
                    <$ty>::array_type()
                }

                fn column_type() -> ColumnType {
                    use ColumnType::*;
                    Array(RcOrArc::new(<$ty>::column_type()))
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
        let values: Vec<Option<_>> = x.into_iter().map(Some).collect();

        Array::TinyUnsigned(values.into_boxed_slice())
    }
}

impl From<Vec<Option<u8>>> for Array {
    fn from(x: Vec<Option<u8>>) -> Array {
        Array::TinyUnsigned(x.into_boxed_slice())
    }
}

impl From<Vec<Option<u8>>> for Value {
    fn from(x: Vec<Option<u8>>) -> Value {
        Value::Array(Some(Array::TinyUnsigned(x.into_boxed_slice())))
    }
}

impl ValueType for Vec<Option<u8>> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Array(Some(Array::TinyUnsigned(inner))) => Ok(inner.into_vec()),
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

#[cfg(feature = "backend-postgres")]
impl From<(Arc<str>, Vec<Option<Arc<Enum>>>)> for Value {
    fn from(x: (Arc<str>, Vec<Option<Arc<Enum>>>)) -> Value {
        Value::Array(Some(Array::Enum(Box::new((x.0, x.1.into_boxed_slice())))))
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
        Value::Array(None)
    }
}

impl Value {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn as_ref_array(&self) -> Option<&Array> {
        match self {
            Self::Array(v) => v.as_ref(),
            _ => panic!("not Value::Array"),
        }
    }
}
