//! Helper for preparing SQL statements.

use crate::*;
pub use std::fmt::Write;

#[derive(Debug, Default)]
pub struct SqlWriter {
    pub(crate) counter: usize,
    pub(crate) string: String,
}

pub fn inject_parameters(
    sql: &str,
    params: &[&dyn QueryValue],
    query_builder: &dyn QueryBuilder,
) -> String
// where
    // I: IntoIterator<Item = &'a dyn QueryValue>,
{
    let params: Vec<_> = params.into_iter().collect();
    let tokenizer = Tokenizer::new(sql);
    let tokens: Vec<Token> = tokenizer.iter().collect();
    let mut counter = 0;
    let mut output = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        match token {
            Token::Punctuation(mark) => {
                if (mark.as_ref(), false) == query_builder.placeholder() {
                    output.push(params[counter].query_value(query_builder));
                    counter += 1;
                    i += 1;
                    continue;
                } else if (mark.as_ref(), true) == query_builder.placeholder()
                    && i + 1 < tokens.len()
                {
                    if let Token::Unquoted(next) = &tokens[i + 1] {
                        if let Ok(num) = next.parse::<usize>() {
                            output.push(params[num - 1].query_value(query_builder));
                            i += 2;
                            continue;
                        }
                    }
                }
                output.push(mark.to_string())
            }
            _ => output.push(token.to_string()),
        }
        i += 1;
    }
    output.into_iter().collect()
}

impl SqlWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_param(&mut self, sign: &str, numbered: bool) {
        self.counter += 1;
        if numbered {
            let counter = self.counter;
            write!(self, "{}{}", sign, counter).unwrap();
        } else {
            write!(self, "{}", sign).unwrap();
        }
    }

    pub fn result(self) -> String {
        self.string
    }

    fn skip_str(s: &str, n: usize) -> &str {
        let mut it = s.chars();
        for _ in 0..n {
            it.next();
        }
        it.as_str()
    }
}

impl std::fmt::Write for SqlWriter {
    fn write_str(&mut self, s: &str) -> std::result::Result<(), std::fmt::Error> {
        write!(
            self.string,
            "{}",
            if self.string.ends_with(' ') && s.starts_with(' ') {
                Self::skip_str(s, 1)
            } else {
                s
            }
        )
    }
}

#[cfg(test)]
#[cfg(feature = "backend-mysql")]
mod tests {
    use super::*;

    #[test]
    fn inject_parameters_1() {
        assert_eq!(
            inject_parameters("WHERE A = ?", &[&"B"], &MysqlQueryBuilder),
            "WHERE A = 'B'"
        );
    }

    #[test]
    fn inject_parameters_2() {
        assert_eq!(
            inject_parameters("WHERE A = '?' AND B = ?", &[&"C"], &MysqlQueryBuilder),
            "WHERE A = '?' AND B = 'C'"
        );
    }

    #[test]
    fn inject_parameters_3() {
        assert_eq!(
            inject_parameters("WHERE A = ? AND C = ?", &[&"B", &"D"], &MysqlQueryBuilder),
            "WHERE A = 'B' AND C = 'D'"
        );
    }

    #[test]
    fn inject_parameters_4() {
        assert_eq!(
            inject_parameters(
                "WHERE A = $1 AND C = $2",
                &[&"B", &"D"],
                &PostgresQueryBuilder
            ),
            "WHERE A = 'B' AND C = 'D'"
        );
    }

    #[test]
    fn inject_parameters_5() {
        assert_eq!(
            inject_parameters(
                "WHERE A = $2 AND C = $1",
                &[&"B", &"D"],
                &PostgresQueryBuilder
            ),
            "WHERE A = 'D' AND C = 'B'"
        );
    }

    #[test]
    fn inject_parameters_6() {
        assert_eq!(
            inject_parameters("WHERE A = $1", &[&"B'C"], &PostgresQueryBuilder),
            "WHERE A = E'B\\'C'"
        );
    }

    #[test]
    fn inject_parameters_7() {
        assert_eq!(
            inject_parameters("?", &[&vec![0xABu8, 0xCD, 0xEF]], &MysqlQueryBuilder),
            "x'ABCDEF'"
        );
    }
}
