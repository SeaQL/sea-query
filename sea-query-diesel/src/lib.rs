use diesel::backend::Backend;
use diesel::result::QueryResult;
use sea_query::{DeleteStatement, InsertStatement, SelectStatement, UpdateStatement, WithQuery};

use self::backend::{ExtractBuilder, TransformValue};
pub use self::query::SeaQuery;

pub mod backend;
mod query;
mod value;

pub trait DieselBinder {
    fn build_diesel<DB: Backend + ExtractBuilder + TransformValue>(
        &self,
    ) -> QueryResult<SeaQuery<DB>>;
}

macro_rules! impl_diesel_binder {
    ($statement: ident) => {
        impl DieselBinder for $statement {
            fn build_diesel<DB: Backend + ExtractBuilder + TransformValue>(
                &self,
            ) -> QueryResult<SeaQuery<DB>> {
                let query_builder = DB::builder();
                let (query, values) = self.build(query_builder);
                let mut binds = Vec::with_capacity(values.0.len());
                for value in values {
                    binds.push(DB::transform_value(value)?);
                }
                Ok(SeaQuery::new(query, binds))
            }
        }
    };
}

impl_diesel_binder!(SelectStatement);
impl_diesel_binder!(UpdateStatement);
impl_diesel_binder!(InsertStatement);
impl_diesel_binder!(DeleteStatement);
impl_diesel_binder!(WithQuery);
