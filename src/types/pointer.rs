use core::hash::Hash;
use std::{
    cell::RefCell,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use super::Value;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Pointer {
    Const(Rc<Value>),
    Var(Rc<RefCell<Value>>),
}

impl Hash for Pointer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Const(val) => val.hash(state),
            Self::Var(var) => var.borrow().hash(state),
        }
    }
}

impl Pointer {
    pub fn clone_inner(&self) -> Value {
        match self {
            Self::Const(val) => (**val).clone(),
            Self::Var(val) => (**val).borrow().clone(),
        }
    }

    pub fn as_const(&self) -> Self {
        match self {
            Self::Const(val) => Self::Const(val.clone()),
            Self::Var(val) => Self::Const(Rc::from(val.borrow().clone())),
        }
    }

    pub fn as_var(&self) -> Self {
        match self {
            Self::Const(val) => Self::Var(Rc::new(RefCell::new((**val).clone()))),
            Self::Var(val) => Self::Var(val.clone()),
        }
    }

    pub fn eq(&self, rhs: &Self, precision: u8) -> Self {
        Self::Const(Rc::new(self.clone_inner().eq(rhs.clone_inner(), precision)))
    }
}

impl PartialEq<Value> for Pointer {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Self::Const(val) => &**val == other,
            Self::Var(val) => &*val.borrow() == other,
        }
    }
}

impl Add for Pointer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() + rhs.clone_inner())
    }
}

impl Sub for Pointer {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() - rhs.clone_inner())
    }
}

impl Mul for Pointer {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() * rhs.clone_inner())
    }
}

impl Div for Pointer {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == Value::Number(0.0) {
            Self::from(Value::Undefined)
        } else {
            Self::from(self.clone_inner() / rhs.clone_inner())
        }
    }
}

impl From<Value> for Pointer {
    fn from(value: Value) -> Self {
        Self::Const(Rc::new(value))
    }
}

impl From<&str> for Pointer {
    fn from(value: &str) -> Self {
        Self::Const(Rc::new(Value::String(String::from(value))))
    }
}
