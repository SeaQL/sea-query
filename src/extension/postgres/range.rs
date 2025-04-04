use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

#[cfg(feature = "with-chrono")]
use chrono::{DateTime, Utc};

impl<T> Display for PgRange<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.start {
            Bound::Unbounded => f.write_str("(,")?,
            Bound::Excluded(v) => write!(f, "({v},")?,
            Bound::Included(v) => write!(f, "[{v},")?,
        }

        match &self.end {
            Bound::Unbounded => f.write_str(")")?,
            Bound::Excluded(v) => write!(f, "{v})")?,
            Bound::Included(v) => write!(f, "{v}]")?,
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PgRange<T> {
    pub start: Bound<T>,
    pub end: Bound<T>,
}


impl<T> From<[Bound<T>; 2]> for PgRange<T> {
    fn from(v: [Bound<T>; 2]) -> Self {
        let [start, end] = v;
        Self { start, end }
    }
}

impl<T> From<(Bound<T>, Bound<T>)> for PgRange<T> {
    fn from(v: (Bound<T>, Bound<T>)) -> Self {
        Self {
            start: v.0,
            end: v.1,
        }
    }
}

impl<T> From<Range<T>> for PgRange<T> {
    fn from(v: Range<T>) -> Self {
        Self {
            start: Bound::Included(v.start),
            end: Bound::Excluded(v.end),
        }
    }
}

impl<T> From<RangeFrom<T>> for PgRange<T> {
    fn from(v: RangeFrom<T>) -> Self {
        Self {
            start: Bound::Included(v.start),
            end: Bound::Unbounded,
        }
    }
}

impl<T> From<RangeInclusive<T>> for PgRange<T> {
    fn from(v: RangeInclusive<T>) -> Self {
        let (start, end) = v.into_inner();
        Self {
            start: Bound::Included(start),
            end: Bound::Included(end),
        }
    }
}

impl<T> From<RangeTo<T>> for PgRange<T> {
    fn from(v: RangeTo<T>) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Excluded(v.end),
        }
    }
}

impl<T> From<RangeToInclusive<T>> for PgRange<T> {
    fn from(v: RangeToInclusive<T>) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Included(v.end),
        }
    }
}

impl<T> RangeBounds<T> for PgRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        match self.start {
            Bound::Included(ref start) => Bound::Included(start),
            Bound::Excluded(ref start) => Bound::Excluded(start),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&T> {
        match self.end {
            Bound::Included(ref end) => Bound::Included(end),
            Bound::Excluded(ref end) => Bound::Excluded(end),
            Bound::Unbounded => Bound::Unbounded,
        }
    }
}

impl PgRange<i32> {
    pub fn new(start: Bound<i32>, end: Bound<i32>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Option<i32> {
        match &self.start {
            Bound::Included(inner) | Bound::Excluded(inner) => Some(*inner),
            Bound::Unbounded => None,
        }
    }

    pub fn end(&self) -> Option<i32> {
        match &self.end {
            Bound::Included(inner) | Bound::Excluded(inner) => Some(*inner),
            Bound::Unbounded => None,
        }
    }
}

impl PgRange<DateTime<Utc>> {
    pub fn new(start: Bound<DateTime<Utc>>, end: Bound<DateTime<Utc>>) -> Self {
        Self { start, end }
    }

    pub fn from_datetime_pair(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self::new(Bound::Included(start), Bound::Excluded(end))
    }

    pub fn start(&self) -> Option<DateTime<Utc>> {
        match &self.start {
            Bound::Included(inner) | Bound::Excluded(inner) => Some(*inner),
            Bound::Unbounded => None,
        }
    }

    pub fn end(&self) -> Option<DateTime<Utc>> {
        match &self.end {
            Bound::Included(inner) | Bound::Excluded(inner) => Some(*inner),
            Bound::Unbounded => None,
        }
    }
}
