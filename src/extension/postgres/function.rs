use crate::{
    QueryBuilder, QuotedBuilder, SqlWriter,
};

pub use create::*;
pub use alter::*;
pub use drop::*;

pub(crate) mod create;
pub(crate) mod alter;
pub(crate) mod drop;

/// Creates a new "CREATE, ALTER or DROP FUNCTION" statement for PostgreSQL.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PgFunctionStmt;

impl PgFunctionStmt {
    /// Creates a new [`FunctionCreateStatement`]
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, extension::postgres::*, tests_cfg::*};
    ///
    /// let create = PgFunctionStmt::create()
    ///     .name(Alias::new("my_function"))
    ///     .arg(FunctionArg::new(ColumnType::Integer).name(Alias::new("a")))
    ///     .returns(FunctionReturns::Type(ColumnType::Integer))
    ///     .language(Alias::new("plpgsql"))
    ///     .as_definition("BEGIN RETURN a + 1; END;")
    ///     .to_string(PostgresQueryBuilder);
    ///
    /// assert_eq!(
    ///     create,
    ///     r#"CREATE FUNCTION "my_function" ("a" integer) RETURNS integer LANGUAGE "plpgsql" AS 'BEGIN RETURN a + 1; END;'"#
    /// );
    /// ```
    pub fn create() -> FunctionCreateStatement {
        FunctionCreateStatement::new()
    }

    /// Creates a new [`FunctionAlterStatement`]
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, extension::postgres::*, tests_cfg::*};
    ///
    /// let alter = PgFunctionStmt::alter()
    ///     .name(Alias::new("my_function"))
    ///     .arg_types([ColumnType::Integer])
    ///     .rename_to(Alias::new("new_function"))
    ///     .to_string(PostgresQueryBuilder);
    ///
    /// assert_eq!(
    ///     alter,
    ///     r#"ALTER FUNCTION "my_function" (integer) RENAME TO "new_function""#
    /// );
    /// ```
    pub fn alter() -> FunctionAlterStatement {
        FunctionAlterStatement::new()
    }

    /// Creates a new [`FunctionDropStatement`]
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, extension::postgres::*, tests_cfg::*};
    ///
    /// let drop = PgFunctionStmt::drop()
    ///     .name(Alias::new("my_function"))
    ///     .if_exists()
    ///     .arg_types([ColumnType::Integer])
    ///     .cascade()
    ///     .to_string(PostgresQueryBuilder);
    ///
    /// assert_eq!(
    ///     drop,
    ///     r#"DROP FUNCTION IF EXISTS "my_function" (integer) CASCADE"#
    /// );
    /// ```
    pub fn drop() -> FunctionDropStatement {
        FunctionDropStatement::new()
    }
}

pub trait FunctionBuilder: QuotedBuilder {
    /// Translate [`FunctionCreateStatement`] into database-specific SQL.
    fn prepare_function_create_statement(
        &self,
        create: &FunctionCreateStatement,
        sql: &mut impl SqlWriter,
    );

    /// Translate [`FunctionDropStatement`] into database-specific SQL.
    fn prepare_function_drop_statement(
        &self,
        drop: &FunctionDropStatement,
        sql: &mut impl SqlWriter,
    );

    /// Translate [`FunctionAlterStatement`] into database-specific SQL.
    fn prepare_function_alter_statement(
        &self,
        alter: &FunctionAlterStatement,
        sql: &mut impl SqlWriter,
    );
}

macro_rules! impl_function_statement_builder {
    ( $struct_name: ident, $func_name: ident ) => {
        impl $struct_name {
            pub fn build_ref<T: FunctionBuilder>(&self, function_builder: &T) -> String {
                let mut sql = String::with_capacity(256);
                self.build_collect_ref(function_builder, &mut sql)
            }

            pub fn build_collect<T: FunctionBuilder>(
                &self,
                function_builder: T,
                sql: &mut impl SqlWriter,
            ) -> String {
                self.build_collect_ref(&function_builder, sql)
            }

            pub fn build_collect_ref<T: FunctionBuilder>(
                &self,
                function_builder: &T,
                sql: &mut impl SqlWriter,
            ) -> String {
                function_builder.$func_name(self, sql);
                sql.to_string()
            }

            /// Build corresponding SQL statement and return SQL string
            pub fn to_string<T>(&self, function_builder: T) -> String
            where
                T: FunctionBuilder + QueryBuilder,
            {
                self.build_ref(&function_builder)
            }
        }
    };
}

