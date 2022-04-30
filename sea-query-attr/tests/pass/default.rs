use sea_query::enum_def;

#[enum_def]
pub struct Hello {
    pub name: String
}

fn main() {
    println!("{:?}", HelloIden::Name);
}