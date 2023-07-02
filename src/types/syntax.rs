use std::rc::Rc;

use super::StringSegment;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Syntax {
    Declare(VarType, Rc<str>, Box<Syntax>),
    Function(Vec<Rc<str>>, Box<Syntax>),
    Call(Rc<str>, Vec<Syntax>),
    Operation(Box<Syntax>, Operation, Box<Syntax>),
    Ident(Rc<str>),
    String(Vec<StringSegment>),
    Block(Vec<Syntax>),
    Debug(Box<Syntax>, u8),
    Negate(Box<Syntax>),
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum VarType {
    ConstConst,
    ConstVar,
    VarConst,
    VarVar,
    // ConstConstConst,
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
    Ls,
    LsEq,
    Gr,
    GrEq,
}
