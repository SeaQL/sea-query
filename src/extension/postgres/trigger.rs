use crate::{QueryBuilder, QuotedBuilder, SqlWriter};

pub use create::*;
pub use alter::*;
pub use drop::*;

pub(crate) mod create;
pub(crate) mod alter;
pub(crate) mod drop;

/// Creates a new "CREATE, ALTER or DROP TRIGGER" statement for PostgreSQL.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PgTriggerStmt;

impl PgTriggerStmt {
    /// Creates a new [`TriggerCreateStatement`]
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, extension::postgres::*, tests_cfg::*};
    ///
    /// let create = PgTriggerStmt::create()
    ///     .name("my_trigger")
    ///     .before()
    ///     .event(TriggerEvent::Insert)
    ///     .table("my_table")
    ///     .for_each_row()
    ///     .function("my_trigger_func")
    ///     .to_string(PostgresQueryBuilder);
    ///
    /// assert_eq!(
    ///     create,
    ///     r#"CREATE TRIGGER "my_trigger" BEFORE INSERT ON "my_table" FOR EACH ROW EXECUTE FUNCTION "my_trigger_func"()"#
    /// );
    /// ```
    pub fn create() -> TriggerCreateStatement {
        TriggerCreateStatement::new()
    }

    /// Creates a new [`TriggerAlterStatement`]
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, extension::postgres::*, tests_cfg::*};
    ///
    /// let alter = PgTriggerStmt::alter()
    ///     .name("my_trigger")
    ///     .table("my_table")
    ///     .rename_to("new_trigger")
    ///     .to_string(PostgresQueryBuilder);
    ///
    /// assert_eq!(
    ///     alter,
    ///     r#"ALTER TRIGGER "my_trigger" ON "my_table" RENAME TO "new_trigger""#
    /// );
    /// ```
    pub fn alter() -> TriggerAlterStatement {
        TriggerAlterStatement::new()
    }

    /// Creates a new [`TriggerDropStatement`]
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, extension::postgres::*, tests_cfg::*};
    ///
    /// let drop = PgTriggerStmt::drop()
    ///     .name("my_trigger")
    ///     .table("my_table")
    ///     .if_exists()
    ///     .cascade()
    ///     .to_string(PostgresQueryBuilder);
    ///
    /// assert_eq!(
    ///     drop,
    ///     r#"DROP TRIGGER IF EXISTS "my_trigger" ON "my_table" CASCADE"#
    /// );
    /// ```
    pub fn drop() -> TriggerDropStatement {
        TriggerDropStatement::new()
    }
}

pub trait TriggerBuilder: QuotedBuilder {
    /// Translate [`TriggerCreateStatement`] into database-specific SQL.
    fn prepare_trigger_create_statement(
        &self,
        create: &TriggerCreateStatement,
        sql: &mut impl SqlWriter,
    );

    /// Translate [`TriggerAlterStatement`] into database-specific SQL.
    fn prepare_trigger_alter_statement(
        &self,
        alter: &TriggerAlterStatement,
        sql: &mut impl SqlWriter,
    );

    /// Translate [`TriggerDropStatement`] into database-specific SQL.
    fn prepare_trigger_drop_statement(
        &self,
        drop: &TriggerDropStatement,
        sql: &mut impl SqlWriter,
    );
}

macro_rules! impl_trigger_statement_builder {
    ( $struct_name: ident, $func_name: ident ) => {
        impl $struct_name {
            pub fn build_ref<T: TriggerBuilder>(&self, trigger_builder: &T) -> String {
                let mut sql = String::with_capacity(256);
                self.build_collect_ref(trigger_builder, &mut sql)
            }

            pub fn build_collect<T: TriggerBuilder>(
                &self,
                trigger_builder: T,
                sql: &mut impl SqlWriter,
            ) -> String {
                self.build_collect_ref(&trigger_builder, sql)
            }

            pub fn build_collect_ref<T: TriggerBuilder>(
                &self,
                trigger_builder: &T,
                sql: &mut impl SqlWriter,
            ) -> String {
                trigger_builder.$func_name(self, sql);
                sql.to_string()
            }

            /// Build corresponding SQL statement and return SQL string
            pub fn to_string<T>(&self, trigger_builder: T) -> String
            where
                T: TriggerBuilder + QueryBuilder,
            {
                self.build_ref(&trigger_builder)
            }
        }
    };
}

