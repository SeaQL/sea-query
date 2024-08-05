use sea_query_attr::enum_def;

#[enum_def]
pub struct Hello {
    pub r#type: String
}

fn main() {
    println!("{:?}", HelloIden::Type);
}