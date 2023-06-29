use crate::types::Value;

use std::f64::consts as f64;

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
