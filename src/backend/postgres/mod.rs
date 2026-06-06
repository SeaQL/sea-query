pub(crate) mod extension;
pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod types;

use super::*;
use crate::extension::postgres::{function::alter::{FunctionAlterOption, FunctionAlterStatement}, function::create::{FunctionArgMode, FunctionBehavior, FunctionCreateStatement, FunctionReturns}, function::drop::FunctionDropStatement, *};

/// Postgres query builder.
#[derive(Default, Debug)]
pub struct PostgresQueryBuilder;

impl FunctionBuilder for PostgresQueryBuilder {
    fn prepare_function_create_statement(
        &self,
        create: &FunctionCreateStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("CREATE ").unwrap();
        if create.or_replace {
            sql.write_str("OR REPLACE ").unwrap();
        }
        sql.write_str("FUNCTION ").unwrap();
        if let Some(name) = &create.name {
            self.prepare_iden(name, sql);
        }
        sql.write_str(" (").unwrap();
        for (i, arg) in create.args.iter().enumerate() {
            if i > 0 {
                sql.write_str(", ").unwrap();
            }
            if let Some(mode) = &arg.mode {
                sql.write_str(match mode {
                    FunctionArgMode::In => "IN ",
                    FunctionArgMode::Out => "OUT ",
                    FunctionArgMode::InOut => "INOUT ",
                    FunctionArgMode::Variadic => "VARIADIC ",
                })
                .unwrap();
            }
            if let Some(name) = &arg.name {
                self.prepare_iden(name, sql);
                sql.write_str(" ").unwrap();
            }
            self.prepare_column_type(&arg.arg_type, sql);
            if let Some(default) = &arg.default {
                sql.write_str(" DEFAULT ").unwrap();
                self.prepare_expr(default, sql);
            }
        }
        sql.write_str(")").unwrap();
        if let Some(returns) = &create.returns {
            sql.write_str(" RETURNS ").unwrap();
            match returns {
                FunctionReturns::Type(t) => self.prepare_column_type(t, sql),
                FunctionReturns::Table(cols) => {
                    sql.write_str("TABLE (").unwrap();
                    for (i, (name, ty)) in cols.iter().enumerate() {
                        if i > 0 {
                            sql.write_str(", ").unwrap();
                        }
                        self.prepare_iden(name, sql);
                        sql.write_str(" ").unwrap();
                        self.prepare_column_type(ty, sql);
                    }
                    sql.write_str(")").unwrap();
                }
            }
        }
        if let Some(language) = &create.language {
            sql.write_str(" LANGUAGE ").unwrap();
            self.prepare_iden(language, sql);
        }
        for behavior in &create.behavior {
            sql.write_str(" ").unwrap();
            sql.write_str(match behavior {
                FunctionBehavior::Immutable => "IMMUTABLE",
                FunctionBehavior::Stable => "STABLE",
                FunctionBehavior::Volatile => "VOLATILE",
                FunctionBehavior::CalledOnNullInput => "CALLED ON NULL INPUT",
                FunctionBehavior::ReturnsNullOnNullInput => "RETURNS NULL ON NULL INPUT",
                FunctionBehavior::Strict => "STRICT",
                FunctionBehavior::SecurityInvoker => "SECURITY INVOKER",
                FunctionBehavior::SecurityDefiner => "SECURITY DEFINER",
                FunctionBehavior::ParallelUnsafe => "PARALLEL UNSAFE",
                FunctionBehavior::ParallelRestricted => "PARALLEL RESTRICTED",
                FunctionBehavior::ParallelSafe => "PARALLEL SAFE",
            })
            .unwrap();
        }
        if let Some(definition) = &create.as_definition {
            sql.write_str(" AS ").unwrap();
            self.write_string_quoted(definition, sql);
        } else if let Some(sql_body) = &create.sql_body {
            sql.write_str(" AS $$ ").unwrap();
            sql.write_str(sql_body).unwrap();
            sql.write_str(" $$").unwrap();
        }
    }

