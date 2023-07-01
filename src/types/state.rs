use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lazy_regex::regex;

use crate::types::prelude::*;

#[derive(Debug, PartialEq)]
pub struct State {
    current: HashMap<String, Pointer>,
    parent: Option<Rc<RefCell<State>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current: HashMap::new(),
            parent: None,
        }
    }

    pub fn from_parent(parent: Rc<RefCell<Self>>) -> Self {
        Self {
            current: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&mut self, key: &str) -> Pointer {
        // println!("{:?}: {key}", self.current);
        // if there's a value here, get it
        if let Some(val) = self.current.get(key) {
            return val.clone();
        }
        // if there's a value in the parent, get it
        if let Some(parent) = &self.parent {
            return (**parent).borrow_mut().get(key);
        }
        // otherwise, parse it in global context
        if let Ok(val) = key.parse() {
            let new_val = Pointer::ConstConst(Rc::new(Value::Number(val)));
            self.current.insert(String::from(key), new_val.clone());
            new_val
        } else if regex!("^f?u?n?c?t?i?o?n?$").is_match(key) {
            let v = Pointer::ConstConst(Rc::new(Value::Keyword(Keyword::Function)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "delete" {
            let v = Pointer::ConstConst(Rc::new(Value::Keyword(Keyword::Delete)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "const" {
            let v = Pointer::ConstConst(Rc::new(Value::Keyword(Keyword::Const)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "var" {
            let v = Pointer::ConstConst(Rc::new(Value::Keyword(Keyword::Var)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "if" {
            let v = Pointer::ConstConst(Rc::new(Value::Keyword(Keyword::If)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "use" {
            let v = Pointer::ConstConst(Rc::new(Value::Keyword(Keyword::Use)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "true" {
            let v = Pointer::ConstConst(Rc::new(Value::from(true)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "false" {
            let v = Pointer::ConstConst(Rc::new(Value::from(false)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "maybe" {
            let v = Pointer::ConstConst(Rc::new(Value::Boolean(Boolean::Maybe)));
            self.current.insert(String::from(key), v.clone());
            v
        } else if key == "undefined" {
            let v = Pointer::ConstConst(Rc::new(Value::Undefined));
            self.current.insert(String::from(key), v.clone());
            v
        } else {
            let v = Pointer::ConstConst(Rc::new(Value::String(String::from(key))));
            self.current.insert(String::from(key), v.clone());
            v
        }
    }

    pub fn insert(&mut self, k: String, v: Pointer) {
        self.current.insert(k, v);
    }

    pub fn delete(&mut self, k: &str) {
        if self.current.contains_key(k) {
            self.current
                .insert(String::from(k), Pointer::from(Value::Undefined));
        }
        if let Some(parent) = &self.parent {
            parent.borrow_mut().delete(k);
        }
    }
}
