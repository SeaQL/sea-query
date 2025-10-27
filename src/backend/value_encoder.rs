use crate::{prepare::write_int, *};
use std::fmt::Write;

#[allow(unused_variables)]
pub trait ValueEncoder: EscapeBuilder {
    fn write_bool(&self, buf: &mut impl Write, value: bool) {
        buf.write_str(if value { "TRUE" } else { "FALSE" }).unwrap();
    }

    fn write_i8(&self, buf: &mut impl Write, value: i8) {
        write_int(buf, value);
    }

    fn write_i16(&self, buf: &mut impl Write, value: i16) {
        write_int(buf, value);
    }

    fn write_i32(&self, buf: &mut impl Write, value: i32) {
        write_int(buf, value);
    }

    fn write_i64(&self, buf: &mut impl Write, value: i64) {
        write_int(buf, value);
    }

    fn write_u8(&self, buf: &mut impl Write, value: u8) {
        write_int(buf, value);
    }

    fn write_u16(&self, buf: &mut impl Write, value: u16) {
        write_int(buf, value);
    }

    fn write_u32(&self, buf: &mut impl Write, value: u32) {
        write_int(buf, value);
    }

    fn write_u64(&self, buf: &mut impl Write, value: u64) {
        write_int(buf, value);
    }

    fn write_f32(&self, buf: &mut impl Write, value: f32) {
        write!(buf, "{value}").unwrap();
    }

    fn write_f64(&self, buf: &mut impl Write, value: f64) {
        write!(buf, "{value}").unwrap();
    }

    fn write_str(&self, buf: &mut impl Write, value: &str) {
        buf.write_str("'").unwrap();
        self.write_escaped(buf, value);
        buf.write_str("'").unwrap();
    }

    fn write_char(&self, buf: &mut impl Write, value: char) {
        let mut tmp = [0u8; 4];
        let s = value.encode_utf8(&mut tmp);
        self.write_str(buf, s);
    }