    fn prepare_function_drop_statement(
        &self,
        drop: &FunctionDropStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("DROP FUNCTION ").unwrap();
        if drop.if_exists {
            sql.write_str("IF EXISTS ").unwrap();
        }
        if let Some(name) = &drop.name {
            self.prepare_iden(name, sql);
        }
        if let Some(args) = &drop.arg_types {
            sql.write_str(" (").unwrap();
            for (i, ty) in args.iter().enumerate() {
                if i > 0 {
                    sql.write_str(", ").unwrap();
                }
                self.prepare_column_type(ty, sql);
            }
            sql.write_str(")").unwrap();
        }
        if drop.cascade {
            sql.write_str(" CASCADE").unwrap();
        }
        if drop.restrict {
            sql.write_str(" RESTRICT").unwrap();
        }
    }

    fn prepare_function_alter_statement(
        &self,
        alter: &FunctionAlterStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("ALTER FUNCTION ").unwrap();
       
        if let Some(name) = &alter.name {
            self.prepare_iden(name, sql);
        }
        if let Some(args) = &alter.arg_types {
            sql.write_str(" (").unwrap();
            for (i, ty) in args.iter().enumerate() {
                if i > 0 {
                    sql.write_str(", ").unwrap();
                }
                self.prepare_column_type(ty, sql);
            }
            sql.write_str(")").unwrap();
        }
        for option in alter.options.iter() {
            sql.write_str(" ").unwrap();
            match option {
                FunctionAlterOption::RenameTo(new_name) => {
                    sql.write_str("RENAME TO ").unwrap();
                    self.prepare_iden(new_name, sql);
                }
                FunctionAlterOption::OwnerTo(new_owner) => {
                    sql.write_str("OWNER TO ").unwrap();
                    self.prepare_iden(new_owner, sql);
                }
                FunctionAlterOption::SetSchema(new_schema) => {
                    sql.write_str("SET SCHEMA ").unwrap();
                    self.prepare_iden(new_schema, sql);
                }
                FunctionAlterOption::Behavior(behavior) => {
                    sql.write_str(match behavior {
                        FunctionBehavior::Immutable => "IMMUTABLE",
                        FunctionBehavior::Stable => "STABLE",
                        FunctionBehavior::Volatile => "VOLATILE",
                        FunctionBehavior::CalledOnNullInput => "CALLED ON NULL INPUT",
                        FunctionBehavior::ReturnsNullOnNullInput => "RETURNS NULL ON NULL INPUT",
                        FunctionBehavior::Strict => "STRICT",
                        FunctionBehavior::SecurityInvoker => "SECURITY INVOKER",
                        FunctionBehavior::SecurityDefiner => "SECURITY DEFINER",
                        FunctionBehavior::ParallelUnsafe => "PARALLEL UNSAFE",
                        FunctionBehavior::ParallelRestricted => "PARALLEL RESTRICTED",
                        FunctionBehavior::ParallelSafe => "PARALLEL SAFE",
                    })
                    .unwrap();
                }
                FunctionAlterOption::Leakproof(leakproof) => {
                    if *leakproof {
                        sql.write_str("LEAKPROOF").unwrap();
                    } else {
                        sql.write_str("NOT LEAKPROOF").unwrap();
                    }
                }
                FunctionAlterOption::Cost(cost) => {
                    write!(sql, "COST {cost}").unwrap();
                }
                FunctionAlterOption::Rows(rows) => {
                    write!(sql, "ROWS {rows}").unwrap();
                }
                FunctionAlterOption::Support(support_fn) => {
                    sql.write_str("SUPPORT ").unwrap();
                    self.prepare_iden(support_fn, sql);
                }
                FunctionAlterOption::DependsOnExtension(ext) => {
                    sql.write_str("DEPENDS ON EXTENSION ").unwrap();
                    self.prepare_iden(ext, sql);
                }
                FunctionAlterOption::NoDependsOnExtension(ext) => {
                    sql.write_str("NO DEPENDS ON EXTENSION ").unwrap();
                    self.prepare_iden(ext, sql);
                }
                FunctionAlterOption::SetConfig(param, value) => {
                    sql.write_str("SET ").unwrap();
                    self.prepare_iden(param, sql);
                    sql.write_str(" TO ").unwrap();
                    sql.write_str(value).unwrap();
                }
                FunctionAlterOption::SetConfigDefault(param) => {
                    sql.write_str("SET ").unwrap();
                    self.prepare_iden(param, sql);
                    sql.write_str(" TO DEFAULT").unwrap();
                }
                FunctionAlterOption::SetConfigFromCurrent(param) => {
                    sql.write_str("SET ").unwrap();
                    self.prepare_iden(param, sql);
                    sql.write_str(" FROM CURRENT").unwrap();
                }
                FunctionAlterOption::ResetConfig(param) => {
                    sql.write_str("RESET ").unwrap();
                    self.prepare_iden(param, sql);
                }
                FunctionAlterOption::ResetAll => {
                    sql.write_str("RESET ALL").unwrap();
                }
            }
        }
        if alter.restrict {
            sql.write_str(" RESTRICT").unwrap();
        }
    }
}

