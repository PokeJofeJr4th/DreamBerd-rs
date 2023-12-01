use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use lazy_regex::regex;

use crate::types::prelude::*;

use core::f64::consts as f64;

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    current: HashMap<Rc<str>, (Pointer, Lifetime)>,
    parent: Option<RcMut<State>>,
    pub undefined: Pointer,
}

macro_rules! kw {
    ($current:ident $str:expr => $kw:expr) => {
        $current.insert(
            $str.into(),
            (Pointer::from(Value::from($kw)), Lifetime::Default),
        )
    };
}

impl State {
    pub fn new() -> Self {
        let mut current = HashMap::new();

        kw!(current "🥧" => f64::PI);
        kw!(current "class" => Keyword::Class);
        kw!(current "className" => Keyword::Class);
        kw!(current "const" => Keyword::Const);
        kw!(current "current" => Keyword::Current);
        kw!(current "delete" => Keyword::Delete);
        kw!(current "eval" => Keyword::Eval);
        kw!(current "false" => false);
        kw!(current "forget" => Keyword::Forget);
        kw!(current "if" => Keyword::If);
        kw!(current "infinity" => Value::Number(f64::INFINITY));
        kw!(current "maybe" => Boolean::Maybe);
        kw!(current "next" => Keyword::Next);
        kw!(current "new" => Keyword::New);
        kw!(current "previous" => Keyword::Previous);
        kw!(current "true" => true);
        kw!(current "var" => Keyword::Var);
        kw!(current "when" => Keyword::When);
        kw!(current "∞" => Value::Number(f64::INFINITY));

        let undefined = Pointer::ConstConst(Rc::new(Value::empty_object()));
        current.insert("undefined".into(), (undefined.clone(), Lifetime::Default));
        Self {
            current,
            parent: None,
            undefined,
        }
    }

    pub fn from_parent(parent: Rc<RefCell<Self>>) -> Self {
        let undefined = parent.borrow().undefined.clone();
        Self {
            current: HashMap::new(),
            undefined,
            parent: Some(parent),
        }
    }

    pub fn get(&mut self, key: Rc<str>) -> Pointer {
        // println!("{:?}: {key}", self.current);
        // if there's a value here, get it
        if let Some(val) = self.current.get(&key) {
            return val.0.clone();
        }
        // if there's a value in the parent, get it
        if let Some(parent) = &self.parent {
            return (**parent).borrow_mut().get(key);
        }
        // otherwise, parse it in global context
        if let Ok(val) = key.parse() {
            let new_val = Pointer::ConstConst(Rc::new(Value::Number(val)));
            self.current
                .insert(key, (new_val.clone(), Lifetime::Default));
            new_val
        } else if regex!("^f?u?n?c?t?i?o?n?$").is_match(&key) {
            let v = Pointer::ConstConst(Rc::new(Value::Keyword(Keyword::Function)));
            self.current.insert(key, (v.clone(), Lifetime::Default));
            v
        } else {
            let v = Pointer::ConstConst(Rc::new(Value::String(key.clone())));
            self.current.insert(key, (v.clone(), Lifetime::Default));
            v
        }
    }

    pub fn insert(&mut self, key: Rc<str>, value: Pointer, lifetime: Lifetime) {
        self.current.insert(key, (value, lifetime));
    }

    pub fn delete(&mut self, k: Rc<str>) {
        match self.current.entry(k.clone()) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                e.insert((self.undefined.clone(), Lifetime::Default));
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                if let Some(parent) = &self.parent {
                    parent.borrow_mut().delete(k);
                } else {
                    e.insert((self.undefined.clone(), Lifetime::Default));
                }
            }
        }
    }

    pub fn locals_to_object(&self) -> BTreeMap<Value, Pointer> {
        self.current
            .iter()
            .map(|(k, v)| (Value::String(k.clone()), v.0.clone()))
            .collect()
    }

    pub fn tick(&mut self) {
        let keys = self.current.keys().cloned().collect::<Vec<_>>();
        for k in keys {
            match self.current.get_mut(&k) {
                Some((_, Lifetime::Ticks(t @ ..=-1))) => {
                    *t += 1;
                }
                Some((_, Lifetime::Ticks(t @ 1..))) => {
                    *t -= 1;
                }
                Some((_, Lifetime::Ticks(0))) => {
                    self.delete(k);
                }
                _ => {}
            }
        }
        if let Some(parent) = &self.parent {
            parent.borrow_mut().tick();
        }
    }
}
