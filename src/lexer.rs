use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Other.
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::StringLiteral(_) => write!(f, "STRING_LITERAL"),
            TokenType::Number(_) => write!(f, "NUMBER"),
            TokenType::And => write!(f, "AND"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::For => write!(f, "FOR"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let literal = match &self.token_type {
            TokenType::StringLiteral(value) => value.into(),
            TokenType::Number(value) => {
                let int = *value as i64;
                if int as f64 == *value {
                    format!("{}.0", int)
                } else {
                    value.to_string()
                }
            }
            _ => "null".into(),
        };

        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Scanner {
    source: String,
    source_len: usize,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub had_error: bool,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source_len: source.len(),
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
            keywords: Scanner::init_keywords(),

        }
    }

    fn init_keywords() -> HashMap<&'static str, TokenType> {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("fun", TokenType::Fun),
            ("for", TokenType::For),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    }

    pub fn scan_tokens(&mut self) {}
}

// pub enum ReservedKeywords {
//     And,
//     Class,
//     Else,
//     False,
//     Fun,
//     For,
//     If,
//     Nil,
//     Or,
//     Print,
//     Return,
//     Super,
//     This,
//     True,
//     Var,
//     While,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_token_type() {
        assert_eq!(format!("{}", TokenType::LeftParen), "LEFT_PAREN");
        assert_eq!(format!("{}", TokenType::RightParen), "RIGHT_PAREN");
        assert_eq!(format!("{}", TokenType::StringLiteral("s".to_string())), "STRING_LITERAL");
    }

    #[test]
    fn test_display_token() {
        let token = Token::new(TokenType::LeftParen, "(".to_string(), 1);
        assert_eq!(format!("{}", token), "LEFT_PAREN ( null");
        assert_eq!(format!("{}", Token::new(TokenType::StringLiteral("hello".to_string()), "hello".to_string(), 1)), "STRING_LITERAL hello hello");
        // test for number
        assert_eq!(format!("{}", Token::new(TokenType::Number(1.0), "1".to_string(), 1)), "NUMBER 1 1.0");
    }

    #[test]
    fn test_scanner_new() {
        let scanner = Scanner::new("(){".to_string());
        assert_eq!(scanner.source, "(){");
        assert_eq!(scanner.source_len, 3);
        assert_eq!(scanner.tokens, vec![]);
        assert_eq!(scanner.start, 0);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.line, 1);
        assert_eq!(scanner.had_error, false);
        assert_eq!(scanner.keywords, Scanner::init_keywords());
    }
}

