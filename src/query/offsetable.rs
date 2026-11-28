/// Provides an interface for utilizing the `OFFSET` clause on a query.
pub trait OffsetableStatement {
    /// Offset number of returned rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .limit(10)
    ///     .offset(10)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10 OFFSET 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10 OFFSET 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10 OFFSET 10"#
    /// );
    /// ```
    fn offset(&mut self, offset: u64) -> &mut Self;

    /// Reset the offset from `Some` quantity to `None`.
    fn reset_offset(&mut self) -> &mut Self;
}
