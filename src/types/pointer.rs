use core::hash::Hash;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, BitAnd, BitOr, DivAssign, MulAssign, Neg, Rem, RemAssign, SubAssign};
use std::{
    cell::RefCell,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use super::prelude::*;

/// A pointer to a reference-counted value
/// A `const const` and `var const` can point to the same value, as can a `const var` and `var var`.
#[derive(PartialEq, Eq, Clone)]
pub enum Pointer {
    ConstConst(Rc<Value>),
    ConstVar(Rc<RefCell<Value>>),
    VarConst(Rc<RefCell<Rc<Value>>>),
    VarVar(Rc<RefCell<Rc<RefCell<Value>>>>),
}

impl Debug for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstConst(val) => write!(f, "ConstConst({val})"),
            Self::ConstVar(val) => write!(f, "ConstVar({})", val.borrow()),
            Self::VarConst(val) => write!(f, "VarConst({})", val.borrow()),
            Self::VarVar(val) => write!(f, "VarVar({})", val.borrow().borrow()),
        }
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstConst(val) => write!(f, "{val}"),
            Self::VarConst(val) => write!(f, "{}", val.borrow()),
            Self::ConstVar(val) => write!(f, "{}", val.borrow()),
            Self::VarVar(val) => write!(f, "{}", val.borrow().borrow()),
        }
    }
}

impl Hash for Pointer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::ConstConst(val) => val.hash(state),
            Self::VarConst(val) => val.borrow().hash(state),
            Self::ConstVar(var) => var.borrow().hash(state),
            Self::VarVar(var) => var.borrow().borrow().hash(state),
        }
    }
}

impl Pointer {
    /// Get the value inside the pointer. This is not a deep clone and should be treated as a reference
    pub fn clone_inner(&self) -> Value {
        match self {
            Self::ConstConst(val) => (**val).clone(),
            Self::VarConst(val) => (**val.borrow()).clone(),
            Self::VarVar(val) => (*val).borrow().borrow().clone(),
            Self::ConstVar(val) => (**val).borrow().clone(),
        }
    }

    /// Convert this pointer to a different type. Performs a shallow clone if switching between inner `const` and `var`
    pub fn convert(&self, vt: VarType) -> Self {
        match vt {
            VarType::ConstConst => Self::ConstConst(self.as_const()),
            VarType::ConstVar => Self::ConstVar(self.as_var()),
            VarType::VarConst => Self::VarConst(Rc::new(RefCell::new(self.as_const()))),
            VarType::VarVar => Self::VarVar(Rc::new(RefCell::new(self.as_var()))),
        }
    }

    // /// Make a new pointer from a value with a given type
    // pub fn from_value(val: Value, vt: VarType) -> Self {
    //     match vt {
    //         VarType::ConstConst => Self::ConstConst(Rc::new(val)),
    //         VarType::ConstVar => Self::ConstVar(Rc::new(RefCell::new(val))),
    //         VarType::VarConst => Self::VarConst(Rc::new(RefCell::new(Rc::new(val)))),
    //         VarType::VarVar => Self::VarVar(Rc::new(RefCell::new(Rc::new(RefCell::new(val))))),
    //     }
    // }

    /// check equality with a given precision. Returns a `const const` pointer to a boolean
    ///
    /// 1. internal data must be pretty similar
    /// 2. internal data must be identical with type coercion
    /// 3. internal data must be identical without type coercion
    /// 4. internal pointers must be identical
    pub fn eq(&self, rhs: &Self, precision: u8) -> Self {
        if precision >= 4 {
            Self::from(match (self, rhs) {
                (
                    Self::ConstConst(_) | Self::VarConst(_),
                    Self::ConstConst(_) | Self::VarConst(_),
                ) => unsafe {
                    core::mem::transmute::<Rc<_>, u64>(self.as_const())
                        == core::mem::transmute::<Rc<_>, u64>(rhs.as_const())
                },
                (Self::ConstVar(_) | Self::VarVar(_), Self::ConstVar(_) | Self::VarVar(_)) => unsafe {
                    core::mem::transmute::<Rc<_>, u64>(self.as_var())
                        == core::mem::transmute::<Rc<_>, u64>(rhs.as_var())
                },
                _ => false,
            })
        } else {
            Self::from(self.clone_inner().eq(rhs.clone_inner(), precision))
        }
    }

