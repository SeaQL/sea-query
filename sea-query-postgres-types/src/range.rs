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
    /// Returns `true` if PostgreSQL stores this range as `empty`.
    ///
    /// This follows PostgreSQL's rules: `(,)` (both bounds unbounded) is the range of all
    /// values, not an empty range, while e.g. `[1,1)`, or `(1,2)` on an integer range, is empty.
    /// Ranges whose lower bound is greater than their upper bound are rejected by PostgreSQL
    /// and are not considered empty.
    pub fn empty(&self) -> bool {
        match self {
            RangeType::Int4Range(a, b) => discrete_range_is_empty(a, b),
            RangeType::Int8Range(a, b) => discrete_range_is_empty(a, b),
            RangeType::NumRange(a, b) => match (bound_value(a), bound_value(b)) {
                (Some((lower, lower_inc)), Some((upper, upper_inc))) => {
                    lower == upper && !(lower_inc && upper_inc)
                }
                _ => false,
            },
        }
    }
}

fn bound_value<T: Clone + Display>(bound: &RangeBound<T>) -> Option<(&T, bool)> {
    match bound {
        RangeBound::Inclusive(v) => Some((v, true)),
        RangeBound::Exclusive(v) => Some((v, false)),
        RangeBound::Unbounded => None,
    }
}

fn discrete_range_is_empty<T: Clone + Copy + Display + Into<i64>>(
    lower: &RangeBound<T>,
    upper: &RangeBound<T>,
) -> bool {
    let (Some((&lower, lower_inc)), Some((&upper, upper_inc))) =
        (bound_value(lower), bound_value(upper))
    else {
        return false;
    };
    let (lower, upper) = (i128::from(lower.into()), i128::from(upper.into()));
    if lower > upper {
        return false;
    }
    // canonical `[lower, upper)` form, as PostgreSQL does for discrete ranges
    let lower = if lower_inc { lower } else { lower + 1 };
    let upper = if upper_inc { upper + 1 } else { upper };
    lower >= upper
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
    // `[` / `]` mark inclusive bounds, `(` / `)` exclusive or unbounded ones
    match a {
        RangeBound::Inclusive(v) => {
            f.write_fmt(format_args!("[{v},"))?;
        }
        RangeBound::Exclusive(v) => {
            f.write_fmt(format_args!("({v},"))?;
        }
        RangeBound::Unbounded => {
            f.write_str("(,")?;
        }
    }

    match b {
        RangeBound::Inclusive(v) => {
            f.write_fmt(format_args!("{v}]"))?;
        }
        RangeBound::Exclusive(v) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_uses_postgres_bound_syntax() {
        assert_eq!(
            RangeType::Int4Range(RangeBound::Inclusive(1), RangeBound::Exclusive(10)).to_string(),
            "[1,10)"
        );
        assert_eq!(
            RangeType::Int8Range(RangeBound::Exclusive(1), RangeBound::Inclusive(10)).to_string(),
            "(1,10]"
        );
        assert_eq!(
            RangeType::NumRange(RangeBound::Inclusive(0.5), RangeBound::Unbounded).to_string(),
            "[0.5,)"
        );
        assert_eq!(
            RangeType::Int4Range(RangeBound::Unbounded, RangeBound::Inclusive(3)).to_string(),
            "(,3]"
        );
        assert_eq!(
            RangeType::Int8Range(RangeBound::Unbounded, RangeBound::Unbounded).to_string(),
            "(,)"
        );
    }

    #[test]
    fn empty_follows_postgres_semantics() {
        use RangeBound::*;

        // `(,)` is the range of all values
        assert!(!RangeType::Int4Range(Unbounded, Unbounded).empty());
        assert!(!RangeType::Int8Range(Unbounded, Unbounded).empty());
        assert!(!RangeType::NumRange(Unbounded, Unbounded).empty());
        assert!(!RangeType::default().empty());

        assert!(!RangeType::Int4Range(Inclusive(1), Inclusive(1)).empty());
        assert!(!RangeType::Int4Range(Inclusive(1), Exclusive(2)).empty());
        assert!(!RangeType::Int4Range(Unbounded, Exclusive(1)).empty());
        assert!(RangeType::Int4Range(Inclusive(1), Exclusive(1)).empty());
        assert!(RangeType::Int4Range(Exclusive(1), Exclusive(2)).empty());
        assert!(RangeType::Int8Range(Exclusive(1), Inclusive(1)).empty());
        assert!(!RangeType::Int8Range(Exclusive(i64::MAX - 1), Inclusive(i64::MAX)).empty());

        assert!(!RangeType::NumRange(Inclusive(0.5), Inclusive(0.5)).empty());
        assert!(!RangeType::NumRange(Exclusive(0.5), Exclusive(0.6)).empty());
        assert!(RangeType::NumRange(Inclusive(0.5), Exclusive(0.5)).empty());

        // lower > upper is an error in PostgreSQL, not an empty range
        assert!(!RangeType::Int4Range(Inclusive(5), Exclusive(1)).empty());
    }
}
