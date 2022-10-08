use rusqlite::{
    types::{Null, ToSqlOutput},
    Result, ToSql,
};
use sea_query::Value;
use sea_query::{query::*, QueryBuilder};

#[derive(Clone, Debug, PartialEq)]
pub struct RusqliteValue(pub sea_query::Value);
#[derive(Clone, Debug, PartialEq)]
pub struct RusqliteValues(pub Vec<RusqliteValue>);

impl<'a> RusqliteValues {
    pub fn as_params(&'a self) -> Vec<&'a dyn ToSql> {
        self.0
            .iter()
            .map(|x| {
                let y: &dyn ToSql = x;
                y
            })
            .collect()
    }
}

pub trait RusqliteBinder {
    fn build_rusqlite<T: QueryBuilder>(&self, query_builder: T) -> (String, RusqliteValues);
}

macro_rules! impl_rusqlite_binder {
    ($l:ident) => {
        impl RusqliteBinder for $l {
            fn build_rusqlite<T: QueryBuilder>(
                &self,
                query_builder: T,
            ) -> (String, RusqliteValues) {
                let (query, values) = self.build(query_builder);
                (
                    query,
                    RusqliteValues(values.into_iter().map(RusqliteValue).collect()),
                )
            }
        }
    };
}

impl_rusqlite_binder!(SelectStatement);
impl_rusqlite_binder!(UpdateStatement);
impl_rusqlite_binder!(InsertStatement);
impl_rusqlite_binder!(DeleteStatement);

impl ToSql for RusqliteValue {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}
