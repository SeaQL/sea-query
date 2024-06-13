pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// BigQuery query builder.
#[derive(Default, Debug)]
pub struct BigQueryQueryBuilder;

const QUOTE: Quote = Quote(b'`', b'`');

impl GenericBuilder for BigQueryQueryBuilder {}

impl SchemaBuilder for BigQueryQueryBuilder {}

impl QuotedBuilder for BigQueryQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for BigQueryQueryBuilder {}

impl TableRefBuilder for BigQueryQueryBuilder {}
