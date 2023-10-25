use std::{fmt::Display, rc::Rc};

use super::{StringSegment, Token};

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Syntax {
    Declare(VarType, Rc<str>, Box<Syntax>),
    Function(Vec<Rc<str>>, Box<Syntax>),
    Call(Rc<str>, Vec<Syntax>),
    Operation(Box<Syntax>, Operation, Box<Syntax>),
    UnaryOperation(UnaryOperation, Box<Syntax>),
    Ident(Rc<str>),
    String(Vec<StringSegment>),
    Block(Vec<Syntax>),
    Statement(bool, Box<Syntax>, u8),
    Negate(Box<Syntax>),
}

impl Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Statement(is_debug, content, count) => {
                write!(
                    f,
                    "{content}{}",
                    if *is_debug { "?" } else { "!" }.repeat(*count as usize)
                )
            }
            Self::Call(func, args) => {
                write!(f, "{func}(")?;
                let arglen = args.len();
                for (idx, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if idx + 1 != arglen {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Self::Block(statements) => {
                write!(f, "{{")?;
                for statement in statements {
                    write!(f, "{statement} ")?;
                }
                write!(f, "}}")
            }
            Self::String(segments) => {
                write!(f, "\"")?;
                for segment in segments {
                    write!(f, "{segment}")?;
                }
                write!(f, "\"")
            }
            Self::Ident(ident) => write!(f, "{ident}"),
            Self::Declare(var_type, name, value) => {
                write!(f, "{var_type} {name} = {value}")
            }
            Self::Operation(lhs, op, rhs) => {
                write!(f, "({lhs}{op}{rhs})")
            }
            // Self::UnaryOperation(UnaryOperation::Call(args), operand) => {
            //     write!(f, "{operand}({args:?})")
            // }
            Self::UnaryOperation(UnaryOperation::Decrement, operand) => {
                write!(f, "{operand}--")
            }
            Self::UnaryOperation(UnaryOperation::Increment, operand) => {
                write!(f, "{operand}++")
            }
            Self::Function(args, body) => {
                write!(f, "{args:?} -> {body}")
            }
            Self::Negate(inner) => write!(f, ";{inner}"),
            // other => write!(f, "{other:?}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum VarType {
    ConstConst,
    ConstVar,
    VarConst,
    VarVar,
    // ConstConstConst,
}

impl Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstConst => write!(f, "const const"),
            Self::ConstVar => write!(f, "const var"),
            Self::VarConst => write!(f, "var const"),
            Self::VarVar => write!(f, "var var"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperation {
    Increment,
    Decrement,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Operation {
    Equal(u8),
    Add,
    AddEq,
    Sub,
    SubEq,
    Mul,
    MulEq,
    Div,
    DivEq,
    Mod,
    ModEq,
    Dot,
    And,
    Or,
    Arrow,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Equal(count) => write!(f, "{}", "=".repeat(*count as usize)),
            Self::Add => write!(f, "+"),
            Self::AddEq => write!(f, "+="),
            Self::Sub => write!(f, "-"),
            Self::SubEq => write!(f, "-="),
            Self::Mul => write!(f, "*"),
            Self::MulEq => write!(f, "*="),
            Self::Div => write!(f, "/"),
            Self::DivEq => write!(f, "/="),
            Self::Mod => write!(f, "%"),
            Self::ModEq => write!(f, "%="),
            Self::Dot => write!(f, "."),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
            Self::Arrow => write!(f, "->"),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
        }
    }
}

impl TryFrom<Token> for Operation {
    type Error = ();
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Equal(val) => Ok(Self::Equal(val)),
            Token::Plus => Ok(Self::Add),
            Token::PlusEq => Ok(Self::AddEq),
            Token::Tack => Ok(Self::Sub),
            Token::TackEq => Ok(Self::SubEq),
            Token::Star => Ok(Self::Mul),
            Token::StarEq => Ok(Self::MulEq),
            Token::Slash => Ok(Self::Div),
            Token::SlashEq => Ok(Self::DivEq),
            Token::Percent => Ok(Self::Mod),
            Token::PercentEq => Ok(Self::ModEq),
            Token::Dot => Ok(Self::Dot),
            Token::And => Ok(Self::And),
            Token::Or => Ok(Self::Or),
            Token::Arrow => Ok(Self::Arrow),
            Token::LCaret => Ok(Self::Lt),
            Token::LCaretEq => Ok(Self::Le),
            Token::RCaret => Ok(Self::Gt),
            Token::RCaretEq => Ok(Self::Ge),
            _ => Err(()),
        }
    }
}
