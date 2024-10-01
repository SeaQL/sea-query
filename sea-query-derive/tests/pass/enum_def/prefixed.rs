use sea_query_derive::enum_def;

#[enum_def(prefix = "Enum", suffix = "")]
pub struct Hello {
    pub name: String
}

fn main() {
    assert_eq!(format!("{:?}", EnumHello::Name), "Name");
}