impl_trigger_statement_builder!(TriggerCreateStatement, prepare_trigger_create_statement);
impl_trigger_statement_builder!(TriggerAlterStatement, prepare_trigger_alter_statement);
impl_trigger_statement_builder!(TriggerDropStatement, prepare_trigger_drop_statement);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Alias, IntoIden};

    // ── TriggerCreateStatement ──────────────────────────────────────────────

    #[test]
    fn create_statement_defaults() {
        let stmt = TriggerCreateStatement::new();
        assert!(stmt.name.is_none());
        assert!(!stmt.or_replace);
        assert!(!stmt.is_constraint);
        assert!(stmt.table.is_none());
        assert!(stmt.referenced_table.is_none());
        assert!(stmt.deferrable.is_none());
        assert!(stmt.initially.is_none());
        assert!(stmt.time.is_none());
        assert!(stmt.events.is_empty());
        assert!(stmt.referencing.is_empty());
        assert!(stmt.each.is_none());
        assert!(stmt.r#when.is_none());
        assert!(stmt.function.is_none());
        assert!(stmt.function_args.is_empty());
        assert!(stmt.execution_type.is_none());
    }

    #[test]
    fn create_statement_or_replace() {
        let mut stmt = TriggerCreateStatement::new();
        assert!(!stmt.or_replace);
        stmt.or_replace();
        assert!(stmt.or_replace);
    }

    #[test]
    fn create_statement_constraint() {
        let mut stmt = TriggerCreateStatement::new();
        assert!(!stmt.is_constraint);
        stmt.constraint();
        assert!(stmt.is_constraint);
    }

    #[test]
    fn create_statement_timing() {
        let mut stmt = TriggerCreateStatement::new();
        stmt.before();
        assert_eq!(stmt.time, Some(TriggerTime::Before));

        let mut stmt = TriggerCreateStatement::new();
        stmt.after();
        assert_eq!(stmt.time, Some(TriggerTime::After));

        let mut stmt = TriggerCreateStatement::new();
        stmt.instead_of();
        assert_eq!(stmt.time, Some(TriggerTime::InsteadOf));
    }

    #[test]
    fn create_statement_events() {
        let mut stmt = TriggerCreateStatement::new();
        stmt.event(TriggerEvent::Insert);
        stmt.event(TriggerEvent::Update(vec![Alias::new("col1").into_iden()]));
        assert_eq!(stmt.events.len(), 2);
    }

    #[test]
    fn create_statement_referencing() {
        let mut stmt = TriggerCreateStatement::new();
        stmt.referencing_old_table(Alias::new("old_t"));
        stmt.referencing_new_table(Alias::new("new_t"));
        assert_eq!(stmt.referencing.len(), 2);
    }

    #[test]
    fn create_statement_each() {
        let mut stmt = TriggerCreateStatement::new();
        stmt.for_each_row();
        assert_eq!(stmt.each, Some(TriggerEach::Row));

        let mut stmt = TriggerCreateStatement::new();
        stmt.for_each_statement();
        assert_eq!(stmt.each, Some(TriggerEach::Statement));
    }

    // ── TriggerAlterStatement ───────────────────────────────────────────────

    #[test]
    fn alter_statement_defaults() {
        let stmt = TriggerAlterStatement::new();
        assert!(stmt.name.is_none());
        assert!(stmt.table.is_none());
        assert!(stmt.option.is_none());
    }

    #[test]
    fn alter_statement_rename() {
        let mut stmt = TriggerAlterStatement::new();
        stmt.name(Alias::new("my_trigger"))
            .table(Alias::new("my_table"))
            .rename_to(Alias::new("new_trigger"));
        assert_eq!(
            stmt.option,
            Some(TriggerAlterOption::RenameTo(Alias::new("new_trigger").into_iden()))
        );
    }

    #[test]
    fn alter_statement_depends() {
        let mut stmt = TriggerAlterStatement::new();
        stmt.name(Alias::new("my_trigger"))
            .table(Alias::new("my_table"))
            .depends_on_extension(Alias::new("my_ext"));
        assert_eq!(
            stmt.option,
            Some(TriggerAlterOption::DependsOnExtension(Alias::new("my_ext").into_iden()))
        );
    }

    // ── TriggerDropStatement ────────────────────────────────────────────────

    #[test]
    fn drop_statement_defaults() {
        let stmt = TriggerDropStatement::new();
        assert!(stmt.name.is_none());
        assert!(!stmt.if_exists);
        assert!(stmt.table.is_none());
        assert!(!stmt.cascade);
        assert!(!stmt.restrict);
    }

    #[test]
    fn drop_statement_options() {
        let mut stmt = TriggerDropStatement::new();
        stmt.name(Alias::new("my_trigger"))
            .table(Alias::new("my_table"))
            .if_exists()
            .cascade();
        assert!(stmt.if_exists);
        assert!(stmt.cascade);
        assert!(!stmt.restrict);
    }

    // ── SQL output (PostgresQueryBuilder) ────────────────────────────────────

    #[cfg(feature = "backend-postgres")]
    mod sql {
        use super::*;
        use crate::{Expr, ExprTrait, PostgresQueryBuilder};

        #[test]
        fn create_basic() {
            let sql = PgTriggerStmt::create()
                .name(Alias::new("my_trigger"))
                .before()
                .event(TriggerEvent::Insert)
                .table(Alias::new("my_table"))
                .for_each_row()
                .function(Alias::new("my_trigger_func"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE TRIGGER "my_trigger" BEFORE INSERT ON "my_table" FOR EACH ROW EXECUTE FUNCTION "my_trigger_func"()"#
            );
        }

        #[test]
        fn create_or_replace() {
            let sql = PgTriggerStmt::create()
                .or_replace()
                .name(Alias::new("my_trigger"))
                .after()
                .event(TriggerEvent::Delete)
                .table(Alias::new("my_table"))
                .for_each_row()
                .function(Alias::new("my_trigger_func"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE OR REPLACE TRIGGER "my_trigger" AFTER DELETE ON "my_table" FOR EACH ROW EXECUTE FUNCTION "my_trigger_func"()"#
            );
        }

        #[test]
        fn create_constraint() {
            let sql = PgTriggerStmt::create()
                .constraint()
                .name(Alias::new("my_trigger"))
                .after()
                .event(TriggerEvent::Insert)
                .table(Alias::new("my_table"))
                .from_table(Alias::new("other_table"))
                .deferrable(true)
                .initially_deferred()
                .for_each_row()
                .function(Alias::new("my_trigger_func"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE CONSTRAINT TRIGGER "my_trigger" AFTER INSERT ON "my_table" FROM "other_table" DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION "my_trigger_func"()"#
            );
        }

        #[test]
        fn create_update_of_columns() {
            let sql = PgTriggerStmt::create()
                .name(Alias::new("my_trigger"))
                .before()
                .event(TriggerEvent::Update(vec![
                    Alias::new("col1").into_iden(),
                    Alias::new("col2").into_iden(),
                ]))
                .table(Alias::new("my_table"))
                .for_each_row()
                .function(Alias::new("my_trigger_func"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE TRIGGER "my_trigger" BEFORE UPDATE OF "col1", "col2" ON "my_table" FOR EACH ROW EXECUTE FUNCTION "my_trigger_func"()"#
            );
        }

        #[test]
        fn create_multiple_events() {
            let sql = PgTriggerStmt::create()
                .name(Alias::new("my_trigger"))
                .before()
                .events([
                    TriggerEvent::Insert,
                    TriggerEvent::Update(vec![Alias::new("col1").into_iden()]),
                    TriggerEvent::Delete,
                ])
                .table(Alias::new("my_table"))
                .for_each_row()
                .function(Alias::new("my_trigger_func"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE TRIGGER "my_trigger" BEFORE INSERT OR UPDATE OF "col1" OR DELETE ON "my_table" FOR EACH ROW EXECUTE FUNCTION "my_trigger_func"()"#
            );
        }

        #[test]
        fn create_referencing() {
            let sql = PgTriggerStmt::create()
                .name(Alias::new("my_trigger"))
                .after()
                .event(TriggerEvent::Update(vec![]))
                .table(Alias::new("my_table"))
                .referencing_old_table(Alias::new("old_t"))
                .referencing_new_table(Alias::new("new_t"))
                .for_each_statement()
                .function(Alias::new("my_trigger_func"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE TRIGGER "my_trigger" AFTER UPDATE ON "my_table" REFERENCING OLD TABLE AS "old_t" NEW TABLE AS "new_t" FOR EACH STATEMENT EXECUTE FUNCTION "my_trigger_func"()"#
            );
        }

        #[test]
        fn create_when_condition() {
            let sql = PgTriggerStmt::create()
                .name(Alias::new("my_trigger"))
                .before()
                .event(TriggerEvent::Insert)
                .table(Alias::new("my_table"))
                .for_each_row()
                .r#when(Expr::col(Alias::new("val")).gt(10))
                .function(Alias::new("my_trigger_func"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE TRIGGER "my_trigger" BEFORE INSERT ON "my_table" FOR EACH ROW WHEN ("val" > 10) EXECUTE FUNCTION "my_trigger_func"()"#
            );
        }

        #[test]
        #[allow(deprecated)]
        fn create_procedure() {
            let sql = PgTriggerStmt::create()
                .name(Alias::new("my_trigger"))
                .before()
                .event(TriggerEvent::Insert)
                .table(Alias::new("my_table"))
                .for_each_row()
                .procedure(Alias::new("my_proc"))
                .function_arg(Expr::val("arg1"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE TRIGGER "my_trigger" BEFORE INSERT ON "my_table" FOR EACH ROW EXECUTE PROCEDURE "my_proc"('arg1')"#
            );
        }

        #[test]
        fn alter_rename() {
            let sql = PgTriggerStmt::alter()
                .name(Alias::new("old_trig"))
                .table(Alias::new("my_table"))
                .rename_to(Alias::new("new_trig"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"ALTER TRIGGER "old_trig" ON "my_table" RENAME TO "new_trig""#
            );
        }

        #[test]
        fn alter_depends() {
            let sql = PgTriggerStmt::alter()
                .name(Alias::new("my_trig"))
                .table(Alias::new("my_table"))
                .depends_on_extension(Alias::new("my_ext"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"ALTER TRIGGER "my_trig" ON "my_table" DEPENDS ON EXTENSION "my_ext""#
            );
        }

        #[test]
        fn alter_no_depends() {
            let sql = PgTriggerStmt::alter()
                .name(Alias::new("my_trig"))
                .table(Alias::new("my_table"))
                .no_depends_on_extension(Alias::new("my_ext"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"ALTER TRIGGER "my_trig" ON "my_table" NO DEPENDS ON EXTENSION "my_ext""#
            );
        }

        #[test]
        fn drop_basic() {
            let sql = PgTriggerStmt::drop()
                .name(Alias::new("my_trigger"))
                .table(Alias::new("my_table"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"DROP TRIGGER "my_trigger" ON "my_table""#
            );
        }

        #[test]
        fn drop_if_exists_cascade() {
            let sql = PgTriggerStmt::drop()
                .name(Alias::new("my_trigger"))
                .table(Alias::new("my_table"))
                .if_exists()
                .cascade()
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"DROP TRIGGER IF EXISTS "my_trigger" ON "my_table" CASCADE"#
            );
        }

        #[test]
        fn drop_restrict() {
            let sql = PgTriggerStmt::drop()
                .name(Alias::new("my_trigger"))
                .table(Alias::new("my_table"))
                .restrict()
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"DROP TRIGGER "my_trigger" ON "my_table" RESTRICT"#
            );
        }
    }
}
