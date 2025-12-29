use super::*;

type_to_box_value!(Json, Json, Json);

impl Value {
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json(_))
    }

    pub fn as_ref_json(&self) -> Option<&Json> {
        match self {
            Self::Json(v) => v.as_deref(),
            _ => panic!("not Value::Json"),
        }
    }
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
pub fn sea_value_to_json_value(value: &Value) -> Json {
    match value {
        Value::Bool(None)
        | Value::TinyInt(None)
        | Value::SmallInt(None)
        | Value::Int(None)
        | Value::BigInt(None)
        | Value::TinyUnsigned(None)
        | Value::SmallUnsigned(None)
        | Value::Unsigned(None)
        | Value::BigUnsigned(None)
        | Value::Float(None)
        | Value::Double(None)
        | Value::String(None)
        | Value::Char(None)
        | Value::Bytes(None)
        | Value::Json(None) => Json::Null,
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(None) => Json::Null,
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(None) => Json::Null,
        #[cfg(feature = "with-uuid")]
        Value::Uuid(None) => Json::Null,
        #[cfg(feature = "postgres-array")]
        Value::Array(_, None) => Json::Null,
        #[cfg(feature = "postgres-vector")]
        Value::Vector(None) => Json::Null,
        #[cfg(feature = "with-ipnetwork")]
        Value::IpNetwork(None) => Json::Null,
        #[cfg(feature = "with-mac_address")]
        Value::MacAddress(None) => Json::Null,
        #[cfg(feature = "postgres-range")]
        Value::Range(None) => Json::Null,
        Value::Bool(Some(b)) => Json::Bool(*b),
        Value::TinyInt(Some(v)) => (*v).into(),
        Value::SmallInt(Some(v)) => (*v).into(),
        Value::Int(Some(v)) => (*v).into(),
        Value::BigInt(Some(v)) => (*v).into(),
        Value::TinyUnsigned(Some(v)) => (*v).into(),
        Value::SmallUnsigned(Some(v)) => (*v).into(),
        Value::Unsigned(Some(v)) => (*v).into(),
        Value::BigUnsigned(Some(v)) => (*v).into(),
        Value::Float(Some(v)) => (*v).into(),
        Value::Double(Some(v)) => (*v).into(),
        Value::String(Some(s)) => Json::String(s.clone()),
        Value::Char(Some(v)) => Json::String(v.to_string()),
        Value::Bytes(Some(s)) => Json::String(std::str::from_utf8(s).unwrap().to_string()),
        Value::Json(Some(v)) => v.as_ref().clone(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeWithTimeZone(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeUtc(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeLocal(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDateTimeWithTimeZone(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-jiff")]
        Value::JiffDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-jiff")]
        Value::JiffTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-jiff")]
        Value::JiffDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-jiff")]
        Value::JiffTimestamp(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-jiff")]
        Value::JiffZoned(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(Some(v)) => {
            use rust_decimal::prelude::ToPrimitive;
            v.to_f64().unwrap().into()
        }
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(Some(v)) => {
            use bigdecimal::ToPrimitive;
            v.to_f64().unwrap().into()
        }
        #[cfg(feature = "with-uuid")]
        Value::Uuid(Some(v)) => Json::String(v.to_string()),
        #[cfg(feature = "postgres-array")]
        Value::Array(_, Some(v)) => Json::Array(v.iter().map(sea_value_to_json_value).collect()),
        #[cfg(feature = "postgres-vector")]
        Value::Vector(Some(v)) => Json::Array(v.as_slice().iter().map(|&v| v.into()).collect()),
        #[cfg(feature = "with-ipnetwork")]
        Value::IpNetwork(Some(_)) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-mac_address")]
        Value::MacAddress(Some(_)) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "postgres-range")]
        Value::Range(Some(_)) => CommonSqlQueryBuilder.value_to_string(value).into(),
    }
}
