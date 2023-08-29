use core::hash::Hash;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, BitAnd, BitOr, DivAssign, MulAssign, Neg, Rem, RemAssign, SubAssign};
use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use super::prelude::*;

/// A pointer to a reference-counted value
/// A `const const` and `var const` can point to the same value, as can a `const var` and `var var`.
#[derive(PartialEq, Eq, Clone)]
pub enum Pointer {
    ConstConst(Rc<Value>),
    ConstVar(RcMut<Value>),
    VarConst(RcMut<Rc<Value>>),
    VarVar(RcMut<RcMut<Value>>),
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
            VarType::VarConst => Self::VarConst(rc_mut_new(self.as_const())),
            VarType::VarVar => Self::VarVar(rc_mut_new(self.as_var())),
        }
    }

    // /// Make a new pointer from a value with a given type
    // pub fn from_value(val: Value, vt: VarType) -> Self {
    //     match vt {
    //         VarType::ConstConst => Self::ConstConst(Rc::new(val)),
    //         VarType::ConstVar => Self::ConstVar(rc_mut_new(val)),
    //         VarType::VarConst => Self::VarConst(rc_mut_new(Rc::new(val))),
    //         VarType::VarVar => Self::VarVar(rc_mut_new(rc_mut_new(val))),
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
            Self::from(self.with_refs(rhs, |val, rhs| val.eq(rhs, precision)))
        }
    }

    /// Apply the dot operator. This has two valid cases: float parsing and object indexing. Otherwise, it returns `undefined`
    #[allow(clippy::option_if_let_else, clippy::single_match_else)]
    pub fn dot(&self, rhs: &Value) -> SResult<Self> {
        let allow_modify = matches!(self, Self::ConstVar(_) | Self::VarVar(_));
        let lhs = self.clone_inner();
        match (lhs, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Ok(Self::from(format!("{lhs}.{rhs}").parse::<f64>().map_err(
                    |e| format!("Error parsing `{lhs}.{rhs}`: {e}"),
                )?))
            }
            (Value::Object(mut obj), key) => match obj.get(key) {
                Some(ptr) => Ok(ptr.clone()),
                None => {
                    let val = if allow_modify {
                        Self::ConstVar(rc_mut_new(Value::empty_object()))
                    } else {
                        Self::ConstConst(Rc::new(Value::empty_object()))
                    };
                    obj.insert(key.clone(), val.clone());
                    Ok(val)
                }
            },
            _ => Ok(Self::from(Value::empty_object())),
        }
    }

    /// Try to replace the current value with given value. Returns `Err` if `self` is ptr-const. Doesn't clone if it's not necessary.
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

    /// Convert to an immutable reference to a value. If `self` is value-var, clones the internal value.
    ///
    /// If you want to use a reference to the value inside of a function, look into `Pointer::with_ref`.
    pub fn as_const(&self) -> Rc<Value> {
        match self {
            Self::ConstConst(val) => val.clone(),
            Self::VarConst(val) => val.borrow().clone(),
            Self::ConstVar(val) => Rc::new(val.borrow().clone()),
            Self::VarVar(val) => Rc::new(val.borrow().borrow().clone()),
        }
    }

    /// Convert to a mutable reference to a value. If `self` is value-const, clones the internal value
    pub fn as_var(&self) -> RcMut<Value> {
        match self {
            Self::ConstConst(val) => rc_mut_new(val.as_ref().clone()),
            Self::VarConst(val) => rc_mut_new(val.borrow().as_ref().clone()),
            Self::ConstVar(val) => val.clone(),
            Self::VarVar(val) => val.borrow().clone(),
        }
    }

    /// Run a function on a reference to the internal value. This does not clone the internal value.
    pub fn with_ref<T, F: FnOnce(&Value) -> T>(&self, func: F) -> T {
        match self {
            Self::ConstConst(val) => func(val.as_ref()),
            Self::VarConst(val) => func(val.borrow().as_ref()),
            Self::ConstVar(val) => func(&val.borrow()),
            Self::VarVar(val) => func(&val.borrow().borrow()),
        }
    }

    /// Run a function on a pair of references to two different Pointers.
    pub fn with_refs<T, F: FnOnce(&Value, &Value) -> T>(&self, other: &Self, func: F) -> T {
        self.with_ref(|val| other.with_ref(|other| func(val, other)))
    }
}

impl PartialEq<Value> for Pointer {
    fn eq(&self, other: &Value) -> bool {
        self.with_ref(|r| r == other)
    }
}

impl PartialOrd for Pointer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.with_refs(other, std::cmp::PartialOrd::partial_cmp)
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
        Self::ConstConst(Rc::new(Value::String(value.into())))
    }
}

impl From<Rc<str>> for Pointer {
    fn from(value: Rc<str>) -> Self {
        Self::ConstConst(Rc::new(Value::String(value)))
    }
}

impl From<f64> for Pointer {
    fn from(value: f64) -> Self {
        Self::ConstConst(Rc::new(Value::Number(value)))
    }
}
