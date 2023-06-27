#[derive(PartialEq, Eq, Debug)]
pub enum Syntax {
    Declare(VarType, String, Box<Syntax>),
    Block(Vec<Syntax>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum VarType {
    ConstConst,
    ConstVar,
    VarConst,
    VarVar,
    ConstConstConst,
}
