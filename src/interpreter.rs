use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lazy_regex::regex;

use crate::types::prelude::*;

#[derive(Debug)]
struct State {
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

pub fn interpret(src: &Syntax) -> SResult<Pointer> {
    inner_interpret(src, Rc::new(RefCell::new(State::new())))
}

fn inner_interpret(src: &Syntax, state: Rc<RefCell<State>>) -> SResult<Pointer> {
    match src {
        Syntax::Debug(content, level) => {
            if *level >= 3 {
                println!("{content:?}");
            }
            let evaluated = inner_interpret(content, state)?;
            if *level >= 2 {
                println!("{evaluated:?}");
            } else {
                println!("{evaluated}");
            }
            Ok(evaluated)
        }
        Syntax::Operation(lhs, op, rhs) => {
            let lhs_eval = inner_interpret(lhs, state.clone())?;
            let rhs_eval = inner_interpret(rhs, state)?;
            // println!("{lhs:?} op {rhs:?}");
            // println!("{lhs_eval:?} op {rhs_eval:?}");
            match op {
                Operation::Equal(precision) => Ok(lhs_eval.eq(&rhs_eval, *precision)),
                Operation::Add => Ok(lhs_eval + rhs_eval),
                Operation::Sub => Ok(lhs_eval - rhs_eval),
                Operation::Mul => Ok(lhs_eval * rhs_eval),
                Operation::Div => Ok(lhs_eval / rhs_eval),
                Operation::Dot => Ok(lhs_eval.dot(rhs_eval.clone_inner())?),
                _ => todo!(),
            }
        }
        Syntax::Block(statements) => {
            let state = Rc::new(RefCell::new(State::from_parent(state)));
            let mut iter = statements.iter();
            let Some(last) = iter.next_back() else {
                return Ok(Pointer::from(Value::Undefined))
            };
            for syn in iter {
                inner_interpret(syn, state.clone())?;
            }
            inner_interpret(last, state)
        }
        Syntax::Declare(var_type, ident, value) => {
            let val = inner_interpret(value, state.clone())?;
            state
                .borrow_mut()
                .insert(ident.clone(), val.convert(*var_type));
            // println!("{state:#?}");
            Ok(Pointer::from(Value::Undefined))
        }
        Syntax::String(str) => Ok(Pointer::from(str.as_ref())),
        Syntax::Function(func, args) => {
            let func = state.borrow_mut().get(func).clone_inner();
            // println!("{state:#?}");
            let result = match func {
                Value::Keyword(Keyword::If) => {
                    let [condition, body] = &args[..] else {
                        return Err(String::from("If statement requires two arguments: condition and body"))
                    };
                    let condition_evaluated = inner_interpret(condition, state.clone())?;
                    // println!("{condition_evaluated:?}");
                    if condition_evaluated == Value::from(true) {
                        return inner_interpret(body, state);
                    }
                    Value::Undefined
                }
                Value::Keyword(Keyword::Delete) => {
                    if let [Syntax::Ident(key)] = &args[..] {
                        state.borrow_mut().delete(key);
                    }
                    Value::Undefined
                }
                Value::Keyword(Keyword::Function) => {
                    let [Syntax::Ident(name), args, body] = &args[..] else {
                        return Err(format!("Invalid arguments for `Function`: `{args:?}`"))
                    };
                    let args = match args {
                        Syntax::Block(args) => args.clone(),
                        other => vec![other.clone()],
                    };
                    let args: Vec<String> = args
                        .into_iter()
                        .map(|syn| match syn {
                            Syntax::Ident(str) => Ok(str),
                            other => Err(format!("Invalid parameter name: `{other:?}`")),
                        })
                        .collect::<Result<_, _>>()?;
                    let inner_val = Value::Function(args, body.clone());
                    state
                        .borrow_mut()
                        .insert(name.clone(), Pointer::from(inner_val));
                    Value::Undefined
                }

                other => return Err(format!("`{other:?}` is not a function")),
            };
            Ok(Pointer::from(result))
        }
        Syntax::Ident(ident) => Ok(state.borrow_mut().get(ident)),
    }
}
