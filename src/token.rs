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
    backend: TokenizerBackend,
    next_single_quote_uses_backslash_escape: bool,
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Token<'a> {
    Quoted(&'a str),
    Unquoted(&'a str),
    Space(&'a str),
    Punctuation(&'a str),
    Comment(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenizerBackend {
    Mysql,
    Postgres,
    Sqlite,
}

impl TokenizerBackend {
    pub(crate) fn from_query_builder(query_builder: &impl crate::QueryBuilder) -> Self {
        let (_, numbered) = query_builder.placeholder();
        if numbered {
            Self::Postgres
        } else if query_builder.quote().left() == '`' {
            Self::Mysql
        } else {
            Self::Sqlite
        }
    }
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
            backend: TokenizerBackend::Mysql,
            next_single_quote_uses_backslash_escape: false,
        }
    }

    pub(crate) fn for_backend(mut self, backend: TokenizerBackend) -> Self {
        self.backend = backend;
        self
    }

    pub(crate) fn for_query_builder(self, query_builder: &impl crate::QueryBuilder) -> Self {
        self.for_backend(TokenizerBackend::from_query_builder(query_builder))
    }

    pub fn iter(self) -> impl Iterator<Item = Token<'a>> {
        self
    }

    fn get(&self) -> char {
        self.c.unwrap()
    }

    fn peek(&self) -> char {
        self.c.unwrap_or('\0')
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

        if a != b {
            Some(Token::Space(&self.input[a..b]))
        } else {
            None
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

        if a != b {
            let string = &self.input[a..b];
            self.next_single_quote_uses_backslash_escape =
                self.next_single_quote_is_postgres_escape_string(string);
            Some(Token::Unquoted(string))
        } else {
            None
        }
    }

    fn quoted(&mut self) -> Option<Token<'a>> {
        let a = self.p;
        let mut b = a;

        let mut first = true;
        let mut escape = false;
        let mut start = ' ';
        let mut uses_backslash_escape = false;
        while !self.end() {
            let c = self.get();
            if first && self.is_string_delimiter_start(c) {
                b = self.p_c(c);
                first = false;
                start = c;
                uses_backslash_escape = self.uses_backslash_escape_for(start);
                self.next_single_quote_uses_backslash_escape = false;
                self.inc();
            } else if !first && !escape && Self::is_string_delimiter_end_for(start, c) {
                b = self.p_c(c);
                self.inc();
                if self.end() {
                    break;
                }
                if !Self::is_string_escape_for(start, self.get()) {
                    break;
                } else {
                    b = self.p_c(c);
                    self.inc();
                }
            } else if !first {
                escape = !escape && Self::is_escape_char_for(start, c, uses_backslash_escape);
                b = self.p_c(c);
                self.inc();
            } else {
                break;
            }
        }
        if a != b {
            Some(Token::Quoted(&self.input[a..b]))
        } else {
            None
        }
    }

    /// unquote a quoted string
    fn unquote(mut self) -> String {
        let mut string = String::new();
        let mut first = true;
        let mut escape = false;
        let mut start = ' ';
        let mut uses_backslash_escape = false;
        while !self.end() {
            let c = self.get();
            if first && self.is_string_delimiter_start(c) {
                first = false;
                start = c;
                uses_backslash_escape = self.uses_backslash_escape_for(start);
                self.next_single_quote_uses_backslash_escape = false;
                self.inc();
            } else if !first && !escape && Self::is_string_delimiter_end_for(start, c) {
                self.inc();
                if self.end() {
                    break;
                }
                if !Self::is_string_escape_for(start, self.get()) {
                    break;
                } else {
                    string.write_char(c).unwrap();
                    self.inc();
                }
            } else if !first {
                escape = !escape && Self::is_escape_char_for(start, c, uses_backslash_escape);
                string.write_char(c).unwrap();
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

        if a != b {
            let string = &self.input[a..b];
            self.next_single_quote_uses_backslash_escape = false;
            if string == "-" && self.peek() == '-' {
                b = self.p_c('-');
                self.inc();
                while !self.end() {
                    let c = self.get();
                    if c == '\n' {
                        break;
                    } else {
                        b = self.p_c(c);
                    }
                    self.inc();
                }
                let string = &self.input[a..b];
                return Some(Token::Comment(string));
            } else if string == "/" && self.peek() == '*' {
                b = self.p_c('*');
                self.inc();
                while !self.end() {
                    let c = self.get();
                    b = self.p_c(c);
                    self.inc();
                    if c == '*' && self.peek() == '/' {
                        b = self.p_c('/');
                        self.inc();
                        break;
                    }
                }
                let string = &self.input[a..b];
                return Some(Token::Comment(string));
            }
            Some(Token::Punctuation(string))
        } else {
            None
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

    fn is_string_delimiter_start(&self, c: char) -> bool {
        match c {
            '`' | '\'' | '"' => true,
            '[' => self.backend != TokenizerBackend::Postgres,
            _ => false,
        }
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

    fn uses_backslash_escape_for(&self, start: char) -> bool {
        if start != '\'' {
            return true;
        }
        self.next_single_quote_uses_backslash_escape || self.backend == TokenizerBackend::Mysql
    }

    fn next_single_quote_is_postgres_escape_string(&self, prefix: &str) -> bool {
        self.backend == TokenizerBackend::Postgres
            && prefix.eq_ignore_ascii_case("E")
            && !self.end()
            && self.get() == '\''
    }

    fn is_escape_char_for(start: char, c: char, uses_backslash_escape: bool) -> bool {
        (start != '\'' || uses_backslash_escape) && c == '\\'
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
            Self::Quoted(string) => string,
            Self::Unquoted(string) => string,
            Self::Space(string) => string,
            Self::Punctuation(string) => string,
            Self::Comment(string) => string,
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_6() {
        let string = "2.3*4/5";
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
                Token::Punctuation("/"),
                Token::Unquoted("5"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_9_postgres_does_not_treat_brackets_as_quoted() {
        let string = r"ARRAY[$1, $2]";
        let tokenizer = Tokenizer::new(string).for_backend(TokenizerBackend::Postgres);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("ARRAY"),
                Token::Punctuation("["),
                Token::Punctuation("$"),
                Token::Unquoted("1"),
                Token::Punctuation(","),
                Token::Space(" "),
                Token::Punctuation("$"),
                Token::Unquoted("2"),
                Token::Punctuation("]"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_9_mysql_and_sqlite_keep_bracketed_quoting() {
        for backend in [TokenizerBackend::Mysql, TokenizerBackend::Sqlite] {
            let string = r"[ab]";
            let tokenizer = Tokenizer::new(string).for_backend(backend);
            let tokens: Vec<Token> = tokenizer.iter().collect();
            assert_eq!(tokens, vec![Token::Quoted("[ab]")], "backend={:?}", backend);
        }
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_10_single_quoted_backslash_does_not_escape_quote() {
        let string = r#"ESCAPE '\' OR id = $1"#;
        let tokenizer = Tokenizer::new(string).for_backend(TokenizerBackend::Postgres);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("ESCAPE"),
                Token::Space(" "),
                Token::Quoted("'\\'"),
                Token::Space(" "),
                Token::Unquoted("OR"),
                Token::Space(" "),
                Token::Unquoted("id"),
                Token::Space(" "),
                Token::Punctuation("="),
                Token::Space(" "),
                Token::Punctuation("$"),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_10_mysql_single_quoted_backslash_escapes_quote() {
        let string = r#"'a\'b' OR id = ?"#;
        let tokenizer = Tokenizer::new(string).for_backend(TokenizerBackend::Mysql);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Quoted("'a\\'b'"),
                Token::Space(" "),
                Token::Unquoted("OR"),
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_10_postgres_escape_string_backslash_escapes_quote() {
        let string = r#"E'a\'b' OR id = $1"#;
        let tokenizer = Tokenizer::new(string).for_backend(TokenizerBackend::Postgres);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("E"),
                Token::Quoted("'a\\'b'"),
                Token::Space(" "),
                Token::Unquoted("OR"),
                Token::Space(" "),
                Token::Unquoted("id"),
                Token::Space(" "),
                Token::Punctuation("="),
                Token::Space(" "),
                Token::Punctuation("$"),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_10_sqlite_does_not_treat_e_prefix_as_escape_string() {
        let string = r#"E'a\'b' OR id = ?"#;
        let tokenizer = Tokenizer::new(string).for_backend(TokenizerBackend::Sqlite);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("E"),
                Token::Quoted("'a\\'"),
                Token::Unquoted("b"),
                Token::Quoted("' OR id = ?"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
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
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_single_line_comment() {
        let string = r#"SELECT
        -- hello 
        1"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space("\n        "),
                Token::Comment("-- hello "),
                Token::Space("\n        "),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT -- hello
        1"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Comment("-- hello"),
                Token::Space("\n        "),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT 1 -- hello"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Unquoted("1"),
                Token::Space(" "),
                Token::Comment("-- hello"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT 1 --"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Unquoted("1"),
                Token::Space(" "),
                Token::Comment("--"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT 1 -"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Unquoted("1"),
                Token::Space(" "),
                Token::Punctuation("-"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn test_block_comment() {
        let string = r#"SELECT /* hello */ 1"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Comment("/* hello */"),
                Token::Space(" "),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT /*hello*/ 1"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Comment("/*hello*/"),
                Token::Space(" "),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT /* --hello */ 1"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Comment("/* --hello */"),
                Token::Space(" "),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT
        /* hello */
        1"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space("\n        "),
                Token::Comment("/* hello */"),
                Token::Space("\n        "),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT /*
        -- hello */
        1"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Comment("/*\n        -- hello */"),
                Token::Space("\n        "),
                Token::Unquoted("1"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT 1/*hello*/"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Unquoted("1"),
                Token::Comment("/*hello*/"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT 1/*hello*"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Unquoted("1"),
                Token::Comment("/*hello*"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT 1/*hello"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Unquoted("1"),
                Token::Comment("/*hello"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );

        let string = r#"SELECT 1 /*"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT"),
                Token::Space(" "),
                Token::Unquoted("1"),
                Token::Space(" "),
                Token::Comment("/*"),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }
}
