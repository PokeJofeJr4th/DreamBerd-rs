use core::hash::Hash;
use std::fmt::Display;
use std::{
    cell::RefCell,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use super::{SResult, Value, VarType};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Pointer {
    ConstConst(Rc<Value>),
    ConstVar(Rc<RefCell<Value>>),
    VarConst(Rc<Value>),
    VarVar(Rc<RefCell<Value>>),
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstConst(val) | Self::VarConst(val) => write!(f, "{val}"),
            Self::ConstVar(val) | Self::VarVar(val) => write!(f, "{}", val.borrow()),
        }
    }
}

impl Hash for Pointer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::ConstConst(val) | Self::VarConst(val) => val.hash(state),
            Self::ConstVar(var) | Self::VarVar(var) => var.borrow().hash(state),
        }
    }
}

impl Pointer {
    pub fn clone_inner(&self) -> Value {
        match self {
            Self::ConstConst(val) | Self::VarConst(val) => (**val).clone(),
            Self::VarVar(val) | Self::ConstVar(val) => (**val).borrow().clone(),
        }
    }

    pub fn convert(&self, vt: VarType) -> Self {
        match (self, vt) {
            (
                Self::ConstConst(val) | Self::VarConst(val),
                VarType::ConstConst | VarType::VarConst,
            ) => {
                if vt == VarType::ConstConst {
                    Self::ConstConst(val.clone())
                } else {
                    Self::VarConst(val.clone())
                }
            }
            (Self::ConstVar(val) | Self::VarVar(val), VarType::ConstVar | VarType::VarVar) => {
                if vt == VarType::VarVar {
                    Self::VarVar(val.clone())
                } else {
                    Self::ConstVar(val.clone())
                }
            }
            (Self::ConstConst(val) | Self::VarConst(val), VarType::ConstVar | VarType::VarVar) => {
                if vt == VarType::ConstVar {
                    Self::ConstVar(Rc::new(RefCell::new((**val).clone())))
                } else {
                    Self::VarVar(Rc::new(RefCell::new((**val).clone())))
                }
            }
            (Self::VarVar(val) | Self::ConstVar(val), VarType::ConstConst | VarType::VarConst) => {
                if vt == VarType::ConstConst {
                    Self::ConstConst(Rc::new(val.borrow().clone()))
                } else {
                    Self::VarConst(Rc::new(val.borrow().clone()))
                }
            }
        }
    }

    pub fn eq(&self, rhs: &Self, precision: u8) -> Self {
        if precision >= 4 {
            Self::from(match (self, rhs) {
                (
                    Self::ConstConst(lhs) | Self::VarConst(lhs),
                    Self::ConstConst(rhs) | Self::VarConst(rhs),
                ) => unsafe {
                    core::mem::transmute::<Rc<_>, u64>(lhs.clone())
                        == core::mem::transmute::<Rc<_>, u64>(rhs.clone())
                },
                (
                    Self::ConstVar(lhs) | Self::VarVar(lhs),
                    Self::ConstVar(rhs) | Self::VarVar(rhs),
                ) => unsafe {
                    core::mem::transmute::<Rc<_>, u64>(lhs.clone())
                        == core::mem::transmute::<Rc<_>, u64>(rhs.clone())
                },
                _ => false,
            })
        } else {
            Self::from(self.clone_inner().eq(rhs.clone_inner(), precision))
        }
    }

    pub fn dot(&self, rhs: Value) -> SResult<Self> {
        // can we return a mutable internal reference?
        let allow_modify = matches!(self, Self::ConstVar(_) | Self::VarVar(_));
        let lhs = self.clone_inner();
        match (lhs, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Ok(Self::from(format!("{lhs}.{rhs}").parse::<f64>().map_err(
                    |e| format!("Error parsing `{lhs}.{rhs}`: {e}"),
                )?))
            }
            _ => Ok(Self::from(Value::Undefined)),
        }
    }
}

impl PartialEq<Value> for Pointer {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Self::ConstConst(val) | Self::VarConst(val) => &**val == other,
            Self::VarVar(val) | Self::ConstVar(val) => &*val.borrow() == other,
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
        Self::ConstConst(Rc::new(value))
    }
}

impl From<bool> for Pointer {
    fn from(value: bool) -> Self {
        Self::ConstConst(Rc::new(Value::from(value)))
    }
}

impl From<&str> for Pointer {
    fn from(value: &str) -> Self {
        Self::ConstConst(Rc::new(Value::String(String::from(value))))
    }
}

impl From<f64> for Pointer {
    fn from(value: f64) -> Self {
        Self::ConstConst(Rc::new(Value::Number(value)))
    }
}
