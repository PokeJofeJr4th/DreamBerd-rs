use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum StringSegment {
    String(Rc<str>),
    Ident(Rc<str>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ident(Rc<str>),
    String(Vec<StringSegment>),
    Space(u8),
    Equal(u8),
    Bang(u8),
    Question(u8),
    Plus,
    PlusPlus,
    PlusEq,
    Tack,
    TackTack,
    TackEq,
    Star,
    StarEq,
    Slash,
    SlashEq,
    Percent,
    PercentEq,
    LCaret,
    LCaretEq,
    RCaret,
    RCaretEq,
    LParen,
    RParen,
    LSquirrely,
    RSquirrely,
    LSquare,
    RSquare,
    Arrow,
    Semicolon,
    Comma,
    Colon,
    Dot,
    And,
    Or,
}
