use super::*;
use bytes::BytesMut;
use postgres_protocol::types;
use postgres_types::{IsNull, Kind, ToSql, Type, to_sql_checked};

impl ToSql for RangeType {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        let element_type = match *ty.kind() {
            Kind::Range(ref ty) => ty,
            _ => return Err(format!("unexpected type {:?}", ty).into()),
        };

        if self.empty() {
            types::empty_range_to_sql(buf);
        } else {
            types::range_to_sql(
                |buf| match self {
                    RangeType::Int4Range(lower, _) => bound_to_sql(lower, element_type, buf),
                    RangeType::Int8Range(lower, _) => bound_to_sql(lower, element_type, buf),
                    RangeType::NumRange(lower, _) => bound_to_sql(lower, element_type, buf),
                },
                |buf| match self {
                    RangeType::Int4Range(_, upper) => bound_to_sql(upper, element_type, buf),
                    RangeType::Int8Range(_, upper) => bound_to_sql(upper, element_type, buf),
                    RangeType::NumRange(_, upper) => bound_to_sql(upper, element_type, buf),
                },
                buf,
            )?;
        }

        Ok(postgres_types::IsNull::No)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        matches!(ty.kind(), &Kind::Range(_))
    }

    to_sql_checked!();
}

fn bound_to_sql<T>(
    bound: &RangeBound<T>,
    ty: &Type,
    buf: &mut BytesMut,
) -> Result<types::RangeBound<postgres_protocol::IsNull>, Box<dyn std::error::Error + Sync + Send>>
where
    T: Clone + Display + ToSql,
{
    match bound {
        RangeBound::Exclusive(v) => {
            let is_null = match v.to_sql(ty, buf)? {
                IsNull::Yes => postgres_protocol::IsNull::Yes,
                IsNull::No => postgres_protocol::IsNull::No,
            };

            Ok(types::RangeBound::Exclusive(is_null))
        }
        RangeBound::Inclusive(v) => {
            let is_null = match v.to_sql(ty, buf)? {
                IsNull::Yes => postgres_protocol::IsNull::Yes,
                IsNull::No => postgres_protocol::IsNull::No,
            };

            Ok(types::RangeBound::Inclusive(is_null))
        }
        RangeBound::Unbounded => Ok(types::RangeBound::Unbounded),
    }
}
