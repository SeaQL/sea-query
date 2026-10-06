use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConstraintMode {
    Alter,
    TableAlter,
}

pub trait ConstraintBuilder: IndexBuilder {
    /// Translate [`ConstraintCreateStatement`] into SQL statement.
    fn prepare_constraint_create_statement(
        &self,
        create: &ConstraintCreateStatement,
        sql: &mut impl SqlWriter,
    ) {
        self.prepare_constraint_create_statement_internal(create, sql, ConstraintMode::Alter)
    }

    #[doc(hidden)]
    /// Internal function to factor constraint with alter and without
    fn prepare_constraint_create_statement_internal(
        &self,
        create: &ConstraintCreateStatement,
        sql: &mut impl SqlWriter,
        mode: ConstraintMode,
    );
}
