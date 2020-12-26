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
    tokens.iter().map(|token| {
        if token == &Token::Punctuation("?".to_string()) {
            let string = value_to_string(&params[counter]);
            counter += 1;
            string
        } else {
            token.to_string()
        }
    }).collect::<String>()
}

impl SqlWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_param(&mut self) {
        write!(&mut self.string, "?").unwrap();
        self.counter += 1;
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
}