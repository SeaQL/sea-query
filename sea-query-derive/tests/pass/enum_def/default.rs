use sea_query_derive::enum_def;

#[enum_def]
pub struct Hello {
    pub name: String
}

fn main() {
    assert_eq!(format!("{:?}", HelloIden::Name), "Name");
}