    /// Apply the dot operator. This has two valid cases: float parsing and object indexing. Otherwise, it returns `undefined`
    #[allow(clippy::option_if_let_else, clippy::single_match_else)]
    pub fn dot(&self, rhs: Value) -> SResult<Self> {
        let allow_modify = matches!(self, Self::ConstVar(_) | Self::VarVar(_));
        let lhs = self.clone_inner();
        match (lhs, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Ok(Self::from(format!("{lhs}.{rhs}").parse::<f64>().map_err(
                    |e| format!("Error parsing `{lhs}.{rhs}`: {e}"),
                )?))
            }
            (Value::Object(mut obj), key) => match obj.get(&key) {
                Some(ptr) => Ok(ptr.clone()),
                None => {
                    let val = if allow_modify {
                        Self::ConstVar(Rc::new(RefCell::new(Value::Undefined)))
                    } else {
                        Self::ConstConst(Rc::new(Value::Undefined))
                    };
                    obj.insert(key, val.clone());
                    Ok(val)
                }
            },
            _ => Ok(Self::from(Value::Undefined)),
        }
    }

    pub fn assign(&self, rhs: &Self) -> SResult<()> {
        match self {
            Self::ConstConst(_) => Err(format!("Can't assign to a `const const` {self:?}")),
            Self::ConstVar(_) => Err(format!("Can't assign to a `const var` {self:?}")),
            Self::VarConst(ptr) => {
                ptr.replace(rhs.as_const());
                Ok(())
            }
            Self::VarVar(ptr) => {
                // println!("{self:?} => {rhs:?}");
                ptr.replace(rhs.as_var());
                // println!("{self:?}");
                Ok(())
            }
        }
    }

    pub fn as_const(&self) -> Rc<Value> {
        match self {
            Self::ConstConst(val) => val.clone(),
            Self::VarConst(val) => val.borrow().clone(),
            Self::ConstVar(val) => Rc::new(val.borrow().clone()),
            Self::VarVar(val) => Rc::new(val.borrow().borrow().clone()),
        }
    }

    pub fn as_var(&self) -> Rc<RefCell<Value>> {
        match self {
            Self::ConstConst(val) => Rc::new(RefCell::new(val.as_ref().clone())),
            Self::VarConst(val) => Rc::new(RefCell::new(val.borrow().as_ref().clone())),
            Self::ConstVar(val) => val.clone(),
            Self::VarVar(val) => val.borrow().clone(),
        }
    }
}

impl PartialEq<Value> for Pointer {
    fn eq(&self, other: &Value) -> bool {
        *self.as_const() == other.clone()
    }
}

impl PartialOrd for Pointer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.clone_inner().partial_cmp(&other.clone_inner())
    }
}

impl Add for Pointer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() + rhs.clone_inner())
    }
}

impl AddAssign for Pointer {
    fn add_assign(&mut self, rhs: Self) {
        let output = self.clone_inner() + rhs.clone_inner();
        match self {
            Self::ConstVar(val) => {
                val.replace(output);
            }
            Self::VarVar(val) => {
                val.borrow().replace(output);
            }
            _ => {}
        };
    }
}

impl Sub for Pointer {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() - rhs.clone_inner())
    }
}

impl SubAssign for Pointer {
    fn sub_assign(&mut self, rhs: Self) {
        let output = self.clone_inner() - rhs.clone_inner();
        match self {
            Self::ConstVar(val) => {
                val.replace(output);
            }
            Self::VarVar(val) => {
                val.borrow().replace(output);
            }
            _ => {}
        };
    }
}

impl Mul for Pointer {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() * rhs.clone_inner())
    }
}

impl MulAssign for Pointer {
    fn mul_assign(&mut self, rhs: Self) {
        let output = self.clone_inner() * rhs.clone_inner();
        match self {
            Self::ConstVar(val) => {
                val.replace(output);
            }
            Self::VarVar(val) => {
                val.borrow().replace(output);
            }
            _ => {}
        };
    }
}

impl Div for Pointer {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() / rhs.clone_inner())
    }
}

impl DivAssign for Pointer {
    fn div_assign(&mut self, rhs: Self) {
        let output = self.clone_inner() / rhs.clone_inner();
        match self {
            Self::ConstVar(val) => {
                val.replace(output);
            }
            Self::VarVar(val) => {
                val.borrow().replace(output);
            }
            _ => {}
        };
    }
}

impl Neg for Pointer {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from(-self.clone_inner())
    }
}

impl Rem for Pointer {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() % rhs.clone_inner())
    }
}

impl RemAssign for Pointer {
    fn rem_assign(&mut self, rhs: Self) {
        let _ = self.assign(&(self.clone() % rhs));
    }
}

impl BitAnd for Pointer {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() & rhs.clone_inner())
    }
}

impl BitOr for Pointer {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from(self.clone_inner() | rhs.clone_inner())
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
