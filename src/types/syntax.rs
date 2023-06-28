#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Syntax {
    Declare(VarType, String, Box<Syntax>),
    Function(String, Vec<Syntax>),
    Ident(String),
    String(String),
    Block(Vec<Syntax>),
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum VarType {
    ConstConst,
    ConstVar,
    VarConst,
    VarVar,
    ConstConstConst,
}
