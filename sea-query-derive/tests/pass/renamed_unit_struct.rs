use sea_query::Iden;

#[derive(Iden)]
#[iden = "another_name"]
pub struct CustomName;

fn main() {
    assert_eq!(Iden::to_string(&CustomName), "another_name")
}
