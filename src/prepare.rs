use crate::*;
pub use std::fmt::Write;

#[derive(Default)]
pub struct SqlWriter {
    pub counter: usize,
    pub string: String,
}

pub fn inject_parameters(sql: &str, params: Vec<Value>) -> String {
    let tokenizer = Tokenizer::new(sql);
    let tokens: Vec<Token> = tokenizer.iter().collect();
    let mut counter = 0;
    let mut output = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        match token {
            Token::Punctuation(mark) => {
                if mark == "?" {
                    output.push(value_to_string(&params[counter]));
                    counter += 1;
                    i += 1;
                    continue;
                } else if mark == "$" && i + 1 < tokens.len() {
                    if let Token::Unquoted(next) = &tokens[i + 1] {
                        if let Ok(num) = next.parse::<usize>() {
                            output.push(value_to_string(&params[num - 1]));
                            i += 2;
                            continue;
                        }
                    }
                }
                output.push(mark.to_string())
            },
            _ => output.push(token.to_string())
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
            write!(&mut self.string, "{}{}", sign, self.counter).unwrap();
        } else {
            write!(&mut self.string, "{}", sign).unwrap();
        }
    }

    pub fn result(self) -> String {
        self.string
    }
}

impl std::fmt::Write for SqlWriter {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        write!(self.string, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inject_parameters_1() {
        assert_eq!(inject_parameters("WHERE A = ?", vec!["B".into()]), "WHERE A = 'B'"); 
    }

    #[test]
    fn inject_parameters_2() {
        assert_eq!(inject_parameters("WHERE A = '?' AND B = ?", vec!["C".into()]), "WHERE A = '?' AND B = 'C'"); 
    }

    #[test]
    fn inject_parameters_3() {
        assert_eq!(inject_parameters("WHERE A = ? AND C = ?", vec!["B".into(), "D".into()]), "WHERE A = 'B' AND C = 'D'"); 
    }

    #[test]
    fn inject_parameters_4() {
        assert_eq!(inject_parameters("WHERE A = $1 AND C = $2", vec!["B".into(), "D".into()]), "WHERE A = 'B' AND C = 'D'"); 
    }

    #[test]
    fn inject_parameters_5() {
        assert_eq!(inject_parameters("WHERE A = $2 AND C = $1", vec!["B".into(), "D".into()]), "WHERE A = 'D' AND C = 'B'"); 
    }
}