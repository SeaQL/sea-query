#[cfg(feature = "with-json")]
pub use serde_json::Value as Json;

#[cfg(feature = "with-chrono")]
pub use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[cfg(feature = "with-time")]
pub use time::{OffsetDateTime, PrimitiveDateTime};

#[cfg(feature = "with-jiff")]
pub use jiff::{Timestamp, Zoned};

#[cfg(feature = "with-rust_decimal")]
pub use rust_decimal::Decimal;

#[cfg(feature = "with-bigdecimal")]
pub use bigdecimal::BigDecimal;

#[cfg(feature = "with-uuid")]
pub use uuid::Uuid;

#[cfg(feature = "with-ipnetwork")]
pub use ipnetwork::IpNetwork;

#[cfg(feature = "with-ipnetwork")]
pub use std::net::IpAddr;

#[cfg(feature = "with-mac_address")]
pub use mac_address::MacAddress;

#[cfg(feature = "postgres-range")]
pub use sea_query_postgres_types::range::RangeType;
