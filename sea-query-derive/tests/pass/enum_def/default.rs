use sea_query_attr::enum_def;

#[enum_def]
pub struct Hello {
    pub name: String
}

fn main() {
    println!("{:?}", HelloIden::Name);
}