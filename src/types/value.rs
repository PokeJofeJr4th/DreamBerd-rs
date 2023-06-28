use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use super::Syntax;

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Boolean {
    True,
    False,
    Maybe,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Pointer {
    Const(Rc<Value>),
    Var(Rc<RefCell<Value>>),
}

impl Hash for Pointer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Const(val) => val.hash(state),
            Self::Var(var) => var.borrow().hash(state),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Boolean(Boolean),
    String(String),
    Number(f64),
    Object(HashMap<Value, Pointer>),
    Function(Vec<String>, Syntax),
    Keyword(Keyword),
    Undefined,
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Boolean(bool) => bool.hash(state),
            Self::String(str) => str.hash(state),
            Self::Number(float) => todo!(),
            Self::Object(obj) => todo!(),
            Self::Function(inputs, content) => {
                inputs.hash(state);
                content.hash(state);
            }
            Self::Keyword(keyword) => keyword.hash(state),
            Self::Undefined => {}
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Keyword {
    Const,
    Var,
    Delete,
    Function,
    If,
}
