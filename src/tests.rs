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

macro_rules! assert_eq_db {
    ($lhs: expr, $rhs: expr) => {
        assert_eq!(eval($lhs), eval($rhs))
    };
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
    assert_eq_db!("true | true", "true");
    assert_eq_db!("true | true", "true");
    assert_eq_db!("true | false", "true");
    assert_eq_db!("false | false", "false");
    assert_eq_db!("maybe | false", "maybe");
    assert_eq_db!("maybe | true", "true");
    assert_eq_db!("maybe | maybe", "maybe");
    assert_eq_db!("true & true", "true");
    assert_eq_db!("true & false", "false");
    assert_eq_db!("false & false", "false");
    assert_eq_db!("maybe & false", "false");
    assert_eq_db!("maybe & true", "maybe");
    assert_eq_db!("maybe & maybe", "maybe");
}

#[test]
fn comparisons() {
    assert_eq_db!("`true` === true", "true");
    assert_eq_db!("`true` ==== true", "false");
    assert_eq_db!("`false` === true", "false");
    assert_eq_db!("` TRUE\n\t ` == true", "true");
    assert_eq_db!("` TRUE ` === true", "false");

    assert_eq_db!("`` == 0", "true");
    assert_eq_db!("`` === 0", "false");
    assert_eq_db!("0 == ``", "true");
    assert_eq_db!("0 === `0`", "true");
    // assert_eq_db!("0 == `Zero`", "true");
    assert_eq_db!("0 === `Zero`", "false");
    assert_eq_db!("22/7 == 🥧", "true");
}

#[test]
fn op_assign() {
    assert_eq_db!("{const var count = 0! count += 1! count}", "1");
    assert_eq_db!(
        "{const var msg = 'hello'! msg += 'world'! msg}",
        "`helloworld`"
    );
    assert_eq_db!(
        "{const var msg = 'i did '! msg += -msg! msg}",
        "`i did  did i`"
    );
}

#[test]
fn function() {
    assert_eq!(
        eval("const const does_she_really_like_you = () -> maybe! does_she_really_like_you"),
        Ok(Value::Function(Vec::new(), Syntax::Ident("maybe".into())))
    );
}

#[test]
fn doc_tests() {
    assert_eq_db!(";'hello there'", "'ereht olleh'");
    assert_eq_db!("-true", "false");
    assert_eq_db!(";1", "-1");
    assert_eq_db!("const var age = 1! age += 1! age", "2");
    assert_eq_db!("var const id = 'name'! id = 'main'! id", "'main'");
    assert_eq_db!("var var count = 0! count += 1! count = 2! count", "2");
    assert_eq_db!("const const 5 = 4! 2 + 2  ====  5", "true");
    assert_eq_db!("const const true = false! true === false", "true");
    assert_eq_db!("1 + 2*3", "7");
    assert_eq_db!("1+2 * 3", "9");
    assert_eq_db!("`he` + `l`*2 + `o ` + ;`dlrow`", "`hello world`");
    assert_eq_db!("`johnny` * 1.5", "`johnnyjoh`");
    assert_eq_db!("`no lemon ` + -`no lemon`", "`no lemon nomel on`");
}
