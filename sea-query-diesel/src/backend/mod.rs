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

mod macros {
    macro_rules! err {
        ($msg: tt) => {
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

    pub(crate) use {bail, build, err};
}