impl_function_statement_builder!(FunctionCreateStatement, prepare_function_create_statement);
impl_function_statement_builder!(FunctionDropStatement, prepare_function_drop_statement);
impl_function_statement_builder!(FunctionAlterStatement, prepare_function_alter_statement);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Alias, ColumnType, IntoIden};

    // ── FunctionArg ──────────────────────────────────────────────────────────

    #[test]
    fn function_arg_defaults() {
        let arg = FunctionArg::new(ColumnType::Integer);
        assert_eq!(arg.arg_type, ColumnType::Integer);
        assert!(arg.mode.is_none());
        assert!(arg.name.is_none());
        assert!(arg.default.is_none());
    }

    #[test]
    fn function_arg_with_name() {
        let arg = FunctionArg::new(ColumnType::Text).name(Alias::new("my_param"));
        assert!(arg.name.is_some());
    }

    #[test]
    fn function_arg_with_mode() {
        let arg = FunctionArg::new(ColumnType::Integer).mode(FunctionArgMode::In);
        assert_eq!(arg.mode, Some(FunctionArgMode::In));

        let arg = FunctionArg::new(ColumnType::Integer).mode(FunctionArgMode::Out);
        assert_eq!(arg.mode, Some(FunctionArgMode::Out));

        let arg = FunctionArg::new(ColumnType::Integer).mode(FunctionArgMode::InOut);
        assert_eq!(arg.mode, Some(FunctionArgMode::InOut));

        let arg = FunctionArg::new(ColumnType::Integer).mode(FunctionArgMode::Variadic);
        assert_eq!(arg.mode, Some(FunctionArgMode::Variadic));
    }

    #[test]
    fn function_arg_all_fields() {
        use crate::Expr;
        let arg = FunctionArg::new(ColumnType::Integer)
            .mode(FunctionArgMode::In)
            .name(Alias::new("x"))
            .default(Expr::val(0i32));
        assert_eq!(arg.mode, Some(FunctionArgMode::In));
        assert!(arg.name.is_some());
        assert!(arg.default.is_some());
    }

    // ── FunctionCreateStatement ──────────────────────────────────────────────

    #[test]
    fn create_statement_defaults() {
        let stmt = FunctionCreateStatement::new();
        assert!(stmt.name.is_none());
        assert!(!stmt.or_replace);
        assert!(stmt.args.is_empty());
        assert!(stmt.returns.is_none());
        assert!(stmt.language.is_none());
        assert!(stmt.behavior.is_empty());
        assert!(stmt.as_definition.is_none());
        assert!(stmt.sql_body.is_none());
    }

    #[test]
    fn create_statement_or_replace() {
        let mut stmt = FunctionCreateStatement::new();
        assert!(!stmt.or_replace);
        stmt.or_replace();
        assert!(stmt.or_replace);
    }

    #[test]
    fn create_statement_single_arg() {
        let mut stmt = FunctionCreateStatement::new();
        stmt.arg(FunctionArg::new(ColumnType::Integer).name(Alias::new("a")));
        assert_eq!(stmt.args.len(), 1);
    }

    #[test]
    fn create_statement_multiple_args_via_args() {
        let mut stmt = FunctionCreateStatement::new();
        stmt.args([
            FunctionArg::new(ColumnType::Integer).name(Alias::new("a")),
            FunctionArg::new(ColumnType::Text).name(Alias::new("b")),
        ]);
        assert_eq!(stmt.args.len(), 2);
    }

    #[test]
    fn create_statement_behaviors() {
        let mut stmt = FunctionCreateStatement::new();
        stmt.behavior(FunctionBehavior::Immutable);
        stmt.behavior(FunctionBehavior::Strict);
        assert_eq!(stmt.behavior.len(), 2);
        assert_eq!(stmt.behavior[0], FunctionBehavior::Immutable);
        assert_eq!(stmt.behavior[1], FunctionBehavior::Strict);
    }

    #[test]
    fn create_statement_returns_type() {
        let mut stmt = FunctionCreateStatement::new();
        stmt.returns(FunctionReturns::Type(ColumnType::Boolean));
        assert!(matches!(
            stmt.returns,
            Some(FunctionReturns::Type(ColumnType::Boolean))
        ));
    }

    #[test]
    fn create_statement_returns_table() {
        let mut stmt = FunctionCreateStatement::new();
        stmt.returns(FunctionReturns::Table(vec![
            (Alias::new("id").into_iden(), ColumnType::Integer),
            (Alias::new("name").into_iden(), ColumnType::Text),
        ]));
        assert!(matches!(stmt.returns, Some(FunctionReturns::Table(_))));
        if let Some(FunctionReturns::Table(cols)) = &stmt.returns {
            assert_eq!(cols.len(), 2);
        }
    }

    #[test]
    fn create_statement_as_definition() {
        let mut stmt = FunctionCreateStatement::new();
        stmt.as_definition("BEGIN RETURN 1; END;");
        assert_eq!(stmt.as_definition.as_deref(), Some("BEGIN RETURN 1; END;"));
        assert!(stmt.sql_body.is_none());
    }

    #[test]
    fn create_statement_sql_body() {
        let mut stmt = FunctionCreateStatement::new();
        stmt.sql_body("RETURN a + b;");
        assert_eq!(stmt.sql_body.as_deref(), Some("RETURN a + b;"));
        assert!(stmt.as_definition.is_none());
    }

    // ── FunctionDropStatement ────────────────────────────────────────────────

    #[test]
    fn drop_statement_defaults() {
        let stmt = FunctionDropStatement::new();
        assert!(stmt.name.is_none());
        assert!(!stmt.if_exists);
        assert!(stmt.arg_types.is_none());
        assert!(!stmt.cascade);
        assert!(!stmt.restrict);
    }

    #[test]
    fn drop_statement_if_exists() {
        let mut stmt = FunctionDropStatement::new();
        stmt.if_exists();
        assert!(stmt.if_exists);
    }

    #[test]
    fn drop_statement_cascade_and_restrict_are_independent() {
        let mut stmt = FunctionDropStatement::new();
        stmt.cascade();
        assert!(stmt.cascade);
        assert!(!stmt.restrict);

        let mut stmt = FunctionDropStatement::new();
        stmt.restrict();
        assert!(stmt.restrict);
        assert!(!stmt.cascade);
    }

    #[test]
    fn drop_statement_arg_types() {
        let mut stmt = FunctionDropStatement::new();
        stmt.arg_types([ColumnType::Integer, ColumnType::Text]);
        let types = stmt.arg_types.as_ref().unwrap();
        assert_eq!(types.len(), 2);
        assert_eq!(types[0], ColumnType::Integer);
        assert_eq!(types[1], ColumnType::Text);
    }

    #[test]
    fn drop_statement_empty_arg_types() {
        let mut stmt = FunctionDropStatement::new();
        stmt.arg_types([] as [ColumnType; 0]);
        assert!(stmt.arg_types.as_ref().unwrap().is_empty());
    }

    // ── FunctionAlterStatement ──────────────────────────────────────────────

    #[test]
    fn alter_statement_defaults() {
        let stmt = FunctionAlterStatement::new();
        assert!(stmt.name.is_none());
        assert!(stmt.arg_types.is_none());
        assert!(stmt.options.is_empty());
    }

    #[test]
    fn alter_statement_name() {
        let mut stmt = FunctionAlterStatement::new();
        stmt.name(Alias::new("my_fn"));
        assert!(stmt.name.is_some());
    }

    #[test]
    fn alter_statement_arg_types() {
        let mut stmt = FunctionAlterStatement::new();
        stmt.arg_types([ColumnType::Integer, ColumnType::Text]);
        let types = stmt.arg_types.as_ref().unwrap();
        assert_eq!(types.len(), 2);
    }

    #[test]
    fn alter_statement_rename_to() {
        let mut stmt = FunctionAlterStatement::new();
        stmt.rename_to(Alias::new("new_fn"));
        assert_eq!(stmt.options.len(), 1);
        assert!(matches!(stmt.options[0], FunctionAlterOption::RenameTo(_)));
    }

    #[test]
    fn alter_statement_owner_to() {
        let mut stmt = FunctionAlterStatement::new();
        stmt.owner_to(Alias::new("new_owner"));
        assert_eq!(stmt.options.len(), 1);
        assert!(matches!(stmt.options[0], FunctionAlterOption::OwnerTo(_)));
    }

    #[test]
    fn alter_statement_set_schema() {
        let mut stmt = FunctionAlterStatement::new();
        stmt.set_schema(Alias::new("new_schema"));
        assert_eq!(stmt.options.len(), 1);
        assert!(matches!(
            stmt.options[0],
            FunctionAlterOption::SetSchema(_)
        ));
    }

    // ── SQL output (PostgresQueryBuilder) ────────────────────────────────────

    #[cfg(feature = "backend-postgres")]
    mod sql {
        use super::*;
        use crate::PostgresQueryBuilder;

        #[test]
        fn create_basic() {
            let sql = PgFunctionStmt::create()
                .name(Alias::new("my_fn"))
                .arg(FunctionArg::new(ColumnType::Integer).name(Alias::new("a")))
                .returns(FunctionReturns::Type(ColumnType::Integer))
                .language(Alias::new("plpgsql"))
                .as_definition("BEGIN RETURN a + 1; END;")
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"CREATE FUNCTION "my_fn" ("a" integer) RETURNS integer LANGUAGE "plpgsql" AS 'BEGIN RETURN a + 1; END;'"#
            );
        }

        #[test]
        fn create_or_replace() {
            let sql = PgFunctionStmt::create()
                .or_replace()
                .name(Alias::new("my_fn"))
                .returns(FunctionReturns::Type(ColumnType::Integer))
                .language(Alias::new("sql"))
                .as_definition("SELECT 1")
                .to_string(PostgresQueryBuilder);

            assert!(sql.starts_with("CREATE OR REPLACE FUNCTION"));
        }

        #[test]
        fn create_no_args() {
            let sql = PgFunctionStmt::create()
                .name(Alias::new("my_fn"))
                .returns(FunctionReturns::Type(ColumnType::Integer))
                .language(Alias::new("sql"))
                .as_definition("SELECT 1")
                .to_string(PostgresQueryBuilder);

            assert!(sql.contains("()"));
        }

        #[test]
        fn create_with_behavior_immutable_strict() {
            let sql = PgFunctionStmt::create()
                .name(Alias::new("my_fn"))
                .returns(FunctionReturns::Type(ColumnType::Integer))
                .language(Alias::new("sql"))
                .behavior(FunctionBehavior::Immutable)
                .behavior(FunctionBehavior::Strict)
                .as_definition("SELECT 1")
                .to_string(PostgresQueryBuilder);

            assert!(sql.contains("IMMUTABLE"));
            assert!(sql.contains("STRICT"));
        }

        #[test]
        fn create_multiple_args() {
            let sql = PgFunctionStmt::create()
                .name(Alias::new("add"))
                .arg(FunctionArg::new(ColumnType::Integer).name(Alias::new("a")))
                .arg(FunctionArg::new(ColumnType::Integer).name(Alias::new("b")))
                .returns(FunctionReturns::Type(ColumnType::Integer))
                .language(Alias::new("sql"))
                .as_definition("SELECT a + b")
                .to_string(PostgresQueryBuilder);

            assert!(sql.contains(r#""a" integer"#));
            assert!(sql.contains(r#""b" integer"#));
        }

        #[test]
        fn drop_basic() {
            let sql = PgFunctionStmt::drop()
                .name(Alias::new("my_fn"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(sql, r#"DROP FUNCTION "my_fn""#);
        }

        #[test]
        fn drop_if_exists_cascade() {
            let sql = PgFunctionStmt::drop()
                .name(Alias::new("my_fn"))
                .if_exists()
                .arg_types([ColumnType::Integer])
                .cascade()
                .to_string(PostgresQueryBuilder);

            assert_eq!(sql, r#"DROP FUNCTION IF EXISTS "my_fn" (integer) CASCADE"#);
        }

        #[test]
        fn drop_restrict() {
            let sql = PgFunctionStmt::drop()
                .name(Alias::new("my_fn"))
                .restrict()
                .to_string(PostgresQueryBuilder);

            assert!(sql.ends_with("RESTRICT"));
        }

        #[test]
        fn drop_multiple_arg_types() {
            let sql = PgFunctionStmt::drop()
                .name(Alias::new("my_fn"))
                .arg_types([ColumnType::Integer, ColumnType::Text])
                .to_string(PostgresQueryBuilder);

            assert!(sql.contains("(integer, text)"));
        }

        #[test]
        fn alter_rename() {
            let sql = PgFunctionStmt::alter()
                .name(Alias::new("old_fn"))
                .arg_types([ColumnType::Integer])
                .rename_to(Alias::new("new_fn"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(
                sql,
                r#"ALTER FUNCTION "old_fn" (integer) RENAME TO "new_fn""#
            );
        }

        #[test]
        fn alter_owner() {
            let sql = PgFunctionStmt::alter()
                .name(Alias::new("my_fn"))
                .owner_to(Alias::new("new_owner"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(sql, r#"ALTER FUNCTION "my_fn" OWNER TO "new_owner""#);
        }

        #[test]
        fn alter_schema() {
            let sql = PgFunctionStmt::alter()
                .name(Alias::new("my_fn"))
                .set_schema(Alias::new("new_schema"))
                .to_string(PostgresQueryBuilder);

            assert_eq!(sql, r#"ALTER FUNCTION "my_fn" SET SCHEMA "new_schema""#);
        }
    }
}
