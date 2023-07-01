use core::hash::Hash;
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{AddAssign, BitAnd, BitOr, DivAssign, MulAssign, Neg, Rem, RemAssign, SubAssign};
use std::{
    cell::RefCell,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use super::{SResult, Value, VarType};

/// A pointer to a reference-counted value
/// A `const const` and `var const` can point to the same value, as can a `const var` and `var var`.
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
    /// Get the value inside the pointer. This is not a deep clone and should be treated as a reference
    pub fn clone_inner(&self) -> Value {
        match self {
            Self::ConstConst(val) | Self::VarConst(val) => (**val).clone(),
            Self::VarVar(val) | Self::ConstVar(val) => (**val).borrow().clone(),
        }
    }

    /// Convert this poiner to a different type. Performs a shallow clone if switching between `const` and `var`
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

    /// Apply the dot operator. This has two valid cases: float parsing and object indexing. Otherwise, it returns `undefined`
    #[allow(clippy::option_if_let_else, clippy::single_match_else)]
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
            // TODO: Include the case for an object
            _ => Ok(Self::from(Value::Undefined)),
        }
    }

    pub fn assign(&mut self, rhs: &Self) -> SResult<()> {
        match self {
            Self::ConstConst(_) => Err(String::from("Can't assign to a `const const`")),
            Self::ConstVar(_) => Err(String::from("Can't assign to a `const var`")),
            Self::VarConst(ptr) => {
                *ptr = rhs.as_const();
                Ok(())
            }
            Self::VarVar(ptr) => {
                *ptr = rhs.as_var();
                Ok(())
            }
        }
    }

    pub fn as_const(&self) -> Rc<Value> {
        match self {
            Self::ConstConst(val) | Self::VarConst(val) => val.clone(),
            Self::ConstVar(val) | Self::VarVar(val) => Rc::new(val.borrow().clone()),
        }
    }

    pub fn as_var(&self) -> Rc<RefCell<Value>> {
        match self {
            Self::ConstConst(val) | Self::VarConst(val) => {
                Rc::new(RefCell::new(val.as_ref().clone()))
            }
            Self::ConstVar(val) | Self::VarVar(val) => val.clone(),
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
        let (Self::ConstVar(val) | Self::VarVar(val)) = self else {
            return
        };
        let value = val.borrow().clone() + rhs.clone_inner();
        *val.borrow_mut() = value;
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
        let (Self::ConstVar(val) | Self::VarVar(val)) = self else {
            return
        };
        *val.borrow_mut() = val.borrow().clone() - rhs.clone_inner();
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
        let (Self::ConstVar(val) | Self::VarVar(val)) = self else {
            return
        };
        *val.borrow_mut() = val.borrow().clone() * rhs.clone_inner();
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
        let (Self::ConstVar(val) | Self::VarVar(val)) = self else {
            return
        };
        *val.borrow_mut() = val.borrow().clone() / rhs.clone_inner();
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
        let (Self::ConstVar(val) | Self::VarVar(val)) = self else {
            return
        };
        *val.borrow_mut() = val.borrow().clone() % rhs.clone_inner();
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
