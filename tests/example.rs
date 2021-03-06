#[macro_use] extern crate hexf;

use std::f64;

#[test]
fn basic() {
    assert_eq!(hexf32!("0x1.99999ap-4"), 0.1f32);
    assert_eq!(hexf64!("0x1.999999999999ap-4"), 0.1f64);
    assert_eq!(hexf64!("0x1.999999999998ap-4"), 0.1f64 - f64::EPSILON);
}

#[test]
fn zeroes() {
    assert_eq!(1.0f64 / hexf64!("0x0.0p0"), f64::INFINITY);
    assert_eq!(1.0f64 / hexf64!("-0x0.0p0"), f64::NEG_INFINITY);
}

