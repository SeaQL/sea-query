use sea_query_attr::enum_def;

#[enum_def(unknown_field)]
pub struct Hello {
    pub name: String,
}

fn main() {}
