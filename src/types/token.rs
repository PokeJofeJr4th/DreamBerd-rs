#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    String(String),
    Space(u8),
    Equal(u8),
    Bang(u8),
    Plus,
    PlusEq,
    Tack,
    TackEq,
    Star,
    StarEq,
    Slash,
    SlashEq,
    LParen,
    RParen,
    LSquirrely,
    RSquirrely,
    LSquare,
    RSquare,
    Arrow,
    Question,
    Semicolon,
    Comma,
    Colon,
    Dot,
}
