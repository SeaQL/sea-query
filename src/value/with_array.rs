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

// TODO impl<T: NotU8> NotU8 for Option<T> {}

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

#[cfg(feature = "with-time")]
impl NotU8 for UtcDateTime {}

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

impl<T> From<Vec<T>> for Value
where
    T: Into<Value> + NotU8 + ValueType,
{
    fn from(x: Vec<T>) -> Value {
        Value::Array(
            T::array_type(),
            Some(Box::new(x.into_iter().map(|e| e.into()).collect())),
        )
    }
}

impl<T> Nullable for Vec<T>
where
    T: Into<Value> + NotU8 + ValueType,
{
    fn null() -> Value {
        Value::Array(T::array_type(), None)
    }
}

impl<T> ValueType for Vec<T>
where
    T: NotU8 + ValueType,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Array(ty, Some(v)) if T::array_type() == ty => {
                Ok(v.into_iter().map(|e| e.unwrap()).collect())
            }
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Vec<T>).to_owned()
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        use ColumnType::*;
        Array(RcOrArc::new(T::column_type()))
    }
}

impl Value {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }

    pub fn as_ref_array(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Array(_, v) => v.as_ref().map(|v| v.as_ref()),
            _ => panic!("not Value::Array"),
        }
    }
}
