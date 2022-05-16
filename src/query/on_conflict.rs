use crate::{DynIden, Expr, IntoIden, SimpleExpr, Value};

#[derive(Debug, Clone, Default)]
pub struct OnConflict {
    pub(crate) target: Option<OnConflictTarget>,
    pub(crate) action: Option<OnConflictAction>,
}

/// Represents ON CONFLICT (upsert) targets
#[derive(Debug, Clone)]
pub enum OnConflictTarget {
    /// A list of columns with unique constraint
    ConflictColumns(Vec<DynIden>),
}

/// Represents ON CONFLICT (upsert) actions
#[derive(Debug, Clone)]
pub enum OnConflictAction {
    /// Do nothing
    DoNothing,
    /// Update column value of existing row with inserting value
    UpdateColumns(Vec<DynIden>),
    /// Update column value of existing row with expression
    UpdateExprs(Vec<(DynIden, SimpleExpr)>),
}

impl OnConflict {
    /// Create a ON CONFLICT expression without target column,
    /// a special method designed for MySQL
    pub fn new() -> Self {
        Default::default()
    }

    /// Set ON CONFLICT target column
    pub fn column<C>(column: C) -> Self
    where
        C: IntoIden,
    {
        Self::columns(vec![column])
    }

    /// Set ON CONFLICT target columns
    pub fn columns<I, C>(columns: I) -> Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        Self {
            target: Some(OnConflictTarget::ConflictColumns(
                columns.into_iter().map(IntoIden::into_iden).collect(),
            )),
            action: None,
        }
    }

    pub fn do_nothing(&mut self) -> &mut Self {
        self.action = Some(OnConflictAction::DoNothing);
        self
    }

    /// Set ON CONFLICT update column
    pub fn update_column<C>(&mut self, column: C) -> &mut Self
    where
        C: IntoIden,
    {
        self.update_columns(vec![column])
    }

    /// Set ON CONFLICT update columns
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic(vec![
    ///         2.into(),
    ///         3.into(),
    ///     ])
    ///     .on_conflict(
    ///         OnConflict::column(Glyph::Id)
    ///             .update_columns([Glyph::Aspect, Glyph::Image])
    ///             .to_owned(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, 3) ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`), `image` = VALUES(`image`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#
    /// );
    /// ```
    pub fn update_columns<C, I>(&mut self, columns: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        self.action = Some(OnConflictAction::UpdateColumns(
            columns.into_iter().map(IntoIden::into_iden).collect(),
        ));
        self
    }

    /// Set ON CONFLICT update value
    pub fn update_value<C>(&mut self, column_value: (C, Value)) -> &mut Self
    where
        C: IntoIden,
    {
        self.update_values(vec![column_value])
    }

    /// Set ON CONFLICT update values
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic(vec![
    ///         2.into(),
    ///         3.into(),
    ///     ])
    ///     .on_conflict(
    ///         OnConflict::column(Glyph::Id)
    ///             .update_values([
    ///                 (Glyph::Aspect, "04108048005887010020060000204E0180400400".into()),
    ///                 (Glyph::Image, 3.1415.into()),
    ///             ])
    ///             .to_owned()
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, 3) ON DUPLICATE KEY UPDATE `aspect` = '04108048005887010020060000204E0180400400', `image` = 3.1415"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "aspect" = '04108048005887010020060000204E0180400400', "image" = 3.1415"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "aspect" = '04108048005887010020060000204E0180400400', "image" = 3.1415"#
    /// );
    /// ```
    pub fn update_values<C, I>(&mut self, column_values: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = (C, Value)>,
    {
        self.action = Some(OnConflictAction::UpdateExprs(
            column_values
                .into_iter()
                .map(|(c, v)| (c.into_iden(), Expr::val(v).into()))
                .collect(),
        ));
        self
    }

    /// Set ON CONFLICT update expr
    pub fn update_expr<C>(&mut self, column_expr: (C, SimpleExpr)) -> &mut Self
    where
        C: IntoIden,
    {
        self.update_exprs(vec![column_expr])
    }

    /// Set ON CONFLICT update exprs
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic(vec![
    ///         2.into(),
    ///         3.into(),
    ///     ])
    ///     .on_conflict(
    ///         OnConflict::column(Glyph::Id)
    ///             .update_expr((Glyph::Image, Expr::val(1).add(2)))
    ///             .to_owned()
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, 3) ON DUPLICATE KEY UPDATE `image` = 1 + 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "image" = 1 + 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "image" = 1 + 2"#
    /// );
    /// ```
    pub fn update_exprs<C, I>(&mut self, column_exprs: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = (C, SimpleExpr)>,
    {
        self.action = Some(OnConflictAction::UpdateExprs(
            column_exprs
                .into_iter()
                .map(|(c, e)| (c.into_iden(), e))
                .collect(),
        ));
        self
    }
}
