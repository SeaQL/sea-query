use crate::*;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Creation,
    Alter,
    TableAlter,
}

pub trait ForeignKeyBuilder: QuotedBuilder {
    /// Translate [`ForeignKeyCreateStatement`] into SQL statement.
    fn prepare_foreign_key_create_statement(
        &self,
        create: &ForeignKeyCreateStatement,
        sql: &mut SqlWriter,
    ) {
        self.prepare_foreign_key_create_statement_internal(create, sql, Mode::Alter)
    }

    /// Translate [`ForeignKeyDropStatement`] into SQL statement.
    fn prepare_foreign_key_drop_statement(
        &self,
        drop: &ForeignKeyDropStatement,
        sql: &mut SqlWriter,
    ) {
        self.prepare_foreign_key_drop_statement_internal(drop, sql, Mode::Alter)
    }

    /// Translate [`ForeignKeyAction`] into SQL statement.
    fn prepare_foreign_key_action(
        &self,
        foreign_key_action: &ForeignKeyAction,
        sql: &mut SqlWriter,
    ) {
        write!(
            sql,
            "{}",
            match foreign_key_action {
                ForeignKeyAction::Restrict => "RESTRICT",
                ForeignKeyAction::Cascade => "CASCADE",
                ForeignKeyAction::SetNull => "SET NULL",
                ForeignKeyAction::NoAction => "NO ACTION",
                ForeignKeyAction::SetDefault => "SET DEFAULT",
            }
        )
        .unwrap()
    }

    #[doc(hidden)]
    /// Internal function to factor foreign key drop in table and outside.
    fn prepare_foreign_key_drop_statement_internal(
        &self,
        drop: &ForeignKeyDropStatement,
        sql: &mut SqlWriter,
        mode: Mode,
    );

    #[doc(hidden)]
    /// Internal function to factor foreign key creation in table and outside.
    fn prepare_foreign_key_create_statement_internal(
        &self,
        create: &ForeignKeyCreateStatement,
        sql: &mut SqlWriter,
        mode: Mode,
    );
}
