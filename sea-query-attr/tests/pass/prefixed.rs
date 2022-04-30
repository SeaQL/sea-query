use sea_query::enum_def;

#[enum_def(prefix = "Enum", suffix = "")]
pub struct Hello {
    pub name: String
}

fn main() {
    println!("{:?}", EnumHello::Name);
}