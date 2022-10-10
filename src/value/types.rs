use std::fmt::Debug;

use crate::SeaRc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    Bool,
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    String,
    Char,
    Bytes,

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone,

    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid,

    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal,

    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal,

    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork,

    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IsNull {
    /// The value is NULL.
    Yes,
    /// The value is not NULL.
    No,
}

#[cfg(feature = "thread-safe")]
pub trait ValueTrait: Sync + Send
where
    Self: Debug,
{
    fn to_sql_string(&self) -> String;

    fn value_type() -> ValueType
    where
        Self: Sized;

    fn value_is_null(&self) -> IsNull;
}

#[cfg(not(feature = "thread-safe"))]
pub trait ValueTrait
where
    Self: Debug,
{
    fn to_sql_string(&self) -> String;

    fn value_type() -> ValueType
    where
        Self: Sized;

    fn value_is_null(&self) -> IsNull;
}

#[derive(Debug, Clone)]
pub struct Value {
    pub(crate) ty: ValueType,
    #[cfg(feature = "thread-safe")]
    pub(crate) object: SeaRc<Box<dyn ValueTrait + Sync + Send>>,
    #[cfg(not(feature = "thread-safe"))]
    pub(crate) object: SeaRc<Box<dyn ValueTrait>>,
}

impl Value {
    pub fn to_sql_string(&self) -> String {
        todo!()
    }

    pub fn ty(&self) -> &ValueType {
        &self.ty
    }

    pub fn is_null(&self) -> bool {
        matches!(self.object.value_is_null(), IsNull::Yes)
    }

    pub fn value<T: ValueTrait>(&self) -> Result<&T, ()> {
        if T::value_type() == self.ty {
            let (v, _): (&T, *const ()) =
                unsafe { std::mem::transmute(self.object.as_ref().as_ref()) };
            Ok(v)
        } else {
            Err(())
        }
    }
}

macro_rules! simple_to {
    ( $type: ty, $value_type: expr ) => {
        impl From<$type> for Value {
            fn from(v: $type) -> Value {
                Value {
                    ty: <$type>::value_type(),
                    object: SeaRc::new(Box::new(v)),
                }
            }
        }

        impl ValueTrait for $type {
            fn to_sql_string(&self) -> String {
                todo!()
            }

            fn value_type() -> ValueType {
                $value_type
            }

            fn value_is_null(&self) -> IsNull {
                IsNull::No
            }
        }
    };
}

simple_to!(bool, ValueType::Bool);
simple_to!(i8, ValueType::TinyInt);
simple_to!(i16, ValueType::SmallInt);
simple_to!(i32, ValueType::Int);
simple_to!(i64, ValueType::BigInt);
simple_to!(u8, ValueType::TinyUnsigned);
simple_to!(u16, ValueType::SmallUnsigned);
simple_to!(u32, ValueType::Unsigned);
simple_to!(u64, ValueType::BigUnsigned);
simple_to!(f32, ValueType::Float);
simple_to!(f64, ValueType::Double);
simple_to!(char, ValueType::Char);

impl<T: ValueTrait> ValueTrait for Option<T> {
    fn to_sql_string(&self) -> String {
        todo!()
    }

    fn value_type() -> ValueType {
        T::value_type()
    }

    fn value_is_null(&self) -> IsNull {
        match self {
            Some(_) => IsNull::Yes,
            None => IsNull::No,
        }
    }
}

impl<T: ValueTrait + 'static> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        let object = Box::new(v) as _;
        Value {
            ty: T::value_type(),
            object: SeaRc::new(object),
        }
    }
}

impl<T: ValueTrait> ValueTrait for Vec<T> {
    fn to_sql_string(&self) -> String {
        todo!()
    }

    fn value_type() -> ValueType {
        T::value_type()
    }

    fn value_is_null(&self) -> IsNull {
        IsNull::No
    }
}

impl<T: ValueTrait + 'static> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        let object = Box::new(v) as _;
        Value {
            ty: T::value_type(),
            object: SeaRc::new(object),
        }
    }
}
