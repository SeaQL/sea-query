use sea_query::Iden;

#[derive(Clone, Iden)]
pub struct SomeType;

fn main() {
    assert_eq!(Iden::to_string(&SomeType), "some_type")
}
