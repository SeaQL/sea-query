use sea_query::Iden;
use sea_query_attr::enum_def;

#[enum_def(table_iden = "HelloTable")]
pub struct Hello {
    pub name: String,
}

fn main() {
    println!("{:?}", HelloIden::Table.to_string());
}
