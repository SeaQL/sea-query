pub trait NumericValue: Sized {}

pub trait DateLikeValue: Sized {}

pub trait TimeLikeValue: Sized {}

pub trait DateTimeLikeValue: Sized {}

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
