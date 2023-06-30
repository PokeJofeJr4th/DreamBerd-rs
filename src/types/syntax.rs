#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Syntax {
    Declare(VarType, String, Box<Syntax>),
    Function(Vec<String>, Box<Syntax>),
    Call(String, Vec<Syntax>),
    Operation(Box<Syntax>, Operation, Box<Syntax>),
    Ident(String),
    String(String),
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
}
