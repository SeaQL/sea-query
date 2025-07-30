use sea_query::Value;

/// If this check fails, you need to run `benches/value.rs`
/// and see if the increased `Value` size hurts performance.
///
/// If it does, box the big variant.
///
/// If it doesn't, bump the constant here in the test.
#[test]
fn value_size() {
    let max = 40;
    if size_of::<Value>() > max {
        panic!("the size of Value shouldn't be greater than {max} bytes");
    }
}
