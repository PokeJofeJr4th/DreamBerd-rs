use crate::types::prelude::*;

use std::f64::consts as f64;

fn eval(src: &str) -> SResult<Value> {
    Ok(
        crate::interpreter::interpret(&crate::parser::parse(crate::lexer::tokenize(src)?)?)?
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
    assert_eq!(eval("(true | true)"), Ok(Value::Boolean(Boolean::True)));
    assert_eq!(eval("(true | false)"), Ok(Value::Boolean(Boolean::True)));
    assert_eq!(eval("(false | false)"), Ok(Value::Boolean(Boolean::False)));
    assert_eq!(eval("(maybe | false)"), Ok(Value::Boolean(Boolean::Maybe)));
    assert_eq!(eval("(maybe | true)"), Ok(Value::Boolean(Boolean::True)));
    assert_eq!(eval("(maybe | maybe)"), Ok(Value::Boolean(Boolean::Maybe)));
    assert_eq!(eval("(true & true)"), Ok(Value::Boolean(Boolean::True)));
    assert_eq!(eval("(true & false)"), Ok(Value::Boolean(Boolean::False)));
    assert_eq!(eval("(false & false)"), Ok(Value::Boolean(Boolean::False)));
    assert_eq!(eval("(maybe & false)"), Ok(Value::Boolean(Boolean::False)));
    assert_eq!(eval("(maybe & true)"), Ok(Value::Boolean(Boolean::Maybe)));
    assert_eq!(eval("(maybe & maybe)"), Ok(Value::Boolean(Boolean::Maybe)));
}
