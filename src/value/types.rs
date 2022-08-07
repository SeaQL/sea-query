use std::error::Error;
use std::fmt::{Debug, Formatter};

use crate::SeaRc;
#[cfg(feature = "with-postgres")]
use bytes::BytesMut;
#[cfg(feature = "with-postgres")]
use postgres_types::{to_sql_checked, IsNull, ToSql as PostgresToSql, Type};
#[cfg(feature = "with-rusqlite")]
use rusqlite::{types::ToSqlOutput, Result, ToSql as RuSqliteToSql};

pub trait ValueTrait: PostgresToSql + RuSqliteToSql
where
    Self: Debug,
{
    fn to_sql_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    TinyUnsigned(u8),
    SmallUnsigned(u16),
    Unsigned(u32),
    BigUnsigned(u64),
    Float(f32),
    Double(f64),
    #[cfg(feature = "thread-safe")]
    Object(SeaRc<Box<dyn ValueTrait + Sync + Send>>),
    #[cfg(not(feature = "thread-safe"))]
    Object(SeaRc<Box<dyn ValueTrait>>),
}

impl PostgresToSql for Value {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            Value::Bool(v) => <bool as PostgresToSql>::to_sql(v, ty, out),
            Value::TinyInt(v) => <i8 as PostgresToSql>::to_sql(v, ty, out),
            Value::SmallInt(v) => <i16 as PostgresToSql>::to_sql(v, ty, out),
            Value::Int(v) => <i32 as PostgresToSql>::to_sql(v, ty, out),
            Value::BigInt(v) => <i64 as PostgresToSql>::to_sql(v, ty, out),
            Value::TinyUnsigned(v) => <u32 as PostgresToSql>::to_sql(&(*v as u32), ty, out),
            Value::SmallUnsigned(v) => <u32 as PostgresToSql>::to_sql(&(*v as u32), ty, out),
            Value::Unsigned(v) => <u32 as PostgresToSql>::to_sql(v, ty, out),
            Value::BigUnsigned(v) => <i64 as PostgresToSql>::to_sql(&(*v as i64), ty, out),
            Value::Float(v) => <f32 as PostgresToSql>::to_sql(v, ty, out),
            Value::Double(v) => <f64 as PostgresToSql>::to_sql(v, ty, out),
            Value::Object(o) => (o as &dyn PostgresToSql).to_sql(),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    to_sql_checked!();
}

impl RuSqliteToSql for Value {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        match self {
            Value::Bool(v) => <bool as RuSqliteToSql>::to_sql(v),
            Value::TinyInt(v) => <i8 as RuSqliteToSql>::to_sql(v),
            Value::SmallInt(v) => <i16 as RuSqliteToSql>::to_sql(v),
            Value::Int(v) => <i32 as RuSqliteToSql>::to_sql(v),
            Value::BigInt(v) => <i64 as RuSqliteToSql>::to_sql(v),
            Value::TinyUnsigned(v) => <u8 as RuSqliteToSql>::to_sql(v),
            Value::SmallUnsigned(v) => <u16 as RuSqliteToSql>::to_sql(v),
            Value::Unsigned(v) => <u32 as RuSqliteToSql>::to_sql(v),
            Value::BigUnsigned(v) => <u64 as RuSqliteToSql>::to_sql(v),
            Value::Float(v) => <f32 as RuSqliteToSql>::to_sql(v),
            Value::Double(v) => <f64 as RuSqliteToSql>::to_sql(v),
            Value::Object(o) => (o as &dyn RuSqliteToSql).to_sql(),
        }
    }
}

impl ValueTrait for Value {
    fn to_sql_string(&self) -> String {
        todo!()
    }
}

macro_rules! simple_to {
    ( $type: ty, $name: expr ) => {
        impl From<$type> for Value {
            fn from(v: $type) -> Value {
                use Value::*;
                $name(v.to_owned())
            }
        }
    };
}

macro_rules! object_to {
    ( $type: ty ) => {
        impl From<$type> for Value {
            fn from(v: $type) -> Value {
                let object = Box::new(v) as _;
                Value::Object(SeaRc::new(object))
            }
        }
    };
}

simple_to!(i8, TinyInt);
simple_to!(i16, SmallInt);
simple_to!(i32, Int);
simple_to!(i64, BigInt);
simple_to!(u8, TinyUnsigned);
simple_to!(u16, SmallUnsigned);
simple_to!(u32, Unsigned);
simple_to!(u64, BigUnsigned);
simple_to!(f32, Float);
simple_to!(f64, Double);

impl ValueTrait for String {
    fn to_sql_string(&self) -> String {
        todo!()
    }
}

object_to!(String);

impl ValueTrait for &str {
    fn to_sql_string(&self) -> String {
        todo!()
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        let object = Box::new(v.to_string()) as _;
        Value::Object(SeaRc::new(object))
    }
}

impl From<char> for Value {
    fn from(v: char) -> Self {
        let object = Box::new(v.to_string()) as _;
        Value::Object(SeaRc::new(object))
    }
}
