use inherent::inherent;

use crate::{Expr, IndexType, IntoIndexColumn, TableConstraint};
use crate::{SchemaStatementBuilder, backend::SchemaBuilder, types::*};

/// Create a constraint for an existing table. Unsupported by Sqlite
///
/// # Examples
///
/// Primary key
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let constraint = Constraint::create()
///     .primary()
///     .constraint_name("PK_2e303c3a712662f1fc2a4d0aad6")
///     .table(Font::Table)
///     .col(Font::Id)
///     .to_owned();
///
/// assert_eq!(
///     constraint.to_string(MysqlQueryBuilder),
///     [
///         r#"ALTER TABLE `font` ADD CONSTRAINT `PK_2e303c3a712662f1fc2a4d0aad6`"#,
///         r#"PRIMARY KEY (`id`)"#,
///     ]
///     .join(" ")
/// );
/// assert_eq!(
///     constraint.to_string(PostgresQueryBuilder),
///     [
///         r#"ALTER TABLE "font" ADD CONSTRAINT "PK_2e303c3a712662f1fc2a4d0aad6""#,
///         r#"PRIMARY KEY ("id")"#,
///     ]
///     .join(" ")
/// );
/// ```
///
/// Unique constraint
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let constraint = Constraint::create()
///     .unique()
///     .constraint_name("UQ_2e303c3a712662f1fc2a4d0aad6")
///     .table(Font::Table)
///     .col(Font::Name)
///     .to_owned();
///
/// assert_eq!(
///     constraint.to_string(MysqlQueryBuilder),
///     [
///         r#"ALTER TABLE `font` ADD CONSTRAINT `UQ_2e303c3a712662f1fc2a4d0aad6`"#,
///         r#"UNIQUE KEY (`name`)"#,
///     ]
///     .join(" ")
/// );
/// assert_eq!(
///     constraint.to_string(PostgresQueryBuilder),
///     [
///         r#"ALTER TABLE "font" ADD CONSTRAINT "UQ_2e303c3a712662f1fc2a4d0aad6""#,
///         r#"UNIQUE ("name")"#,
///     ]
///     .join(" ")
/// );
/// ```
///
/// Check constraint
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let constraint = Constraint::create()
///     .constraint_name("id_range")
///     .check(Expr::col(Glyph::Id).lt(20))
///     .table(Glyph::Table)
///     .to_owned();
///
/// assert_eq!(
///     constraint.to_string(MysqlQueryBuilder),
///     r#"ALTER TABLE `glyph` ADD CONSTRAINT `id_range` CHECK (`id` < 20)"#
/// );
/// assert_eq!(
///     constraint.to_string(PostgresQueryBuilder),
///     r#"ALTER TABLE "glyph" ADD CONSTRAINT "id_range" CHECK ("id" < 20)"#
/// );
/// ```
#[derive(Default, Debug, Clone)]
pub struct ConstraintCreateStatement {
    pub(crate) table: Option<TableRef>,
    pub(crate) constraint: TableConstraint,
}

impl ConstraintCreateStatement {
    /// Construct a new [`ConstraintCreateStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set constraint name
    pub fn constraint_name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.constraint.constraint_name(name);
        self
    }

    /// Set index name. Only available on MySQL.
    pub fn index_name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.constraint.index_name(name);
        self
    }

    /// Set target table
    pub fn table<T>(&mut self, table: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(table.into_table_ref());
        self
    }

    /// Add constraint column
    pub fn col<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoIndexColumn,
    {
        self.constraint.col(col);
        self
    }

    /// Set constraint as primary key
    pub fn primary(&mut self) -> &mut Self {
        self.constraint.primary();
        self
    }

    /// Set constraint as unique
    pub fn unique(&mut self) -> &mut Self {
        self.constraint.unique();
        self
    }

    /// Set constraint as check
    pub fn check<T>(&mut self, expr: T) -> &mut Self
    where
        T: Into<Expr>,
    {
        self.constraint.check(expr);
        self
    }

    /// Set nulls to not be treated as distinct values. Only available on Postgres.
    pub fn nulls_not_distinct(&mut self) -> &mut Self {
        self.constraint.nulls_not_distinct();
        self
    }

    /// Set index as full text. Only available on MySQL.
    pub fn full_text(&mut self) -> &mut Self {
        self.index_type(IndexType::FullText)
    }

    /// Set index type. Only available on MySQL.
    pub fn index_type(&mut self, index_type: IndexType) -> &mut Self {
        self.constraint.index_type(index_type);
        self
    }

    /// Use an existing index for the constraint. Only available on Postgres.
    pub fn using_index<T>(&mut self, using_index: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.constraint.using_index(using_index);
        self
    }

    /// Add include column. Only available on Postgres.
    pub fn include<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoIden,
    {
        self.constraint.include(col);
        self
    }

    pub fn get_table(&self) -> Option<&TableRef> {
        self.table.as_ref()
    }

    pub fn get_constraint(&self) -> &TableConstraint {
        &self.constraint
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            constraint: self.constraint.take(),
        }
    }
}

#[inherent]
impl SchemaStatementBuilder for ConstraintCreateStatement {
    pub fn build<T>(&self, schema_builder: T) -> String
    where
        T: SchemaBuilder,
    {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_constraint_create_statement(self, &mut sql);
        sql
    }

    pub fn to_string<T>(&self, schema_builder: T) -> String
    where
        T: SchemaBuilder;
}

/* For future EXCLUDE constraint support
impl ConditionalStatement for ConstraintCreateStatement {
    fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self {
        self.constraint.r#where.add_and_or(condition);
        self
    }

    fn cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.constraint
            .r#where
            .add_condition(condition.into_condition());
        self
    }
}
*/
