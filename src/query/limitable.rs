/// Provides an interface for calling or using the `LIMIT` clause
/// on queries.
pub trait LimitableStatement {
    /// Set the limit on a query.
    fn limit(&mut self, limit: u64) -> &mut Self;

    /// Reset the limit on a query, setting the value to `None`.
    fn reset_limit(&mut self) -> &mut Self;
}