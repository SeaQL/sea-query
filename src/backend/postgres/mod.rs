pub(crate) mod constraint;
pub(crate) mod extension;
pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod types;

use super::*;

/// Postgres query builder.
#[derive(Default, Debug)]
pub struct PostgresQueryBuilder;

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
                let octal = format!("\\{:03o}", b);
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
