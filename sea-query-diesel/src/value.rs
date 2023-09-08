use std::marker::PhantomData;

use diesel::backend::Backend;
use diesel::query_builder::{AstPass, QueryFragment};
use diesel::serialize::ToSql;
use diesel::sql_types::HasSqlType;
use diesel::QueryResult;

pub struct SeaValue<ST, U> {
    value: U,
    p: PhantomData<ST>,
}

impl<ST, U> SeaValue<ST, U> {
    pub fn build<'f, DB>(value: U) -> Box<dyn QueryFragment<DB> + Send + 'f>
    where
        DB: Backend + HasSqlType<ST>,
        U: ToSql<ST, DB> + Send + 'f,
        ST: Send + 'f,
    {
        Box::new(Self {
            value,
            p: PhantomData,
        }) as Box<_>
    }
}

impl<ST, U, DB> QueryFragment<DB> for SeaValue<ST, U>
where
    DB: Backend + HasSqlType<ST>,
    U: ToSql<ST, DB>,
{
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        pass.push_bind_param_value_only(&self.value)
    }
}
