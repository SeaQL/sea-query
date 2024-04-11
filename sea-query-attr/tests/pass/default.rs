use sea_query::Iden;
use sea_query_attr::enum_def;

#[enum_def]
pub struct Hello {
    pub name: String,
}

fn main() {
    assert_eq!(HelloIden::Table.to_string(), "hello".to_string());
    println!("{:?}", HelloIden::Name);
}
