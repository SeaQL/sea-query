use sea_query::gen_type_def;

#[gen_type_def(prefix = "Enum", suffix = "")]
pub struct Hello {
    pub name: String
}

fn main() {
    println!("{:?}", EnumHello::Name);
}