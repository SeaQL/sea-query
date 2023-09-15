use diesel::backend::Backend;
use diesel::query_builder::QueryFragment;
use diesel::result::QueryResult;
use sea_query::{QueryBuilder, Value};

#[cfg(feature = "mysql")]
mod mysql;

#[cfg(feature = "postgres")]
mod postgres;

#[cfg(feature = "sqlite")]
mod sqlite;

pub trait ExtractBuilder {
    type Builder: QueryBuilder;

    fn builder() -> Self::Builder;
}

pub trait TransformValue: Backend {
    fn transform_value(value: Value) -> QueryResult<Box<dyn QueryFragment<Self> + Send>>;
}

#[allow(unused_macros, unused_imports)]
mod macros {
    macro_rules! err {
        ($msg: expr) => {
            ::diesel::result::Error::SerializationError($msg.into())
        };
    }

    macro_rules! bail {
        ($msg: tt) => {
            return Err($crate::backend::macros::err!($msg))
        };
    }

    macro_rules! build {
        ($type: ty, $value: expr) => {
            $crate::value::SeaValue::<::diesel::sql_types::Nullable<$type>, _>::build($value)
        };
    }

    macro_rules! refine {
        ($target: ty, $source: expr, $value: expr) => {
            <Option<Vec<$target>> as ::sea_query::ValueType>::try_from(::sea_query::Value::Array(
                $source, $value,
            ))
            .map_err(|_| {
                err!(::std::concat!(
                    "This Value::Array should consist of ",
                    ::std::stringify!($target),
                    " values"
                ))
            })?
        };
    }

    pub(crate) use {bail, build, err, refine};
}
