#[test]
fn build_tests() {
    let t = trybuild::TestCases::new();
    //t.compile_fail("./tests/compile-fail/*.rs");

    // all of these are exactly the same as the examples in `examples/derive.rs`
    t.pass("./tests/pass/*.rs");
}
