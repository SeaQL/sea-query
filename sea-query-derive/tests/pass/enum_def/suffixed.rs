use sea_query_derive::enum_def;

#[enum_def(prefix = "", suffix = "Def")]
pub struct Hello {
    pub name: String
}

fn main() {
    assert_eq!(format!("{:?}", HelloDef::Name), "Name");
}