use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

use super::{Pointer, Syntax};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Boolean {
    True,
    False,
    Maybe,
}

#[derive(PartialEq, Debug, Clone)]
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
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Boolean(bool) => bool.hash(state),
            Self::String(str) => str.hash(state),
            Self::Number(float) => (*float).to_bits().hash(state),
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

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(if value { Boolean::True } else { Boolean::False })
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(lhs), Self::Number(rhs)) => Self::Number(lhs + rhs),
            (Self::String(lhs), Self::String(rhs)) => Self::String(lhs + &rhs),
            _ => Self::Undefined,
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(lhs), Self::Number(rhs)) => Self::Number(lhs - rhs),
            _ => Self::Undefined,
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(lhs), Self::Number(rhs)) => Self::Number(lhs * rhs),
            _ => Self::Undefined,
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(lhs), Self::Number(rhs)) => {
                if rhs == 0.0 {
                    Self::Undefined
                } else {
                    Self::Number(lhs / rhs)
                }
            }
            _ => Self::Undefined,
        }
    }
}

impl Value {
    pub fn eq(&self, rhs: Self, precision: u8) -> Self {
        match (self, rhs) {
            (&Self::Number(lhs), Self::Number(rhs)) => {
                Self::from(lhs == rhs || (precision == 1 && (lhs / rhs).ln().abs() < 0.1))
            }
            (Self::String(lhs), Self::String(rhs)) => {
                // println!("{lhs} =? {rhs}");
                Self::from(if precision == 1 {
                    lhs.to_lowercase().trim() == rhs.to_lowercase().trim()
                } else {
                    *lhs == rhs
                })
            }
            _ => Self::Boolean(Boolean::Maybe),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Keyword {
    Const,
    Var,
    Delete,
    Function,
    If,
}
