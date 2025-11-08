//#[cfg(feature = "hashable-value")]
use std::hash::{Hash, Hasher};
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use bytes::BytesMut;
use postgres_protocol::types;
use postgres_types::{IsNull, Kind, ToSql, Type, to_sql_checked};

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
//#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RangeBound<T: Clone + Display + ToSql> {
    Exclusive(T),
    Inclusive(T),
    Unbounded,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
//#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RangeType {
    Int4Range(RangeBound<i32>, RangeBound<i32>),
    Int8Range(RangeBound<i64>, RangeBound<i64>),
    NumRange(RangeBound<f64>, RangeBound<f64>),
}

impl RangeType {
    pub fn empty(&self) -> bool {
        matches!(
            self,
            &RangeType::Int4Range(RangeBound::Unbounded, RangeBound::Unbounded)
                | &RangeType::Int8Range(RangeBound::Unbounded, RangeBound::Unbounded)
                | &RangeType::NumRange(RangeBound::Unbounded, RangeBound::Unbounded)
        )
    }
}

impl Default for RangeType {
    fn default() -> Self {
        Self::Int4Range(RangeBound::Unbounded, RangeBound::Unbounded)
    }
}

impl Display for RangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeType::Int4Range(a, b) => display_range(a, b, f),
            RangeType::Int8Range(a, b) => display_range(a, b, f),
            RangeType::NumRange(a, b) => display_range(a, b, f),
        }
    }
}

fn display_range<T: Clone + Display + ToSql>(
    a: &RangeBound<T>,
    b: &RangeBound<T>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    match a {
        RangeBound::Exclusive(v) => {
            f.write_fmt(format_args!("[{v},"))?;
        }
        RangeBound::Inclusive(v) => {
            f.write_fmt(format_args!("({v},"))?;
        }
        RangeBound::Unbounded => {
            f.write_str("(,")?;
        }
    }

    match b {
        RangeBound::Exclusive(v) => {
            f.write_fmt(format_args!("{v}]"))?;
        }
        RangeBound::Inclusive(v) => {
            f.write_fmt(format_args!("{v})"))?;
        }
        RangeBound::Unbounded => {
            f.write_str(")")?;
        }
    }

    Ok(())
}

// TODO even if I put Hash impl behind feature gate, compilation fails
//#[cfg(feature = "hashable-value")]
impl Hash for RangeType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            RangeType::Int4Range(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            RangeType::Int8Range(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            RangeType::NumRange(a, b) => {
                hash_range_bound(a, state);
                hash_range_bound(b, state);
            }
        }
    }
}

//#[cfg(feature = "hashable-value")]
fn hash_range_bound<H: Hasher>(rb: &RangeBound<f64>, state: &mut H) {
    match rb {
        RangeBound::Exclusive(v) => ordered_float::OrderedFloat(*v).hash(state),
        RangeBound::Inclusive(v) => ordered_float::OrderedFloat(*v).hash(state),
        RangeBound::Unbounded => (),
    }
}

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
) -> Result<types::RangeBound<postgres_protocol::IsNull>, Box<dyn Error + Sync + Send>>
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
