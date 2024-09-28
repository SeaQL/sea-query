use sea_query_derive::enum_def;

#[enum_def(unknown_field)]
pub struct Hello {
    pub name: String,
}

fn main() {}
