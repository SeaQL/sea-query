use crate::{prepare::write_int, *};
use std::fmt::Write;

#[allow(unused_variables)]
pub trait ValueEncoder: EscapeBuilder {
    fn write_bool_to(&self, buf: &mut impl Write, value: bool) {
        buf.write_str(if value { "TRUE" } else { "FALSE" }).unwrap();
    }

    fn write_i8_to(&self, buf: &mut impl Write, value: i8) {
        write_int(buf, value);
    }

    fn write_i16_to(&self, buf: &mut impl Write, value: i16) {
        write_int(buf, value);
    }

    fn write_i32_to(&self, buf: &mut impl Write, value: i32) {
        write_int(buf, value);
    }

    fn write_i64_to(&self, buf: &mut impl Write, value: i64) {
        write_int(buf, value);
    }

    fn write_u8_to(&self, buf: &mut impl Write, value: u8) {
        write_int(buf, value);
    }

    fn write_u16_to(&self, buf: &mut impl Write, value: u16) {
        write_int(buf, value);
    }

    fn write_u32_to(&self, buf: &mut impl Write, value: u32) {
        write_int(buf, value);
    }

    fn write_u64_to(&self, buf: &mut impl Write, value: u64) {
        write_int(buf, value);
    }

    fn write_f32_to(&self, buf: &mut impl Write, value: f32) {
        write!(buf, "{value}").unwrap();
    }

    fn write_f64_to(&self, buf: &mut impl Write, value: f64) {
        write!(buf, "{value}").unwrap();
    }

    fn write_str_to(&self, buf: &mut impl Write, value: &str) {
        buf.write_str("'").unwrap();
        self.write_escaped(buf, value);
        buf.write_str("'").unwrap();
    }

    fn write_char_to(&self, buf: &mut impl Write, value: char) {
        let mut tmp = [0u8; 4];
        let s = value.encode_utf8(&mut tmp);
        self.write_str_to(buf, s);
    }

