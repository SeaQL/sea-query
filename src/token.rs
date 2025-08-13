#![allow(dead_code)]
use std::fmt::Write;
use std::iter::Iterator;

/// Tokenizer for processing SQL.
#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    c: Option<char>,
    p: usize,
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Token<'a> {
    Quoted(&'a str),
    Unquoted(&'a str),
    Space(&'a str),
    Punctuation(&'a str),
}

impl<'a> Tokenizer<'a> {
    pub fn new(string: &'a str) -> Self {
        let mut chars = string.chars();
        let c = chars.next();
        Self {
            input: string,
            chars,
            c,
            p: 0,
        }
    }

    pub fn iter(self) -> impl Iterator<Item = Token<'a>> {
        self
    }

    fn get(&self) -> char {
        self.c.unwrap()
    }

    fn inc(&mut self) {
        let c = self.get();
        self.c = self.chars.next();
        self.p += c.len_utf8();
    }

    fn end(&self) -> bool {
        self.c.is_none()
    }

    fn p_c(&self, c: char) -> usize {
        self.p + c.len_utf8()
    }

    fn space(&mut self) -> Option<Token<'a>> {
        let a = self.p;
        let mut b = a;

        while !self.end() {
            let c = self.get();
            if Self::is_space(c) {
                b = self.p_c(c);
            } else {
                break;
            }
            self.inc();
        }

        if a == b {
            None
        } else {
            Some(Token::Space(&self.input[a..b]))
        }
    }

    fn unquoted(&mut self) -> Option<Token<'a>> {
        let a = self.p;
        let mut b = a;

        let mut first = true;
        while !self.end() {
            let c = self.get();
            if Self::is_alphanumeric(c) {
                b = self.p_c(c);
                first = false;
                self.inc();
            } else if !first && Self::is_identifier(c) {
                b = self.p_c(c);
                self.inc();
            } else {
                break;
            }
        }

        if a == b {
            None
        } else {
            Some(Token::Unquoted(&self.input[a..b]))
        }
    }

    fn quoted(&mut self) -> Option<Token<'a>> {
        let a = self.p;
        let mut b = a;

        let mut first = true;
        let mut escape = false;
        let mut start = ' ';
        while !self.end() {
            let c = self.get();
            if first && Self::is_string_delimiter_start(c) {
                b = self.p_c(c);
                first = false;
                start = c;
                self.inc();
            } else if !first && !escape && Self::is_string_delimiter_end_for(start, c) {
                b = self.p_c(c);
                self.inc();
                if self.end() {
                    break;
                }
                if Self::is_string_escape_for(start, self.get()) {
                    b = self.p_c(c);
                    self.inc();
                } else {
                    break;
                }
            } else if !first {
                escape = !escape && Self::is_escape_char(c);
                b = self.p_c(c);
                self.inc();
            } else {
                break;
            }
        }
        if a == b {
            None
        } else {
            Some(Token::Quoted(&self.input[a..b]))
        }
    }

    /// unquote a quoted string
    fn unquote(mut self) -> String {
        let mut string = String::new();
        let mut first = true;
        let mut escape = false;
        let mut start = ' ';
        while !self.end() {
            let c = self.get();
            if first && Self::is_string_delimiter_start(c) {
                first = false;
                start = c;
                self.inc();
            } else if !first && !escape && Self::is_string_delimiter_end_for(start, c) {
                self.inc();
                if self.end() {
                    break;
                }
                if Self::is_string_escape_for(start, self.get()) {
                    write!(string, "{c}").unwrap();
                    self.inc();
                } else {
                    break;
                }
            } else if !first {
                escape = !escape && Self::is_escape_char(c);
                write!(string, "{c}").unwrap();
                self.inc();
            } else {
                break;
            }
        }
        string
    }

    fn punctuation(&mut self) -> Option<Token<'a>> {
        let a = self.p;
        let mut b = a;

        if !self.end() {
            let c = self.get();
            if !Self::is_space(c) && !Self::is_alphanumeric(c) {
                b = self.p_c(c);
                self.inc();
            }
        }

        if a == b {
            None
        } else {
            Some(Token::Punctuation(&self.input[a..b]))
        }
    }

    fn is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\r' | '\n')
    }

    fn is_identifier(c: char) -> bool {
        matches!(c, '_' | '$')
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_alphabetic() || c.is_ascii_digit()
    }

    fn is_string_delimiter_start(c: char) -> bool {
        matches!(c, '`' | '[' | '\'' | '"')
    }

    fn is_string_escape_for(start: char, c: char) -> bool {
        match start {
            '`' => c == '`',
            '\'' => c == '\'',
            '"' => c == '"',
            _ => false,
        }
    }

    fn is_string_delimiter_end_for(start: char, c: char) -> bool {
        match start {
            '`' => c == '`',
            '[' => c == ']',
            '\'' => c == '\'',
            '"' => c == '"',
            _ => false,
        }
    }

    fn is_escape_char(c: char) -> bool {
        c == '\\'
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(space) = self.space() {
            return Some(space);
        }
        if let Some(unquoted) = self.unquoted() {
            return Some(unquoted);
        }
        if let Some(quoted) = self.quoted() {
            return Some(quoted);
        }
        if let Some(punctuation) = self.punctuation() {
            return Some(punctuation);
        }
        None
    }
}

