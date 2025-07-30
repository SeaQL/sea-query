use sea_query::Value;

/// This test is to check if the size of [`Value`] exceeds the limit.
/// If the size exceeds the limit, you should try boxing the variant, update and run the benchmark of [`Value`] to compare performance.
///
/// If the boxed variant causes a greater performance loss, update the size limit instead.
#[test]
fn value_size() {
    let max = 40;
    if size_of::<Value>() > max {
        panic!("the size of Value shouldn't be greater than {max} bytes");
    }
}
