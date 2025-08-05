use super::*;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hashable-value", derive(Hash, Eq))]
#[non_exhaustive]
pub enum ValueTuple {
    One(Value),
    Two(Value, Value),
    Three(Value, Value, Value),
    Many(Vec<Value>),
}

pub trait IntoValueTuple {
    fn into_value_tuple(self) -> ValueTuple;
}

pub trait FromValueTuple: Sized {
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple;
}

impl ValueTuple {
    pub fn arity(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Two(_, _) => 2,
            Self::Three(_, _, _) => 3,
            Self::Many(vec) => vec.len(),
        }
    }
}

impl IntoIterator for ValueTuple {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ValueTuple::One(v) => vec![v].into_iter(),
            ValueTuple::Two(v, w) => vec![v, w].into_iter(),
            ValueTuple::Three(u, v, w) => vec![u, v, w].into_iter(),
            ValueTuple::Many(vec) => vec.into_iter(),
        }
    }
}

impl IntoValueTuple for ValueTuple {
    fn into_value_tuple(self) -> ValueTuple {
        self
    }
}

impl<V> IntoValueTuple for V
where
    V: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::One(self.into())
    }
}

impl<V, W> IntoValueTuple for (V, W)
where
    V: Into<Value>,
    W: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Two(self.0.into(), self.1.into())
    }
}

impl<U, V, W> IntoValueTuple for (U, V, W)
where
    U: Into<Value>,
    V: Into<Value>,
    W: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Three(self.0.into(), self.1.into(), self.2.into())
    }
}

impl<V> FromValueTuple for V
where
    V: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::One(u) => u.unwrap(),
            _ => panic!("not ValueTuple::One"),
        }
    }
}

impl<V, W> FromValueTuple for (V, W)
where
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Two(v, w) => (v.unwrap(), w.unwrap()),
            _ => panic!("not ValueTuple::Two"),
        }
    }
}

impl<U, V, W> FromValueTuple for (U, V, W)
where
    U: Into<Value> + ValueType,
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Three(u, v, w) => (u.unwrap(), v.unwrap(), w.unwrap()),
            _ => panic!("not ValueTuple::Three"),
        }
    }
}

macro_rules! impl_from_value_tuple {
    ( $len:expr, $($T:ident),+ $(,)? ) => {
        impl< $($T),+ > FromValueTuple for ( $($T),+ )
        where
            $($T: Into<Value> + ValueType),+
        {
            fn from_value_tuple<Z>(i: Z) -> Self
            where
                Z: IntoValueTuple,
            {
                match i.into_value_tuple() {
                    ValueTuple::Many(vec) if vec.len() == $len => {
                        let mut iter = vec.into_iter();
                        (
                            $(<$T as ValueType>::unwrap(iter.next().unwrap())),+
                        )
                    }
                    _ => panic!("not ValueTuple::Many with length of {}", $len),
                }
            }
        }
    };
}

impl_from_value_tuple!(4, T0, T1, T2, T3);
impl_from_value_tuple!(5, T0, T1, T2, T3, T4);
impl_from_value_tuple!(6, T0, T1, T2, T3, T4, T5);
impl_from_value_tuple!(7, T0, T1, T2, T3, T4, T5, T6);
impl_from_value_tuple!(8, T0, T1, T2, T3, T4, T5, T6, T7);
impl_from_value_tuple!(9, T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_from_value_tuple!(10, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_from_value_tuple!(11, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_from_value_tuple!(12, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

macro_rules! impl_into_value_tuple {
    ( $($idx:tt : $T:ident),+ $(,)? ) => {
        impl< $($T),+ > IntoValueTuple for ( $($T),+ )
        where
            $($T: Into<Value>),+
        {
            fn into_value_tuple(self) -> ValueTuple {
                ValueTuple::Many(vec![
                    $(self.$idx.into()),+
                ])
            }
        }
    };
}

impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3);
impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4);
impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5);
impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6);
impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7);
impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7, 8:T8);
impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7, 8:T8, 9:T9);
impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7, 8:T8, 9:T9, 10:T10);
impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7, 8:T8, 9:T9, 10:T10, 11:T11);
