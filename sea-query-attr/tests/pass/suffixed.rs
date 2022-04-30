use sea_query_attr::enum_def;

#[enum_def(prefix = "", suffix = "Def")]
pub struct Hello {
    pub name: String
}

fn main() {
    println!("{:?}", HelloDef::Name);
}