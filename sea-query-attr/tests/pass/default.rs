use sea_query::gen_type_def;

#[gen_type_def]
pub struct Hello {
    pub name: String
}

fn main() {
    println!("{:?}", HelloTypeDef::Name);
}