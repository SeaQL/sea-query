use sea_query::Iden;

#[derive(Copy, Clone, Iden)]
pub struct SomeType;

fn main() {
    assert_eq!(SomeType.to_string(), "some_type");
}
