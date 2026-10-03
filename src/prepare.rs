//! Helper for preparing SQL statements.

use crate::{token::TokenizerBackend, *};
pub use std::fmt::Write;

pub trait SqlWriter: Write + Sized + ToString {
    fn push_param<T: QueryBuilder>(&mut self, value: Value, query_builder: &T);

    /// Upcast this into parent trait. Still needed in 1.85
    fn as_writer(&mut self) -> &mut dyn Write;
}

impl SqlWriter for String {
    fn push_param<T: QueryBuilder>(&mut self, value: Value, query_builder: &T) {
        query_builder.write_value(self, &value).unwrap();
    }

    fn as_writer(&mut self) -> &mut dyn Write {
        self as _
    }
}

#[derive(Debug, Clone)]
pub struct SqlWriterValues {
    counter: usize,
    placeholder: String,
    numbered: bool,
    string: String,
    values: Vec<Value>,
}

impl SqlWriterValues {
    pub fn new<T>(placeholder: T, numbered: bool) -> Self
    where
        T: Into<String>,
    {
        Self {
            counter: 0,
            placeholder: placeholder.into(),
            numbered,
            string: String::with_capacity(256),
            values: Vec::new(),
        }
    }

    pub fn into_parts(self) -> (String, Values) {
        (self.string, Values(self.values))
    }
}

impl Write for SqlWriterValues {
    #[inline]
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.string.write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.string.write_char(c)
    }
}

impl std::fmt::Display for SqlWriterValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string)
    }
}

impl SqlWriter for SqlWriterValues {
    fn push_param<T: QueryBuilder>(&mut self, value: Value, _: &T) {
        self.string.push_str(&self.placeholder);
        if self.numbered {
            self.counter += 1;
            write_int(&mut self.string, self.counter);
        }
        self.values.push(value)
    }

    fn as_writer(&mut self) -> &mut dyn Write {
        self as _
    }
}

#[cfg(feature = "itoa")]
#[inline]
pub(crate) fn write_int(w: &mut (impl Write + ?Sized), n: impl itoa::Integer) {
    let mut buf = itoa::Buffer::new();
    let s = buf.format(n);
    w.write_str(s).unwrap();
}

#[cfg(not(feature = "itoa"))]
#[inline(always)]
pub(crate) fn write_int(w: &mut (impl Write + ?Sized), n: impl std::fmt::Display) {
    write!(w, "{n}").unwrap();
}

pub fn inject_parameters(sql: &str, params: &[Value], query_builder: &impl QueryBuilder) -> String {
    let mut counter = 0;
    let mut output = String::new();
    let backend = TokenizerBackend::from_query_builder(query_builder);
    let (placeholder, numbered) = query_builder.placeholder();

    let mut tokenizer = Tokenizer::new(sql).for_backend(backend).iter().peekable();

    while let Some(token) = tokenizer.next() {
        match token {
            Token::Punctuation(mark) if mark == placeholder => {}
            _ => {
                output.push_str(token.as_str());
                continue;
            }
        }

        match tokenizer.peek() {
            Some(Token::Unquoted(next)) if backend == TokenizerBackend::Sqlite => {
                let number_end = next.bytes().take_while(u8::is_ascii_digit).count();
                let (number, suffix) = next.split_at(number_end);
                if let Ok(number) = number.parse::<usize>() {
                    query_builder
                        .write_value(&mut output, &params[number - 1])
                        .unwrap();

                    // Anonymous SQLite parameters follow the largest assigned index.
                    counter = std::cmp::max(counter, number);
                    if !suffix.is_empty() {
                        output.push(' ');
                        output.push_str(suffix);
                    }
                    tokenizer.next();
                    continue;
                }
            }
            Some(Token::Unquoted(next)) if numbered => {
                if let Ok(number) = next.parse::<usize>() {
                    query_builder
                        .write_value(&mut output, &params[number - 1])
                        .unwrap();

                    tokenizer.next();
                    continue;
                }
            }
            _ => {}
        }

        if !numbered {
            query_builder
                .write_value(&mut output, &params[counter])
                .unwrap();
            counter += 1;
        } else {
            output.push_str(placeholder);
        }
    }

    output
}

