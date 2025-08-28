use super::{Value, ValueType};
use std::fmt::Debug;

fn test_serde_round_trip<V: Debug + Clone + PartialEq + Into<Value> + ValueType>(v: V) {
    // v: original value
    let vv: Value = v.clone().into(); // wrapped with Value
    let vvv: Value = serde_json::from_value(serde_json::to_value(&vv).unwrap()).unwrap(); // round tripped Value
    assert_eq!(vv, vvv);
    let vvvv: V = vvv.unwrap(); // round tripped primitive
    assert_eq!(v, vvvv);
}

#[test]
fn test_serde() {
    test_serde_round_trip(12345678i32);
    test_serde_round_trip(1234.5678f64);
    test_serde_round_trip("hello".to_string());
}
