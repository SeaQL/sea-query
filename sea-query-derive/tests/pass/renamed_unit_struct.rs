use sea_query::Iden;

#[derive(Copy, Clone, Iden)]
#[iden = "another_name"]
pub struct CustomName;

fn main() {
    assert_eq!(CustomName.to_string(), "another_name");
}