impl TriggerBuilder for PostgresQueryBuilder {
    fn prepare_trigger_create_statement(
        &self,
        create: &TriggerCreateStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("CREATE ").unwrap();
        if create.or_replace {
            sql.write_str("OR REPLACE ").unwrap();
        }
        if create.is_constraint {
            sql.write_str("CONSTRAINT ").unwrap();
        }
        sql.write_str("TRIGGER ").unwrap();
        if let Some(name) = &create.name {
            self.prepare_iden(name, sql);
        }
        if let Some(time) = &create.time {
            sql.write_str(" ").unwrap();
            sql.write_str(match time {
                TriggerTime::Before => "BEFORE",
                TriggerTime::After => "AFTER",
                TriggerTime::InsteadOf => "INSTEAD OF",
            })
            .unwrap();
        }
        if !create.events.is_empty() {
            sql.write_str(" ").unwrap();
            for (i, event) in create.events.iter().enumerate() {
                if i > 0 {
                    sql.write_str(" OR ").unwrap();
                }
                match event {
                    TriggerEvent::Insert => sql.write_str("INSERT").unwrap(),
                    TriggerEvent::Update(cols) => {
                        sql.write_str("UPDATE").unwrap();
                        if !cols.is_empty() {
                            sql.write_str(" OF ").unwrap();
                            for (j, col) in cols.iter().enumerate() {
                                if j > 0 {
                                    sql.write_str(", ").unwrap();
                                }
                                self.prepare_iden(col, sql);
                            }
                        }
                    }
                    TriggerEvent::Delete => sql.write_str("DELETE").unwrap(),
                    TriggerEvent::Truncate => sql.write_str("TRUNCATE").unwrap(),
                }
            }
        }
        if let Some(table) = &create.table {
            sql.write_str(" ON ").unwrap();
            match table {
                TableRef::Table(table_name, _) => self.prepare_table_name(table_name, sql),
                _ => panic!("Expected TableRef::Table"),
            }
        }
        if let Some(referenced_table) = &create.referenced_table {
            sql.write_str(" FROM ").unwrap();
            match referenced_table {
                TableRef::Table(table_name, _) => self.prepare_table_name(table_name, sql),
                _ => panic!("Expected TableRef::Table"),
            }
        }
        if let Some(deferrable) = create.deferrable {
            if deferrable {
                sql.write_str(" DEFERRABLE").unwrap();
            } else {
                sql.write_str(" NOT DEFERRABLE").unwrap();
            }
        }
        if let Some(initially) = &create.initially {
            sql.write_str(" ").unwrap();
            sql.write_str(match initially {
                TriggerInitially::Immediate => "INITIALLY IMMEDIATE",
                TriggerInitially::Deferred => "INITIALLY DEFERRED",
            })
            .unwrap();
        }
        if !create.referencing.is_empty() {
            sql.write_str(" REFERENCING").unwrap();
            for ref_table in &create.referencing {
                sql.write_str(" ").unwrap();
                match ref_table {
                    TriggerReferencing::OldTable(name) => {
                        sql.write_str("OLD TABLE AS ").unwrap();
                        self.prepare_iden(name, sql);
                    }
                    TriggerReferencing::NewTable(name) => {
                        sql.write_str("NEW TABLE AS ").unwrap();
                        self.prepare_iden(name, sql);
                    }
                }
            }
        }
        if let Some(each) = &create.each {
            sql.write_str(" FOR EACH ").unwrap();
            sql.write_str(match each {
                TriggerEach::Row => "ROW",
                TriggerEach::Statement => "STATEMENT",
            })
            .unwrap();
        }
        if let Some(condition) = &create.r#when {
            sql.write_str(" WHEN (").unwrap();
            self.prepare_expr(condition, sql);
            sql.write_str(")").unwrap();
        }
        if let Some(function) = &create.function {
            sql.write_str(" EXECUTE FUNCTION ").unwrap();
            self.prepare_iden(function, sql);
            sql.write_str("(").unwrap();
            for (i, arg) in create.function_args.iter().enumerate() {
                if i > 0 {
                    sql.write_str(", ").unwrap();
                }
                self.prepare_expr(arg, sql);
            }
            sql.write_str(")").unwrap();
        }
    }

    fn prepare_trigger_alter_statement(
        &self,
        alter: &TriggerAlterStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("ALTER TRIGGER ").unwrap();
        if let Some(name) = &alter.name {
            self.prepare_iden(name, sql);
        }
        if let Some(table) = &alter.table {
            sql.write_str(" ON ").unwrap();
            match table {
                TableRef::Table(table_name, _) => self.prepare_table_name(table_name, sql),
                _ => panic!("Expected TableRef::Table"),
            }
        }
        if let Some(option) = &alter.option {
            match option {
                TriggerAlterOption::RenameTo(new_name) => {
                    sql.write_str(" RENAME TO ").unwrap();
                    self.prepare_iden(new_name, sql);
                }
                TriggerAlterOption::DependsOnExtension(extension_name) => {
                    sql.write_str(" DEPENDS ON EXTENSION ").unwrap();
                    self.prepare_iden(extension_name, sql);
                }
                TriggerAlterOption::NoDependsOnExtension(extension_name) => {
                    sql.write_str(" NO DEPENDS ON EXTENSION ").unwrap();
                    self.prepare_iden(extension_name, sql);
                }
            }
        }
    }

    fn prepare_trigger_drop_statement(
        &self,
        drop: &TriggerDropStatement,
        sql: &mut impl SqlWriter,
    ) {
        sql.write_str("DROP TRIGGER ").unwrap();
        if drop.if_exists {
            sql.write_str("IF EXISTS ").unwrap();
        }
        if let Some(name) = &drop.name {
            self.prepare_iden(name, sql);
        }
        if let Some(table) = &drop.table {
            sql.write_str(" ON ").unwrap();
            match table {
                TableRef::Table(table_name, _) => self.prepare_table_name(table_name, sql),
                _ => panic!("Expected TableRef::Table"),
            }
        }
        if drop.cascade {
            sql.write_str(" CASCADE").unwrap();
        }
        if drop.restrict {
            sql.write_str(" RESTRICT").unwrap();
        }
    }
}

