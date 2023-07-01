use crate::types::prelude::*;

use std::{f64::consts as f64, fmt::Display};

fn eval<T: Display>(src: T) -> SResult<Value> {
    Ok(
        crate::interpreter::interpret(&crate::parser::parse(crate::lexer::tokenize(&format!(
            "{{{src}}}"
        ))?)?)?
        .clone_inner(),
    )
}

#[test]
fn divide() {
    assert_eq!(Value::from(1.0) / Value::from(0.0), Value::Undefined);
    assert_eq!(Value::from(5.0) / Value::from(2.0), Value::from(2.5));
}

#[test]
fn eq() {
    assert_eq!(
        Value::from(f64::PI).eq(Value::from(3.0), 1),
        Value::from(true)
    );
    assert_eq!(
        Value::from(f64::PI).eq(Value::from(3.0), 2),
        Value::from(false)
    );
}

#[test]
fn maybe_or_and() {
    assert_eq!(eval("(true | true)"), Ok(Value::from(true)));
    assert_eq!(eval("(true | false)"), Ok(Value::from(true)));
    assert_eq!(eval("(false | false)"), Ok(Value::from(false)));
    assert_eq!(eval("(maybe | false)"), Ok(Value::Boolean(Boolean::Maybe)));
    assert_eq!(eval("(maybe | true)"), Ok(Value::from(true)));
    assert_eq!(eval("(maybe | maybe)"), Ok(Value::Boolean(Boolean::Maybe)));
    assert_eq!(eval("(true & true)"), Ok(Value::from(true)));
    assert_eq!(eval("(true & false)"), Ok(Value::from(false)));
    assert_eq!(eval("(false & false)"), Ok(Value::from(false)));
    assert_eq!(eval("(maybe & false)"), Ok(Value::from(false)));
    assert_eq!(eval("(maybe & true)"), Ok(Value::Boolean(Boolean::Maybe)));
    assert_eq!(eval("(maybe & maybe)"), Ok(Value::Boolean(Boolean::Maybe)));
}

#[test]
fn comparisons() {
    assert_eq!(eval("`true` === true"), Ok(Value::from(true)));
    assert_eq!(eval("`true` ==== true"), Ok(Value::from(false)));
    assert_eq!(eval("`false` === true"), Ok(Value::from(false)));
    assert_eq!(eval("` TRUE\n\t ` == true"), Ok(Value::from(true)));
    assert_eq!(eval("` TRUE ` === true"), Ok(Value::from(false)));
}

#[test]
fn op_assign() {
    assert_eq!(
        eval("{const var count = 0! count += 1! count}"),
        Ok(Value::Number(1.0))
    );
    assert_eq!(
        eval("{const var msg = 'hello'! msg += 'world'! msg}"),
        Ok(Value::String(String::from("helloworld")))
    );
    assert_eq!(
        eval("{const var msg = 'i did '! msg += -msg! msg}"),
        Ok(Value::String(String::from("i did  did i")))
    );
}

#[test]
fn function() {
    assert_eq!(
        eval("const const does_she_really_like_you = () -> maybe! does_she_really_like_you"),
        Ok(Value::Function(Vec::new(), Syntax::Ident("maybe".into())))
    );
}