    fn write_bytes(&self, buf: &mut impl Write, value: &[u8]) {
        buf.write_str("x'").unwrap();
        for b in value {
            write!(buf, "{b:02X}").unwrap()
        }
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-json")]
    fn write_json(&self, buf: &mut impl Write, value: &serde_json::Value) {
        self.write_str(buf, &value.to_string());
    }

    #[cfg(feature = "with-chrono")]
    fn write_naive_date(&self, buf: &mut impl Write, value: &chrono::NaiveDate) {
        buf.write_str("'").unwrap();
        value.format("%Y-%m-%d").write_to(buf).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_naive_time(&self, buf: &mut impl Write, value: &chrono::NaiveTime) {
        buf.write_str("'").unwrap();
        value.format("%H:%M:%S%.6f").write_to(buf).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_naive_datetime(&self, buf: &mut impl Write, value: &chrono::NaiveDateTime) {
        buf.write_str("'").unwrap();
        value.format("%Y-%m-%d %H:%M:%S%.6f").write_to(buf).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_datetime_utc(&self, buf: &mut impl Write, value: &chrono::DateTime<chrono::Utc>) {
        buf.write_str("'").unwrap();
        value
            .format("%Y-%m-%d %H:%M:%S%.6f %:z")
            .write_to(buf)
            .unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_datetime_local(&self, buf: &mut impl Write, value: &chrono::DateTime<chrono::Local>) {
        buf.write_str("'").unwrap();
        value
            .format("%Y-%m-%d %H:%M:%S%.6f %:z")
            .write_to(buf)
            .unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_datetime_fixed(
        &self,
        buf: &mut impl Write,
        value: &chrono::DateTime<chrono::FixedOffset>,
    ) {
        buf.write_str("'").unwrap();
        value
            .format("%Y-%m-%d %H:%M:%S%.6f %:z")
            .write_to(buf)
            .unwrap();
        buf.write_str("'").unwrap();
    }

    // TODO: https://github.com/time-rs/time/issues/375
    // Currently, time crate dosen't support formatting into impl fmt::Write
    // So this solution must allocate a temporary String
    // Fix it when the issue is resolved
    #[cfg(feature = "with-time")]
    fn write_time_date(&self, buf: &mut impl Write, value: &time::Date) {
        buf.write_str("'").unwrap();
        let s = value
            .format(crate::value::time_format::FORMAT_DATE)
            .unwrap();
        buf.write_str(&s).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-time")]
    fn write_time_time(&self, buf: &mut impl Write, value: &time::Time) {
        buf.write_str("'").unwrap();
        let s = value
            .format(crate::value::time_format::FORMAT_TIME)
            .unwrap();
        buf.write_str(&s).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-time")]
    fn write_time_datetime(&self, buf: &mut impl Write, value: &time::PrimitiveDateTime) {
        buf.write_str("'").unwrap();
        let s = value
            .format(crate::value::time_format::FORMAT_DATETIME)
            .unwrap();
        buf.write_str(&s).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-time")]
    fn write_time_datetime_tz(&self, buf: &mut impl Write, value: &time::OffsetDateTime) {
        buf.write_str("'").unwrap();
        let s = value
            .format(crate::value::time_format::FORMAT_DATETIME_TZ)
            .unwrap();
        buf.write_str(&s).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_date(&self, buf: &mut impl Write, value: &jiff::civil::Date) {
        buf.write_str("'").unwrap();
        write!(buf, "{value}").unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_time(&self, buf: &mut impl Write, value: &jiff::civil::Time) {
        buf.write_str("'").unwrap();
        write!(buf, "{value}").unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_datetime(&self, buf: &mut impl Write, value: &jiff::civil::DateTime) {
        use crate::value::with_jiff::JIFF_DATE_TIME_FMT_STR;
        buf.write_str("'").unwrap();
        write!(buf, "{}", value.strftime(JIFF_DATE_TIME_FMT_STR)).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_timestamp(&self, buf: &mut impl Write, value: &jiff::Timestamp) {
        use crate::value::with_jiff::JIFF_TIMESTAMP_FMT_STR;
        buf.write_str("'").unwrap();
        write!(buf, "{}", value.strftime(JIFF_TIMESTAMP_FMT_STR)).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_zoned(&self, buf: &mut impl Write, value: &jiff::Zoned) {
        use crate::value::with_jiff::JIFF_ZONE_FMT_STR;
        buf.write_str("'").unwrap();
        write!(buf, "{}", value.strftime(JIFF_ZONE_FMT_STR)).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "postgres-vector")]
    fn write_vector(&self, buf: &mut impl Write, value: &pgvector::Vector) {
        buf.write_str("'[").unwrap();
        let mut iter = value.as_slice().iter();
        if let Some(first) = iter.next() {
            write!(buf, "{first}").unwrap();
        }
        for v in iter {
            buf.write_char(',').unwrap();
            write!(buf, "{v}").unwrap();
        }
        buf.write_str("]'").unwrap();
    }

    #[cfg(feature = "with-rust_decimal")]
    fn write_decimal(&self, buf: &mut impl Write, value: &rust_decimal::Decimal) {
        write!(buf, "{value}").unwrap();
    }

    #[cfg(feature = "with-bigdecimal")]
    fn write_bigdecimal(&self, buf: &mut impl Write, value: &bigdecimal::BigDecimal) {
        write!(buf, "{value}").unwrap();
    }

    #[cfg(feature = "with-uuid")]
    fn write_uuid(&self, buf: &mut impl Write, value: &uuid::Uuid) {
        self.write_str(buf, &value.to_string());
    }

    #[cfg(feature = "with-ipnetwork")]
    fn write_ipnetwork(&self, buf: &mut impl Write, value: &ipnetwork::IpNetwork) {
        self.write_str(buf, &value.to_string());
    }

    #[cfg(feature = "with-mac_address")]
    fn write_mac(&self, buf: &mut impl Write, value: &mac_address::MacAddress) {
        self.write_str(buf, &value.to_string());
    }

    #[cfg(feature = "backend-postgres")]
    fn write_enum(&self, buf: &mut impl Write, value: &crate::value::Enum) {
        buf.write_str(value.value.as_str());
    }

    #[cfg(feature = "postgres-array")]
    fn write_array(&self, buf: &mut impl Write, array: &crate::value::Array) {
        use crate::value::Array;

        fn write_items<QB, T, F, W>(encoder: &QB, buf: &mut W, items: &[Option<T>], mut f: F)
        where
            QB: ValueEncoder + ?Sized,
            W: Write,
            F: FnMut(&QB, &mut W, &T),
        {
            let mut first = true;
            for item in items {
                if !first {
                    buf.write_char(',').unwrap();
                }
                first = false;
                match item {
                    Some(value) => f(encoder, buf, value),
                    None => buf.write_str("NULL").unwrap(),
                }
            }
        }

        fn write_array_recursive<QB, W>(encoder: &QB, buf: &mut W, array: &Array)
        where
            QB: ValueEncoder + ?Sized,
            W: Write,
        {
            if array.is_empty() {
                buf.write_str("'{}'").unwrap();
                return;
            }

            buf.write_str("'ARRAY[").unwrap();
            match array {
                Array::Bool(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_bool(buf, *val))
                }
                Array::TinyInt(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_i8(buf, *val))
                }
                Array::SmallInt(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_i16(buf, *val))
                }
                Array::Int(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_i32(buf, *val))
                }
                Array::BigInt(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_i64(buf, *val))
                }
                Array::TinyUnsigned(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_u8(buf, *val))
                }
                Array::SmallUnsigned(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_u16(buf, *val))
                }
                Array::Unsigned(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_u32(buf, *val))
                }
                Array::BigUnsigned(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_u64(buf, *val))
                }
                Array::Float(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_f32(buf, *val))
                }
                Array::Double(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_f64(buf, *val))
                }
                Array::String(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_str(buf, val))
                }
                Array::Char(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_char(buf, *val))
                }
                Array::Bytes(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_bytes(buf, val))
                }
                #[cfg(feature = "backend-postgres")]
                Array::Enum(boxed) => {
                    write_items(encoder, buf, &boxed.as_ref().1, |qb, buf, val| {
                        qb.write_enum(buf, val.as_ref())
                    })
                }
                Array::Array(boxed) => {
                    let (_, inner) = boxed.as_ref();
                    let mut first = true;
                    for item in inner.iter() {
                        if !first {
                            buf.write_char(',').unwrap();
                        }
                        first = false;
                        match item {
                            Some(array) => write_array_recursive(encoder, buf, array),
                            None => buf.write_str("NULL").unwrap(),
                        }
                    }
                }
                #[cfg(feature = "with-json")]
                Array::Json(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_json(buf, val))
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoDate(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_naive_date(buf, val)
                }),
                #[cfg(feature = "with-chrono")]
                Array::ChronoTime(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_naive_time(buf, val)
                }),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTime(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_naive_datetime(buf, val)
                }),
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeUtc(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| {
                        qb.write_datetime_utc(buf, val)
                    })
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeLocal(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| {
                        qb.write_datetime_local(buf, val)
                    })
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeWithTimeZone(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| {
                        qb.write_datetime_fixed(buf, val)
                    })
                }
                #[cfg(feature = "with-time")]
                Array::TimeDate(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_time_date(buf, val)
                }),
                #[cfg(feature = "with-time")]
                Array::TimeTime(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_time_time(buf, val)
                }),
                #[cfg(feature = "with-time")]
                Array::TimeDateTime(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_time_datetime(buf, val)
                }),
                #[cfg(feature = "with-time")]
                Array::TimeDateTimeWithTimeZone(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| {
                        qb.write_time_datetime_tz(buf, val)
                    })
                }
                #[cfg(feature = "with-jiff")]
                Array::JiffDate(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_jiff_date(buf, val)
                }),
                #[cfg(feature = "with-jiff")]
                Array::JiffTime(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_jiff_time(buf, val)
                }),
                #[cfg(feature = "with-jiff")]
                Array::JiffDateTime(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_jiff_datetime(buf, val)
                }),
                #[cfg(feature = "with-jiff")]
                Array::JiffTimestamp(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_jiff_timestamp(buf, val)
                }),
                #[cfg(feature = "with-jiff")]
                Array::JiffZoned(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_jiff_zoned(buf, val)
                }),
                #[cfg(feature = "with-uuid")]
                Array::Uuid(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_uuid(buf, val))
                }
                #[cfg(feature = "with-rust_decimal")]
                Array::Decimal(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_decimal(buf, val)
                }),
                #[cfg(feature = "with-bigdecimal")]
                Array::BigDecimal(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_bigdecimal(buf, val)
                }),
                #[cfg(feature = "with-ipnetwork")]
                Array::IpNetwork(items) => write_items(encoder, buf, items, |qb, buf, val| {
                    qb.write_ipnetwork(buf, val)
                }),
                #[cfg(feature = "with-mac_address")]
                Array::MacAddress(items) => {
                    write_items(encoder, buf, items, |qb, buf, val| qb.write_mac(buf, val))
                }
            }
            buf.write_str("]'").unwrap();
        }

        write_array_recursive(self, buf, array);
    }
}
