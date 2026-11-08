use std::ops::{Index, Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceSpan {
    start: usize,
    end: usize,
}

impl SourceSpan {
    fn len(&self) -> usize {
        self.end - self.start
    }

    fn full(source: &str) -> Self {
        Self {
            start: 0,
            end: source.len(),
        }
    }

    fn subspan(self, range: Range<usize>) -> Self {
        Self {
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }

    fn try_join(self, next: Self) -> Option<Self> {
        if self.end == next.start {
            Some(Self {
                start: self.start,
                end: next.end,
            })
        } else {
            None
        }
    }
}

impl Index<SourceSpan> for str {
    type Output = str;

    fn index(&self, span: SourceSpan) -> &Self::Output {
        &self[span.start..span.end]
    }
}

#[derive(Debug, Clone, Copy)]
enum InputFragment {
    Verbatim(SourceSpan),
    EscapedDollar(SourceSpan),
}

impl InputFragment {
    fn visible_span(&self) -> SourceSpan {
        match self {
            Self::Verbatim(span) => *span,
            Self::EscapedDollar(span) => span.subspan(0..1),
        }
    }

    fn is_escaped_dollar(&self) -> bool {
        matches!(self, Self::EscapedDollar(_))
    }

    fn try_join(self, next: Self) -> Option<Self> {
        match (self, next) {
            (Self::Verbatim(previous), Self::Verbatim(next)) => {
                previous.try_join(next).map(Self::Verbatim)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LogicalPosition {
    fragment_index: usize,
    byte_offset: usize,
}

impl LogicalPosition {
    fn advance_bytes(&mut self, bytes: usize) {
        self.byte_offset += bytes;
    }

    fn next_fragment(&mut self) {
        self.fragment_index += 1;
        self.byte_offset = 0;
    }
}

#[derive(Debug, Clone, Copy)]
struct LogicalSpan {
    start: LogicalPosition,
    end: LogicalPosition,
}

#[derive(Debug, Clone)]
struct Cursor<'a> {
    source: &'a str,
    fragments: &'a [InputFragment],
    position: LogicalPosition,
}

impl<'a> Cursor<'a> {
    fn position(&self) -> LogicalPosition {
        self.position
    }

    fn fragment_at(&self, position: LogicalPosition) -> Option<InputFragment> {
        if self.fragments.is_empty() {
            if position.fragment_index == 0 && !self.source.is_empty() {
                Some(InputFragment::Verbatim(SourceSpan::full(self.source)))
            } else {
                None
            }
        } else {
            self.fragments.get(position.fragment_index).copied()
        }
    }

    fn fragment(&self) -> Option<InputFragment> {
        self.fragment_at(self.position)
    }

    fn remaining_text(&self) -> Option<&'a str> {
        let span = self.fragment()?.visible_span();
        let remaining = span.subspan(self.position.byte_offset..span.len());
        Some(&self.source[remaining])
    }

    fn peek(&self) -> Option<char> {
        self.remaining_text()?.chars().next()
    }

    fn consumed_since(&self, start: LogicalPosition) -> LogicalSpan {
        LogicalSpan {
            start,
            end: self.position,
        }
    }

    fn source_span(&self, span: LogicalSpan) -> SourceSpan {
        debug_assert!(self.fragments.is_empty());
        let end = if span.end.fragment_index == 0 {
            span.end.byte_offset
        } else {
            self.source.len()
        };
        SourceSpan {
            start: span.start.byte_offset,
            end,
        }
    }

    fn raw_text(&self, span: LogicalSpan) -> &'a str {
        let source_span = self.source_span(span);
        &self.source[source_span]
    }

    fn new(source: &'a str, fragments: &'a [InputFragment]) -> Self {
        Self {
            source,
            fragments,
            position: LogicalPosition::default(),
        }
    }

    fn fork_at(&self, position: LogicalPosition) -> Self {
        Self {
            position,
            ..self.clone()
        }
    }

    fn resume_after(&mut self, opener: LogicalSpan) {
        self.position = opener.end;
    }

    fn advance(&mut self) -> Option<char> {
        let remaining = self.remaining_text()?;
        let c = remaining.chars().next()?;

        if c.len_utf8() == remaining.len() {
            self.position.next_fragment();
        } else {
            self.position.advance_bytes(c.len_utf8());
        }

        Some(c)
    }

    fn span_equals(&self, span: LogicalSpan, text: &str) -> bool {
        let mut cursor = self.fork_at(span.start);
        for c in text.chars() {
            if cursor.position == span.end || cursor.advance() != Some(c) {
                return false;
            }
        }
        cursor.position == span.end
    }

    fn advance_while(&mut self, predicate: impl Fn(char) -> bool) {
        while self.peek().is_some_and(&predicate) {
            self.advance();
        }
    }

    fn for_each_source_span(&self, span: LogicalSpan, mut emit: impl FnMut(SourceSpan)) {
        let mut cursor = self.fork_at(span.start);
        while cursor.position < span.end {
            let fragment = cursor.fragment().unwrap().visible_span();
            let end = if cursor.position.fragment_index == span.end.fragment_index {
                span.end.byte_offset
            } else {
                fragment.len()
            };
            emit(fragment.subspan(cursor.position.byte_offset..end));
            cursor.position.next_fragment();
        }
    }

    fn consume_if_matches(&mut self, span: LogicalSpan) -> bool {
        let mut candidate = self.clone();
        let mut expected = self.fork_at(span.start);
        while expected.position != span.end {
            let Some(c) = expected.advance() else {
                return false;
            };
            if candidate.advance() != Some(c) {
                return false;
            }
        }
        *self = candidate;
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenizerBackend {
    Mysql,
    Postgres,
    Sqlite,
}

impl TokenizerBackend {
    pub(crate) fn infer_from_query_builder(query_builder: &impl crate::QueryBuilder) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Quoted,
    Unquoted,
    Space,
    Punctuation,
    Comment,
}

#[derive(Debug, Clone, Copy)]
struct Lexeme {
    kind: TokenKind,
    span: LogicalSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexingMode {
    Template,
    Sql,
}

#[derive(Debug, Clone, Copy)]
struct Quote {
    opening_delimiter: char,
    allows_backslash_escape: bool,
}

impl Quote {
    fn closes(&self, c: char) -> bool {
        c == if self.opening_delimiter == '[' {
            ']'
        } else {
            self.opening_delimiter
        }
    }

    fn is_doubled_delimiter(&self, c: char) -> bool {
        self.opening_delimiter != '[' && self.closes(c)
    }

    fn scan(&self, cursor: &mut Cursor<'_>, mut emit: impl FnMut(char)) {
        cursor.advance();
        let mut escaped = false;
        while let Some(c) = cursor.advance() {
            if !escaped && self.closes(c) {
                if cursor.peek().is_some_and(|c| self.is_doubled_delimiter(c)) {
                    emit(c);
                    cursor.advance();
                } else {
                    break;
                }
            } else {
                escaped = !escaped && self.allows_backslash_escape && c == '\\';
                emit(c);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SqlState {
    Code,
    Identifier { end: LogicalPosition },
    Quoted(Quote),
    DollarQuoted { delimiter: LogicalSpan },
    LineComment,
    BlockComment,
}

fn is_sql_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn is_alphanumeric(c: char) -> bool {
    c.is_alphabetic() || c.is_ascii_digit()
}

fn is_pg_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || !c.is_ascii()
}

fn is_dollar_tag_part(c: char) -> bool {
    is_pg_identifier_start(c) || c.is_ascii_digit()
}

fn is_pg_identifier_part(c: char) -> bool {
    is_dollar_tag_part(c) || c == '$'
}

#[derive(Debug)]
struct SqlLexer<'a> {
    cursor: Cursor<'a>,
    backend: TokenizerBackend,
    mode: LexingMode,
    state: SqlState,
}

impl<'a> SqlLexer<'a> {
    fn quote_for(&self, c: char) -> Option<Quote> {
        match c {
            '\'' | '"' | '`' => Some(Quote {
                opening_delimiter: c,
                allows_backslash_escape: c != '\'' || self.backend == TokenizerBackend::Mysql,
            }),
            '[' if self.backend != TokenizerBackend::Postgres => Some(Quote {
                opening_delimiter: c,
                allows_backslash_escape: true,
            }),
            _ => None,
        }
    }

    fn new(cursor: Cursor<'a>, backend: TokenizerBackend, mode: LexingMode) -> Self {
        Self {
            cursor,
            backend,
            mode,
            state: SqlState::Code,
        }
    }

    fn peek_pg_dollar_delimiter(&self) -> Option<LogicalSpan> {
        let mut cursor = self.cursor.clone();
        let start = cursor.position();
        if cursor.advance()? != '$' {
            return None;
        }
        match cursor.peek()? {
            '$' => {}
            c if is_pg_identifier_start(c) => cursor.advance_while(is_dollar_tag_part),
            _ => return None,
        }
        if cursor.advance()? == '$' {
            Some(cursor.consumed_since(start))
        } else {
            None
        }
    }

    fn begin_region(&mut self, c: char) {
        if self.backend == TokenizerBackend::Postgres && self.mode == LexingMode::Sql {
            if is_pg_identifier_start(c) {
                // The whole PostgreSQL identifier excludes dollar-quote openers;
                // tokens inside it still follow legacy unquoted boundaries.
                let mut end = self.cursor.clone();
                end.advance_while(is_pg_identifier_part);
                self.state = SqlState::Identifier {
                    end: end.position(),
                };
            } else if c == '$'
                && let Some(delimiter) = self.peek_pg_dollar_delimiter()
            {
                self.state = SqlState::DollarQuoted { delimiter };
            }
        }
        if matches!(self.state, SqlState::Code) {
            if let Some(quote) = self.quote_for(c) {
                self.state = SqlState::Quoted(quote);
            } else {
                let mut next = self.cursor.clone();
                next.advance();
                if c == '-' && next.peek() == Some('-') {
                    self.state = SqlState::LineComment;
                } else if c == '/' && next.peek() == Some('*') {
                    self.state = SqlState::BlockComment;
                }
            }
        }
    }
}

impl Iterator for SqlLexer<'_> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.peek()?;
        let start = self.cursor.position();
        if let SqlState::Identifier { end } = self.state
            && start == end
        {
            self.state = SqlState::Code;
        }
        if matches!(self.state, SqlState::Code) {
            self.begin_region(c);
        }

        let kind = match self.state {
            SqlState::Quoted(quote) => {
                quote.scan(&mut self.cursor, |_| {});
                self.state = SqlState::Code;
                TokenKind::Quoted
            }
            SqlState::DollarQuoted { delimiter } => {
                self.cursor.resume_after(delimiter);
                while self.cursor.peek().is_some() {
                    if self.cursor.consume_if_matches(delimiter) {
                        break;
                    }
                    self.cursor.advance();
                }
                self.state = SqlState::Code;
                TokenKind::Quoted
            }
            SqlState::LineComment => {
                self.cursor.advance_while(|c| c != '\n');
                self.state = SqlState::Code;
                TokenKind::Comment
            }
            SqlState::BlockComment => {
                self.cursor.advance();
                self.cursor.advance();
                while let Some(c) = self.cursor.advance() {
                    if c == '*' && self.cursor.peek() == Some('/') {
                        self.cursor.advance();
                        break;
                    }
                }
                self.state = SqlState::Code;
                TokenKind::Comment
            }
            SqlState::Code | SqlState::Identifier { .. } => {
                if is_sql_whitespace(c) {
                    self.cursor.advance_while(is_sql_whitespace);
                    TokenKind::Space
                } else if is_alphanumeric(c) {
                    self.cursor.advance();
                    self.cursor
                        .advance_while(|c| is_alphanumeric(c) || c == '_' || c == '$');
                    let span = self.cursor.consumed_since(start);
                    if self.backend == TokenizerBackend::Postgres
                        && self.cursor.peek() == Some('\'')
                        && (self.cursor.span_equals(span, "E")
                            || self.cursor.span_equals(span, "e"))
                    {
                        self.state = SqlState::Quoted(Quote {
                            opening_delimiter: '\'',
                            allows_backslash_escape: true,
                        });
                    }
                    TokenKind::Unquoted
                } else {
                    self.cursor.advance();
                    TokenKind::Punctuation
                }
            }
        };
        Some(Lexeme {
            kind,
            span: self.cursor.consumed_since(start),
        })
    }
}

pub(crate) enum Command {
    Text(SourceSpan),
    NumberedArgument(usize),
    SequentialArgument,
}

struct TemplateEmitter<F> {
    pending_text: Option<SourceSpan>,
    emit: F,
}

impl<F: FnMut(Command)> TemplateEmitter<F> {
    fn flush_text(&mut self) {
        if let Some(span) = self.pending_text.take() {
            (self.emit)(Command::Text(span));
        }
    }

    fn text(&mut self, cursor: &Cursor<'_>, span: LogicalSpan) {
        cursor.for_each_source_span(span, |span| {
            if let Some(joined) = self
                .pending_text
                .and_then(|previous| previous.try_join(span))
            {
                self.pending_text = Some(joined);
            } else {
                self.flush_text();
                self.pending_text = Some(span);
            }
        });
    }

    fn argument(&mut self, command: Command) {
        self.flush_text();
        (self.emit)(command);
    }
}

fn decode_template(source: &str, backend: TokenizerBackend) -> Vec<InputFragment> {
    let mut lexer = SqlLexer::new(Cursor::new(source, &[]), backend, LexingMode::Template);
    let mut fragments: Vec<InputFragment> = Vec::new();
    while let Some(token) = lexer.next() {
        let fragment = if token.kind == TokenKind::Punctuation
            && lexer.cursor.span_equals(token.span, "$")
            && lexer.cursor.peek() == Some('$')
        {
            lexer
                .next()
                .expect("a peeked dollar must produce a lexer token");
            let consumed = lexer.cursor.consumed_since(token.span.start);
            InputFragment::EscapedDollar(lexer.cursor.source_span(consumed))
        } else {
            InputFragment::Verbatim(lexer.cursor.source_span(token.span))
        };
        if let Some(previous) = fragments.last_mut()
            && let Some(joined) = previous.try_join(fragment)
        {
            *previous = joined;
            continue;
        }
        fragments.push(fragment);
    }
    fragments
}

fn parse_argument_number(input: &Cursor<'_>, span: LogicalSpan) -> Option<usize> {
    let mut cursor = input.fork_at(span.start);
    let mut number = 0usize;
    while cursor.position != span.end {
        let digit = cursor.advance()?.to_digit(10)? as usize;
        number = number.checked_mul(10)?.checked_add(digit)?;
    }
    Some(number)
}

pub(crate) fn visit_template(
    source: &str,
    query_builder: &impl crate::QueryBuilder,
    emit: impl FnMut(Command),
) {
    let backend = TokenizerBackend::infer_from_query_builder(query_builder);
    let (placeholder, numbered) = query_builder.placeholder();
    let (fragments, mode) = if backend == TokenizerBackend::Postgres && placeholder == "$" {
        (decode_template(source, backend), LexingMode::Sql)
    } else {
        (Vec::new(), LexingMode::Template)
    };
    let input = Cursor::new(source, &fragments);
    let mut tokens = SqlLexer::new(input.clone(), backend, mode).peekable();
    let mut output = TemplateEmitter {
        pending_text: None,
        emit,
    };
    while let Some(token) = tokens.next() {
        if token.kind == TokenKind::Punctuation
            && input.span_equals(token.span, placeholder)
            && !input
                .fragment_at(token.span.start)
                .is_some_and(|fragment| fragment.is_escaped_dollar())
        {
            match tokens.peek().copied() {
                Some(next)
                    if next.kind == TokenKind::Punctuation
                        && input.span_equals(next.span, placeholder) =>
                {
                    if numbered && mode == LexingMode::Sql {
                        output.text(&input, token.span);
                    }
                    output.text(&input, next.span);
                    tokens.next();
                }
                Some(next) if numbered && next.kind == TokenKind::Unquoted => {
                    if let Some(number) = parse_argument_number(&input, next.span) {
                        output.argument(Command::NumberedArgument(number));
                    }
                    tokens.next();
                }
                _ => output.argument(Command::SequentialArgument),
            }
        } else {
            output.text(&input, token.span);
        }
    }
    output.flush_text();
}

/// Tokenizer for processing SQL.
#[derive(Debug)]
pub struct Tokenizer<'a> {
    lexer: SqlLexer<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(string: &'a str) -> Self {
        Self::new_with_backend(string, TokenizerBackend::Mysql)
    }

    pub(crate) fn new_with_backend(string: &'a str, backend: TokenizerBackend) -> Self {
        Self {
            lexer: SqlLexer::new(Cursor::new(string, &[]), backend, LexingMode::Sql),
        }
    }

    pub fn iter(self) -> impl Iterator<Item = Token<'a>> {
        self
    }

    /// unquote a quoted string
    fn unquote(self) -> String {
        let mut string = String::new();
        let Some(quote) = self
            .lexer
            .cursor
            .peek()
            .and_then(|c| self.lexer.quote_for(c))
        else {
            return string;
        };
        let mut cursor = self.lexer.cursor;
        quote.scan(&mut cursor, |c| string.push(c));
        string
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.next()?;
        let text = self.lexer.cursor.raw_text(token.span);
        Some(match token.kind {
            TokenKind::Quoted => Token::Quoted(text),
            TokenKind::Unquoted => Token::Unquoted(text),
            TokenKind::Space => Token::Space(text),
            TokenKind::Punctuation => Token::Punctuation(text),
            TokenKind::Comment => Token::Comment(text),
        })
    }
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

impl Token<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Quoted(string) => string,
            Self::Unquoted(string) => string,
            Self::Space(string) => string,
            Self::Punctuation(string) => string,
            Self::Comment(string) => string,
        }
    }

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
        let tokens: Vec<Token> = tokenizer.collect();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_1() {
        let string = "SELECT * FROM `character`";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
        assert_eq!(tokens, vec![Token::Quoted("`a\"b`"), Token::Space(" ")]);
        assert_eq!(
            string,
            tokens.iter().map(|x| x.as_str()).collect::<String>()
        );
    }

    #[test]
    fn mysql_brackets_are_quoted() {
        assert_eq!(
            Tokenizer::new_with_backend("[ab]", TokenizerBackend::Mysql).collect::<Vec<_>>(),
            vec![Token::Quoted("[ab]")]
        );
    }

    #[test]
    fn sqlite_brackets_are_quoted() {
        assert_eq!(
            Tokenizer::new_with_backend("[ab]", TokenizerBackend::Sqlite).collect::<Vec<_>>(),
            vec![Token::Quoted("[ab]")]
        );
    }

    #[test]
    fn test_10() {
        let string = r#" 'a"b' "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokenizer = Tokenizer::new_with_backend(string, TokenizerBackend::Postgres);
        let tokens: Vec<Token> = tokenizer.collect();
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
    }

    #[test]
    fn test_10_mysql_single_quoted_backslash_escapes_quote() {
        let string = r#"'a\'b' OR id = ?"#;
        let tokenizer = Tokenizer::new_with_backend(string, TokenizerBackend::Mysql);
        let tokens: Vec<Token> = tokenizer.collect();
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
    }

    #[test]
    fn test_10_postgres_escape_string_backslash_escapes_quote() {
        let string = r#"E'a\'b' OR id = $1"#;
        let tokenizer = Tokenizer::new_with_backend(string, TokenizerBackend::Postgres);
        let tokens: Vec<Token> = tokenizer.collect();
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
    }

    #[test]
    fn test_10_sqlite_does_not_treat_e_prefix_as_escape_string() {
        let string = r#"E'a\'b' OR id = ?"#;
        let tokenizer = Tokenizer::new_with_backend(string, TokenizerBackend::Sqlite);
        let tokens: Vec<Token> = tokenizer.collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("E"),
                Token::Quoted("'a\\'"),
                Token::Unquoted("b"),
                Token::Quoted("' OR id = ?"),
            ]
        );
    }

    #[test]
    fn test_11() {
        let string = r" `a``b` ";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
        let tokens: Vec<Token> = tokenizer.collect();
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
