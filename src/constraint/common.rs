use crate::{Check, ConditionHolder, Expr, IndexType, IntoIndexColumn, TableIndex, types::*};

/// Specification of a constraint
#[derive(Default, Debug, Clone)]
pub struct TableConstraint {
    pub(crate) name: Option<String>,
    pub(crate) index: TableIndex,
    pub(crate) constraint_type: Option<ConstraintCreateStatementType>,
    pub(crate) nulls_not_distinct: bool,
    pub(crate) index_type: Option<IndexType>,
    pub(crate) using_index: Option<DynIden>,
    pub(crate) r#where: ConditionHolder,
    pub(crate) include_columns: Vec<DynIden>,
}

impl TableConstraint {
    /// Construct a new constraint
    pub fn new() -> Self {
        Self::default()
    }

    /// Set constraint name
    pub fn constraint_name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        let name = name.into();
        if let Some(ConstraintCreateStatementType::Check(check)) = &mut self.constraint_type {
            check.name = Some(name.clone().into());
        }
        self.name = Some(name);
        self
    }

    /// Set index name
    pub fn index_name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.index.name(name);
        self
    }

    /// Add constraint column
    pub fn col<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoIndexColumn,
    {
        self.index.col(col.into_index_column());
        self
    }

    /// Set constraint as primary key
    pub fn primary(&mut self) -> &mut Self {
        self.constraint_type = Some(ConstraintCreateStatementType::PrimaryKey);
        self
    }

    /// Set constraint as unique
    pub fn unique(&mut self) -> &mut Self {
        self.constraint_type = Some(ConstraintCreateStatementType::Unique);
        self
    }

    /// Set constraint as check
    pub fn check<T>(&mut self, expr: T) -> &mut Self
    where
        T: Into<Expr>,
    {
        self.constraint_type = Some(ConstraintCreateStatementType::Check(
            match self.name.clone() {
                Some(name) => Check::named(name, expr),
                None => Check::unnamed(expr),
            },
        ));
        self
    }

    /// Set nulls to not be treated as distinct values. Only available on Postgres.
    pub fn nulls_not_distinct(&mut self) -> &mut Self {
        self.nulls_not_distinct = true;
        self
    }

    /// Set index as full text. Only available on MySQL.
    pub fn full_text(&mut self) -> &mut Self {
        self.index_type(IndexType::FullText)
    }

    /// Set index type. Only available on MySQL.
    pub fn index_type(&mut self, index_type: IndexType) -> &mut Self {
        self.index_type = Some(index_type);
        self
    }

    /// Use an existing index for the constraint. Only available on Postgres.
    pub fn using_index<T>(&mut self, using_index: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.using_index = Some(using_index.into_iden());
        self
    }

    /// Add include column. Only available on Postgres.
    pub fn include<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoIden,
    {
        self.include_columns.push(col.into_iden());
        self
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_index_name(&self) -> Option<&str> {
        self.index.get_name()
    }

    pub fn get_columns(&self) -> Vec<String> {
        self.index.get_column_names()
    }

    pub fn get_check(&self) -> Option<&Check> {
        match &self.constraint_type {
            Some(ConstraintCreateStatementType::Check(check)) => Some(check),
            _ => None,
        }
    }

    pub fn is_primary_key(&self) -> bool {
        matches!(
            self.constraint_type,
            Some(ConstraintCreateStatementType::PrimaryKey)
        )
    }

    pub fn is_unique_key(&self) -> bool {
        matches!(
            self.constraint_type,
            Some(ConstraintCreateStatementType::Unique)
        )
    }

    pub fn is_check(&self) -> bool {
        matches!(
            self.constraint_type,
            Some(ConstraintCreateStatementType::Check(_))
        )
    }

    pub fn is_nulls_not_distinct(&self) -> bool {
        self.nulls_not_distinct
    }

    pub fn get_index_type(&self) -> Option<&IndexType> {
        self.index_type.as_ref()
    }

    pub fn get_using_index(&self) -> Option<&DynIden> {
        self.using_index.as_ref()
    }

    pub fn get_include_columns(&self) -> Vec<String> {
        self.include_columns
            .iter()
            .map(|col| col.to_string())
            .collect()
    }

    pub fn get_index_spec(&self) -> &TableIndex {
        &self.index
    }

    pub fn take(&mut self) -> Self {
        Self {
            name: self.name.take(),
            index: self.index.take(),
            constraint_type: self.constraint_type.take(),
            nulls_not_distinct: self.nulls_not_distinct,
            index_type: self.index_type.take(),
            using_index: self.using_index.take(),
            r#where: self.r#where.clone(),
            include_columns: self.include_columns.clone(),
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) enum ConstraintCreateStatementType {
    Check(Check),
    Unique,
    PrimaryKey,
}
