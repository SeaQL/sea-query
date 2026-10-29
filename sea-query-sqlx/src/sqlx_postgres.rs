#[cfg(all(feature = "postgres-array", feature = "with-bigdecimal"))]
use sea_query::prelude::BigDecimal;
#[cfg(all(feature = "postgres-array", feature = "with-rust_decimal"))]
use sea_query::prelude::Decimal;
#[cfg(all(feature = "postgres-array", feature = "with-ipnetwork"))]
use sea_query::prelude::IpNetwork;
#[cfg(all(feature = "postgres-array", feature = "with-json"))]
use sea_query::prelude::Json;
#[cfg(all(feature = "postgres-array", feature = "with-mac_address"))]
use sea_query::prelude::MacAddress;
#[cfg(all(feature = "postgres-array", feature = "with-uuid"))]
use sea_query::prelude::Uuid;
#[cfg(all(feature = "postgres-array", feature = "with-time"))]
use sea_query::prelude::time;
#[cfg(all(feature = "postgres-array", feature = "with-chrono"))]
use sea_query::prelude::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[cfg(feature = "postgres-array")]
use sea_query::ArrayType;
use sea_query::{OptionEnum, Value};

use crate::{SqlxValues, TryIntoArguments};

impl sqlx::IntoArguments<sqlx::postgres::Postgres> for SqlxValues {
    fn into_arguments(self) -> sqlx::postgres::PgArguments {
        TryIntoArguments::<sqlx::postgres::Postgres>::try_into_arguments(self)
            .expect("value cannot be bound, use `try_into_arguments` to handle the error")
    }
}

