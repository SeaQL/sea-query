use std::{fmt::Display, fmt::Debug};

use bytes::BytesMut;
use postgres_types::{Kind, ToSql, to_sql_checked};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RangeBound<T: Clone + Default + Debug + Display> {
    Exclusive(T),
    Inclusive(T),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeType<T: Clone + Default + Debug + Display> {
    lower: RangeBound<T>,
    upper: RangeBound<T>,
}

impl<T: Clone + Default + Debug + Display> Default for RangeType<T> {
    fn default() -> Self {
        Self {
            lower: RangeBound::Inclusive(Default::default()),
            upper: RangeBound::Inclusive(Default::default()),
        }
    }
}

impl<T: Clone + Default + Debug + Display> Display for RangeType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.lower {
            RangeBound::Exclusive(a) => f.write_fmt(format_args!("[{},", a))?,
            RangeBound::Inclusive(a) => f.write_fmt(format_args!("({},", a))?,
        };

        match &self.upper {
            RangeBound::Exclusive(b) => f.write_fmt(format_args!("{}]", b))?,
            RangeBound::Inclusive(b) => f.write_fmt(format_args!("{})", b))?,
        };

        Ok(())
    }
}

//impl<T: Clone + Default + Debug + Display> From<&RangeType<T>> for PgRange<T> {
//    fn from(value: &RangeType<T>) -> Self {
//        PgRange {
//            start: (&value.lower).into(),
//            end: (&value.upper).into(),
//        }
//    }
//}
//
//impl<T: Clone + Default + Debug + Display> From<&RangeBound<T>> for Bound<T> {
//    fn from(value: &RangeBound<T>) -> Self {
//        match value {
//            RangeBound::Exclusive(v) => Bound::Excluded(v.clone()),
//            RangeBound::Inclusive(v) => Bound::Included(v.clone()),
//        }
//    }
//}

impl<T: Clone + Default + Debug + Display> ToSql for RangeType<T> {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        use postgres_protocol::IsNull;
        use postgres_protocol::types::RangeBound;
        use postgres_protocol::types::range_to_sql;

        if Self::accepts(ty) {
            range_to_sql(
                |_| { Ok(RangeBound::Exclusive(IsNull::No)) }, 
                |_| { Ok(RangeBound::Exclusive(IsNull::No)) },
                out
            )?;
        }

        Ok(postgres_types::IsNull::No)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        matches!(ty.kind(), Kind::Range(_))
    }

    to_sql_checked!();
}
