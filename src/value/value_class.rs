pub trait NumericValue: Sized {}
pub trait NumericValueNullable: Sized {}
impl<T: NumericValue> NumericValueNullable for T {}

pub trait DateLikeValue: Sized {}
pub trait DateLikeValueNullable: Sized {}
impl<T: DateLikeValue> DateLikeValueNullable for T {}

pub trait TimeLikeValue: Sized {}
pub trait TimeLikeValueNullable: Sized {}
impl<T: TimeLikeValue> TimeLikeValueNullable for T {}

pub trait DateTimeLikeValue: Sized {}
pub trait DateTimeLikeValueNullable: Sized {}
impl<T: DateTimeLikeValue> DateTimeLikeValueNullable for T {}

impl NumericValue for i8 {}
impl NumericValue for i16 {}
impl NumericValue for i32 {}
impl NumericValue for i64 {}
impl NumericValue for u8 {}
impl NumericValue for u16 {}
impl NumericValue for u32 {}
impl NumericValue for u64 {}
impl NumericValue for f32 {}
impl NumericValue for f64 {}

impl NumericValueNullable for Option<i8> {}
impl NumericValueNullable for Option<i16> {}
impl NumericValueNullable for Option<i32> {}
impl NumericValueNullable for Option<i64> {}
impl NumericValueNullable for Option<u8> {}
impl NumericValueNullable for Option<u16> {}
impl NumericValueNullable for Option<u32> {}
impl NumericValueNullable for Option<u64> {}
impl NumericValueNullable for Option<f32> {}
impl NumericValueNullable for Option<f64> {}