impl TryIntoArguments<sqlx::postgres::Postgres> for SqlxValues {
    fn try_into_arguments(self) -> Result<sqlx::postgres::PgArguments, sqlx::Error> {
        let mut args = sqlx::postgres::PgArguments::default();
        for arg in self.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    args.add(b).map_err(sqlx::Error::Encode)?;
                }
                Value::TinyInt(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::SmallInt(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::Int(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::BigInt(i) => {
                    args.add(i).map_err(sqlx::Error::Encode)?;
                }
                Value::TinyUnsigned(i) => {
                    args.add(i.map(|i| i as i16)).map_err(sqlx::Error::Encode)?;
                }
                Value::SmallUnsigned(i) => {
                    args.add(i.map(|i| i as i32)).map_err(sqlx::Error::Encode)?;
                }
                Value::Unsigned(i) => {
                    args.add(i.map(|i| i as i64)).map_err(sqlx::Error::Encode)?;
                }
                Value::BigUnsigned(i) => {
                    args.add(
                        i.map(i64::try_from)
                            .transpose()
                            .map_err(|e| sqlx::Error::Encode(e.into()))?,
                    )
                    .map_err(sqlx::Error::Encode)?;
                }
                Value::Float(f) => {
                    args.add(f).map_err(sqlx::Error::Encode)?;
                }
                Value::Double(d) => {
                    args.add(d).map_err(sqlx::Error::Encode)?;
                }
                Value::String(s) => {
                    args.add(s.as_deref()).map_err(sqlx::Error::Encode)?;
                }
                Value::Enum(e) => {
                    let value = match e {
                        OptionEnum::Some(v) => Some(v.value.into_owned()),
                        OptionEnum::None(_) => None,
                    };
                    args.add(value).map_err(sqlx::Error::Encode)?;
                }
                Value::Char(c) => {
                    args.add(c.map(|c| c.to_string()))
                        .map_err(sqlx::Error::Encode)?;
                }
                Value::Bytes(b) => {
                    args.add(b.as_deref()).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    args.add(d).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    args.add(t).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDate(_) => {
                    panic!("SQLx 0.9 does not support Jiff arguments for Postgres yet");
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTime(_) => {
                    panic!("SQLx 0.9 does not support Jiff arguments for Postgres yet");
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffDateTime(_) => {
                    panic!("SQLx 0.9 does not support Jiff arguments for Postgres yet");
                }
                #[cfg(feature = "with-jiff")]
                Value::JiffTimestamp(_) => {
                    panic!("SQLx 0.9 does not support Jiff arguments for Postgres yet");
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    args.add(uuid).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(d) => {
                    args.add(d).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(d) => {
                    args.add(d.as_deref()).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    args.add(j.as_deref()).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(ip) => {
                    args.add(ip).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(mac) => {
                    args.add(mac).map_err(sqlx::Error::Encode)?;
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(ty, v) => match ty {
                    ArrayType::Bool => {
                        let value: Option<Vec<bool>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Bool");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::TinyInt => {
                        let value: Option<Vec<i8>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TinyInt");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::SmallInt => {
                        let value: Option<Vec<i16>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::SmallInt");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::Int => {
                        let value: Option<Vec<i32>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Int");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::BigInt => {
                        let value: Option<Vec<i64>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::BigInt");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::TinyUnsigned => {
                        let value: Option<Vec<u8>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TinyUnsigned");
                        let value: Option<Vec<i16>> =
                            value.map(|vec| vec.into_iter().map(|i| i as i16).collect());
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::SmallUnsigned => {
                        let value: Option<Vec<u16>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::SmallUnsigned");
                        let value: Option<Vec<i32>> =
                            value.map(|vec| vec.into_iter().map(|i| i as i32).collect());
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::Unsigned => {
                        let value: Option<Vec<u32>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Unsigned");
                        let value: Option<Vec<i64>> =
                            value.map(|vec| vec.into_iter().map(|i| i as i64).collect());
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::BigUnsigned => {
                        let value: Option<Vec<u64>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::BigUnsigned");
                        let value: Option<Vec<i64>> = value
                            .map(|vec| vec.into_iter().map(i64::try_from).collect())
                            .transpose()
                            .map_err(|e| sqlx::Error::Encode(e.into()))?;
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::Float => {
                        let value: Option<Vec<f32>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Float");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::Double => {
                        let value: Option<Vec<f64>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Double");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::String => {
                        let value: Option<Vec<String>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::String");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::Enum(_) => {
                        let value: Option<Vec<String>> = v.map(|values| {
                            values
                                .into_iter()
                                .map(|value| match value {
                                    Value::Enum(OptionEnum::Some(value))  => value.value.into_owned(),
                                    _ => {
                                        panic!(
                                            "Value::Array(ArrayType::Enum) should contain Value::Enum"
                                        );
                                    }
                                })
                                .collect()
                        });
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::Char => {
                        let value: Option<Vec<char>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Char");
                        let value: Option<Vec<String>> =
                            value.map(|vec| vec.into_iter().map(|c| c.to_string()).collect());
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    ArrayType::Bytes => {
                        let value: Option<Vec<Vec<u8>>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Bytes");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDate => {
                        let value: Option<Vec<NaiveDate>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::ChronoDate");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoTime => {
                        let value: Option<Vec<NaiveTime>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::ChronoTime");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDateTime => {
                        let value: Option<Vec<NaiveDateTime>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::ChronoDateTime");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDateTimeUtc => {
                        let value: Option<Vec<DateTime<Utc>>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::ChronoDateTimeUtc");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDateTimeLocal => {
                        let value: Option<Vec<DateTime<Local>>> = Value::Array(ty, v).expect(
                            "This Value::Array should consist of Value::ChronoDateTimeLocal",
                        );
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDateTimeWithTimeZone => {
                        let value: Option<Vec<DateTime<FixedOffset>>> = Value::Array(ty, v).expect(
                            "This Value::Array should consist of Value::ChronoDateTimeWithTimeZone",
                        );
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-time")]
                    ArrayType::TimeDate => {
                        let value: Option<Vec<time::Date>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TimeDate");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-time")]
                    ArrayType::TimeTime => {
                        let value: Option<Vec<time::Time>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TimeTime");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-time")]
                    ArrayType::TimeDateTime => {
                        let value: Option<Vec<time::PrimitiveDateTime>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TimeDateTime");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-time")]
                    ArrayType::TimeDateTimeWithTimeZone => {
                        let value: Option<Vec<time::OffsetDateTime>> = Value::Array(ty, v).expect(
                            "This Value::Array should consist of Value::TimeDateTimeWithTimeZone",
                        );
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-jiff")]
                    ArrayType::JiffDate => {
                        panic!("SQLx 0.9 does not support Jiff array arguments for Postgres yet");
                    }
                    #[cfg(feature = "with-jiff")]
                    ArrayType::JiffTime => {
                        panic!("SQLx 0.9 does not support Jiff array arguments for Postgres yet");
                    }
                    #[cfg(feature = "with-jiff")]
                    ArrayType::JiffDateTime => {
                        panic!("SQLx 0.9 does not support Jiff array arguments for Postgres yet");
                    }
                    #[cfg(feature = "with-jiff")]
                    ArrayType::JiffTimestamp => {
                        panic!("SQLx 0.9 does not support Jiff array arguments for Postgres yet");
                    }
                    #[cfg(feature = "with-uuid")]
                    ArrayType::Uuid => {
                        let value: Option<Vec<Uuid>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Uuid");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-rust_decimal")]
                    ArrayType::Decimal => {
                        let value: Option<Vec<Decimal>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Decimal");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-bigdecimal")]
                    ArrayType::BigDecimal => {
                        let value: Option<Vec<BigDecimal>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::BigDecimal");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-json")]
                    ArrayType::Json => {
                        let value: Option<Vec<Json>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Json");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-ipnetwork")]
                    ArrayType::IpNetwork => {
                        let value: Option<Vec<IpNetwork>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::IpNetwork");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                    #[cfg(feature = "with-mac_address")]
                    ArrayType::MacAddress => {
                        let value: Option<Vec<MacAddress>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::MacAddress");
                        args.add(value).map_err(sqlx::Error::Encode)?;
                    }
                },
                #[cfg(feature = "postgres-vector")]
                Value::Vector(v) => {
                    args.add(v).map_err(sqlx::Error::Encode)?;
                } /* #[cfg(feature = "postgres-range")]
                  Value::Range(v) => {
                  args.add(v).map_err(sqlx::Error::Encode)?;
                  } */
            }
        }
        Ok(args)
    }
}
