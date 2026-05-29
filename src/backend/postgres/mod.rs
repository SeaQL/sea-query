pub(crate) mod extension;
pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod types;

use super::*;
use crate::extension::postgres::*;

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
                    FunctionArgMode::Inout => "INOUT ",
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
    use crate::{EscapeBuilder, PostgresQueryBuilder};

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
}
