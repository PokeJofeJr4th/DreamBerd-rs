use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::types::prelude::*;

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
        Syntax::Negate(content) => {
            let evaluated = inner_interpret(content, state)?;
            Ok(-evaluated)
        }
        Syntax::Operation(lhs, op, rhs) => {
            let mut lhs_eval = inner_interpret(lhs, state.clone())?;
            let rhs_eval = inner_interpret(rhs, state)?;
            // println!("{lhs:?} op {rhs:?}");
            // println!("{lhs_eval:?} op {rhs_eval:?}");
            match op {
                Operation::Equal(1) => {
                    lhs_eval.assign(&rhs_eval)?;
                    Ok(rhs_eval)
                }
                Operation::Equal(precision) => Ok(lhs_eval.eq(&rhs_eval, *precision - 1)),
                Operation::Add => Ok(lhs_eval + rhs_eval),
                Operation::Sub => Ok(lhs_eval - rhs_eval),
                Operation::Mul => Ok(lhs_eval * rhs_eval),
                Operation::Div => Ok(lhs_eval / rhs_eval),
                Operation::Mod => Ok(lhs_eval % rhs_eval),
                Operation::Dot => Ok(lhs_eval.dot(rhs_eval.clone_inner())?),
                Operation::And => Ok(lhs_eval & rhs_eval),
                Operation::Or => Ok(lhs_eval | rhs_eval),
                Operation::AddEq => {
                    lhs_eval += rhs_eval;
                    Ok(lhs_eval)
                }
                Operation::SubEq => {
                    lhs_eval -= rhs_eval;
                    Ok(lhs_eval)
                }
                Operation::MulEq => {
                    lhs_eval *= rhs_eval;
                    Ok(lhs_eval)
                }
                Operation::DivEq => {
                    lhs_eval /= rhs_eval;
                    Ok(lhs_eval)
                }
                Operation::ModEq => {
                    lhs_eval %= rhs_eval;
                    Ok(lhs_eval)
                }
                Operation::Ls => Ok(Pointer::from(lhs_eval < rhs_eval)),
                Operation::LsEq => Ok(Pointer::from(lhs_eval <= rhs_eval)),
                Operation::Gr => Ok(Pointer::from(lhs_eval > rhs_eval)),
                Operation::GrEq => Ok(Pointer::from(lhs_eval >= rhs_eval)),
                Operation::Arrow => todo!(),
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
            let res = inner_interpret(last, state)?;
            if res == Value::Undefined {
                Ok(Pointer::from(Value::empty_object()))
            } else {
                Ok(res)
            }
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
        Syntax::Call(func, args) => {
            let func = state.borrow_mut().get(func).clone_inner();
            interpret_function(func, args, state)
        }
        Syntax::Ident(ident) => Ok(state.borrow_mut().get(ident)),
        Syntax::Function(args, body) => {
            Ok(Pointer::from(Value::Function(args.clone(), *body.clone())))
        }
    }
}

fn interpret_function(func: Value, args: &[Syntax], state: Rc<RefCell<State>>) -> SResult<Pointer> {
    match func {
        Value::Keyword(Keyword::If) => {
            let [condition, body] = args else {
                    return Err(String::from("If statement requires two arguments: condition and body"))
                };
            let condition_evaluated = inner_interpret(condition, state.clone())?;
            // println!("{condition_evaluated:?}");
            if condition_evaluated == Value::from(true) {
                inner_interpret(body, state)
            } else {
                Ok(Value::Undefined.into())
            }
        }
        Value::Keyword(Keyword::Delete) => {
            if let [Syntax::Ident(key)] = args {
                state.borrow_mut().delete(key);
            }
            Ok(Value::Undefined.into())
        }
        Value::Keyword(Keyword::Function) => {
            let [Syntax::Ident(name), args, body] = args else {
                    return Err(format!("Invalid arguments for `function`: `{args:?}`; expected name, args, and body"))
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
            Ok(Value::Undefined.into())
        }
        Value::Keyword(Keyword::Use) => {
            let [value] = args else {
                return Err(String::from("Invalid arguments for `use`; expected value"))
            };
            let evaluated = inner_interpret(value, state)?;
            Ok(Value::Object(BTreeMap::from([
                ("value".into(), evaluated),
                ("call".into(), Value::Keyword(Keyword::UseInner).into()),
            ]))
            .into())
        }
        Value::Keyword(Keyword::UseInner) => {
            if args.is_empty() {
                Ok(state.borrow_mut().get("value"))
            } else if let [value] = args {
                // set the value
                todo!()
            } else {
                Err(format!("A hook takes 0 or 1 arguments; got `{args:?}`"))
            }
        }
        Value::Object(obj) => {
            let Some(call) = obj.get(&"call".into()) else {
                return Err(format!("`Object({obj:?})` is not a function"))
            };
            interpret_function(call.clone_inner(), args, state)
        }
        Value::Function(fn_args, body) => {
            let mut inner_state = State::from_parent(state.clone());
            for (idx, ident) in fn_args.into_iter().enumerate() {
                let arg_eval = if let Some(syn) = args.get(idx) {
                    inner_interpret(syn, state.clone())?
                } else {
                    Pointer::from(Value::Undefined)
                };
                inner_state.insert(ident, arg_eval);
            }
            inner_interpret(&body, Rc::new(RefCell::new(inner_state)))
        }

        other => Err(format!("`{other:?}` is not a function")),
    }
}
