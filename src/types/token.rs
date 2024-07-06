use std::{fmt::Display, rc::Rc};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum StringSegment {
    String(Rc<str>),
    Ident(Rc<str>),
    ///  The symbol for the Cape Verdean escudo is placed in the decimal separator position, as in 2$50.
    /// Developers from the Republic of Cape Verde can benefit from this syntax: `{e$code}`
    Escudo(Rc<str>, Rc<str>),
}

impl Display for StringSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident) => write!(f, "${{{ident}}}"),
            Self::String(str) => write!(f, "{str}"),
            Self::Escudo(var, field) => write!(f, "{{{var}${field}}}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
