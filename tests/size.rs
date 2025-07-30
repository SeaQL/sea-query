use sea_query::Value;

#[test]
fn value_size() {
    let max = 40;
    if size_of::<Value>() > max {
        panic!("the size of Value shouldn't be greater than {max} bytes");
    }
}