const QUOTE: Quote = Quote(b'"', b'"');

impl GenericBuilder for PostgresQueryBuilder {}

impl SchemaBuilder for PostgresQueryBuilder {}

impl QuotedBuilder for PostgresQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

// https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-BACKSLASH-TABLE
impl EscapeBuilder for PostgresQueryBuilder {
    fn needs_escape(&self, s: &str) -> bool {
        s.chars().any(|c| match c {
            '\x08' | '\x0C' | '\n' | '\r' | '\t' | '\\' | '\'' | '\0' => true,
            c if c.is_ascii_control() => true,
            _ => false,
        })
    }

    fn write_escaped(&self, buffer: &mut impl Write, string: &str) {
        for c in string.chars() {
            match c {
                '\x08' => buffer.write_str(r#"\b"#),
                '\x0C' => buffer.write_str(r#"\f"#),
                '\n' => buffer.write_str(r"\n"),
                '\r' => buffer.write_str(r"\r"),
                '\t' => buffer.write_str(r"\t"),
                '\\' => buffer.write_str(r#"\\"#),
                '\'' => buffer.write_str(r#"\'"#),
                '\0' => buffer.write_str(r#"\0"#),
                c if c.is_ascii_control() => write!(buffer, "\\{:03o}", c as u32),
                _ => buffer.write_char(c),
            }
            .unwrap();
        }
    }

    fn unescape_string(&self, string: &str) -> String {
        let mut chars = string.chars().peekable();
        let mut result = String::with_capacity(string.len());

        while let Some(c) = chars.next() {
            if c != '\\' {
                result.push(c);
                continue;
            }

            let Some(next) = chars.next() else {
                result.push('\\');
                continue;
            };

            match next {
                'b' => result.push('\x08'),
                'f' => result.push('\x0C'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                '0' => result.push('\0'),
                '\'' => result.push('\''),
                '\\' => result.push('\\'),
                'u' => {
                    let mut hex = String::new();
                    for _ in 0..4 {
                        if let Some(h) = chars.next() {
                            hex.push(h);
                        }
                    }
                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = std::char::from_u32(code) {
                            result.push(ch);
                        }
                    }
                }
                'U' => {
                    let mut hex = String::new();
                    for _ in 0..8 {
                        if let Some(h) = chars.next() {
                            hex.push(h);
                        }
                    }
                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = std::char::from_u32(code) {
                            result.push(ch);
                        }
                    }
                }
                c @ '0'..='7' => {
                    let mut oct = String::new();
                    oct.push(c);
                    for _ in 0..2 {
                        if let Some(next_o) = chars.peek() {
                            if ('0'..='7').contains(next_o) {
                                oct.push(chars.next().unwrap());
                            }
                        }
                    }
                    if let Ok(val) = u8::from_str_radix(&oct, 8) {
                        result.push(val as char);
                    }
                }
                other => {
                    result.push('\\');
                    result.push(other);
                }
            }
        }

        result
    }
}

impl TableRefBuilder for PostgresQueryBuilder {}

#[cfg(test)]
mod tests {
    use crate::{EscapeBuilder, PostgresQueryBuilder, IntoIden, ExprTrait};
    use crate::extension::postgres::{FunctionBuilder, TriggerBuilder};

    #[test]
    fn test_write_escaped() {
        let escaper = PostgresQueryBuilder;

        let control_chars: String = (0u8..=31).map(|b| b as char).collect();

        let escaped = escaper.escape_string(&control_chars);

        assert!(escaped.contains(r"\b")); // 0x08
        assert!(escaped.contains(r"\f")); // 0x0C
        assert!(escaped.contains(r"\n")); // 0x0A
        assert!(escaped.contains(r"\r")); // 0x0D
        assert!(escaped.contains(r"\t")); // 0x09
        assert!(escaped.contains(r"\0")); // 0x00

        for b in 0u8..=31 {
            let c = b as char;
            if !matches!(c, '\x00' | '\x08' | '\x09' | '\x0A' | '\x0C' | '\x0D') {
                let octal = format!("\\{b:03o}");
                assert!(escaped.contains(&octal));
            }
        }
    }

    #[test]
    fn test_unescape_string() {
        let escaper = PostgresQueryBuilder;

        let escaped = r"\b\f\n\r\t\0\'\\\101\102\103\u4F60\U0001F600";
        let unescaped = escaper.unescape_string(escaped);

        let expected = "\x08\x0C\n\r\t\0'\\ABC你😀";

        assert_eq!(unescaped, expected);

        let escaped_expected = escaper.escape_string(expected);

        // We don't convert ASCII chars back to octal in escaping
        assert_eq!(r"\b\f\n\r\t\0\'\\ABC你😀", escaped_expected);
    }

    #[test]
    fn test_function_create() {
        use crate::{Alias, ColumnType, Expr};
        use crate::extension::postgres::{FunctionArg, FunctionArgMode, FunctionBehavior, FunctionCreateStatement, FunctionReturns};

        let mut basic_function_stmt = FunctionCreateStatement::new();
        basic_function_stmt.name(Alias::new("my_func"))
            .arg(FunctionArg::new(ColumnType::Integer).name(Alias::new("a")))
            .returns(FunctionReturns::Type(ColumnType::Integer))
            .language(Alias::new("plpgsql"))
            .as_definition("BEGIN RETURN a + 1; END;");
        assert_eq!(
            basic_function_stmt.to_string(PostgresQueryBuilder),
            r#"CREATE FUNCTION "my_func" ("a" integer) RETURNS integer LANGUAGE "plpgsql" AS 'BEGIN RETURN a + 1; END;'"#
        );

        let mut replace_function_stmt = FunctionCreateStatement::new();
        replace_function_stmt.or_replace()
            .name(Alias::new("complex_func"))
            .arg(FunctionArg::new(ColumnType::Integer).mode(FunctionArgMode::In).name(Alias::new("x")).default(Expr::val(0)))
            .arg(FunctionArg::new(ColumnType::Text).mode(FunctionArgMode::Out).name(Alias::new("y")))
            .returns(FunctionReturns::Table(vec![
                (Alias::new("id").into_iden(), ColumnType::Integer),
                (Alias::new("val").into_iden(), ColumnType::Text),
            ]))
            .language(Alias::new("sql"))
            .behavior(FunctionBehavior::Immutable)
            .behavior(FunctionBehavior::Strict)
            .behavior(FunctionBehavior::SecurityDefiner)
            .sql_body("SELECT x, 'hello';");
        assert_eq!(
            replace_function_stmt.to_string(PostgresQueryBuilder),
            r#"CREATE OR REPLACE FUNCTION "complex_func" (IN "x" integer DEFAULT 0, OUT "y" text) RETURNS TABLE ("id" integer, "val" text) LANGUAGE "sql" IMMUTABLE STRICT SECURITY DEFINER AS $$ SELECT x, 'hello'; $$"#
        );
    }

    #[test]
    fn test_function_alter() {
        use crate::{Alias, ColumnType};
        use crate::extension::postgres::{FunctionAlterStatement, FunctionBehavior};

        let mut rename_function_stmt = FunctionAlterStatement::new();
        rename_function_stmt.name(Alias::new("old_func"))
            .arg_types([ColumnType::Integer, ColumnType::Text])
            .rename_to(Alias::new("new_func"));
        assert_eq!(
            rename_function_stmt.to_string(PostgresQueryBuilder),
            r#"ALTER FUNCTION "old_func" (integer, text) RENAME TO "new_func""#
        );

        let mut change_owner_function_stmt = FunctionAlterStatement::new();
        change_owner_function_stmt.name(Alias::new("my_func"))
            .owner_to(Alias::new("new_owner"))
            .set_schema(Alias::new("new_schema"));
        assert_eq!(
            change_owner_function_stmt.to_string(PostgresQueryBuilder),
            r#"ALTER FUNCTION "my_func" OWNER TO "new_owner" SET SCHEMA "new_schema""#
        );

        
        let mut behaviour_function_stmt = FunctionAlterStatement::new();
        behaviour_function_stmt.name(Alias::new("my_func"))
            .behavior(FunctionBehavior::Immutable)
            .leakproof(true)
            .cost(10.0)
            .rows(100.0)
            .set_config(Alias::new("search_path"), "public")
            .reset_config(Alias::new("search_path"))
            .reset_all()
            .restrict();
        assert_eq!(
            behaviour_function_stmt.to_string(PostgresQueryBuilder),
            r#"ALTER FUNCTION "my_func" IMMUTABLE LEAKPROOF COST 10 ROWS 100 SET "search_path" TO public RESET "search_path" RESET ALL RESTRICT"#
        );
    }

    #[test]
    fn test_function_drop() {
        use crate::{Alias, ColumnType};
        use crate::extension::postgres::FunctionDropStatement;

        let mut basic_drop_stmt = FunctionDropStatement::new();
        basic_drop_stmt.name(Alias::new("my_func"))
            .if_exists()
            .arg_types([ColumnType::Integer, ColumnType::Text])
            .cascade();
        assert_eq!(
            basic_drop_stmt.to_string(PostgresQueryBuilder),
            r#"DROP FUNCTION IF EXISTS "my_func" (integer, text) CASCADE"#
        );

        let mut drop_restrict_stmt = FunctionDropStatement::new();
        drop_restrict_stmt.name(Alias::new("my_func"))
            .restrict();
        assert_eq!(
            drop_restrict_stmt.to_string(PostgresQueryBuilder),
            r#"DROP FUNCTION "my_func" RESTRICT"#
        );
    }

    #[test]
    fn test_trigger_create() {
        use crate::{Alias, Expr};
        use crate::extension::postgres::{TriggerCreateStatement, TriggerEvent};

        let mut before_insert_trigger = TriggerCreateStatement::new();
        before_insert_trigger.name(Alias::new("my_trigger"))
            .before()
            .event(TriggerEvent::Insert)
            .table(Alias::new("my_table"))
            .for_each_row()
            .function(Alias::new("my_trigger_func"));
        assert_eq!(
            before_insert_trigger.to_string(PostgresQueryBuilder),
            r#"CREATE TRIGGER "my_trigger" BEFORE INSERT ON "my_table" FOR EACH ROW EXECUTE FUNCTION "my_trigger_func"()"#
        );

        let mut replace_after_delete_trigger = TriggerCreateStatement::new();
        replace_after_delete_trigger.or_replace()
            .constraint()
            .name(Alias::new("my_constraint_trig"))
            .after()
            .event(TriggerEvent::Delete)
            .table(Alias::new("my_table"))
            .from_table(Alias::new("other_table"))
            .deferrable(true)
            .initially_deferred()
            .for_each_row()
            .function(Alias::new("my_trigger_func"));
        assert_eq!(
            replace_after_delete_trigger.to_string(PostgresQueryBuilder),
            r#"CREATE OR REPLACE CONSTRAINT TRIGGER "my_constraint_trig" AFTER DELETE ON "my_table" FROM "other_table" DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION "my_trigger_func"()"#
        );

        let mut update_col_trigger = TriggerCreateStatement::new();
        update_col_trigger.name(Alias::new("complex_trigger"))
            .before()
            .event(TriggerEvent::Update(vec![Alias::new("col1").into_iden(), Alias::new("col2").into_iden()]))
            .table(Alias::new("my_table"))
            .referencing_old_table(Alias::new("old_t"))
            .referencing_new_table(Alias::new("new_t"))
            .for_each_statement()
            .r#when(Expr::col(Alias::new("col1")).gt(10))
            .function(Alias::new("my_proc"))
            .function_arg(Expr::val("arg1"))
            .function_arg(Expr::val(42));
        assert_eq!(
            update_col_trigger.to_string(PostgresQueryBuilder),
            r#"CREATE TRIGGER "complex_trigger" BEFORE UPDATE OF "col1", "col2" ON "my_table" REFERENCING OLD TABLE AS "old_t" NEW TABLE AS "new_t" FOR EACH STATEMENT WHEN ("col1" > 10) EXECUTE FUNCTION "my_proc"('arg1', 42)"#
        );
    }

    #[test]
    fn test_trigger_alter() {
        use crate::Alias;
        use crate::extension::postgres::TriggerAlterStatement;

        let mut rename_trigger = TriggerAlterStatement::new();
        rename_trigger.name(Alias::new("old_trig"))
            .table(Alias::new("my_table"))
            .rename_to(Alias::new("new_trig"));
        assert_eq!(
            rename_trigger.to_string(PostgresQueryBuilder),
            r#"ALTER TRIGGER "old_trig" ON "my_table" RENAME TO "new_trig""#
        );

        let mut depends_on_trigger = TriggerAlterStatement::new();
        depends_on_trigger.name(Alias::new("my_trig"))
            .table(Alias::new("my_table"))
            .depends_on_extension(Alias::new("my_ext"));
        assert_eq!(
            depends_on_trigger.to_string(PostgresQueryBuilder),
            r#"ALTER TRIGGER "my_trig" ON "my_table" DEPENDS ON EXTENSION "my_ext""#
        );

        let mut no_depends_on_trigger = TriggerAlterStatement::new();
        no_depends_on_trigger.name(Alias::new("my_trig"))
            .table(Alias::new("my_table"))
            .no_depends_on_extension(Alias::new("my_ext"));
        assert_eq!(
            no_depends_on_trigger.to_string(PostgresQueryBuilder),
            r#"ALTER TRIGGER "my_trig" ON "my_table" NO DEPENDS ON EXTENSION "my_ext""#
        );
    }

    #[test]
    fn test_trigger_drop() {
        use crate::Alias;
        use crate::extension::postgres::TriggerDropStatement;

        let mut drop_exists_trigger = TriggerDropStatement::new();
        drop_exists_trigger.name(Alias::new("my_trigger"))
            .table(Alias::new("my_table"))
            .if_exists()
            .cascade();
        assert_eq!(
            drop_exists_trigger.to_string(PostgresQueryBuilder),
            r#"DROP TRIGGER IF EXISTS "my_trigger" ON "my_table" CASCADE"#
        );

        let mut drop_restrict_trigger = TriggerDropStatement::new();
        drop_restrict_trigger.name(Alias::new("my_trigger"))
            .table(Alias::new("my_table"))
            .restrict();
        assert_eq!(
            drop_restrict_trigger.to_string(PostgresQueryBuilder),
            r#"DROP TRIGGER "my_trigger" ON "my_table" RESTRICT"#
        );
    }
}