    fn write_bytes_to(&self, buf: &mut impl Write, value: &[u8]) {
        buf.write_str("x'").unwrap();
        for b in value {
            write!(buf, "{b:02X}").unwrap()
        }
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-json")]
    fn write_json_to(&self, buf: &mut impl Write, value: &serde_json::Value) {
        self.write_str_to(buf, &value.to_string());
    }

    #[cfg(feature = "with-chrono")]
    fn write_naive_date_to(&self, buf: &mut impl Write, value: chrono::NaiveDate) {
        buf.write_str("'").unwrap();
        value.format("%Y-%m-%d").write_to(buf).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_naive_time_to(&self, buf: &mut impl Write, value: chrono::NaiveTime) {
        buf.write_str("'").unwrap();
        value.format("%H:%M:%S%.6f").write_to(buf).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_naive_datetime_to(&self, buf: &mut impl Write, value: chrono::NaiveDateTime) {
        buf.write_str("'").unwrap();
        value.format("%Y-%m-%d %H:%M:%S%.6f").write_to(buf).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_datetime_utc_to(&self, buf: &mut impl Write, value: &chrono::DateTime<chrono::Utc>) {
        buf.write_str("'").unwrap();
        value
            .format("%Y-%m-%d %H:%M:%S%.6f %:z")
            .write_to(buf)
            .unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_datetime_local_to(
        &self,
        buf: &mut impl Write,
        value: &chrono::DateTime<chrono::Local>,
    ) {
        buf.write_str("'").unwrap();
        value
            .format("%Y-%m-%d %H:%M:%S%.6f %:z")
            .write_to(buf)
            .unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-chrono")]
    fn write_datetime_fixed_to(
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
    fn write_time_date_to(&self, buf: &mut impl Write, value: time::Date) {
        buf.write_str("'").unwrap();
        let s = value
            .format(crate::value::time_format::FORMAT_DATE)
            .unwrap();
        buf.write_str(&s).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-time")]
    fn write_time_time_to(&self, buf: &mut impl Write, value: time::Time) {
        buf.write_str("'").unwrap();
        let s = value
            .format(crate::value::time_format::FORMAT_TIME)
            .unwrap();
        buf.write_str(&s).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-time")]
    fn write_time_datetime_to(&self, buf: &mut impl Write, value: time::PrimitiveDateTime) {
        buf.write_str("'").unwrap();
        let s = value
            .format(crate::value::time_format::FORMAT_DATETIME)
            .unwrap();
        buf.write_str(&s).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-time")]
    fn write_time_datetime_tz_to(&self, buf: &mut impl Write, value: time::OffsetDateTime) {
        buf.write_str("'").unwrap();
        let s = value
            .format(crate::value::time_format::FORMAT_DATETIME_TZ)
            .unwrap();
        buf.write_str(&s).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_date_to(&self, buf: &mut impl Write, value: jiff::civil::Date) {
        buf.write_str("'").unwrap();
        write!(buf, "{value}").unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_time_to(&self, buf: &mut impl Write, value: jiff::civil::Time) {
        buf.write_str("'").unwrap();
        write!(buf, "{value}").unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_datetime_to(&self, buf: &mut impl Write, value: jiff::civil::DateTime) {
        use crate::value::with_jiff::JIFF_DATE_TIME_FMT_STR;
        buf.write_str("'").unwrap();
        write!(buf, "{}", value.strftime(JIFF_DATE_TIME_FMT_STR)).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_timestamp_to(&self, buf: &mut impl Write, value: jiff::Timestamp) {
        use crate::value::with_jiff::JIFF_TIMESTAMP_FMT_STR;
        buf.write_str("'").unwrap();
        write!(buf, "{}", value.strftime(JIFF_TIMESTAMP_FMT_STR)).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-jiff")]
    fn write_jiff_zoned_to(&self, buf: &mut impl Write, value: &jiff::Zoned) {
        use crate::value::with_jiff::JIFF_ZONE_FMT_STR;
        buf.write_str("'").unwrap();
        write!(buf, "{}", value.strftime(JIFF_ZONE_FMT_STR)).unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "postgres-vector")]
    fn write_vector_to(&self, buf: &mut impl Write, value: &pgvector::Vector) {
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
    fn write_decimal_to(&self, buf: &mut impl Write, value: rust_decimal::Decimal) {
        write!(buf, "{value}").unwrap();
    }

    #[cfg(feature = "with-bigdecimal")]
    fn write_bigdecimal_to(&self, buf: &mut impl Write, value: &bigdecimal::BigDecimal) {
        write!(buf, "{value}").unwrap();
    }

    #[cfg(feature = "with-uuid")]
    fn write_uuid_to(&self, buf: &mut impl Write, value: uuid::Uuid) {
        buf.write_str("'").unwrap();
        write!(buf, "{value}").unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-ipnetwork")]
    fn write_ipnetwork_to(&self, buf: &mut impl Write, value: ipnetwork::IpNetwork) {
        buf.write_str("'").unwrap();
        write!(buf, "{value}").unwrap();
        buf.write_str("'").unwrap();
    }

    #[cfg(feature = "with-mac_address")]
    fn write_mac_to(&self, buf: &mut impl Write, value: mac_address::MacAddress) {
        buf.write_str("'").unwrap();
        write!(buf, "{value}").unwrap();
        buf.write_str("'").unwrap();
    }

    fn write_enum_to(&self, buf: &mut impl Write, value: &crate::value::Enum) {
        self.write_str_to(buf, value.value.as_str());
    }

    #[cfg(feature = "postgres-array")]
    fn write_array_to(&self, buf: &mut impl Write, array: &crate::value::Array) {
        use std::fmt;

        use crate::value::Array;

        fn write_array_values<VE, T, F, W>(
            encoder: &VE,
            buf: &mut W,
            items: &[Option<T>],
            mut f: F,
        ) -> fmt::Result
        where
            VE: ValueEncoder + ?Sized,
            W: Write,
            F: FnMut(&VE, &mut W, &T),
        {
            use crate::utils::join_write;

            join_write(
                buf,
                items,
                |buf| buf.write_char(','),
                |buf, item| {
                    match item.as_ref() {
                        Some(val) => f(encoder, buf, val),
                        None => buf.write_str("NULL")?,
                    }

                    Ok(())
                },
            )
        }

        fn write_array_recursive<VE, W>(encoder: &VE, buf: &mut W, array: &Array) -> fmt::Result
        where
            VE: ValueEncoder + ?Sized,
            W: Write,
        {
            if array.is_empty() {
                return buf.write_str("'{}'");
            }

            buf.write_str("ARRAY[")?;
            match array {
                Array::Bool(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_bool_to(buf, *val)
                    })
                }
                Array::TinyInt(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_i8_to(buf, *val)
                    })
                }
                Array::SmallInt(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_i16_to(buf, *val)
                    })
                }
                Array::Int(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_i32_to(buf, *val)
                    })
                }
                Array::BigInt(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_i64_to(buf, *val)
                    })
                }
                Array::TinyUnsigned(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_u8_to(buf, *val)
                    })
                }
                Array::SmallUnsigned(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_u16_to(buf, *val)
                    })
                }
                Array::Unsigned(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_u32_to(buf, *val)
                    })
                }
                Array::BigUnsigned(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_u64_to(buf, *val)
                    })
                }
                Array::Float(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_f32_to(buf, *val)
                    })
                }
                Array::Double(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_f64_to(buf, *val)
                    })
                }
                Array::String(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_str_to(buf, val)
                    })
                }
                Array::Char(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_char_to(buf, *val)
                    })
                }
                Array::Bytes(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_bytes_to(buf, val)
                    })
                }
                Array::Enum(boxed) => {
                    write_array_values(encoder, buf, &boxed.as_ref().1, |encoder, buf, val| {
                        encoder.write_enum_to(buf, val.as_ref())
                    })
                }
                #[cfg(feature = "with-json")]
                Array::Json(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_json_to(buf, val)
                    })
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoDate(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_naive_date_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoTime(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_naive_time_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTime(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_naive_datetime_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeUtc(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_datetime_utc_to(buf, val)
                    })
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeLocal(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_datetime_local_to(buf, val)
                    })
                }
                #[cfg(feature = "with-chrono")]
                Array::ChronoDateTimeWithTimeZone(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_datetime_fixed_to(buf, val)
                    })
                }
                #[cfg(feature = "with-time")]
                Array::TimeDate(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_time_date_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-time")]
                Array::TimeTime(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_time_time_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-time")]
                Array::TimeDateTime(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_time_datetime_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-time")]
                Array::TimeDateTimeWithTimeZone(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_time_datetime_tz_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-jiff")]
                Array::JiffDate(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_jiff_date_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-jiff")]
                Array::JiffTime(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_jiff_time_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-jiff")]
                Array::JiffDateTime(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_jiff_datetime_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-jiff")]
                Array::JiffTimestamp(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_jiff_timestamp_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-jiff")]
                Array::JiffZoned(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_jiff_zoned_to(buf, val)
                    })
                }
                #[cfg(feature = "with-uuid")]
                Array::Uuid(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_uuid_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-rust_decimal")]
                Array::Decimal(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_decimal_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-bigdecimal")]
                Array::BigDecimal(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_bigdecimal_to(buf, val)
                    })
                }
                #[cfg(feature = "with-ipnetwork")]
                Array::IpNetwork(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_ipnetwork_to(buf, *val)
                    })
                }
                #[cfg(feature = "with-mac_address")]
                Array::MacAddress(items) => {
                    write_array_values(encoder, buf, items, |encoder, buf, val| {
                        encoder.write_mac_to(buf, *val)
                    })
                }
            }
            .unwrap();
            buf.write_str("]")
        }

        write_array_recursive(self, buf, array).unwrap()
    }
}
