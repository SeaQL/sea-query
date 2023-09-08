use diesel::backend::Backend;
use diesel::connection::Connection;
use diesel::query_builder::{AstPass, Query, QueryFragment, QueryId};
use diesel::query_dsl::RunQueryDsl;
use diesel::result::QueryResult;
use diesel::sql_types::Untyped;

#[must_use = "Queries are only executed when calling `load`, `get_result`, or similar."]
pub struct SeaQuery<DB: Backend> {
    query: String,
    binds: Vec<Box<dyn QueryFragment<DB> + Send>>,
}

impl<DB: Backend> SeaQuery<DB> {
    pub(crate) fn new(query: String, binds: Vec<Box<dyn QueryFragment<DB> + Send>>) -> Self {
        SeaQuery { query, binds }
    }
}

impl<DB> QueryFragment<DB> for SeaQuery<DB>
where
    DB: Backend,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        out.unsafe_to_cache_prepared();
        out.push_sql(&self.query);

        for b in &self.binds {
            b.walk_ast(out.reborrow())?;
        }

        Ok(())
    }
}

impl<DB: Backend> QueryId for SeaQuery<DB> {
    type QueryId = ();

    const HAS_STATIC_QUERY_ID: bool = false;
}

impl<DB> Query for SeaQuery<DB>
where
    DB: Backend,
{
    type SqlType = Untyped;
}

impl<Conn: Connection> RunQueryDsl<Conn> for SeaQuery<Conn::Backend> {}
