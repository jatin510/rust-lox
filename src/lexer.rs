pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Slash,
    Semicolon,
    Equal,
    EqualEqual,
    Bang,
    Space,
    Newline,
    Digit,
    Tab,
    Comment,
    DoubleQuote,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Identifier,
}
// TODO
// i will use it later

//
// impl TryFrom<&str> for Token {
//     type Error = ();
//
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }
