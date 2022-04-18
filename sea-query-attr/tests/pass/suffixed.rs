use sea_query::gen_type_def;

#[gen_type_def(prefix = "", suffix = "Def")]
pub struct Hello {
    pub name: String
}

fn main() {
    println!("{:?}", HelloDef::Name);
}