impl Token<'_> {
    pub fn is_quoted(&self) -> bool {
        matches!(self, Self::Quoted(_))
    }

    pub fn is_unquoted(&self) -> bool {
        matches!(self, Self::Unquoted(_))
    }

    pub fn is_space(&self) -> bool {
        matches!(self, Self::Space(_))
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::Punctuation(_))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Quoted(string)
            | Self::Unquoted(string)
            | Self::Space(string)
            | Self::Punctuation(string) => string,
        }
    }

    pub fn unquote(&self) -> Option<String> {
        if self.is_quoted() {
            let tokenizer = Tokenizer::new(self.as_str());
            Some(tokenizer.unquote())
        } else {
            None
        }
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let tokenizer = Tokenizer::new("");
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_1() {
        let string = "SELECT * FROM `character`";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Punctuation("*"),
                Token::Space(" "),
                Token::Unquoted("FROM"),
                Token::Space(" "),
                Token::Quoted("`character`"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_2() {
        let string = "SELECT * FROM `character` WHERE id = ?";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Punctuation("*"),
                Token::Space(" "),
                Token::Unquoted("FROM"),
                Token::Space(" "),
                Token::Quoted("`character`"),
                Token::Space(" "),
                Token::Unquoted("WHERE"),
                Token::Space(" "),
                Token::Unquoted("id"),
                Token::Space(" "),
                Token::Punctuation("="),
                Token::Space(" "),
                Token::Punctuation("?"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_3() {
        let string = r#"? = "?" "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("?"),
                Token::Space(" "),
                Token::Punctuation("="),
                Token::Space(" "),
                Token::Quoted(r#""?""#),
                Token::Space(" "),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_4() {
        let string = r#""a\"bc""#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![Token::Quoted("\"a\\\"bc\"")]);
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_5() {
        let string = "abc123";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![Token::Unquoted(string)]);
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_6() {
        let string = "2.3*4";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("2"),
                Token::Punctuation("."),
                Token::Unquoted("3"),
                Token::Punctuation("*"),
                Token::Unquoted("4"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_7() {
        let string = r#""a\\" B"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Quoted("\"a\\\\\""),
                Token::Space(" "),
                Token::Unquoted("B"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_8() {
        let string = r#"`a"b` "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![Token::Quoted("`a\"b`"), Token::Space(" ")]);
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_9() {
        let string = r"[ab] ";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![Token::Quoted("[ab]"), Token::Space(" ")]);
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_10() {
        let string = r#" 'a"b' "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" "),
                Token::Quoted("'a\"b'"),
                Token::Space(" "),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_11() {
        let string = r" `a``b` ";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" "),
                Token::Quoted("`a``b`"),
                Token::Space(" "),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_12() {
        let string = r" 'a''b' ";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" "),
                Token::Quoted("'a''b'"),
                Token::Space(" "),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_13() {
        let string = r"(?)";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("("),
                Token::Punctuation("?"),
                Token::Punctuation(")"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_14() {
        let string = r"($1 = $2)";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("("),
                Token::Punctuation("$"),
                Token::Unquoted("1"),
                Token::Space(" "),
                Token::Punctuation("="),
                Token::Space(" "),
                Token::Punctuation("$"),
                Token::Unquoted("2"),
                Token::Punctuation(")"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_15() {
        let string = r#" "Hello World" "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" "),
                Token::Quoted("\"Hello World\""),
                Token::Space(" "),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_16() {
        let string = "abc_$123";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![Token::Unquoted(string)]);
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_17() {
        let string = "$abc$123";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![Token::Punctuation("$"), Token::Unquoted("abc$123"),]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_18() {
        let string = "_$abc_123$";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("_"),
                Token::Punctuation("$"),
                Token::Unquoted("abc_123$"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_19() {
        let string = r#""a\"bc""#;
        let tokenizer = Tokenizer::new(string);
        assert_eq!(tokenizer.unquote(), "a\\\"bc".to_owned());
    }

    #[test]
    fn test_20() {
        let string = r#""a""bc""#;
        let tokenizer = Tokenizer::new(string);
        assert_eq!(tokenizer.unquote(), "a\"bc".to_owned());
    }

    #[test]
    fn test_21() {
        assert_eq!(
            Token::Quoted("'a\\nb'").unquote().unwrap(),
            "a\\nb".to_owned()
        );
    }

    #[test]
    fn test_22() {
        let string = r#" "Hello\nWorld" "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" "),
                Token::Quoted("\"Hello\\nWorld\""),
                Token::Space(" "),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_23() {
        let string = "{ab} '{cd}'";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("{"),
                Token::Unquoted("ab"),
                Token::Punctuation("}"),
                Token::Space(" "),
                Token::Quoted("'{cd}'"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_24() {
        let string = r#"新"老虎","#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("新"),
                Token::Quoted("\"老虎\""),
                Token::Punctuation(","),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_25() {
        let string = r#"{a.1:2}"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("{"),
                Token::Unquoted("a"),
                Token::Punctuation("."),
                Token::Unquoted("1"),
                Token::Punctuation(":"),
                Token::Unquoted("2"),
                Token::Punctuation("}"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }

    #[test]
    fn test_26() {
        let string = r#"{..(a.1:2)}"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("{"),
                Token::Punctuation("."),
                Token::Punctuation("."),
                Token::Punctuation("("),
                Token::Unquoted("a"),
                Token::Punctuation("."),
                Token::Unquoted("1"),
                Token::Punctuation(":"),
                Token::Unquoted("2"),
                Token::Punctuation(")"),
                Token::Punctuation("}"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(super::Token::as_str).collect::<String>()
        );
    }
}
