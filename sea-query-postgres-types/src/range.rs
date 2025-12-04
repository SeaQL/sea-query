#[cfg(feature = "postgres-driver")]
mod postgres_driver;

use std::fmt::{Debug, Display};
#[cfg(feature = "hashable-value")]
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RangeBound<T: Clone + Display> {
    Exclusive(T),
    Inclusive(T),
    Unbounded,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

fn display_range<T: Clone + Display>(
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

#[cfg(feature = "hashable-value")]
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

#[cfg(feature = "hashable-value")]
fn hash_range_bound<H: Hasher>(rb: &RangeBound<f64>, state: &mut H) {
    match rb {
        RangeBound::Exclusive(v) => ordered_float::OrderedFloat(*v).hash(state),
        RangeBound::Inclusive(v) => ordered_float::OrderedFloat(*v).hash(state),
        RangeBound::Unbounded => (),
    }
}
