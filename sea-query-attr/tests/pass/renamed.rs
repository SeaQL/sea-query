use sea_query::Iden;
use sea_query_attr::enum_def;

#[enum_def(rename = "users")]
pub struct User {
    pub name: String,
}

fn main() {
    assert_eq!(UserIden::Table.to_string(), "users".to_string());
}
