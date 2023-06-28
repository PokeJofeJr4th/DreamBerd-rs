use std::{collections::HashMap, rc::Rc};

use lazy_regex::regex;

use crate::types::prelude::*;

pub struct GlobalThis(HashMap<String, Pointer>);

impl GlobalThis {
    pub fn get(&mut self, key: &str) -> Option<Pointer> {
        if let Some(val) = self.0.get(key) {
            Some(val.clone())
        } else if let Ok(val) = key.parse() {
            let new_val = Pointer::Const(Rc::new(Value::Number(val)));
            self.0.insert(String::from(key), new_val.clone());
            Some(new_val)
        } else if regex!("^f?u?n?c?t?i?o?n?$").is_match(key) {
            Some(Pointer::Const(Rc::new(Value::Keyword(Keyword::Function))))
        } else if key == "delete" {
            Some(Pointer::Const(Rc::new(Value::Keyword(Keyword::Delete))))
        } else if key == "const" {
            Some(Pointer::Const(Rc::new(Value::Keyword(Keyword::Const))))
        } else if key == "var" {
            Some(Pointer::Const(Rc::new(Value::Keyword(Keyword::Var))))
        } else if key == "if" {
            Some(Pointer::Const(Rc::new(Value::Keyword(Keyword::If))))
        } else {
            None
        }
    }
}