#[cfg(test)]
#[cfg(feature = "backend-sqlite")]
mod tests_sqlite {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn numbered_parameters() {
        assert_eq!(
            inject_parameters(
                "SELECT * FROM products WHERE (minimum_stock > ?1 OR current_stock < ?1) AND category_id = ?2;",
                &[10.into(), 4.into()],
                &SqliteQueryBuilder,
            ),
            "SELECT * FROM products WHERE (minimum_stock > 10 OR current_stock < 10) AND category_id = 4;"
        );

        let params: Vec<Value> = (1..=12).map(Value::from).collect();
        assert_eq!(
            inject_parameters("SELECT ?12, ?2, ?01, ?12", &params, &SqliteQueryBuilder),
            "SELECT 12, 2, 1, 12"
        );
    }

    #[test]
    fn aliases_after_numbered_parameters_preserved() {
        assert_eq!(
            inject_parameters(
                "SELECT ?1stock_threshold",
                &[10.into()],
                &SqliteQueryBuilder
            ),
            "SELECT 10 stock_threshold"
        );
    }

    #[test]
    fn anonymous_parameters_use_right_index() {
        let params = [10.into(), 20.into(), 30.into(), 40.into(), 50.into()];
        assert_eq!(
            inject_parameters("SELECT ?2, ?, ?1, ?, ?", &params, &SqliteQueryBuilder),
            "SELECT 20, 30, 10, 40, 50"
        );
        assert_eq!(
            inject_parameters("SELECT ?, ?1, ?", &params, &SqliteQueryBuilder),
            "SELECT 10, 10, 20"
        );
    }
}

#[cfg(test)]
#[cfg(feature = "backend-mysql")]
mod tests_mysql {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn inject_parameters_1() {
        assert_eq!(
            inject_parameters("WHERE A = ?", &["B".into()], &MysqlQueryBuilder),
            "WHERE A = 'B'"
        );
    }

    #[test]
    fn inject_parameters_2() {
        assert_eq!(
            inject_parameters("WHERE A = '?' AND B = ?", &["C".into()], &MysqlQueryBuilder),
            "WHERE A = '?' AND B = 'C'"
        );
    }

    #[test]
    fn inject_parameters_3() {
        assert_eq!(
            inject_parameters(
                "WHERE A = ? AND C = ?",
                &["B".into(), "D".into()],
                &MysqlQueryBuilder
            ),
            "WHERE A = 'B' AND C = 'D'"
        );
    }

    #[test]
    fn inject_parameters_4() {
        assert_eq!(
            inject_parameters("?", &[vec![0xABu8, 0xCD, 0xEF].into()], &MysqlQueryBuilder),
            "x'ABCDEF'"
        );
    }
}

#[cfg(test)]
#[cfg(feature = "backend-postgres")]
mod tests_postgres {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn inject_parameters_5() {
        assert_eq!(
            inject_parameters(
                "WHERE A = $1 AND C = $2",
                &["B".into(), "D".into()],
                &PostgresQueryBuilder
            ),
            "WHERE A = 'B' AND C = 'D'"
        );
    }

    #[test]
    fn inject_parameters_6() {
        assert_eq!(
            inject_parameters(
                "WHERE A = $2 AND C = $1",
                &["B".into(), "D".into()],
                &PostgresQueryBuilder
            ),
            "WHERE A = 'D' AND C = 'B'"
        );
    }

    #[test]
    fn inject_parameters_7() {
        assert_eq!(
            inject_parameters("WHERE A = $1", &[Value::from("B'C")], &PostgresQueryBuilder),
            "WHERE A = E'B\\'C'"
        );
    }
}
