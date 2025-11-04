use std::fmt::Display;
#[cfg(feature = "hashable-value")]
use std::hash::{Hash, Hasher};

use super::*;

//type_to_value!(RangeType, Range, Range);

impl Default for RangeType {
    fn default() -> Self {
        Self::Int4Range(RangeBoundary::Exclusive(0), RangeBoundary::Exclusive(0))
    }
}

impl Display for RangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeType::Int4Range(a, b) => display_range(a, b, f),
            RangeType::Int8Range(a, b) => display_range(a, b, f),
            RangeType::NumRange(a, b) => display_range(a, b, f),
            #[cfg(feature = "with-chrono")]
            RangeType::ChronoDateTime(a, b) => display_range(a, b, f),
            #[cfg(feature = "with-chrono")]
            RangeType::ChronoDateTimeRange(a, b) => display_range(a, b, f),
            #[cfg(feature = "with-chrono")]
            RangeType::ChronoDateTimeWithTimeZoneRange(a, b) => display_range(a, b, f),
            #[cfg(feature = "with-chrono")]
            RangeType::ChronoDateRange(a, b) => display_range(a, b, f),
            #[cfg(feature = "with-time")]
            RangeType::TimeDateTimeRange(a, b) => display_range(a, b, f),
            #[cfg(feature = "with-time")]
            RangeType::TimeDateTimeWithTimeZoneRange(a, b) => display_range(a, b, f),
            #[cfg(feature = "with-time")]
            RangeType::TimeDateRange(a, b) => display_range(a, b, f),
        }
    }
}

fn display_range<T: Display>(
    a: &RangeBoundary<T>,
    b: &RangeBoundary<T>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    match a {
        RangeBoundary::Exclusive(v) => {
            f.write_fmt(format_args!("[{v}"))?;
        }
        RangeBoundary::Inclusive(v) => {
            f.write_fmt(format_args!("({v}"))?;
        }
    }

    f.write_str(",")?;

    match b {
        RangeBoundary::Exclusive(v) => {
            f.write_fmt(format_args!("{v}]"))?;
        }
        RangeBoundary::Inclusive(v) => {
            f.write_fmt(format_args!("{v})"))?;
        }
    }

    Ok(())
}

#[cfg(feature = "hashable-value")]
impl Hash for RangeType {
    fn hash<H: Hasher>(&self, state: &mut H) {
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
                hash_range_boundary(a, state);
                hash_range_boundary(b, state);
            }
            #[cfg(feature = "with-chrono")]
            RangeType::ChronoDateTime(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            #[cfg(feature = "with-chrono")]
            RangeType::ChronoDateTimeRange(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            #[cfg(feature = "with-chrono")]
            RangeType::ChronoDateTimeWithTimeZoneRange(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            #[cfg(feature = "with-chrono")]
            RangeType::ChronoDateRange(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            #[cfg(feature = "with-time")]
            RangeType::TimeDateTimeRange(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            #[cfg(feature = "with-time")]
            RangeType::TimeDateTimeWithTimeZoneRange(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            #[cfg(feature = "with-time")]
            RangeType::TimeDateRange(a, b) => {
                a.hash(state);
                b.hash(state);
            }
        }
    }
}

#[cfg(feature = "hashable-value")]
fn hash_range_boundary<H: Hasher>(rb: &RangeBoundary<f64>, state: &mut H) {
    match rb {
        RangeBoundary::Exclusive(v) => ordered_float::OrderedFloat(*v).hash(state),
        RangeBoundary::Inclusive(v) => ordered_float::OrderedFloat(*v).hash(state),
    }
}
