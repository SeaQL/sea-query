use inherent::inherent;

use crate::{
    Check, ConditionalStatement, IndexType, IntoCondition, IntoIndexColumn, TableConstraint,
    TableIndex,
};
use crate::{SchemaStatementBuilder, backend::SchemaBuilder, types::*};

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
    pub fn check<T>(&mut self, check: T) -> &mut Self
    where
        T: Into<Check>,
    {
        self.constraint.check(check);
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

    /// Set index type. Only available on MySQL.
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

    pub fn is_nulls_not_distinct(&self) -> bool {
        self.constraint.is_nulls_not_distinct()
    }

    pub fn get_index_spec(&self) -> &TableIndex {
        self.constraint.get_index_spec()
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
