use sea_query::{Iden, IdenStatic};

#[derive(Copy, Clone, IdenStatic)]
pub struct SomeType;

fn main() {
    assert_eq!(SomeType.to_string(), "some_type");
    assert_eq!(SomeType.as_str(), "some_type");
}
